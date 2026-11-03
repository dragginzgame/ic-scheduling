//! Interactive host client for the real qualification canister in local PocketIC.
use clap::{CommandFactory, Parser, Subcommand};
use ic_jobs_canister_tests::harness;
use ic_jobs_test_consumer::{
    api::{Create, Error as ApiError, Init, Status, Summary, Task, Timing, View},
    management_wire::Method,
};
use ic_platform_pocket_ic::{PocketIc, PocketIcBuilder};
use ic_platform_types::principal::ExternalPrincipal;
use ic_testkit::pic::{
    CanisterInstallExt, PocketIcInstance, PocketIcStartupConfig, advance_and_tick,
};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::BTreeMap,
    io::{self, BufRead, IsTerminal, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);
const DURATION_UNITS: [(char, u64); 5] = [
    ('s', 1),
    ('m', 60),
    ('h', 3600),
    ('d', 86400),
    ('w', 604800),
];

fn duration(value: &str) -> Result<Duration, String> {
    let (unit, factor) = DURATION_UNITS
        .into_iter()
        .find(|(unit, _)| value.ends_with(*unit))
        .ok_or("use a whole duration such as 5s, 2m, 3h, 4d or 1w")?;
    let count: u64 = value[..value.len() - unit.len_utf8()]
        .parse()
        .map_err(|_| "duration must be a nonnegative whole number followed by s, m, h, d or w")?;
    let nanos = count
        .checked_mul(factor)
        .and_then(|seconds| seconds.checked_mul(1_000_000_000))
        .ok_or("duration exceeds the canister timestamp range")?;
    Ok(Duration::from_nanos(nanos))
}

fn positive_duration(value: &str) -> Result<Duration, String> {
    let parsed = duration(value)?;
    if parsed.is_zero() {
        return Err("duration must be greater than zero".into());
    }
    Ok(parsed)
}

#[derive(Parser)]
#[command(
    name = "queue",
    no_binary_name = true,
    subcommand_required = true,
    about = "Manage jobs in the local canister. Durations: 5s, 2m, 3h, 4d, 1w.",
    after_help = "Clock time advances only with advance or watch. Names are single words.\nTry: sample; watch 6s; advance 1m; advance 1w (one command per line)."
)]
struct Prompt {
    #[command(subcommand)]
    command: QueueCommand,
}

#[derive(Subcommand)]
enum QueueCommand {
    /// Queue jobs for 5s, 1m and every 1w
    Sample,
    /// Queue a counter increment in the canister
    Add {
        #[arg(value_name = "name")]
        name: String,
        #[arg(long, value_parser = duration, value_name = "after")]
        after: Duration,
        #[arg(long, value_parser = positive_duration, value_name = "every")]
        every: Option<Duration>,
        #[arg(long, requires = "every")]
        catch_up: bool,
    },
    /// Read the current schedule and execution counts
    List,
    /// Read committed job-outcome observations for this canister/version/window
    Metrics,
    /// Read up to eight recent committed transitions; follow next_before for older events
    Logs {
        #[arg(long)]
        before: Option<u64>,
    },
    /// Clear diagnostic counters without changing jobs or retry state
    ResetMetrics,
    /// Fast-forward IC time once, then deliver 32 canister rounds
    Advance {
        #[arg(value_parser = duration, value_name = "duration")]
        duration: Duration,
    },
    /// Advance one IC second per display, waiting between displays
    Watch {
        #[arg(value_parser = positive_duration, value_name = "duration")]
        duration: Duration,
    },
    Pause {
        #[arg(value_name = "id")]
        id: u64,
    },
    Resume {
        #[arg(value_name = "id")]
        id: u64,
    },
    Cancel {
        #[arg(value_name = "id")]
        id: u64,
    },
    /// End this temporary canister session
    #[command(alias = "exit")]
    Quit,
}

#[derive(Parser)]
#[command(
    name = "queue-demo",
    about = "Watch real Jobs canister scheduling in local PocketIC",
    version
)]
struct Options {
    #[arg(long, value_name = "server")]
    server: PathBuf,
    #[arg(long, value_name = "wasm")]
    wasm: PathBuf,
}

