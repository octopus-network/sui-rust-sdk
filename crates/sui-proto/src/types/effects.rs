use crate::TryFromProtoError;
use tap::Pipe;

//
// TransactionEffects
//

impl From<sui_sdk_types::types::TransactionEffects> for super::TransactionEffects {
    fn from(value: sui_sdk_types::types::TransactionEffects) -> Self {
        use super::transaction_effects::Version;
        use sui_sdk_types::types::TransactionEffects::*;

        let version = match value {
            V1(v1) => Version::V1((*v1).into()),
            V2(v2) => Version::V2((*v2).into()),
        };

        Self {
            version: Some(version),
        }
    }
}

impl TryFrom<&super::TransactionEffects> for sui_sdk_types::types::TransactionEffects {
    type Error = TryFromProtoError;

    fn try_from(value: &super::TransactionEffects) -> Result<Self, Self::Error> {
        use super::transaction_effects::Version::*;

        match value
            .version
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("version"))?
        {
            V1(v1) => Self::V1(Box::new(v1.try_into()?)),
            V2(v2) => Self::V2(Box::new(v2.try_into()?)),
        }
        .pipe(Ok)
    }
}

//
// TransactionEffectsV1
//

impl From<sui_sdk_types::types::TransactionEffectsV1> for super::TransactionEffectsV1 {
    fn from(value: sui_sdk_types::types::TransactionEffectsV1) -> Self {
        Self {
            status: Some(value.status.into()),
            epoch: value.epoch,
            gas_used: Some(value.gas_used.into()),
            modified_at_versions: value
                .modified_at_versions
                .into_iter()
                .map(Into::into)
                .collect(),
            shared_objects: value.shared_objects.into_iter().map(Into::into).collect(),
            transaction_digest: Some(value.transaction_digest.into()),
            created: value.created.into_iter().map(Into::into).collect(),
            mutated: value.mutated.into_iter().map(Into::into).collect(),
            unwrapped: value.unwrapped.into_iter().map(Into::into).collect(),
            deleted: value.deleted.into_iter().map(Into::into).collect(),
            unwrapped_then_deleted: value
                .unwrapped_then_deleted
                .into_iter()
                .map(Into::into)
                .collect(),
            wrapped: value.wrapped.into_iter().map(Into::into).collect(),
            gas_object: Some(value.gas_object.into()),
            events_digest: value.events_digest.map(Into::into),
            dependencies: value.dependencies.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<&super::TransactionEffectsV1> for sui_sdk_types::types::TransactionEffectsV1 {
    type Error = TryFromProtoError;

    fn try_from(value: &super::TransactionEffectsV1) -> Result<Self, Self::Error> {
        let status = value
            .status
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("status"))?
            .pipe(TryInto::try_into)?;

        let gas_used = value
            .gas_used
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("gas_used"))?
            .pipe(TryInto::try_into)?;

        let transaction_digest = value
            .transaction_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("transaction_digest"))?
            .pipe(TryInto::try_into)?;

        let modified_at_versions = value
            .modified_at_versions
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let shared_objects = value
            .shared_objects
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let created = value
            .created
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let mutated = value
            .mutated
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let unwrapped = value
            .unwrapped
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let deleted = value
            .deleted
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let unwrapped_then_deleted = value
            .unwrapped_then_deleted
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let wrapped = value
            .wrapped
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let gas_object = value
            .gas_object
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("gas_object"))?
            .pipe(TryInto::try_into)?;

        let events_digest = value
            .events_digest
            .as_ref()
            .map(TryInto::try_into)
            .transpose()?;

        let dependencies = value
            .dependencies
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            status,
            epoch: value.epoch,
            gas_used,
            modified_at_versions,
            shared_objects,
            transaction_digest,
            created,
            mutated,
            unwrapped,
            deleted,
            unwrapped_then_deleted,
            wrapped,
            gas_object,
            events_digest,
            dependencies,
        })
    }
}

//
// TransactionEffectsV2
//

impl From<sui_sdk_types::types::TransactionEffectsV2> for super::TransactionEffectsV2 {
    fn from(value: sui_sdk_types::types::TransactionEffectsV2) -> Self {
        Self {
            status: Some(value.status.into()),
            epoch: value.epoch,
            gas_used: Some(value.gas_used.into()),
            transaction_digest: Some(value.transaction_digest.into()),
            gas_object_index: value.gas_object_index,
            events_digest: value.events_digest.map(Into::into),
            dependencies: value.dependencies.into_iter().map(Into::into).collect(),
            lamport_version: value.lamport_version,
            changed_objects: value.changed_objects.into_iter().map(Into::into).collect(),
            unchanged_shared_objects: value
                .unchanged_shared_objects
                .into_iter()
                .map(Into::into)
                .collect(),
            auxiliary_data_digest: value.auxiliary_data_digest.map(Into::into),
        }
    }
}

impl TryFrom<&super::TransactionEffectsV2> for sui_sdk_types::types::TransactionEffectsV2 {
    type Error = TryFromProtoError;

