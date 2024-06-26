// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_prepared_statement::_get_prepared_statement_output::GetPreparedStatementOutputBuilder;

pub use crate::operation::get_prepared_statement::_get_prepared_statement_input::GetPreparedStatementInputBuilder;

impl crate::operation::get_prepared_statement::builders::GetPreparedStatementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_prepared_statement::GetPreparedStatementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_prepared_statement::GetPreparedStatementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_prepared_statement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPreparedStatement`.
///
/// <p>Retrieves the prepared statement with the specified name from the specified workgroup.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPreparedStatementFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_prepared_statement::builders::GetPreparedStatementInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_prepared_statement::GetPreparedStatementOutput,
        crate::operation::get_prepared_statement::GetPreparedStatementError,
    > for GetPreparedStatementFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_prepared_statement::GetPreparedStatementOutput,
            crate::operation::get_prepared_statement::GetPreparedStatementError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPreparedStatementFluentBuilder {
    /// Creates a new `GetPreparedStatement`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPreparedStatement as a reference.
    pub fn as_input(&self) -> &crate::operation::get_prepared_statement::builders::GetPreparedStatementInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_prepared_statement::GetPreparedStatementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_prepared_statement::GetPreparedStatementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_prepared_statement::GetPreparedStatement::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_prepared_statement::GetPreparedStatement::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_prepared_statement::GetPreparedStatementOutput,
        crate::operation::get_prepared_statement::GetPreparedStatementError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the prepared statement to retrieve.</p>
    pub fn statement_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.statement_name(input.into());
        self
    }
    /// <p>The name of the prepared statement to retrieve.</p>
    pub fn set_statement_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_statement_name(input);
        self
    }
    /// <p>The name of the prepared statement to retrieve.</p>
    pub fn get_statement_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_statement_name()
    }
    /// <p>The workgroup to which the statement to be retrieved belongs.</p>
    pub fn work_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.work_group(input.into());
        self
    }
    /// <p>The workgroup to which the statement to be retrieved belongs.</p>
    pub fn set_work_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_work_group(input);
        self
    }
    /// <p>The workgroup to which the statement to be retrieved belongs.</p>
    pub fn get_work_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_work_group()
    }
}
