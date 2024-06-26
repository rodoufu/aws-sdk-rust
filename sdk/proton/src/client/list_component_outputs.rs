// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListComponentOutputs`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`component_name(impl Into<String>)`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::component_name) / [`set_component_name(Option<String>)`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::set_component_name):<br>required: **true**<br><p>The name of the component whose outputs you want.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::set_next_token):<br>required: **false**<br><p>A token that indicates the location of the next output in the array of outputs, after the list of outputs that was previously requested.</p><br>
    ///   - [`deployment_id(impl Into<String>)`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::deployment_id) / [`set_deployment_id(Option<String>)`](crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::set_deployment_id):<br>required: **false**<br><p>The ID of the deployment whose outputs you want.</p><br>
    /// - On success, responds with [`ListComponentOutputsOutput`](crate::operation::list_component_outputs::ListComponentOutputsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_component_outputs::ListComponentOutputsOutput::next_token): <p>A token that indicates the location of the next output in the array of outputs, after the list of outputs that was previously requested.</p>
    ///   - [`outputs(Vec::<Output>)`](crate::operation::list_component_outputs::ListComponentOutputsOutput::outputs): <p>An array of component Infrastructure as Code (IaC) outputs.</p>
    /// - On failure, responds with [`SdkError<ListComponentOutputsError>`](crate::operation::list_component_outputs::ListComponentOutputsError)
    pub fn list_component_outputs(&self) -> crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder {
        crate::operation::list_component_outputs::builders::ListComponentOutputsFluentBuilder::new(self.handle.clone())
    }
}
