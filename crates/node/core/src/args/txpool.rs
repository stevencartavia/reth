//! Transaction pool arguments

use crate::cli::config::RethTransactionPoolConfig;
use alloy_eips::eip1559::{ETHEREUM_BLOCK_GAS_LIMIT_30M, MIN_PROTOCOL_BASE_FEE};
use alloy_primitives::Address;
use clap::Args;
use reth_cli_util::parse_duration_from_secs_or_ms;
use reth_transaction_pool::{
    blobstore::disk::DEFAULT_MAX_CACHED_BLOBS,
    maintain::MAX_QUEUED_TRANSACTION_LIFETIME,
    pool::{NEW_TX_LISTENER_BUFFER_SIZE, PENDING_TX_LISTENER_BUFFER_SIZE},
    validate::DEFAULT_MAX_TX_INPUT_BYTES,
    LocalTransactionConfig, PoolConfig, PriceBumpConfig, SubPoolLimit, DEFAULT_PRICE_BUMP,
    DEFAULT_TXPOOL_ADDITIONAL_VALIDATION_TASKS, MAX_NEW_PENDING_TXS_NOTIFICATIONS,
    REPLACE_BLOB_PRICE_BUMP, TXPOOL_MAX_ACCOUNT_SLOTS_PER_SENDER,
    TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT, TXPOOL_SUBPOOL_MAX_TXS_DEFAULT,
};
use std::time::Duration;

/// Parameters for debugging purposes
#[derive(Debug, Clone, Args, PartialEq, Eq)]
#[command(next_help_heading = "TxPool")]
pub struct TxPoolArgs {
    /// Max number of transaction in the pending sub-pool.
    #[arg(long = "txpool.pending-max-count", alias = "txpool.pending_max_count", default_value_t = TXPOOL_SUBPOOL_MAX_TXS_DEFAULT)]
    pub pending_max_count: usize,
    /// Max size of the pending sub-pool in megabytes.
    #[arg(long = "txpool.pending-max-size", alias = "txpool.pending_max_size", default_value_t = TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT)]
    pub pending_max_size: usize,

    /// Max number of transaction in the basefee sub-pool
    #[arg(long = "txpool.basefee-max-count", alias = "txpool.basefee_max_count", default_value_t = TXPOOL_SUBPOOL_MAX_TXS_DEFAULT)]
    pub basefee_max_count: usize,
    /// Max size of the basefee sub-pool in megabytes.
    #[arg(long = "txpool.basefee-max-size", alias = "txpool.basefee_max_size", default_value_t = TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT)]
    pub basefee_max_size: usize,

    /// Max number of transaction in the queued sub-pool
    #[arg(long = "txpool.queued-max-count", alias = "txpool.queued_max_count", default_value_t = TXPOOL_SUBPOOL_MAX_TXS_DEFAULT)]
    pub queued_max_count: usize,
    /// Max size of the queued sub-pool in megabytes.
    #[arg(long = "txpool.queued-max-size", alias = "txpool.queued_max_size", default_value_t = TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT)]
    pub queued_max_size: usize,

    /// Max number of transaction in the blobpool
    #[arg(long = "txpool.blobpool-max-count", alias = "txpool.blobpool_max_count", default_value_t = TXPOOL_SUBPOOL_MAX_TXS_DEFAULT)]
    pub blobpool_max_count: usize,
    /// Max size of the blobpool in megabytes.
    #[arg(long = "txpool.blobpool-max-size", alias = "txpool.blobpool_max_size", default_value_t = TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT)]
    pub blobpool_max_size: usize,

    /// Max number of entries for the in memory cache of the blob store.
    #[arg(long = "txpool.blob-cache-size", alias = "txpool.blob_cache_size")]
    pub blob_cache_size: Option<u32>,

    /// Max number of executable transaction slots guaranteed per account
    #[arg(long = "txpool.max-account-slots", alias = "txpool.max_account_slots", default_value_t = TXPOOL_MAX_ACCOUNT_SLOTS_PER_SENDER)]
    pub max_account_slots: usize,

    /// Price bump (in %) for the transaction pool underpriced check.
    #[arg(long = "txpool.pricebump", default_value_t = DEFAULT_PRICE_BUMP)]
    pub price_bump: u128,

    /// Minimum base fee required by the protocol.
    #[arg(long = "txpool.minimal-protocol-fee", default_value_t = MIN_PROTOCOL_BASE_FEE)]
    pub minimal_protocol_basefee: u64,

    /// Minimum priority fee required for transaction acceptance into the pool.
    /// Transactions with priority fee below this value will be rejected.
    #[arg(long = "txpool.minimum-priority-fee")]
    pub minimum_priority_fee: Option<u128>,