    fn try_from(value: &super::TransactionEffectsV2) -> Result<Self, Self::Error> {
        let status = value
            .status
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("status"))?
            .pipe(TryInto::try_into)?;

        let gas_used = value
            .gas_used
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("gas_used"))?
            .pipe(TryInto::try_into)?;

        let transaction_digest = value
            .transaction_digest
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("transaction_digest"))?
            .pipe(TryInto::try_into)?;

        let events_digest = value
            .events_digest
            .as_ref()
            .map(TryInto::try_into)
            .transpose()?;

        let dependencies = value
            .dependencies
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let changed_objects = value
            .changed_objects
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let unchanged_shared_objects = value
            .unchanged_shared_objects
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let auxiliary_data_digest = value
            .auxiliary_data_digest
            .as_ref()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            status,
            epoch: value.epoch,
            gas_used,
            transaction_digest,
            gas_object_index: value.gas_object_index,
            events_digest,
            dependencies,
            lamport_version: value.lamport_version,
            changed_objects,
            unchanged_shared_objects,
            auxiliary_data_digest,
        })
    }
}

//
// ModifiedAtVersion
//

impl From<sui_sdk_types::types::ModifiedAtVersion> for super::ModifiedAtVersion {
    fn from(value: sui_sdk_types::types::ModifiedAtVersion) -> Self {
        Self {
            object_id: Some(value.object_id.into()),
            version: value.version,
        }
    }
}

impl TryFrom<&super::ModifiedAtVersion> for sui_sdk_types::types::ModifiedAtVersion {
    type Error = TryFromProtoError;

    fn try_from(value: &super::ModifiedAtVersion) -> Result<Self, Self::Error> {
        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .pipe(TryInto::try_into)?;

        Ok(Self {
            object_id,
            version: value.version,
        })
    }
}

//
// ObjectReferenceWithOwner
//

impl From<sui_sdk_types::types::ObjectReferenceWithOwner> for super::ObjectReferenceWithOwner {
    fn from(value: sui_sdk_types::types::ObjectReferenceWithOwner) -> Self {
        Self {
            reference: Some(value.reference.into()),
            owner: Some(value.owner.into()),
        }
    }
}

impl TryFrom<&super::ObjectReferenceWithOwner> for sui_sdk_types::types::ObjectReferenceWithOwner {
    type Error = TryFromProtoError;

    fn try_from(value: &super::ObjectReferenceWithOwner) -> Result<Self, Self::Error> {
        let reference = value
            .reference
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("reference"))?
            .pipe(TryInto::try_into)?;

        let owner = value
            .owner
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("owner"))?
            .pipe(TryInto::try_into)?;

        Ok(Self { reference, owner })
    }
}

//
// ChangedObject
//

impl From<sui_sdk_types::types::ChangedObject> for super::ChangedObject {
    fn from(value: sui_sdk_types::types::ChangedObject) -> Self {
        Self {
            object_id: Some(value.object_id.into()),
            input_state: Some(value.change.input_state.into()),
            output_state: Some(value.change.output_state.into()),
            id_operation: super::IdOperation::from(value.change.id_operation) as i32,
        }
    }
}

impl TryFrom<&super::ChangedObject> for sui_sdk_types::types::ChangedObject {
    type Error = TryFromProtoError;

    fn try_from(value: &super::ChangedObject) -> Result<Self, Self::Error> {
        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .pipe(TryInto::try_into)?;

        let input_state = value
            .input_state
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("input_state"))?
            .pipe(TryInto::try_into)?;

        let output_state = value
            .output_state
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("output_state"))?
            .pipe(TryInto::try_into)?;

        let id_operation = value.id_operation().try_into()?;

        Ok(Self {
            object_id,
            change: sui_sdk_types::types::EffectsObjectChange {
                input_state,
                output_state,
                id_operation,
            },
        })
    }
}

//
// InputState
//

impl From<sui_sdk_types::types::ObjectIn> for super::changed_object::InputState {
    fn from(value: sui_sdk_types::types::ObjectIn) -> Self {
        match value {
            sui_sdk_types::types::ObjectIn::NotExist => Self::NotExist(()),
            sui_sdk_types::types::ObjectIn::Exist {
                version,
                digest,
                owner,
            } => Self::Exist(super::ObjectExist {
                version,
                digest: Some(digest.into()),
                owner: Some(owner.into()),
            }),
        }
    }
}

impl TryFrom<&super::changed_object::InputState> for sui_sdk_types::types::ObjectIn {
    type Error = TryFromProtoError;

    fn try_from(value: &super::changed_object::InputState) -> Result<Self, Self::Error> {
        use super::changed_object::InputState::*;

        match value {
            NotExist(()) => Self::NotExist,
            Exist(super::ObjectExist {
                version,
                digest,
                owner,
            }) => Self::Exist {
                version: *version,
                digest: digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .pipe(TryInto::try_into)?,
                owner: owner
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("owner"))?
                    .pipe(TryInto::try_into)?,
            },
        }
        .pipe(Ok)
    }
}