fn failure(error: impl std::fmt::Debug) -> io::Error {
    io::Error::other(format!("{error:?}"))
}

fn span(nanos: u64) -> String {
    let mut seconds = nanos.div_ceil(1_000_000_000);
    let mut parts = vec![];
    for (unit, factor) in DURATION_UNITS.into_iter().rev() {
        let count = seconds / factor;
        if count != 0 {
            parts.push(format!("{count}{unit}"));
            seconds %= factor;
        }
    }
    if parts.is_empty() {
        "0s".into()
    } else {
        parts.join(" ")
    }
}

fn deadline(due_ns: u64, now_ns: u64, overdue: &str) -> String {
    if due_ns <= now_ns {
        overdue.into()
    } else {
        format!("in {}", span(due_ns - now_ns))
    }
}

struct Demo {
    pic: PocketIcInstance<PocketIc>,
    canister: ExternalPrincipal,
    manager: ExternalPrincipal,
    origin_ns: u64,
    // Display labels only; deadlines, state and execution counts come from RPCs.
    labels: BTreeMap<u64, (String, String)>,
}

impl Demo {
    fn new(wasm: &Path, config: PocketIcStartupConfig) -> io::Result<Self> {
        let pic = config
            .build_instance(PocketIcBuilder::new().with_application_subnet())
            .map_err(failure)?;
        let manager = ExternalPrincipal::self_authenticating(b"jobs-queue-demo-manager");
        let canister = pic
            .instance()
            .try_create_and_install(
                harness::install_spec(
                    std::fs::read(wasm)?,
                    &Init {
                        manager: manager.into(),
                        delivery_sender: None,
                    },
                    "jobs-queue-demo",
                )
                .map_err(failure)?,
            )
            .map_err(failure)?;
        let origin_ns = pic
            .instance()
            .get_time()
            .map_err(failure)?
            .as_nanos_since_unix_epoch();
        println!(
            "Connected to local canister {canister} (PocketIC).\n\
            Each job increments the canister counter. This session starts fresh.\n\
            Use advance/watch to move IC time; waiting at the prompt does not move it."
        );
        Ok(Self {
            pic,
            canister,
            manager,
            origin_ns,
            labels: BTreeMap::new(),
        })
    }

    fn now(&self) -> io::Result<u64> {
        Ok(self
            .pic
            .instance()
            .get_time()
            .map_err(failure)?
            .as_nanos_since_unix_epoch())
    }

    fn query<T: DeserializeOwned>(&self, method: Method, args: &impl Serialize) -> io::Result<T> {
        self.client()
            .query::<Result<T, ApiError>>(method, args)
            .map_err(failure)?
            .map_err(failure)
    }

    fn update<T: DeserializeOwned>(&self, method: Method, args: &impl Serialize) -> io::Result<T> {
        self.client()
            .update::<Result<T, ApiError>>(method, args)
            .map_err(failure)?
            .map_err(failure)
    }

