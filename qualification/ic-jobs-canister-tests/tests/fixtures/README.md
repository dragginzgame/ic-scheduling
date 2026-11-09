# Frozen management response

`pending-page-0162.didbin` is a negative Candid fixture from Jobs 0.16.2 API
source `41d9e31ff92de1703976d642b28d20ff08bae8aa`, built on Rust 1.91 with the
incoming Testkit/Candid 0.52 graph. Before changing the API, the old encoder and
decoder round-tripped `Ok([View { id: 1, task: Increment(1), status: Pending,
due_ns: Some(100), sequence: 0, occurrence: 0 }])`.

The current Wire decoder refuses that old Candid response. These bytes grant no
storage, effect-replay or conversion authority; no old response reader is kept.
SHA-256: `11957480f84d5f722031cba17fc3b7c888c8e6bc5f78e61e173b05d097927bbc`.
