//! Internal errors for the tree module.

use alloy_consensus::BlockHeader;
use alloy_primitives::B256;
use reth_consensus::ConsensusError;
use reth_errors::{BlockExecutionError, BlockValidationError, ProviderError};
use reth_evm::execute::InternalBlockExecutionError;
use reth_payload_primitives::NewPayloadError;
use reth_primitives_traits::{Block, BlockBody, SealedBlock};
use tokio::sync::oneshot::error::TryRecvError;

/// This is an error that can come from advancing persistence. Either this can be a
/// [`TryRecvError`], or this can be a [`ProviderError`]
#[derive(Debug, thiserror::Error)]
pub enum AdvancePersistenceError {
    /// An error that can be from failing to receive a value from persistence
    #[error(transparent)]
    RecvError(#[from] TryRecvError),
    /// A provider error
    #[error(transparent)]
    Provider(#[from] ProviderError),
    /// Missing ancestor.
    ///
    /// This error occurs when we need to compute the state root for a block with missing trie
    /// updates, but the ancestor block is not available. State root computation requires the state
    /// from the parent block as a starting point.
    ///
    /// A block may be missing the trie updates when it's a fork chain block building on top of the
    /// historical database state. Since we don't store the historical trie state, we cannot
    /// generate the trie updates for it until the moment when database is unwound to the canonical
    /// chain.
    ///
    /// Also see [`reth_chain_state::ExecutedTrieUpdates::Missing`].
    #[error("Missing ancestor with hash {0}")]
    MissingAncestor(B256),
}

#[derive(thiserror::Error)]
#[error("Failed to insert block (hash={}, number={}, parent_hash={}): {}",
    .block.hash(),
    .block.number(),
    .block.parent_hash(),
    .kind)]
struct InsertBlockErrorData<B: Block> {
    block: SealedBlock<B>,
    #[source]
    kind: InsertBlockErrorKind,
}

impl<B: Block> std::fmt::Debug for InsertBlockErrorData<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InsertBlockError")
            .field("error", &self.kind)
            .field("hash", &self.block.hash())
            .field("number", &self.block.number())
            .field("parent_hash", &self.block.parent_hash())
            .field("num_txs", &self.block.body().transactions().len())
            .finish_non_exhaustive()
    }
}

impl<B: Block> InsertBlockErrorData<B> {
    const fn new(block: SealedBlock<B>, kind: InsertBlockErrorKind) -> Self {
        Self { block, kind }
    }

    fn boxed(block: SealedBlock<B>, kind: InsertBlockErrorKind) -> Box<Self> {
        Box::new(Self::new(block, kind))
    }
}

/// Error thrown when inserting a block failed because the block is considered invalid.
#[derive(thiserror::Error)]
#[error(transparent)]
pub struct InsertBlockError<B: Block> {
    inner: Box<InsertBlockErrorData<B>>,
}

// === impl InsertBlockErrorTwo ===

impl<B: Block> InsertBlockError<B> {
    /// Create a new `InsertInvalidBlockErrorTwo`
    pub fn new(block: SealedBlock<B>, kind: InsertBlockErrorKind) -> Self {
        Self { inner: InsertBlockErrorData::boxed(block, kind) }
    }

    /// Create a new `InsertInvalidBlockError` from a consensus error
    pub fn consensus_error(error: ConsensusError, block: SealedBlock<B>) -> Self {
        Self::new(block, InsertBlockErrorKind::Consensus(error))
    }

    /// Consumes the error and returns the block that resulted in the error
    #[inline]
    pub fn into_block(self) -> SealedBlock<B> {
        self.inner.block
    }

    /// Returns the error kind
    #[inline]
    pub const fn kind(&self) -> &InsertBlockErrorKind {
        &self.inner.kind
    }

    /// Returns the block that resulted in the error
    #[inline]
    pub const fn block(&self) -> &SealedBlock<B> {
        &self.inner.block
    }

    /// Consumes the type and returns the block and error kind.
    #[inline]
    pub fn split(self) -> (SealedBlock<B>, InsertBlockErrorKind) {
        let inner = *self.inner;
        (inner.block, inner.kind)
    }
}

impl<B: Block> std::fmt::Debug for InsertBlockError<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

/// All error variants possible when inserting a block
#[derive(Debug, thiserror::Error)]
pub enum InsertBlockErrorKind {
    /// Block violated consensus rules.
    #[error(transparent)]
    Consensus(#[from] ConsensusError),
    /// Block execution failed.
    #[error(transparent)]
    Execution(#[from] BlockExecutionError),
    /// Provider error.
    #[error(transparent)]
    Provider(#[from] ProviderError),
    /// Other errors.
    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
}

impl InsertBlockErrorKind {
    /// Returns an [`InsertBlockValidationError`] if the error is caused by an invalid block.
    ///
    /// Returns an [`InsertBlockFatalError`] if the error is caused by an error that is not
    /// validation related or is otherwise fatal.
    ///
    /// This is intended to be used to determine if we should respond `INVALID` as a response when
    /// processing a new block.
    pub fn ensure_validation_error(
        self,
    ) -> Result<InsertBlockValidationError, InsertBlockFatalError> {
        match self {
            Self::Consensus(err) => Ok(InsertBlockValidationError::Consensus(err)),
            // other execution errors that are considered internal errors
            Self::Execution(err) => {
                match err {
                    BlockExecutionError::Validation(err) => {
                        Ok(InsertBlockValidationError::Validation(err))
                    }
                    // these are internal errors, not caused by an invalid block
                    BlockExecutionError::Internal(error) => {
                        Err(InsertBlockFatalError::BlockExecutionError(error))
                    }
                }
            }
            Self::Provider(err) => Err(InsertBlockFatalError::Provider(err)),
            Self::Other(err) => Err(InternalBlockExecutionError::Other(err).into()),
        }
    }
}

/// Error variants that are not caused by invalid blocks
#[derive(Debug, thiserror::Error)]
pub enum InsertBlockFatalError {
    /// A provider error
    #[error(transparent)]
    Provider(#[from] ProviderError),
    /// An internal / fatal block execution error
    #[error(transparent)]
    BlockExecutionError(#[from] InternalBlockExecutionError),
}

/// Error variants that are caused by invalid blocks
#[derive(Debug, thiserror::Error)]
pub enum InsertBlockValidationError {
    /// Block violated consensus rules.
    #[error(transparent)]
    Consensus(#[from] ConsensusError),
    /// Validation error, transparently wrapping [`BlockValidationError`]
    #[error(transparent)]
    Validation(#[from] BlockValidationError),
}

/// Errors that may occur when inserting a payload.
#[derive(Debug, thiserror::Error)]
pub enum InsertPayloadError<B: Block> {
    /// Block validation error
    #[error(transparent)]
    Block(#[from] InsertBlockError<B>),
    /// Payload validation error
    #[error(transparent)]
    Payload(#[from] NewPayloadError),
}