    fn client(&self) -> harness::Client<'_> {
        harness::Client {
            pic: self.pic.instance(),
            canister: self.canister,
            caller: self.manager,
        }
    }

    fn views(&self) -> io::Result<Vec<View>> {
        const PAGE_LIMIT: u8 = 8;
        let mut views = vec![];
        let mut after = None;
        loop {
            let page: Vec<View> = self.query(Method::List, &(after, PAGE_LIMIT))?;
            let complete = page.len() < usize::from(PAGE_LIMIT);
            after = page.last().map(|view| view.id);
            views.extend(page);
            if complete {
                return Ok(views);
            }
        }
    }

    fn add(
        &mut self,
        name: &str,
        after: Duration,
        every: Option<Duration>,
        catch_up: bool,
    ) -> io::Result<()> {
        let first_ns = self
            .now()?
            .checked_add(u64::try_from(after.as_nanos()).map_err(failure)?)
            .ok_or_else(|| io::Error::other("deadline exceeds the canister timestamp range"))?;
        let (timing, schedule) = if let Some(interval) = every {
            let every_ns = u64::try_from(interval.as_nanos()).map_err(failure)?;
            (
                Timing::FixedRate {
                    first_ns,
                    every_ns,
                    catch_up,
                },
                format!(
                    "every {} ({})",
                    span(every_ns),
                    if catch_up { "catch-up" } else { "skip" }
                ),
            )
        } else {
            (Timing::Once(first_ns), "once".into())
        };
        let id: u64 = self.update(
            Method::Create,
            &(Create {
                timing,
                task: Task::Increment(1),
            },),
        )?;
        self.labels.insert(id, (name.into(), schedule));
        println!(
            "QUEUED #{id} {name}: first run in {}",
            span(after.as_nanos() as u64)
        );
        Ok(())
    }

    fn show(&self) -> io::Result<()> {
        self.show_views(&mut self.views()?)
    }

    fn show_views(&self, views: &mut [View]) -> io::Result<()> {
        let summary: Summary = self.query(Method::Summary, &())?;
        let now = self.now()?;
        println!(
            "\nIC clock +{} | counter={} | watchdog={}",
            span(now.saturating_sub(self.origin_ns) / 1_000_000_000 * 1_000_000_000),
            summary.counter,
            if summary.armed { "armed" } else { "idle" }
        );
        println!(
            "{:<4} {:<18} {:<24} {:<12} {:<20} RUNS",
            "ID", "NAME", "SCHEDULE", "STATE", "NEXT RUN"
        );
        views.sort_by_key(|view| {
            (
                match view.status {
                    Status::Pending { due_ns } => due_ns,
                    _ => u64::MAX,
                },
                view.id,
            )
        });
        for view in views {
            let (name, schedule) = &self.labels[&view.id];
            let (state, next) = match view.status {
                Status::Pending { due_ns } => ("pending", deadline(due_ns, now, "due now")),
                Status::Paused => ("paused", "—".into()),
                Status::Running => ("running", "—".into()),
                Status::Uncertain => ("uncertain", "—".into()),
                Status::Completed => ("completed", "—".into()),
                Status::Failed => ("failed", "—".into()),
                Status::Cancelled => ("cancelled", "—".into()),
            };
            println!(
                "{:<4} {:<18} {:<24} {:<12} {:<20} {}",
                view.id, name, schedule, state, next, view.sequence
            );
        }
        let next = summary.next_due_ns.map_or_else(
            || "none".into(),
            |due| {
                deadline(
                    due,
                    now,
                    "due now (more rounds/time may be needed for catch-up)",
                )
            },
        );
        println!("Next wakeup: {next}\n");
        Ok(())
    }

    fn advance(&self, advance: Duration, before: Option<Vec<View>>) -> io::Result<Vec<View>> {
        self.now()?
            .checked_add(u64::try_from(advance.as_nanos()).map_err(failure)?)
            .ok_or_else(|| io::Error::other("clock exceeds the canister timestamp range"))?;
        // Watch carries only the previous step's result; other commands start
        // with a fresh read. Clock admission still precedes any RPC.
        let before = match before {
            Some(views) => views,
            None => self.views()?,
        };
        // Drive real timer callbacks; never invoke dispatch_due from the host.
        // Fixed rounds keep a large CatchUp backlog from blocking the terminal.
        advance_and_tick(self.pic.instance(), advance, 32).map_err(failure)?;
        let mut after = self.views()?;
        for view in &after {
            let previous = before
                .iter()
                .find(|entry| entry.id == view.id)
                .map_or(0, |entry| entry.sequence);
            if view.sequence > previous {
                println!(
                    "RAN #{} {}: {} execution(s), total {}",
                    view.id,
                    self.labels[&view.id].0,
                    view.sequence - previous,
                    view.sequence
                );
            }
        }
        self.show_views(&mut after)?;
        Ok(after)
    }

    fn manage(&self, method: Method, id: u64) -> io::Result<()> {
        self.update::<()>(method, &(id,))?;
        self.show()
    }

    fn execute(&mut self, command: &QueueCommand) -> io::Result<bool> {
        match command {
            QueueCommand::Quit => return Ok(false),
            QueueCommand::Sample => {
                self.add("seconds", Duration::from_secs(5), None, false)?;
                self.add("minutes", Duration::from_secs(60), None, false)?;
                self.add(
                    "weekly",
                    Duration::from_secs(604800),
                    Some(Duration::from_secs(604800)),
                    false,
                )?;
                self.show()?;
            }
            QueueCommand::Add {
                name,
                after,
                every,
                catch_up,
            } => {
                self.add(name, *after, *every, *catch_up)?;
                self.show()?;
            }
            QueueCommand::List => self.show()?,
            QueueCommand::Metrics => {
                let metrics: ic_jobs_test_consumer::api::JobMetrics =
                    self.query(Method::JobMetrics, &())?;
                println!("{metrics:#?}");
            }
            QueueCommand::Logs { before } => {
                let page: ic_jobs_test_consumer::api::JobLogPage = self.query(
                    Method::JobLogs,
                    &ic_jobs_test_consumer::api::JobLogRequest {
                        before: *before,
                        min_level: None,
                        limit: 8,
                    },
                )?;
                println!("{page:#?}");
            }
            QueueCommand::ResetMetrics => {
                let metrics: ic_jobs_test_consumer::api::JobMetrics =
                    self.update(Method::ResetJobMetrics, &())?;
                println!("{metrics:#?}");
            }
            QueueCommand::Advance { duration } => {
                self.advance(*duration, None)?;
            }
            QueueCommand::Watch { duration } => {
                let mut remaining = *duration;
                let mut before = None;
                println!("Watching; press Ctrl+C to stop.");
                while !remaining.is_zero() && !INTERRUPTED.load(Ordering::Relaxed) {
                    let step = remaining.min(Duration::from_secs(1));
                    std::thread::sleep(step);
                    if INTERRUPTED.load(Ordering::Relaxed) {
                        break;
                    }
                    before = Some(self.advance(step, before)?);
                    remaining -= step;
                }
            }
            QueueCommand::Pause { id } => self.manage(Method::Pause, *id)?,
            QueueCommand::Resume { id } => self.manage(Method::Resume, *id)?,
            QueueCommand::Cancel { id } => self.manage(Method::Cancel, *id)?,
        }
        Ok(true)
    }
}

