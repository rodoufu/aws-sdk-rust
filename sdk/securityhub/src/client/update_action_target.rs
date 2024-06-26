// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateActionTarget`](crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`action_target_arn(impl Into<String>)`](crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder::action_target_arn) / [`set_action_target_arn(Option<String>)`](crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder::set_action_target_arn):<br>required: **true**<br><p>The ARN of the custom action target to update.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder::set_name):<br>required: **false**<br><p>The updated name of the custom action target.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder::set_description):<br>required: **false**<br><p>The updated description for the custom action target.</p><br>
    /// - On success, responds with [`UpdateActionTargetOutput`](crate::operation::update_action_target::UpdateActionTargetOutput)
    /// - On failure, responds with [`SdkError<UpdateActionTargetError>`](crate::operation::update_action_target::UpdateActionTargetError)
    pub fn update_action_target(&self) -> crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder {
        crate::operation::update_action_target::builders::UpdateActionTargetFluentBuilder::new(self.handle.clone())
    }
}
