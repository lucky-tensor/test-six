// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This file defines state store APIs that are related account state Merkle tree.

#[cfg(test)]
mod state_store_test;

use crate::{
    change_set::ChangeSet,
    ledger_counters::LedgerCounter,
    schema::{
        jellyfish_merkle_node::JellyfishMerkleNodeSchema, stale_node_index::StaleNodeIndexSchema,
    },
};
use anyhow::Result;
use jellyfish_merkle::{
    node_type::{LeafNode, Node, NodeKey},
    JellyfishMerkleTree, NodeBatch, TreeReader, TreeWriter,
};
use libra_crypto::{hash::CryptoHash, HashValue};
use libra_types::{
    account_address::AccountAddress,
    account_state_blob::AccountStateBlob,
    proof::{SparseMerkleProof, SparseMerkleRangeProof},
    transaction::Version,
};
use schemadb::{SchemaBatch, DB};
use std::{collections::HashMap, sync::Arc};

pub(crate) struct StateStore {
    db: Arc<DB>,
}

impl StateStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    /// Get the account state blob given account address and root hash of state Merkle tree
    pub fn get_account_state_with_proof_by_version(
        &self,
        address: AccountAddress,
        version: Version,
    ) -> Result<(Option<AccountStateBlob>, SparseMerkleProof)> {
        let (blob, proof) =
            JellyfishMerkleTree::new(self).get_with_proof(address.hash(), version)?;
        Ok((blob, proof))
    }

    /// Gets the proof that proves a range of accounts.
    pub fn get_account_state_range_proof(
        &self,
        rightmost_key: HashValue,
        version: Version,
    ) -> Result<SparseMerkleRangeProof> {
        JellyfishMerkleTree::new(self).get_range_proof(rightmost_key, version)
    }

    /// Put the results generated by `account_state_sets` to `batch` and return the result root
    /// hashes for each write set.
    pub fn put_account_state_sets(
        &self,
        account_state_sets: Vec<HashMap<AccountAddress, AccountStateBlob>>,
        first_version: Version,
        cs: &mut ChangeSet,
    ) -> Result<Vec<HashValue>> {
        let blob_sets = account_state_sets
            .into_iter()
            .map(|account_states| {
                account_states
                    .into_iter()
                    .map(|(addr, blob)| (addr.hash(), blob))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let (new_root_hash_vec, tree_update_batch) =
            JellyfishMerkleTree::new(self).put_blob_sets(blob_sets, first_version)?;

        cs.counter_bumps.bump(
            LedgerCounter::NewStateNodes,
            tree_update_batch.node_batch.len(),
        );
        cs.counter_bumps.bump(
            LedgerCounter::NewStateLeaves,
            tree_update_batch.num_new_leaves,
        );
        add_node_batch(&mut cs.batch, &tree_update_batch.node_batch)?;

        cs.counter_bumps.bump(
            LedgerCounter::StaleStateNodes,
            tree_update_batch.stale_node_index_batch.len(),
        );
        cs.counter_bumps.bump(
            LedgerCounter::StaleStateLeaves,
            tree_update_batch.num_stale_leaves,
        );
        tree_update_batch
            .stale_node_index_batch
            .iter()
            .map(|row| cs.batch.put::<StaleNodeIndexSchema>(row, &()))
            .collect::<Result<Vec<()>>>()?;

        Ok(new_root_hash_vec)
    }

    #[cfg(test)]
    pub fn get_root_hash(&self, version: Version) -> Result<HashValue> {
        JellyfishMerkleTree::new(self).get_root_hash(version)
    }
}

impl TreeReader for StateStore {
    fn get_node_option(&self, node_key: &NodeKey) -> Result<Option<Node>> {
        Ok(self.db.get::<JellyfishMerkleNodeSchema>(node_key)?)
    }

    fn get_rightmost_leaf(&self) -> Result<Option<(NodeKey, LeafNode)>> {
        // TODO(wqfish): implement this for real. For now we just assume we never crash in the
        // middle of restore, then this will only be called before anything is written to DB.
        Ok(None)
    }
}

impl TreeWriter for StateStore {
    fn write_node_batch(&self, node_batch: &NodeBatch) -> Result<()> {
        let mut batch = SchemaBatch::new();
        add_node_batch(&mut batch, node_batch)?;
        self.db.write_schemas(batch)
    }
}

fn add_node_batch(batch: &mut SchemaBatch, node_batch: &NodeBatch) -> Result<()> {
    node_batch
        .iter()
        .map(|(node_key, node)| batch.put::<JellyfishMerkleNodeSchema>(node_key, node))
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}
