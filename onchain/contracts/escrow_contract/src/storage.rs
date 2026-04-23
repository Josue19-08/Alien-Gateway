use crate::errors::EscrowError;
use crate::types::{AutoPay, DataKey, LegacyVault, ScheduledPayment, VaultConfig, VaultState};
use shared::storage as shared_storage;
use soroban_sdk::{Address, BytesN, Env};

/// TTL constants for persistent storage entries.
/// Bump amount: ~30 days (at ~5s per ledger close).
#[allow(dead_code)]
pub(crate) const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
/// Lifetime threshold: ~7 days — entries are extended when remaining TTL drops below this.
#[allow(dead_code)]
pub(crate) const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960;

/// Reads a vault's immutable configuration from persistent storage.
///
/// Checks the new `VaultConfig` key first; if absent, falls back to the legacy `Vault` key and
/// projects the combined record into a `VaultConfig` for backward compatibility.
pub fn read_vault_config(env: &Env, commitment: &BytesN<32>) -> Option<VaultConfig> {
    let key = DataKey::VaultConfig(commitment.clone());
    if let Some(config) = shared_storage::get_persistent(env, &key) {
        return Some(config);
    }
    let legacy: LegacyVault =
        shared_storage::get_persistent(env, &DataKey::Vault(commitment.clone()))?;
    Some(VaultConfig {
        owner: legacy.owner,
        token: legacy.token,
        created_at: legacy.created_at,
    })
}

/// Writes a vault's immutable configuration to persistent storage.
pub fn write_vault_config(env: &Env, commitment: &BytesN<32>, config: &VaultConfig) {
    let key = DataKey::VaultConfig(commitment.clone());
    shared_storage::set_persistent(env, &key, config);
}

/// Reads a vault's mutable state from persistent storage.
///
/// Checks the new `VaultState` key first; if absent, falls back to the legacy `Vault` key and
/// projects the combined record into a `VaultState` for backward compatibility.
pub fn read_vault_state(env: &Env, commitment: &BytesN<32>) -> Option<VaultState> {
    let key = DataKey::VaultState(commitment.clone());
    if let Some(state) = shared_storage::get_persistent(env, &key) {
        return Some(state);
    }
    let legacy: LegacyVault =
        shared_storage::get_persistent(env, &DataKey::Vault(commitment.clone()))?;
    Some(VaultState {
        balance: legacy.balance,
        is_active: legacy.is_active,
    })
}

/// Writes a vault's mutable state to persistent storage.
pub fn write_vault_state(env: &Env, commitment: &BytesN<32>, state: &VaultState) {
    let key = DataKey::VaultState(commitment.clone());
    shared_storage::set_persistent(env, &key, state);
}

/// Increments the global payment counter and returns the previous ID.
///
/// ### Errors
/// - Returns `EscrowError::PaymentCounterOverflow` if the counter reaches `u32::MAX`.
pub fn increment_payment_id(env: &Env) -> Result<u32, EscrowError> {
    let id: u32 = shared_storage::get_instance(env, &DataKey::PaymentCounter).unwrap_or(0);

    let next = id
        .checked_add(1)
        .ok_or(EscrowError::PaymentCounterOverflow)?;

    shared_storage::set_instance(env, &DataKey::PaymentCounter, &next);

    Ok(id)
}

/// Reads the Registration contract address from instance storage.
pub fn read_registration_contract(env: &Env) -> Option<Address> {
    shared_storage::get_instance(env, &DataKey::RegistrationContract)
}

/// Writes the Registration contract address to instance storage.
pub fn write_registration_contract(env: &Env, address: &Address) {
    shared_storage::set_instance(env, &DataKey::RegistrationContract, address);
}

/// Records a new scheduled payment in persistent storage.
pub fn write_scheduled_payment(env: &Env, id: u32, payment: &ScheduledPayment) {
    let key = DataKey::ScheduledPayment(id);
    shared_storage::set_persistent(env, &key, payment);
}

/// Increments the global auto-pay counter and returns the previous ID.
///
/// ### Errors
/// - Returns `EscrowError::AutoPayCounterOverflow` if the counter reaches `u32::MAX`.
pub fn increment_auto_pay_id(env: &Env) -> Result<u32, EscrowError> {
    let id: u32 = shared_storage::get_instance(env, &DataKey::AutoPayCounter).unwrap_or(0);

    let next = id
        .checked_add(1)
        .ok_or(EscrowError::AutoPayCounterOverflow)?;

    shared_storage::set_instance(env, &DataKey::AutoPayCounter, &next);

    Ok(id)
}

/// Reads the current auto-pay counter from instance storage.
///
/// Returns `0` when no auto-pay rules have been created yet.
pub fn read_auto_pay_count(env: &Env) -> u32 {
    shared_storage::get_instance(env, &DataKey::AutoPayCounter).unwrap_or(0)
}

/// Records an auto-pay rule in persistent storage under the composite key (vault, rule_id).
pub fn write_auto_pay(env: &Env, commitment: &BytesN<32>, rule_id: u32, auto_pay: &AutoPay) {
    let key = DataKey::AutoPay(commitment.clone(), rule_id as u64);
    shared_storage::set_persistent(env, &key, auto_pay);
}

/// Reads an auto-pay rule from persistent storage by vault commitment and rule ID.
pub fn read_auto_pay(env: &Env, commitment: &BytesN<32>, rule_id: u32) -> Option<AutoPay> {
    shared_storage::get_persistent(env, &DataKey::AutoPay(commitment.clone(), rule_id as u64))
}

/// Deletes an auto-pay rule from persistent storage by vault commitment and rule ID.
pub fn delete_auto_pay(env: &Env, from: &BytesN<32>, rule_id: u32) {
    let key = DataKey::AutoPay(from.clone(), rule_id as u64);
    env.storage().persistent().remove(&key);
}
