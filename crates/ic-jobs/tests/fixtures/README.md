# Stored record fixtures

`uncertain-record.json` is hand-authored current-format JSON. Its outcome and
timestamp form one result. The codec test checks exact re-encoding and validated
restoration, including blocked dispatch until explicit result reconciliation.
The bounded public transition-history test separately covers every state and
checks restoration after management, retries and completion.

The consumer's frozen `snapshot-075.json` remains solely as a refusal fixture,
from archived Jobs 0.7.5 commit `060fb9d226389bab702132ba23f42bd0b887a690`.
The consumer rejects it on current Memory backing without replacing its cell.
Its sparse `stable-image-075.json` and `stable-image-memory040.json` fixtures
retain earlier physical images for the independent incompatible-ledger refusal
proof. Their zero bytes are implicit; nonzero runs and producer bytes remain exact.
These are negative inputs, not compatibility readers or an upgrade path.
Native fixture checks do not establish live IC upgrades or message rollback.

## Memory 0.56 allocation-ledger cut

The consumer's `stable-image-memory0552.json` freezes a complete three-page
Memory 0.55.2 installation from Jobs 0.17.47 at
`cdb48cc9d1c3281e2b898bcfe4a19e2a673b4560`, produced on Rust 1.91. It contains
one valid Running delivery intent. After dropping all earlier handles, the old
producer reconstructed the exact physical image, reopened it, recovered the
same delivery and refused redispatch of the blocked job.

The fixture retains ten nonzero runs and the unchanged Jobs JSON payload. Memory
0.56 refuses its retired CBOR ledger during recovery and fresh creation with
zero writes, unchanged pages and identical bytes. Independently decoding the
frozen queue payload in tests still uses Job::restore and leaves the job Running/blocked. This
separates the infrastructure format cut from unchanged job/effect authority;
it supplies neither a converter nor permission to discard retained work.
The fixture SHA-256 is
`780ffd07de7ccc8d47398fce67d223e62e6d3c37b2618af502fa299eeaa0c7a7`.
Generation inputs and old-producer qualification remain at
`/tmp/ic-jobs-tooling0702-memory056.8W4xazfe/old-producer/`.

## Qualification queue JSON retirement

The consumer's `stable-image-json01749.json` freezes a complete Memory 0.56.0
installation produced from Jobs release `26108182368b3ef9687f195feaf2ce6c38021dfc`
with the incoming 2026-10-30 dependency lock and Rust 1.91. It retains one
Uncertain and one Running delivery, with their immutable envelopes and two exact
receipts. The old producer cold-opened the queue and refused overdue redispatch
before capture. A test-only metadata read establishes these unresolved effects;
the production reader never decodes retained JSON.

At the original qualification, Memory accepted this ledger while the queue
refused the retired payload without replacing its Cell header or payload bytes;
bootstrap could update allocation bookkeeping. Current Storage refuses that
complete retired image before payload admission, with zero writes. This proof is distinct from the
zero-write refusal of incompatible Memory ledgers above. No existing installation
is reset, converted or authorized to repeat an effect.

Fixture SHA-256:
`67a99afc6e2649e14dd8ef67d302116cd21df3205ee193f2c616ca762fa96ec9`.
The original source archive, one-shot producer/test log, lockfile and old Wasm
are retained at `/tmp/ic-jobs-78-upstream.Mso2Zod0/`; the current consumer guide
owns the explicit retained-installation retirement procedure.

## Storage Cell transition evidence

The consumer retains two additional complete physical images:

- [Storage 0.63 / Memory image](../../../../apps/ic-jobs-test-consumer/src/queue/fixtures/stable-image-storage063.json):
  a valid Wire queue with Running/Uncertain attempts and two exact receipts.
  Current Storage refuses the retired physical format without writes or reset;
  the unchanged application payload still enters through Job::restore blocked.
- [Storage 0.64 Cell image](../../../../apps/ic-jobs-test-consumer/src/queue/fixtures/stable-image-storage064.json):
  captured with the previous reader before the Storage 0.65, Runtime 0.18 and
  Wire 0.15 update. Historical 0.65–0.67 readers preserved both blocked states,
  authority and exact receipts without writes or growth. Storage 0.68 refuses
  this previous Cell framing during both recovery and fresh creation with zero
  writes/growth and identical bytes. Frozen metadata still establishes the
  blocked attempts and receipts; it does not authorize replay or reset.

Each image records its producer, old dependency-lock digest, bounded physical
pages, sparse runs and snapshot metadata; the Wire images retain exact payload
hex too. The 0.64 capture log and complete incoming source/lock evidence are under
`target/current-all-upstream-adoption`. These are native backing observations,
not installed IC recovery, rollback or authorization to reset retained data.

## 0.16 refusal inputs

`running-record-015.json` freezes the previous timestamp-bearing Running record.
The consumer also retains `snapshot-running-015.json` and
`snapshot-receipt-015.json` under its queue fixtures and
`delivery-reply-015.didbin` under its reply fixtures. These freeze the old Running
snapshot, flat receipt snapshot and successful flat Candid delivery reply.

They were produced using the incoming 0.15 Job/Task/Delivery serializers at Jobs
`71761e0c8ce3c6b2a4ce0226b80691892aa57f63`, with the existing Build/consumer
cleanup and selected Timers 0.27 graph. Snapshot headers were assembled around
those typed values. The prior queue decoder admits both snapshots, and the prior
Candid decoder admits the reply; that focused qualification is retained in
`target/qualification/running-payload-cut/`. The capture helper and original
source remain in `/tmp/jobs-running-payload-cut.9krmjk9u/before-source/`.

Current checks refuse each changed shape, preserving refused stable payloads
through recovery and fresh-install refusal. No reader, converter or deployed
reset is added. The unchanged current Uncertain fixture above remains positive
blocked-restoration evidence.
