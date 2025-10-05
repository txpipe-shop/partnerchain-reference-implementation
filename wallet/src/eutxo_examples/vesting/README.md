# Vesting Example

This contract allows an "owner" (indicated in the datum via a public key hash) to withdraw funds after a certain deadline (indicated in the datum as POSIX time). To ensure the latter condition, the transaction must have a validity interval starting at or after the deadline.

Keep in mind that:
- the `validity_interval_start` is expressed in slots.
- the initial POSIX time of a slot can be obtained as `(slot_number - zero_slot) * MILLI_SECS_PER_SLOT + zero_time`, where `zero_slot` and `zero_time` are genesis configuration parameters and the constant `MILLI_SECS_PER_SLOT` is set in the `griffin_core` crate.
- with the actual configuration, `zero_slot = 0`, `zero_time = 1747081100000` (i.e., 2025-06-12 14:51:40 UTC), `MILLI_SECS_PER_SLOT = 3000` and a deadline set in the datum of `1747081220000` (i.e., 2025-06-12 14:53:40 UTC), the `validity_interval_start` must be at least `(1747081220000 - 1747081100000) / 3000 = 40`, which means that the owner will be able to unlock the funds 40 slots after the genesis.
