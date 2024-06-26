// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_agent_action_group::_update_agent_action_group_output::UpdateAgentActionGroupOutputBuilder;

pub use crate::operation::update_agent_action_group::_update_agent_action_group_input::UpdateAgentActionGroupInputBuilder;

impl crate::operation::update_agent_action_group::builders::UpdateAgentActionGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_agent_action_group::UpdateAgentActionGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_agent_action_group::UpdateAgentActionGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_agent_action_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateAgentActionGroup`.
///
/// <p>Updates the configuration for an action group for an agent.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAgentActionGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_agent_action_group::builders::UpdateAgentActionGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_agent_action_group::UpdateAgentActionGroupOutput,
        crate::operation::update_agent_action_group::UpdateAgentActionGroupError,
    > for UpdateAgentActionGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_agent_action_group::UpdateAgentActionGroupOutput,
            crate::operation::update_agent_action_group::UpdateAgentActionGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAgentActionGroupFluentBuilder {
    /// Creates a new `UpdateAgentActionGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateAgentActionGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::update_agent_action_group::builders::UpdateAgentActionGroupInputBuilder {
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
        crate::operation::update_agent_action_group::UpdateAgentActionGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_agent_action_group::UpdateAgentActionGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_agent_action_group::UpdateAgentActionGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_agent_action_group::UpdateAgentActionGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_agent_action_group::UpdateAgentActionGroupOutput,
        crate::operation::update_agent_action_group::UpdateAgentActionGroupError,
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
    /// <p>The unique identifier of the agent for which to update the action group.</p>
    pub fn agent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_id(input.into());
        self
    }
    /// <p>The unique identifier of the agent for which to update the action group.</p>
    pub fn set_agent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_agent_id(input);
        self
    }
    /// <p>The unique identifier of the agent for which to update the action group.</p>
    pub fn get_agent_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_agent_id()
    }
    /// <p>The unique identifier of the agent version for which to update the action group.</p>
    pub fn agent_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_version(input.into());
        self
    }
    /// <p>The unique identifier of the agent version for which to update the action group.</p>
    pub fn set_agent_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_agent_version(input);
        self
    }
    /// <p>The unique identifier of the agent version for which to update the action group.</p>
    pub fn get_agent_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_agent_version()
    }
    /// <p>The unique identifier of the action group.</p>
    pub fn action_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action_group_id(input.into());
        self
    }
    /// <p>The unique identifier of the action group.</p>
    pub fn set_action_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_action_group_id(input);
        self
    }
    /// <p>The unique identifier of the action group.</p>
    pub fn get_action_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_action_group_id()
    }
    /// <p>Specifies a new name for the action group.</p>
    pub fn action_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action_group_name(input.into());
        self
    }
    /// <p>Specifies a new name for the action group.</p>
    pub fn set_action_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_action_group_name(input);
        self
    }
    /// <p>Specifies a new name for the action group.</p>
    pub fn get_action_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_action_group_name()
    }
    /// <p>Specifies a new name for the action group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Specifies a new name for the action group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Specifies a new name for the action group.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>To allow your agent to request the user for additional information when trying to complete a task, set this field to <code>AMAZON.UserInput</code>. You must leave the <code>description</code>, <code>apiSchema</code>, and <code>actionGroupExecutor</code> fields blank for this action group.</p>
    /// <p>During orchestration, if your agent determines that it needs to invoke an API in an action group, but doesn't have enough information to complete the API request, it will invoke this action group instead and return an <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_Observation.html">Observation</a> reprompting the user for more information.</p>
    pub fn parent_action_group_signature(mut self, input: crate::types::ActionGroupSignature) -> Self {
        self.inner = self.inner.parent_action_group_signature(input);
        self
    }
    /// <p>To allow your agent to request the user for additional information when trying to complete a task, set this field to <code>AMAZON.UserInput</code>. You must leave the <code>description</code>, <code>apiSchema</code>, and <code>actionGroupExecutor</code> fields blank for this action group.</p>
    /// <p>During orchestration, if your agent determines that it needs to invoke an API in an action group, but doesn't have enough information to complete the API request, it will invoke this action group instead and return an <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_Observation.html">Observation</a> reprompting the user for more information.</p>
    pub fn set_parent_action_group_signature(mut self, input: ::std::option::Option<crate::types::ActionGroupSignature>) -> Self {
        self.inner = self.inner.set_parent_action_group_signature(input);
        self
    }
    /// <p>To allow your agent to request the user for additional information when trying to complete a task, set this field to <code>AMAZON.UserInput</code>. You must leave the <code>description</code>, <code>apiSchema</code>, and <code>actionGroupExecutor</code> fields blank for this action group.</p>
    /// <p>During orchestration, if your agent determines that it needs to invoke an API in an action group, but doesn't have enough information to complete the API request, it will invoke this action group instead and return an <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_Observation.html">Observation</a> reprompting the user for more information.</p>
    pub fn get_parent_action_group_signature(&self) -> &::std::option::Option<crate::types::ActionGroupSignature> {
        self.inner.get_parent_action_group_signature()
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function containing the business logic that is carried out upon invoking the action.</p>
    pub fn action_group_executor(mut self, input: crate::types::ActionGroupExecutor) -> Self {
        self.inner = self.inner.action_group_executor(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function containing the business logic that is carried out upon invoking the action.</p>
    pub fn set_action_group_executor(mut self, input: ::std::option::Option<crate::types::ActionGroupExecutor>) -> Self {
        self.inner = self.inner.set_action_group_executor(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function containing the business logic that is carried out upon invoking the action.</p>
    pub fn get_action_group_executor(&self) -> &::std::option::Option<crate::types::ActionGroupExecutor> {
        self.inner.get_action_group_executor()
    }
    /// <p>Specifies whether the action group is available for the agent to invoke or not when sending an <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html">InvokeAgent</a> request.</p>
    pub fn action_group_state(mut self, input: crate::types::ActionGroupState) -> Self {
        self.inner = self.inner.action_group_state(input);
        self
    }
    /// <p>Specifies whether the action group is available for the agent to invoke or not when sending an <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html">InvokeAgent</a> request.</p>
    pub fn set_action_group_state(mut self, input: ::std::option::Option<crate::types::ActionGroupState>) -> Self {
        self.inner = self.inner.set_action_group_state(input);
        self
    }
    /// <p>Specifies whether the action group is available for the agent to invoke or not when sending an <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html">InvokeAgent</a> request.</p>
    pub fn get_action_group_state(&self) -> &::std::option::Option<crate::types::ActionGroupState> {
        self.inner.get_action_group_state()
    }
    /// <p>Contains either details about the S3 object containing the OpenAPI schema for the action group or the JSON or YAML-formatted payload defining the schema. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/agents-api-schema.html">Action group OpenAPI schemas</a>.</p>
    pub fn api_schema(mut self, input: crate::types::ApiSchema) -> Self {
        self.inner = self.inner.api_schema(input);
        self
    }
    /// <p>Contains either details about the S3 object containing the OpenAPI schema for the action group or the JSON or YAML-formatted payload defining the schema. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/agents-api-schema.html">Action group OpenAPI schemas</a>.</p>
    pub fn set_api_schema(mut self, input: ::std::option::Option<crate::types::ApiSchema>) -> Self {
        self.inner = self.inner.set_api_schema(input);
        self
    }
    /// <p>Contains either details about the S3 object containing the OpenAPI schema for the action group or the JSON or YAML-formatted payload defining the schema. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/agents-api-schema.html">Action group OpenAPI schemas</a>.</p>
    pub fn get_api_schema(&self) -> &::std::option::Option<crate::types::ApiSchema> {
        self.inner.get_api_schema()
    }
    /// <p>Contains details about the function schema for the action group or the JSON or YAML-formatted payload defining the schema.</p>
    pub fn function_schema(mut self, input: crate::types::FunctionSchema) -> Self {
        self.inner = self.inner.function_schema(input);
        self
    }
    /// <p>Contains details about the function schema for the action group or the JSON or YAML-formatted payload defining the schema.</p>
    pub fn set_function_schema(mut self, input: ::std::option::Option<crate::types::FunctionSchema>) -> Self {
        self.inner = self.inner.set_function_schema(input);
        self
    }
    /// <p>Contains details about the function schema for the action group or the JSON or YAML-formatted payload defining the schema.</p>
    pub fn get_function_schema(&self) -> &::std::option::Option<crate::types::FunctionSchema> {
        self.inner.get_function_schema()
    }
}
