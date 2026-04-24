use shared::storage as shared_storage;
use soroban_sdk::{Address, BytesN, Env};

use crate::types::{DataKey, DeployConfig, UsernameRecord};

/// TTL constants for persistent storage entries.
/// Bump amount: ~30 days (at ~5s per ledger close).
#[allow(dead_code)]
pub(crate) const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
/// Lifetime threshold: ~7 days — entries are extended when remaining TTL drops below this.
#[allow(dead_code)]
pub(crate) const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960;

/// Persists the auction contract address in instance storage.
pub fn set_auction_contract(env: &Env, auction_contract: &Address) {
    shared_storage::set_instance(env, &DataKey::AuctionContract, auction_contract);
}

/// Returns the configured auction contract address, or `None` if unset.
pub fn get_auction_contract(env: &Env) -> Option<Address> {
    shared_storage::get_instance(env, &DataKey::AuctionContract)
}

/// Persists the core contract address in instance storage.
pub fn set_core_contract(env: &Env, core_contract: &Address) {
    shared_storage::set_instance(env, &DataKey::CoreContract, core_contract);
}

/// Returns the configured core contract address, or `None` if unset.
pub fn get_core_contract(env: &Env) -> Option<Address> {
    shared_storage::get_instance(env, &DataKey::CoreContract)
}

/// Stores a username record in persistent storage, extending its TTL.
pub fn set_username(env: &Env, hash: &BytesN<32>, record: &UsernameRecord) {
    let key = DataKey::Username(hash.clone());
    shared_storage::set_persistent(env, &key, record);
}

/// Returns the username record for the given hash, or `None` if not registered.
pub fn get_username(env: &Env, hash: &BytesN<32>) -> Option<UsernameRecord> {
    let key = DataKey::Username(hash.clone());
    shared_storage::get_persistent_with_ttl(env, &key)
}

/// Returns `true` if a username record exists for the given hash.
pub fn has_username(env: &Env, hash: &BytesN<32>) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Username(hash.clone()))
}

/// Returns the deploy configuration, or `None` if not set.
#[allow(dead_code)]
pub fn get_config(env: &Env) -> Option<DeployConfig> {
    shared_storage::get_persistent(env, &DataKey::Config)
}

/// Persists the deploy configuration in persistent storage, extending its TTL.
#[allow(dead_code)]
pub fn set_config(env: &Env, config: &DeployConfig) {
    let key = DataKey::Config;
    shared_storage::set_persistent(env, &key, config);
}
