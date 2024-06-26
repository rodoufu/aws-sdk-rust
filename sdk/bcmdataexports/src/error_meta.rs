// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The specified Amazon Resource Name (ARN) in the request doesn't exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>You've reached the limit on the number of resources you can create, or exceeded the size of an individual resource.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints specified by an Amazon Web Services service.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(_) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl From<::aws_smithy_types::error::operation::BuildError> for Error {
    fn from(value: ::aws_smithy_types::error::operation::BuildError) -> Self {
        Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: value.into(),
            meta: ::std::default::Default::default(),
        })
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
    fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
        match self {
            Self::InternalServerException(inner) => inner.meta(),
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::ServiceQuotaExceededException(inner) => inner.meta(),
            Self::ThrottlingException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_export::CreateExportError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_export::CreateExportError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::create_export::CreateExportError> for Error {
    fn from(err: crate::operation::create_export::CreateExportError) -> Self {
        match err {
            crate::operation::create_export::CreateExportError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::create_export::CreateExportError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::create_export::CreateExportError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::create_export::CreateExportError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::create_export::CreateExportError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_export::DeleteExportError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_export::DeleteExportError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::delete_export::DeleteExportError> for Error {
    fn from(err: crate::operation::delete_export::DeleteExportError) -> Self {
        match err {
            crate::operation::delete_export::DeleteExportError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::delete_export::DeleteExportError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::delete_export::DeleteExportError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::delete_export::DeleteExportError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_export::DeleteExportError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_execution::GetExecutionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_execution::GetExecutionError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_execution::GetExecutionError> for Error {
    fn from(err: crate::operation::get_execution::GetExecutionError) -> Self {
        match err {
            crate::operation::get_execution::GetExecutionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_execution::GetExecutionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_execution::GetExecutionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_execution::GetExecutionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_execution::GetExecutionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_export::GetExportError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_export::GetExportError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_export::GetExportError> for Error {
    fn from(err: crate::operation::get_export::GetExportError) -> Self {
        match err {
            crate::operation::get_export::GetExportError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_export::GetExportError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_export::GetExportError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_export::GetExportError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_export::GetExportError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_table::GetTableError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_table::GetTableError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_table::GetTableError> for Error {
    fn from(err: crate::operation::get_table::GetTableError) -> Self {
        match err {
            crate::operation::get_table::GetTableError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_table::GetTableError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_table::GetTableError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_table::GetTableError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_executions::ListExecutionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_executions::ListExecutionsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_executions::ListExecutionsError> for Error {
    fn from(err: crate::operation::list_executions::ListExecutionsError) -> Self {
        match err {
            crate::operation::list_executions::ListExecutionsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_executions::ListExecutionsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_executions::ListExecutionsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_executions::ListExecutionsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_executions::ListExecutionsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_exports::ListExportsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_exports::ListExportsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_exports::ListExportsError> for Error {
    fn from(err: crate::operation::list_exports::ListExportsError) -> Self {
        match err {
            crate::operation::list_exports::ListExportsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_exports::ListExportsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_exports::ListExportsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_exports::ListExportsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tables::ListTablesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tables::ListTablesError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_tables::ListTablesError> for Error {
    fn from(err: crate::operation::list_tables::ListTablesError) -> Self {
        match err {
            crate::operation::list_tables::ListTablesError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_tables::ListTablesError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_tables::ListTablesError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_tables::ListTablesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_tags_for_resource::ListTagsForResourceError> for Error {
    fn from(err: crate::operation::list_tags_for_resource::ListTagsForResourceError) -> Self {
        match err {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::tag_resource::TagResourceError> for Error {
    fn from(err: crate::operation::tag_resource::TagResourceError) -> Self {
        match err {
            crate::operation::tag_resource::TagResourceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::tag_resource::TagResourceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::tag_resource::TagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::tag_resource::TagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::untag_resource::UntagResourceError> for Error {
    fn from(err: crate::operation::untag_resource::UntagResourceError) -> Self {
        match err {
            crate::operation::untag_resource::UntagResourceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::untag_resource::UntagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_export::UpdateExportError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_export::UpdateExportError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::update_export::UpdateExportError> for Error {
    fn from(err: crate::operation::update_export::UpdateExportError) -> Self {
        match err {
            crate::operation::update_export::UpdateExportError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::update_export::UpdateExportError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::update_export::UpdateExportError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::update_export::UpdateExportError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::update_export::UpdateExportError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ServiceQuotaExceededException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
        }
    }
}
impl ::aws_types::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
