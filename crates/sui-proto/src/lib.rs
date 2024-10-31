
pub mod node;
pub mod types;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct TryFromProtoError {
    missing_field: Option<&'static str>,
    source: Option<BoxError>,
}

impl TryFromProtoError {
    fn missing(field: &'static str) -> Self {
        Self {
            missing_field: Some(field),
            source: None,
        }
    }

    fn from_error<E: Into<BoxError>>(error: E) -> Self {
        Self {
            missing_field: None,
            source: Some(error.into()),
        }
    }
}

impl From<std::num::TryFromIntError> for TryFromProtoError {
    fn from(value: std::num::TryFromIntError) -> Self {
        Self::from_error(value)
    }
}

impl From<std::str::Utf8Error> for TryFromProtoError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::from_error(value)
    }
}

impl From<std::array::TryFromSliceError> for TryFromProtoError {
    fn from(value: std::array::TryFromSliceError) -> Self {
        Self::from_error(value)
    }
}