fn main() -> io::Result<()> {
    let args = Options::parse();
    ctrlc::set_handler(|| INTERRUPTED.store(true, Ordering::Relaxed)).map_err(failure)?;
    let config = PocketIcStartupConfig::Spawn {
        server_binary: args.server,
        timeout: Duration::from_secs(30),
        server_hard_ttl: Some(Duration::from_secs(3600)),
        server_idle_ttl: Some(Duration::from_secs(3600)),
        server_output_files: None,
    };
    let mut demo = Demo::new(&args.wasm, config)?;
    Prompt::command().print_help()?;
    println!();
    let stdin = io::stdin();
    let interactive = stdin.is_terminal();
    // Read in a bounded channel so Ctrl+C can unwind the owned Testkit server
    // even while the terminal is idle. No signal-unsafe cleanup runs in a handler.
    let (sender, lines) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        for line in stdin.lock().lines() {
            if sender.send(line).is_err() {
                break;
            }
        }
    });
    loop {
        if INTERRUPTED.load(Ordering::Relaxed) {
            break;
        }
        if interactive {
            print!("jobs> ");
            io::stdout().flush()?;
        }
        let line = loop {
            if INTERRUPTED.load(Ordering::Relaxed) {
                return Ok(());
            }
            match lines.recv_timeout(Duration::from_millis(100)) {
                Ok(line) => break line,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
            }
        };
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let command = match Prompt::try_parse_from(line.split_whitespace()) {
            Ok(command) => command,
            Err(error)
                if interactive || matches!(error.kind(), clap::error::ErrorKind::DisplayHelp) =>
            {
                error.print()?;
                continue;
            }
            Err(error) => return Err(failure(error)),
        };
        match demo.execute(&command.command) {
            Ok(false) => break,
            Ok(true) => {}
            Err(error) if interactive => eprintln!("{error}"),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_and_commands_reject_invalid_or_unbounded_input() {
        assert!(matches!(
            Prompt::try_parse_from(["logs", "--before", "8"])
                .unwrap()
                .command,
            QueueCommand::Logs { before: Some(8) }
        ));
        assert!(Prompt::try_parse_from(["logs", "--before", "-1"]).is_err());
        assert!(matches!(
            Prompt::try_parse_from(["metrics"]).unwrap().command,
            QueueCommand::Metrics
        ));
        assert!(matches!(
            Prompt::try_parse_from(["reset-metrics"]).unwrap().command,
            QueueCommand::ResetMetrics
        ));
        for (input, seconds) in [
            ("5s", 5),
            ("2m", 120),
            ("3h", 10800),
            ("4d", 345600),
            ("2w", 1209600),
        ] {
            assert_eq!(duration(input).unwrap(), Duration::from_secs(seconds));
            assert_eq!(span(seconds * 1_000_000_000), input);
        }
        assert_eq!(span(0), "0s");
        assert_eq!(span(1), "1s");
        assert_eq!(
            span((604800 + 86400 + 3600 + 60 + 1) * 1_000_000_000),
            "1w 1d 1h 1m 1s"
        );
        for input in ["", "5", "1.5m", "-1s", "1m30s", "999999999999999999w"] {
            assert!(duration(input).is_err());
        }
        let QueueCommand::Add {
            name,
            after,
            every,
            catch_up,
        } = Prompt::try_parse_from(
            "add report --after 0s --every 2w --catch-up".split_whitespace(),
        )
        .unwrap()
        .command
        else {
            panic!("expected typed add command");
        };
        assert_eq!(name, "report");
        assert_eq!(after, Duration::ZERO);
        assert_eq!(every, Some(Duration::from_secs(1_209_600)));
        assert!(catch_up);
        assert!(matches!(
            Prompt::try_parse_from(["exit"]).unwrap().command,
            QueueCommand::Quit
        ));
        for input in [
            "add x --after 1s --every 0s",
            "add x --after 1s --catch-up",
            "watch 0s",
            "pause nope",
        ] {
            assert!(Prompt::try_parse_from(input.split_whitespace()).is_err());
        }
    }

    #[test]
    #[ignore = "explicit local canister demo: make test-demo"]
    fn canister_session_queues_seconds_minutes_weeks_and_manages_recurrence() {
        let wasm =
            std::env::var_os("IC_JOBS_CONSUMER_WASM").expect("make test-demo selects the Wasm");
        let config = PocketIcStartupConfig::from_env(Duration::from_secs(30)).unwrap();
        let mut demo = Demo::new(Path::new(&wasm), config).unwrap();
        let run = |demo: &mut Demo, line: &str| {
            let args = Prompt::try_parse_from(line.split_whitespace()).unwrap();
            assert!(demo.execute(&args.command).unwrap());
        };
        run(&mut demo, "sample");
        assert!(
            demo.views()
                .unwrap()
                .iter()
                .all(|view| matches!(view.status, Status::Pending { .. }))
        );
        // Two watch steps exercise carrying the previous display's views.
        run(&mut demo, "watch 2s");
        run(&mut demo, "advance 3s");
        assert_eq!(demo.views().unwrap()[0].status, Status::Completed);
        assert!(matches!(
            demo.views().unwrap()[1].status,
            Status::Pending { .. }
        ));
        run(&mut demo, "advance 55s");
        assert_eq!(demo.views().unwrap()[1].status, Status::Completed);
        run(&mut demo, "advance 1w");
        assert_eq!(demo.views().unwrap()[2].sequence, 1);
        run(&mut demo, "pause 3");
        run(&mut demo, "advance 2w");
        assert_eq!(demo.views().unwrap()[2].status, Status::Paused);
        assert_eq!(demo.views().unwrap()[2].sequence, 1);
        run(&mut demo, "resume 3");
        run(&mut demo, "advance 1s");
        assert_eq!(demo.views().unwrap()[2].sequence, 2);
        run(&mut demo, "cancel 3");
        run(&mut demo, "advance 1w");
        assert_eq!(demo.views().unwrap()[2].status, Status::Cancelled);
        let summary: Summary = demo.query(Method::Summary, &()).unwrap();
        assert_eq!(summary.counter, 4);
        assert!(!summary.armed);
    }
}
