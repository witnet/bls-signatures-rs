//! Errors returned by the bn256 library
use bn::{CurveError, FieldError, GroupError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to find a valid point while converting hash to point")]
    HashToPointError,
    #[error("Failed to get data from an index out of bounds")]
    IndexOutOfBounds,
    #[error("Failed to create group or field due to invalid input encoding")]
    InvalidEncoding,
    #[error("Failed to map point to a curve")]
    InvalidGroupPoint,
    #[error("Failed to create group or field due to invalid input length")]
    InvalidLength,
    #[error("Failed to create a field element")]
    NotMemberError,
    #[error("Failed to convert to affine coordinates")]
    ToAffineConversion,
    #[error("Point was already in affine coordinates (division-by-zero)")]
    PointInJacobian,
    #[error("BLS verification failed")]
    VerificationFailed,
}

impl From<CurveError> for Error {
    fn from(error: CurveError) -> Self {
        match error {
            CurveError::InvalidEncoding => Error::InvalidEncoding,
            CurveError::NotMember => Error::NotMemberError,
            CurveError::Field(field_error) => field_error.into(),
            CurveError::ToAffineConversion => Error::ToAffineConversion,
        }
    }
}

impl From<FieldError> for Error {
    fn from(error: FieldError) -> Self {
        match error {
            FieldError::NotMember => Error::NotMemberError,
            FieldError::InvalidSliceLength => Error::InvalidLength,
            FieldError::InvalidU512Encoding => Error::InvalidEncoding,
        }
    }
}

impl From<GroupError> for Error {
    fn from(_error: GroupError) -> Self {
        Error::InvalidGroupPoint
    }
}

impl From<bn::arith::Error> for Error {
    fn from(_error: bn::arith::Error) -> Self {
        Error::InvalidLength
    }
}
