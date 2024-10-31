use crate::TryFromProtoError;
use tap::Pipe;

use super::{proto_to_timestamp_ms, timestamp_ms_to_proto};

//
// CheckpointSummary
//

impl From<sui_sdk_types::types::CheckpointSummary> for super::CheckpointSummary {
    fn from(value: sui_sdk_types::types::CheckpointSummary) -> Self {
        Self {
            epoch: value.epoch,
            sequence_number: value.sequence_number,
            total_network_transactions: value.network_total_transactions,
            content_digest: Some(value.content_digest.into()),
            previous_digest: value.previous_digest.map(Into::into),
            epoch_rolling_gas_cost_summary: Some(value.epoch_rolling_gas_cost_summary.into()),
            timestamp: Some(timestamp_ms_to_proto(value.timestamp_ms)),
            commitments: value
                .checkpoint_commitments
                .into_iter()
                .map(Into::into)
                .collect(),
            end_of_epoch_data: value.end_of_epoch_data.map(Into::into),
            version_specific_data: value.version_specific_data.into(),
        }
    }
}

impl TryFrom<&super::CheckpointSummary> for sui_sdk_types::types::CheckpointSummary {
    type Error = TryFromProtoError;

    fn try_from(value: &super::CheckpointSummary) -> Result<Self, Self::Error> {
        let content_digest = value
            .content_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("content_digest"))?
            .pipe(TryInto::try_into)?;
        let previous_digest = value
            .previous_digest
            .as_ref()
            .map(TryInto::try_into)
            .transpose()?;
        let epoch_rolling_gas_cost_summary = value
            .epoch_rolling_gas_cost_summary
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("epoch_rolling_gas_cost_summary"))?
            .pipe(TryInto::try_into)?;

        let timestamp_ms = value
            .timestamp
            .ok_or_else(|| TryFromProtoError::missing("timestamp"))?
            .pipe(proto_to_timestamp_ms)?;

        let checkpoint_commitments = value
            .commitments
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let end_of_epoch_data = value
            .end_of_epoch_data
            .as_ref()
            .map(TryInto::try_into)
            .transpose()?;

        let version_specific_data = value.version_specific_data.to_vec();

        Ok(Self {
            epoch: value.epoch,
            sequence_number: value.sequence_number,
            network_total_transactions: value.total_network_transactions,
            content_digest,
            previous_digest,
            epoch_rolling_gas_cost_summary,
            timestamp_ms,
            checkpoint_commitments,
            end_of_epoch_data,
            version_specific_data,
        })
    }
}

//
// GasCostSummary
//

impl From<sui_sdk_types::types::GasCostSummary> for super::GasCostSummary {
    fn from(value: sui_sdk_types::types::GasCostSummary) -> Self {
        Self {
            computation_cost: value.computation_cost,
            storage_cost: value.storage_cost,
            storage_rebate: value.storage_rebate,
            non_refundable_storage_fee: value.non_refundable_storage_fee,
        }
    }
}

impl TryFrom<&super::GasCostSummary> for sui_sdk_types::types::GasCostSummary {
    type Error = TryFromProtoError;

    fn try_from(value: &super::GasCostSummary) -> Result<Self, Self::Error> {
        Ok(Self {
            computation_cost: value.computation_cost,
            storage_cost: value.storage_cost,
            storage_rebate: value.storage_rebate,
            non_refundable_storage_fee: value.non_refundable_storage_fee,
        })
    }
}

//
// CheckpointCommitment
//

impl From<sui_sdk_types::types::CheckpointCommitment> for super::CheckpointCommitment {
    fn from(value: sui_sdk_types::types::CheckpointCommitment) -> Self {
        let commitment = match value {
            sui_sdk_types::types::CheckpointCommitment::EcmhLiveObjectSet { digest } => {
                super::checkpoint_commitment::Commitment::EcmhLiveObjectSet(digest.into())
            }
        };

        Self {
            commitment: Some(commitment),
        }
    }
}

impl TryFrom<&super::CheckpointCommitment> for sui_sdk_types::types::CheckpointCommitment {
    type Error = TryFromProtoError;

    fn try_from(value: &super::CheckpointCommitment) -> Result<Self, Self::Error> {
        match value
            .commitment
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("commitment"))?
        {
            super::checkpoint_commitment::Commitment::EcmhLiveObjectSet(digest) => {
                Self::EcmhLiveObjectSet {
                    digest: digest.try_into()?,
                }
            }
        }
        .pipe(Ok)
    }
}

//
// EndOfEpochData
//

impl From<sui_sdk_types::types::EndOfEpochData> for super::EndOfEpochData {
    fn from(value: sui_sdk_types::types::EndOfEpochData) -> Self {
        Self {
            next_epoch_committee: value
                .next_epoch_committee
                .into_iter()
                .map(Into::into)
                .collect(),
            next_epoch_protocol_version: value.next_epoch_protocol_version,
            epoch_commitments: value
                .epoch_commitments
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<&super::EndOfEpochData> for sui_sdk_types::types::EndOfEpochData {
    type Error = TryFromProtoError;

    fn try_from(value: &super::EndOfEpochData) -> Result<Self, Self::Error> {
        Ok(Self {
            next_epoch_committee: value
                .next_epoch_committee
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            next_epoch_protocol_version: value.next_epoch_protocol_version,
            epoch_commitments: value
                .epoch_commitments
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

//
// CheckpointedTransactionInfo
//

impl From<sui_sdk_types::types::CheckpointTransactionInfo> for super::CheckpointedTransactionInfo {
    fn from(value: sui_sdk_types::types::CheckpointTransactionInfo) -> Self {
        Self {
            transaction: Some(value.transaction.into()),
            effects: Some(value.effects.into()),
            signatures: value.signatures.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&super::CheckpointedTransactionInfo>
    for sui_sdk_types::types::CheckpointTransactionInfo
{
    type Error = TryFromProtoError;

    fn try_from(value: &super::CheckpointedTransactionInfo) -> Result<Self, Self::Error> {
        let transaction = value
            .transaction
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("transaction"))?
            .pipe(TryInto::try_into)?;

        let effects = value
            .effects
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("effects"))?
            .pipe(TryInto::try_into)?;

        let signatures = value
            .signatures
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            transaction,
            effects,
            signatures,
        })
    }
}

//
// CheckpointContents
//

impl From<sui_sdk_types::types::CheckpointContents> for super::CheckpointContents {
    fn from(value: sui_sdk_types::types::CheckpointContents) -> Self {
        let contents = super::checkpoint_contents::Contents::V1(super::checkpoint_contents::V1 {
            transactions: value.into_v1().into_iter().map(Into::into).collect(),
        });

        Self {
            contents: Some(contents),
        }
    }
}

impl TryFrom<&super::CheckpointContents> for sui_sdk_types::types::CheckpointContents {
    type Error = TryFromProtoError;

    fn try_from(value: &super::CheckpointContents) -> Result<Self, Self::Error> {
        match value
            .contents
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("commitment"))?
        {
            super::checkpoint_contents::Contents::V1(v1) => Self::new(
                v1.transactions
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            ),
        }
        .pipe(Ok)
    }
}
