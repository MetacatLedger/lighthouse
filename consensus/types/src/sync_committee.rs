use crate::test_utils::TestRandom;
use crate::typenum::Unsigned;
use crate::{EthSpec, FixedVector};
use bls::PublicKeyBytes;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

#[cfg_attr(feature = "arbitrary-fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash, TestRandom)]
#[serde(bound = "T: EthSpec")]
pub struct SyncCommittee<T: EthSpec> {
    pub pubkeys: FixedVector<PublicKeyBytes, T::SyncCommitteeSize>,
    pub pubkey_aggregates: FixedVector<PublicKeyBytes, T::SyncAggregateSize>,
}

impl<T: EthSpec> SyncCommittee<T> {
    /// Create a temporary sync committee that should *never* be used.
    pub fn temporary() -> Result<Self, ssz_types::Error> {
        Ok(Self {
            pubkeys: FixedVector::new(vec![
                PublicKeyBytes::empty();
                T::SyncCommitteeSize::to_usize()
            ])?,
            pubkey_aggregates: FixedVector::new(vec![
                PublicKeyBytes::empty();
                T::SyncAggregateSize::to_usize()
            ])?,
        })
    }
}
