// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateContext`](crate::operation::update_context::builders::UpdateContextFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`context_name(impl Into<String>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::context_name) / [`set_context_name(Option<String>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::set_context_name):<br>required: **true**<br><p>The name of the context to update.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::set_description):<br>required: **false**<br><p>The new description for the context.</p><br>
    ///   - [`properties(impl Into<String>, impl Into<String>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::properties) / [`set_properties(Option<HashMap::<String, String>>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::set_properties):<br>required: **false**<br><p>The new list of properties. Overwrites the current property list.</p><br>
    ///   - [`properties_to_remove(impl Into<String>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::properties_to_remove) / [`set_properties_to_remove(Option<Vec::<String>>)`](crate::operation::update_context::builders::UpdateContextFluentBuilder::set_properties_to_remove):<br>required: **false**<br><p>A list of properties to remove.</p><br>
    /// - On success, responds with [`UpdateContextOutput`](crate::operation::update_context::UpdateContextOutput) with field(s):
    ///   - [`context_arn(Option<String>)`](crate::operation::update_context::UpdateContextOutput::context_arn): <p>The Amazon Resource Name (ARN) of the context.</p>
    /// - On failure, responds with [`SdkError<UpdateContextError>`](crate::operation::update_context::UpdateContextError)
    pub fn update_context(&self) -> crate::operation::update_context::builders::UpdateContextFluentBuilder {
        crate::operation::update_context::builders::UpdateContextFluentBuilder::new(self.handle.clone())
    }
}
