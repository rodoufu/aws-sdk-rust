// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The request is denied because of missing access permissions. Check your permissions and retry your request.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>There was an issue with a dependency due to a server issue. Retry your request.</p>
    BadGatewayException(crate::types::error::BadGatewayException),
    /// <p>There was a conflict performing an operation. Resolve the conflict and retry your request.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>There was an issue with a dependency. Check the resource configurations and retry the request.</p>
    DependencyFailedException(crate::types::error::DependencyFailedException),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The specified resource Amazon Resource Name (ARN) was not found. Check the Amazon Resource Name (ARN) and try your request again.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The number of requests exceeds the service quota. Resubmit your request later.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>Input validation failed. Check your request parameters and retry the request.</p>
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
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::BadGatewayException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::DependencyFailedException(inner) => inner.fmt(f),
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
            Self::AccessDeniedException(inner) => inner.meta(),
            Self::BadGatewayException(inner) => inner.meta(),
            Self::ConflictException(inner) => inner.meta(),
            Self::DependencyFailedException(inner) => inner.meta(),
            Self::InternalServerException(inner) => inner.meta(),
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::ServiceQuotaExceededException(inner) => inner.meta(),
            Self::ThrottlingException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::invoke_agent::InvokeAgentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::invoke_agent::InvokeAgentError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::invoke_agent::InvokeAgentError> for Error {
    fn from(err: crate::operation::invoke_agent::InvokeAgentError) -> Self {
        match err {
            crate::operation::invoke_agent::InvokeAgentError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::invoke_agent::InvokeAgentError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::invoke_agent::InvokeAgentError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::invoke_agent::InvokeAgentError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::invoke_agent::InvokeAgentError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::invoke_agent::InvokeAgentError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::invoke_agent::InvokeAgentError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::invoke_agent::InvokeAgentError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::invoke_agent::InvokeAgentError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::invoke_agent::InvokeAgentError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::retrieve::RetrieveError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::retrieve::RetrieveError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::retrieve::RetrieveError> for Error {
    fn from(err: crate::operation::retrieve::RetrieveError) -> Self {
        match err {
            crate::operation::retrieve::RetrieveError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::retrieve::RetrieveError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::retrieve::RetrieveError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::retrieve::RetrieveError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::retrieve::RetrieveError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::retrieve::RetrieveError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::retrieve::RetrieveError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::retrieve::RetrieveError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::retrieve::RetrieveError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::retrieve::RetrieveError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::retrieve_and_generate::RetrieveAndGenerateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::retrieve_and_generate::RetrieveAndGenerateError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::retrieve_and_generate::RetrieveAndGenerateError> for Error {
    fn from(err: crate::operation::retrieve_and_generate::RetrieveAndGenerateError) -> Self {
        match err {
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::DependencyFailedException(inner) => {
                Error::DependencyFailedException(inner)
            }
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::retrieve_and_generate::RetrieveAndGenerateError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::ResponseStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::ResponseStreamError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::types::error::ResponseStreamError> for Error {
    fn from(err: crate::types::error::ResponseStreamError) -> Self {
        match err {
            crate::types::error::ResponseStreamError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::types::error::ResponseStreamError::ValidationException(inner) => Error::ValidationException(inner),
            crate::types::error::ResponseStreamError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::types::error::ResponseStreamError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::types::error::ResponseStreamError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::types::error::ResponseStreamError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::types::error::ResponseStreamError::ConflictException(inner) => Error::ConflictException(inner),
            crate::types::error::ResponseStreamError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::types::error::ResponseStreamError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::types::error::ResponseStreamError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::BadGatewayException(inner) => inner.source(),
            Error::ConflictException(inner) => inner.source(),
            Error::DependencyFailedException(inner) => inner.source(),
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
            Self::AccessDeniedException(e) => e.request_id(),
            Self::BadGatewayException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::DependencyFailedException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