    /// The default enforced gas limit for transactions entering the pool
    #[arg(long = "txpool.gas-limit", default_value_t = ETHEREUM_BLOCK_GAS_LIMIT_30M)]
    pub enforced_gas_limit: u64,

    /// Maximum gas limit for individual transactions. Transactions exceeding this limit will be
    /// rejected by the transaction pool
    #[arg(long = "txpool.max-tx-gas")]
    pub max_tx_gas_limit: Option<u64>,

    /// Price bump percentage to replace an already existing blob transaction
    #[arg(long = "blobpool.pricebump", default_value_t = REPLACE_BLOB_PRICE_BUMP)]
    pub blob_transaction_price_bump: u128,

    /// Max size in bytes of a single transaction allowed to enter the pool
    #[arg(long = "txpool.max-tx-input-bytes", alias = "txpool.max_tx_input_bytes", default_value_t = DEFAULT_MAX_TX_INPUT_BYTES)]
    pub max_tx_input_bytes: usize,

    /// The maximum number of blobs to keep in the in memory blob cache.
    #[arg(long = "txpool.max-cached-entries", alias = "txpool.max_cached_entries", default_value_t = DEFAULT_MAX_CACHED_BLOBS)]
    pub max_cached_entries: u32,

    /// Flag to disable local transaction exemptions.
    #[arg(long = "txpool.nolocals")]
    pub no_locals: bool,
    /// Flag to allow certain addresses as local.
    #[arg(long = "txpool.locals")]
    pub locals: Vec<Address>,
    /// Flag to toggle local transaction propagation.
    #[arg(long = "txpool.no-local-transactions-propagation")]
    pub no_local_transactions_propagation: bool,
    /// Number of additional transaction validation tasks to spawn.
    #[arg(long = "txpool.additional-validation-tasks", alias = "txpool.additional_validation_tasks", default_value_t = DEFAULT_TXPOOL_ADDITIONAL_VALIDATION_TASKS)]
    pub additional_validation_tasks: usize,

    /// Maximum number of pending transactions from the network to buffer
    #[arg(long = "txpool.max-pending-txns", alias = "txpool.max_pending_txns", default_value_t = PENDING_TX_LISTENER_BUFFER_SIZE)]
    pub pending_tx_listener_buffer_size: usize,

    /// Maximum number of new transactions to buffer
    #[arg(long = "txpool.max-new-txns", alias = "txpool.max_new_txns", default_value_t = NEW_TX_LISTENER_BUFFER_SIZE)]
    pub new_tx_listener_buffer_size: usize,

    /// How many new pending transactions to buffer and send to in progress pending transaction
    /// iterators.
    #[arg(long = "txpool.max-new-pending-txs-notifications", alias = "txpool.max-new-pending-txs-notifications", default_value_t = MAX_NEW_PENDING_TXS_NOTIFICATIONS)]
    pub max_new_pending_txs_notifications: usize,

    /// Maximum amount of time non-executable transaction are queued.
    #[arg(long = "txpool.lifetime", value_parser = parse_duration_from_secs_or_ms, default_value = "10800", value_name = "DURATION")]
    pub max_queued_lifetime: Duration,

    /// Path to store the local transaction backup at, to survive node restarts.
    #[arg(long = "txpool.transactions-backup", alias = "txpool.journal", value_name = "PATH")]
    pub transactions_backup_path: Option<std::path::PathBuf>,

    /// Disables transaction backup to disk on node shutdown.
    #[arg(
        long = "txpool.disable-transactions-backup",
        alias = "txpool.disable-journal",
        conflicts_with = "transactions_backup_path"
    )]
    pub disable_transactions_backup: bool,
}

