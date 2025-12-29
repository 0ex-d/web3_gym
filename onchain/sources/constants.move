module gym::constants;

// Membership tier constants
const TIER_BASIC: u8 = 1;
const TIER_STANDARD: u8 = 2;
const TIER_PREMIUM: u8 = 3;

// Getter functions for tiers
public fun tier_basic(): u8 {
    TIER_BASIC
}

public fun tier_standard(): u8 {
    TIER_STANDARD
}

public fun tier_premium(): u8 {
    TIER_PREMIUM
}

// Optional: Tier validation function
public fun is_valid_tier(tier: u8): bool {
    tier == TIER_BASIC
            || tier == TIER_STANDARD
            || tier == TIER_PREMIUM
}
