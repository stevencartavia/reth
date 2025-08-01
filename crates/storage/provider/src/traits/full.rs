//! Helper provider traits to encapsulate all provider traits for simplicity.

use crate::{
    AccountReader, BlockReader, BlockReaderIdExt, ChainSpecProvider, ChangeSetReader,
    DatabaseProviderFactory, HashedPostStateProvider, StageCheckpointReader,
    StateCommitmentProvider, StateProviderFactory, StateReader, StaticFileProviderFactory,
};
use reth_chain_state::{CanonStateSubscriptions, ForkChoiceSubscriptions};
use reth_node_types::{BlockTy, HeaderTy, NodeTypesWithDB, ReceiptTy, TxTy};
use reth_storage_api::NodePrimitivesProvider;
use std::fmt::Debug;

/// Helper trait to unify all provider traits for simplicity.
pub trait FullProvider<N: NodeTypesWithDB>:
    DatabaseProviderFactory<DB = N::DB, Provider: BlockReader>
    + NodePrimitivesProvider<Primitives = N::Primitives>
    + StaticFileProviderFactory<Primitives = N::Primitives>
    + BlockReaderIdExt<
        Transaction = TxTy<N>,
        Block = BlockTy<N>,
        Receipt = ReceiptTy<N>,
        Header = HeaderTy<N>,
    > + AccountReader
    + StateProviderFactory
    + StateReader
    + StateCommitmentProvider
    + HashedPostStateProvider
    + ChainSpecProvider<ChainSpec = N::ChainSpec>
    + ChangeSetReader
    + CanonStateSubscriptions
    + ForkChoiceSubscriptions<Header = HeaderTy<N>>
    + StageCheckpointReader
    + Clone
    + Debug
    + Unpin
    + 'static
{
}

impl<T, N: NodeTypesWithDB> FullProvider<N> for T where
    T: DatabaseProviderFactory<DB = N::DB, Provider: BlockReader>
        + NodePrimitivesProvider<Primitives = N::Primitives>
        + StaticFileProviderFactory<Primitives = N::Primitives>
        + BlockReaderIdExt<
            Transaction = TxTy<N>,
            Block = BlockTy<N>,
            Receipt = ReceiptTy<N>,
            Header = HeaderTy<N>,
        > + AccountReader
        + StateProviderFactory
        + StateReader
        + StateCommitmentProvider
        + HashedPostStateProvider
        + ChainSpecProvider<ChainSpec = N::ChainSpec>
        + ChangeSetReader
        + CanonStateSubscriptions
        + ForkChoiceSubscriptions<Header = HeaderTy<N>>
        + StageCheckpointReader
        + Clone
        + Debug
        + Unpin
        + 'static
{
}
