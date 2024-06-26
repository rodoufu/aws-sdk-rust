// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListComponents`](crate::operation::list_components::builders::ListComponentsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_components::builders::ListComponentsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::set_workspace_id):<br>required: **true**<br><p>The workspace ID.</p><br>
    ///   - [`entity_id(impl Into<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::entity_id) / [`set_entity_id(Option<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::set_entity_id):<br>required: **true**<br><p>The ID for the entity whose metadata (component/properties) is returned by the operation.</p><br>
    ///   - [`component_path(impl Into<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::component_path) / [`set_component_path(Option<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::set_component_path):<br>required: **false**<br><p>This string specifies the path to the composite component, starting from the top-level component.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned at one time. The default is 25.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_components::builders::ListComponentsFluentBuilder::set_next_token):<br>required: **false**<br><p>The string that specifies the next page of results.</p><br>
    /// - On success, responds with [`ListComponentsOutput`](crate::operation::list_components::ListComponentsOutput) with field(s):
    ///   - [`component_summaries(Vec::<ComponentSummary>)`](crate::operation::list_components::ListComponentsOutput::component_summaries): <p>A list of objects that contain information about the components.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_components::ListComponentsOutput::next_token): <p>The string that specifies the next page of component results.</p>
    /// - On failure, responds with [`SdkError<ListComponentsError>`](crate::operation::list_components::ListComponentsError)
    pub fn list_components(&self) -> crate::operation::list_components::builders::ListComponentsFluentBuilder {
        crate::operation::list_components::builders::ListComponentsFluentBuilder::new(self.handle.clone())
    }
}
