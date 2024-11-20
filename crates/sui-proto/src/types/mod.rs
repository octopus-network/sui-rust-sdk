use crate::TryFromProtoError;

#[path = "../generated/sui.types.rs"]
pub mod generated;
pub use generated::*;

mod object;
mod events;
mod checkpoint;
mod transaction_convert;
mod effects;
mod execution_status;
mod move_types;
mod signatures;

//
// Address
//

impl From<sui_sdk_types::types::Address> for Address {
    fn from(value: sui_sdk_types::types::Address) -> Self {
        Self {
            address: value.as_bytes().to_vec().into(),
        }
    }
}

impl TryFrom<&Address> for sui_sdk_types::types::Address {
    type Error = TryFromProtoError;

    fn try_from(value: &Address) -> Result<Self, Self::Error> {
        Ok(Self::new(value.address.as_ref().try_into()?))
    }
}

//
// ObjectId
//

impl From<sui_sdk_types::types::ObjectId> for ObjectId {
    fn from(value: sui_sdk_types::types::ObjectId) -> Self {
        Self {
            object_id: value.as_bytes().to_vec().into(),
        }
    }
}

impl TryFrom<&ObjectId> for sui_sdk_types::types::ObjectId {
    type Error = TryFromProtoError;

    fn try_from(value: &ObjectId) -> Result<Self, Self::Error> {
        Ok(Self::new(value.object_id.as_ref().try_into()?))
    }
}

//
// Digest
//

impl From<sui_sdk_types::types::Digest> for Digest {
    fn from(value: sui_sdk_types::types::Digest) -> Self {
        Self {
            digest: value.as_bytes().to_vec().into(),
        }
    }
}

impl TryFrom<&Digest> for sui_sdk_types::types::Digest {
    type Error = TryFromProtoError;

    fn try_from(value: &Digest) -> Result<Self, Self::Error> {
        Ok(Self::new(value.digest.as_ref().try_into()?))
    }
}

macro_rules! impl_digest_proto {
    ($t:ident) => {
        impl From<sui_sdk_types::types::$t> for Digest {
            fn from(value: sui_sdk_types::types::$t) -> Self {
                Self {
                    digest: value.as_bytes().to_vec().into(),
                }
            }
        }

        impl TryFrom<&Digest> for sui_sdk_types::types::$t {
            type Error = TryFromProtoError;

            fn try_from(value: &Digest) -> Result<Self, Self::Error> {
                Ok(Self::new(value.digest.as_ref().try_into()?))
            }
        }
    };
}

impl_digest_proto!(CheckpointDigest);
impl_digest_proto!(CheckpointContentsDigest);
impl_digest_proto!(TransactionDigest);
impl_digest_proto!(TransactionEffectsDigest);
impl_digest_proto!(TransactionEventsDigest);
impl_digest_proto!(ObjectDigest);
impl_digest_proto!(ConsensusCommitDigest);
impl_digest_proto!(EffectsAuxiliaryDataDigest);

//
// TimeStamp
//

pub fn timestamp_ms_to_proto(timestamp_ms: u64) -> prost_types::Timestamp {
    let timestamp = std::time::Duration::from_millis(timestamp_ms);
    prost_types::Timestamp {
        seconds: timestamp.as_secs() as i64,
        nanos: timestamp.subsec_nanos() as i32,
    }
}

pub fn proto_to_timestamp_ms(timestamp: prost_types::Timestamp) -> Result<u64, TryFromProtoError> {
    let seconds = std::time::Duration::from_secs(timestamp.seconds.try_into()?);
    let nanos = std::time::Duration::from_nanos(timestamp.nanos.try_into()?);

    Ok((seconds + nanos).as_millis().try_into()?)
}
