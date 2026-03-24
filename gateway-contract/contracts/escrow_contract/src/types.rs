use soroban_sdk::{contracttype, Address, BytesN};

/// Storage keys for the Escrow contract's persistent and instance storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Key for a vault's state, indexed by its BytesN<32> commitment.
    Vault(BytesN<32>),
    /// Key for a specific scheduled payment, indexed by its unique payment_id (u32).
    ScheduledPayment(u32),
    /// Key for the auto-incrementing payment counter in instance storage.
    PaymentCounter,
    /// Key for a specific auto-pay rule, indexed by its unique auto_pay_id (u32).
    AutoPay(u32),
    /// Key for the auto-incrementing auto-pay counter in instance storage.
    AutoPayCounter,
}

/// Represents the state of a user's vault within the contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultState {
    /// The Stellar address authorized to manage this vault.
    pub owner: Address,
    /// The asset token currently stored in the vault.
    pub token: Address,
    /// The current available balance in the vault.
    pub balance: i128,
}

/// Represents a payment that has been scheduled but not yet executed.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledPayment {
    /// The commitment identifier of the source vault.
    pub from: BytesN<32>,
    /// The commitment identifier of the intended recipient.
    pub to: BytesN<32>,
    /// The token to be transferred upon execution.
    pub token: Address,
    /// The amount of tokens to be transferred.
    pub amount: i128,
    /// The timestamp at or after which the payment can be executed.
    pub release_at: u64,
    /// Whether the payment has already been executed.
    pub executed: bool,
}

/// Represents a recurring payment rule.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoPay {
    /// The commitment identifier of the source vault.
    pub from: BytesN<32>,
    /// The commitment identifier of the intended recipient.
    pub to: BytesN<32>,
    /// The token to be transferred upon execution.
    pub token: Address,
    /// The amount of tokens to be transferred each interval.
    pub amount: i128,
    /// The interval in seconds between automatic payments.
    pub interval: u64,
    /// The timestamp of the last payment execution (0 if never executed).
    pub last_paid: u64,
}
