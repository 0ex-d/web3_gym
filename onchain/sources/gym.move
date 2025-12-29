module gym::gym;

use sui::clock::{Self, Clock};

const E_FROZEN: u64 = 1;
const E_EXPIRED: u64 = 2;
const E_TOO_SOON: u64 = 3;
const E_NO_VISITS: u64 = 4;
const E_NOT_MEMBER: u64 = 5;

const MIN_ENTRY_INTERVAL_MS: u64 = 90 * 60 * 1000;

public struct AdminCap has key {
    id: UID,
}

public struct GymMembership has key {
    id: UID,
    member: address,
    tier: u8,
    valid_until: u64,
    remaining_visits: Option<u32>,
    last_entry_timestamp: u64,
    frozen: bool,
}

fun init(ctx: &mut TxContext) {
    let cap = AdminCap { id: object::new(ctx) };
    transfer::transfer(cap, tx_context::sender(ctx));
}

entry fun verify_and_enter(membership: &mut GymMembership, clock: &Clock, ctx: &TxContext) {
    let now = clock::timestamp_ms(clock);

    assert!(membership.member == tx_context::sender(ctx), E_NOT_MEMBER);
    assert!(!membership.frozen, E_FROZEN);
    assert!(membership.valid_until >= now, E_EXPIRED);
    assert!(now >= membership.last_entry_timestamp + MIN_ENTRY_INTERVAL_MS, E_TOO_SOON);

    if (option::is_some(&membership.remaining_visits)) {
        let remaining_ref = option::borrow_mut(&mut membership.remaining_visits);
        let remaining = *remaining_ref;
        assert!(remaining > 0, E_NO_VISITS);
        *remaining_ref = remaining - 1;
    };

    membership.last_entry_timestamp = now;
}

// === Test Functions ===
#[test_only]
public fun init_for_test(ctx: &mut TxContext) {
    init(ctx);
}

#[test_only]
public fun create_membership_for_test(
    member: address,
    tier: u8,
    valid_until: u64,
    remaining_visits: Option<u32>,
    ctx: &mut TxContext,
): GymMembership {
    GymMembership {
        id: object::new(ctx),
        member,
        tier,
        valid_until,
        remaining_visits,
        last_entry_timestamp: 0,
        frozen: false,
    }
}

#[test_only]
public fun destroy_membership_for_test(membership: GymMembership) {
    let GymMembership {
        id,
        member: _,
        tier: _,
        valid_until: _,
        remaining_visits: _,
        last_entry_timestamp: _,
        frozen: _,
    } = membership;
    object::delete(id);
}

#[test_only]
public fun freeze_membership_for_test(membership: &mut GymMembership) {
    membership.frozen = true;
}

#[test_only]
public fun get_remaining_visits(membership: &GymMembership): Option<u32> {
    membership.remaining_visits
}

#[test_only]
public fun get_last_entry_timestamp(membership: &GymMembership): u64 {
    membership.last_entry_timestamp
}
