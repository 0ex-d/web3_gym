#[test_only]
module gym::gym_tests;

use gym::constants;
use gym::gym::{Self, AdminCap};
use sui::clock;
use sui::test_scenario as ts;

const ADMIN: address = @0x1;
const MEMBER: address = @0x2;
const OTHER_USER: address = @0x3;

const DAY_MS: u64 = 24 * 60 * 60 * 1000;
const HOUR_MS: u64 = 60 * 60 * 1000;
const MIN_ENTRY_INTERVAL_MS: u64 = 90 * 60 * 1000; // 90 minutes

#[test]
fun test_init_creates_admin_cap() {
    let mut scenario = ts::begin(ADMIN);
    {
        gym::init_for_test(ts::ctx(&mut scenario));
    };

    ts::next_tx(&mut scenario, ADMIN);
    {
        assert!(ts::has_most_recent_for_sender<AdminCap>(&scenario), 0);
        let cap = ts::take_from_sender<AdminCap>(&scenario);
        ts::return_to_sender(&scenario, cap);
    };

    ts::end(scenario);
}

#[test]
fun test_successful_entry_with_unlimited_visits() {
    let mut scenario = ts::begin(MEMBER);
    let mut clock = clock::create_for_testing(ts::ctx(&mut scenario));

    // Set current time to some base value
    clock::set_for_testing(&mut clock, MIN_ENTRY_INTERVAL_MS + 1);

    let mut membership = gym::create_membership_for_test(
        MEMBER,
        constants::tier_premium(),
        clock::timestamp_ms(&clock) + 30 * DAY_MS, // Valid for 30 days
        option::none(), // Unlimited visits
        ts::ctx(&mut scenario),
    );

    gym::verify_and_enter(&mut membership, &clock, ts::ctx(&mut scenario));
    assert!(gym::get_last_entry_timestamp(&membership) == clock::timestamp_ms(&clock), 0);

    gym::destroy_membership_for_test(membership);
    clock::destroy_for_testing(clock);
    ts::end(scenario);
}

#[test, expected_failure(abort_code = gym::E_EXPIRED)]
fun test_entry_rejects_expired_membership() {
    let mut scenario = ts::begin(MEMBER);
    let mut clock = clock::create_for_testing(ts::ctx(&mut scenario));
    clock::set_for_testing(&mut clock, MIN_ENTRY_INTERVAL_MS + 1);

    let mut membership = gym::create_membership_for_test(
        MEMBER,
        constants::tier_basic(),
        clock::timestamp_ms(&clock) - 1,
        option::none(),
        ts::ctx(&mut scenario),
    );

    gym::verify_and_enter(&mut membership, &clock, ts::ctx(&mut scenario));

    gym::destroy_membership_for_test(membership);
    clock::destroy_for_testing(clock);
    ts::end(scenario);
}

#[test, expected_failure(abort_code = gym::E_FROZEN)]
fun test_entry_rejects_frozen_membership() {
    let mut scenario = ts::begin(MEMBER);
    let mut clock = clock::create_for_testing(ts::ctx(&mut scenario));
    clock::set_for_testing(&mut clock, MIN_ENTRY_INTERVAL_MS + 1);

    let mut membership = gym::create_membership_for_test(
        MEMBER,
        constants::tier_standard(),
        clock::timestamp_ms(&clock) + 7 * DAY_MS,
        option::none(),
        ts::ctx(&mut scenario),
    );

    gym::freeze_membership_for_test(&mut membership);
    gym::verify_and_enter(&mut membership, &clock, ts::ctx(&mut scenario));

    gym::destroy_membership_for_test(membership);
    clock::destroy_for_testing(clock);
    ts::end(scenario);
}