impl Default for TxPoolArgs {
    fn default() -> Self {
        Self {
            pending_max_count: TXPOOL_SUBPOOL_MAX_TXS_DEFAULT,
            pending_max_size: TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT,
            basefee_max_count: TXPOOL_SUBPOOL_MAX_TXS_DEFAULT,
            basefee_max_size: TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT,
            queued_max_count: TXPOOL_SUBPOOL_MAX_TXS_DEFAULT,
            queued_max_size: TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT,
            blobpool_max_count: TXPOOL_SUBPOOL_MAX_TXS_DEFAULT,
            blobpool_max_size: TXPOOL_SUBPOOL_MAX_SIZE_MB_DEFAULT,
            blob_cache_size: None,
            max_account_slots: TXPOOL_MAX_ACCOUNT_SLOTS_PER_SENDER,
            price_bump: DEFAULT_PRICE_BUMP,
            minimal_protocol_basefee: MIN_PROTOCOL_BASE_FEE,
            minimum_priority_fee: None,
            enforced_gas_limit: ETHEREUM_BLOCK_GAS_LIMIT_30M,
            max_tx_gas_limit: None,
            blob_transaction_price_bump: REPLACE_BLOB_PRICE_BUMP,
            max_tx_input_bytes: DEFAULT_MAX_TX_INPUT_BYTES,
            max_cached_entries: DEFAULT_MAX_CACHED_BLOBS,
            no_locals: false,
            locals: Default::default(),
            no_local_transactions_propagation: false,
            additional_validation_tasks: DEFAULT_TXPOOL_ADDITIONAL_VALIDATION_TASKS,
            pending_tx_listener_buffer_size: PENDING_TX_LISTENER_BUFFER_SIZE,
            new_tx_listener_buffer_size: NEW_TX_LISTENER_BUFFER_SIZE,
            max_new_pending_txs_notifications: MAX_NEW_PENDING_TXS_NOTIFICATIONS,
            max_queued_lifetime: MAX_QUEUED_TRANSACTION_LIFETIME,
            transactions_backup_path: None,
            disable_transactions_backup: false,
        }
    }
}

impl RethTransactionPoolConfig for TxPoolArgs {
    /// Returns transaction pool configuration.
    fn pool_config(&self) -> PoolConfig {
        PoolConfig {
            local_transactions_config: LocalTransactionConfig {
                no_exemptions: self.no_locals,
                local_addresses: self.locals.clone().into_iter().collect(),
                propagate_local_transactions: !self.no_local_transactions_propagation,
            },
            pending_limit: SubPoolLimit {
                max_txs: self.pending_max_count,
                max_size: self.pending_max_size.saturating_mul(1024 * 1024),
            },
            basefee_limit: SubPoolLimit {
                max_txs: self.basefee_max_count,
                max_size: self.basefee_max_size.saturating_mul(1024 * 1024),
            },
            queued_limit: SubPoolLimit {
                max_txs: self.queued_max_count,
                max_size: self.queued_max_size.saturating_mul(1024 * 1024),
            },
            blob_limit: SubPoolLimit {
                max_txs: self.blobpool_max_count,
                max_size: self.blobpool_max_size.saturating_mul(1024 * 1024),
            },
            blob_cache_size: self.blob_cache_size,
            max_account_slots: self.max_account_slots,
            price_bumps: PriceBumpConfig {
                default_price_bump: self.price_bump,
                replace_blob_tx_price_bump: self.blob_transaction_price_bump,
            },
            minimal_protocol_basefee: self.minimal_protocol_basefee,
            minimum_priority_fee: self.minimum_priority_fee,
            gas_limit: self.enforced_gas_limit,
            pending_tx_listener_buffer_size: self.pending_tx_listener_buffer_size,
            new_tx_listener_buffer_size: self.new_tx_listener_buffer_size,
            max_new_pending_txs_notifications: self.max_new_pending_txs_notifications,
            max_queued_lifetime: self.max_queued_lifetime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// A helper type to parse Args more easily
    #[derive(Parser)]
    struct CommandParser<T: Args> {
        #[command(flatten)]
        args: T,
    }

    #[test]
    fn txpool_args_default_sanity_test() {
        let default_args = TxPoolArgs::default();
        let args = CommandParser::<TxPoolArgs>::parse_from(["reth"]).args;
        assert_eq!(args, default_args);
    }

    #[test]
    fn txpool_parse_locals() {
        let args = CommandParser::<TxPoolArgs>::parse_from([
            "reth",
            "--txpool.locals",
            "0x0000000000000000000000000000000000000000",
        ])
        .args;
        assert_eq!(args.locals, vec![Address::ZERO]);
    }

    #[test]
    fn txpool_parse_max_tx_lifetime() {
        // Test with a custom duration
        let args =
            CommandParser::<TxPoolArgs>::parse_from(["reth", "--txpool.lifetime", "300"]).args;
        assert_eq!(args.max_queued_lifetime, Duration::from_secs(300));

        // Test with the default value
        let args = CommandParser::<TxPoolArgs>::parse_from(["reth"]).args;
        assert_eq!(args.max_queued_lifetime, Duration::from_secs(3 * 60 * 60)); // Default is 3h
    }

    #[test]
    fn txpool_parse_max_tx_lifetime_invalid() {
        let result =
            CommandParser::<TxPoolArgs>::try_parse_from(["reth", "--txpool.lifetime", "invalid"]);

        assert!(result.is_err(), "Expected an error for invalid duration");
    }
}
