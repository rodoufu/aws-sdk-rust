// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_data_set_notification::_send_data_set_notification_output::SendDataSetNotificationOutputBuilder;

pub use crate::operation::send_data_set_notification::_send_data_set_notification_input::SendDataSetNotificationInputBuilder;

impl crate::operation::send_data_set_notification::builders::SendDataSetNotificationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::send_data_set_notification::SendDataSetNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_data_set_notification::SendDataSetNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.send_data_set_notification();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SendDataSetNotification`.
///
/// <p>The type of event associated with the data set.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendDataSetNotificationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::send_data_set_notification::builders::SendDataSetNotificationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::send_data_set_notification::SendDataSetNotificationOutput,
        crate::operation::send_data_set_notification::SendDataSetNotificationError,
    > for SendDataSetNotificationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::send_data_set_notification::SendDataSetNotificationOutput,
            crate::operation::send_data_set_notification::SendDataSetNotificationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SendDataSetNotificationFluentBuilder {
    /// Creates a new `SendDataSetNotification`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SendDataSetNotification as a reference.
    pub fn as_input(&self) -> &crate::operation::send_data_set_notification::builders::SendDataSetNotificationInputBuilder {
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
        crate::operation::send_data_set_notification::SendDataSetNotificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_data_set_notification::SendDataSetNotificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::send_data_set_notification::SendDataSetNotification::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::send_data_set_notification::SendDataSetNotification::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::send_data_set_notification::SendDataSetNotificationOutput,
        crate::operation::send_data_set_notification::SendDataSetNotificationError,
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
    /// <p>Affected scope of this notification such as the underlying resources affected by the notification event.</p>
    pub fn scope(mut self, input: crate::types::ScopeDetails) -> Self {
        self.inner = self.inner.scope(input);
        self
    }
    /// <p>Affected scope of this notification such as the underlying resources affected by the notification event.</p>
    pub fn set_scope(mut self, input: ::std::option::Option<crate::types::ScopeDetails>) -> Self {
        self.inner = self.inner.set_scope(input);
        self
    }
    /// <p>Affected scope of this notification such as the underlying resources affected by the notification event.</p>
    pub fn get_scope(&self) -> &::std::option::Option<crate::types::ScopeDetails> {
        self.inner.get_scope()
    }
    /// <p>Idempotency key for the notification, this key allows us to deduplicate notifications that are sent in quick succession erroneously.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Idempotency key for the notification, this key allows us to deduplicate notifications that are sent in quick succession erroneously.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Idempotency key for the notification, this key allows us to deduplicate notifications that are sent in quick succession erroneously.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>Free-form text field for providers to add information about their notifications.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }
    /// <p>Free-form text field for providers to add information about their notifications.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
    /// <p>Free-form text field for providers to add information about their notifications.</p>
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_comment()
    }
    /// <p>Affected data set of the notification.</p>
    pub fn data_set_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_set_id(input.into());
        self
    }
    /// <p>Affected data set of the notification.</p>
    pub fn set_data_set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_set_id(input);
        self
    }
    /// <p>Affected data set of the notification.</p>
    pub fn get_data_set_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_set_id()
    }
    /// <p>Extra details specific to this notification type.</p>
    pub fn details(mut self, input: crate::types::NotificationDetails) -> Self {
        self.inner = self.inner.details(input);
        self
    }
    /// <p>Extra details specific to this notification type.</p>
    pub fn set_details(mut self, input: ::std::option::Option<crate::types::NotificationDetails>) -> Self {
        self.inner = self.inner.set_details(input);
        self
    }
    /// <p>Extra details specific to this notification type.</p>
    pub fn get_details(&self) -> &::std::option::Option<crate::types::NotificationDetails> {
        self.inner.get_details()
    }
    /// <p>The type of the notification. Describing the kind of event the notification is alerting you to.</p>
    pub fn r#type(mut self, input: crate::types::NotificationType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of the notification. Describing the kind of event the notification is alerting you to.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::NotificationType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of the notification. Describing the kind of event the notification is alerting you to.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::NotificationType> {
        self.inner.get_type()
    }
}