//
// OutputState
//

impl From<sui_sdk_types::types::ObjectOut> for super::changed_object::OutputState {
    fn from(value: sui_sdk_types::types::ObjectOut) -> Self {
        use sui_sdk_types::types::ObjectOut::*;
        match value {
            NotExist => Self::Removed(()),
            ObjectWrite { digest, owner } => Self::ObjectWrite(super::ObjectWrite {
                digest: Some(digest.into()),
                owner: Some(owner.into()),
            }),
            PackageWrite { version, digest } => Self::PackageWrite(super::PackageWrite {
                version,
                digest: Some(digest.into()),
            }),
        }
    }
}

impl TryFrom<&super::changed_object::OutputState> for sui_sdk_types::types::ObjectOut {
    type Error = TryFromProtoError;

    fn try_from(value: &super::changed_object::OutputState) -> Result<Self, Self::Error> {
        use super::changed_object::OutputState::*;

        match value {
            Removed(()) => Self::NotExist,
            ObjectWrite(super::ObjectWrite { digest, owner }) => Self::ObjectWrite {
                digest: digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .pipe(TryInto::try_into)?,

                owner: owner
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("owner"))?
                    .pipe(TryInto::try_into)?,
            },
            PackageWrite(super::PackageWrite { version, digest }) => Self::PackageWrite {
                version: *version,
                digest: digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .pipe(TryInto::try_into)?,
            },
        }
        .pipe(Ok)
    }
}

//
// IdOperation
//

impl From<sui_sdk_types::types::IdOperation> for super::IdOperation {
    fn from(value: sui_sdk_types::types::IdOperation) -> Self {
        use sui_sdk_types::types::IdOperation::*;

        match value {
            None => Self::None,
            Created => Self::Created,
            Deleted => Self::Deleted,
        }
    }
}

impl TryFrom<super::IdOperation> for sui_sdk_types::types::IdOperation {
    type Error = TryFromProtoError;

    fn try_from(value: super::IdOperation) -> Result<Self, Self::Error> {
        match value {
            super::IdOperation::Unknown => Err(TryFromProtoError::missing("unknown id operation")),
            super::IdOperation::None => Ok(Self::None),
            super::IdOperation::Created => Ok(Self::Created),
            super::IdOperation::Deleted => Ok(Self::Deleted),
        }
    }
}

//
// UnchangedSharedObject
//

impl From<sui_sdk_types::types::UnchangedSharedObject> for super::UnchangedSharedObject {
    fn from(value: sui_sdk_types::types::UnchangedSharedObject) -> Self {
        Self {
            object_id: Some(value.object_id.into()),
            kind: Some(value.kind.into()),
        }
    }
}

impl TryFrom<&super::UnchangedSharedObject> for sui_sdk_types::types::UnchangedSharedObject {
    type Error = TryFromProtoError;

    fn try_from(value: &super::UnchangedSharedObject) -> Result<Self, Self::Error> {
        let object_id = value
            .object_id
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("object_id"))?
            .pipe(TryInto::try_into)?;

        let kind = value
            .kind
            .as_ref()
            .ok_or_else(|| TryFromProtoError::missing("kind"))?
            .pipe(TryInto::try_into)?;

        Ok(Self { object_id, kind })
    }
}

//
// UnchangedSharedKind
//

impl From<sui_sdk_types::types::UnchangedSharedKind> for super::unchanged_shared_object::Kind {
    fn from(value: sui_sdk_types::types::UnchangedSharedKind) -> Self {
        use sui_sdk_types::types::UnchangedSharedKind::*;

        match value {
            ReadOnlyRoot { version, digest } => Self::ReadOnlyRoot(super::ReadOnlyRoot {
                version,
                digest: Some(digest.into()),
            }),
            MutateDeleted { version } => Self::MutateDeleted(version),
            ReadDeleted { version } => Self::ReadDeleted(version),
            Cancelled { version } => Self::Cancelled(version),
            PerEpochConfig => Self::PerEpochConfig(()),
        }
    }
}

impl TryFrom<&super::unchanged_shared_object::Kind> for sui_sdk_types::types::UnchangedSharedKind {
    type Error = TryFromProtoError;

    fn try_from(value: &super::unchanged_shared_object::Kind) -> Result<Self, Self::Error> {
        use super::unchanged_shared_object::Kind::*;

        match value {
            ReadOnlyRoot(super::ReadOnlyRoot { version, digest }) => Self::ReadOnlyRoot {
                version: *version,

                digest: digest
                    .as_ref()
                    .ok_or_else(|| TryFromProtoError::missing("digest"))?
                    .pipe(TryInto::try_into)?,
            },
            MutateDeleted(version) => Self::MutateDeleted { version: *version },
            ReadDeleted(version) => Self::ReadDeleted { version: *version },
            Cancelled(version) => Self::Cancelled { version: *version },
            PerEpochConfig(()) => Self::PerEpochConfig,
        }
        .pipe(Ok)
    }
}
