// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateApp`](crate::operation::update_app::builders::UpdateAppFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::set_app_id):<br>required: **false**<br><p>The ID of the application.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::set_name):<br>required: **false**<br><p>The new name of the application.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::set_description):<br>required: **false**<br><p>The new description of the application.</p><br>
    ///   - [`role_name(impl Into<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::role_name) / [`set_role_name(Option<String>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::set_role_name):<br>required: **false**<br><p>The name of the service role in the customer's account used by Server Migration Service.</p><br>
    ///   - [`server_groups(ServerGroup)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::server_groups) / [`set_server_groups(Option<Vec::<ServerGroup>>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::set_server_groups):<br>required: **false**<br><p>The server groups in the application to update.</p><br>
    ///   - [`tags(Tag)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::update_app::builders::UpdateAppFluentBuilder::set_tags):<br>required: **false**<br><p>The tags to associate with the application.</p><br>
    /// - On success, responds with [`UpdateAppOutput`](crate::operation::update_app::UpdateAppOutput) with field(s):
    ///   - [`app_summary(Option<AppSummary>)`](crate::operation::update_app::UpdateAppOutput::app_summary): <p>A summary description of the application.</p>
    ///   - [`server_groups(Option<Vec::<ServerGroup>>)`](crate::operation::update_app::UpdateAppOutput::server_groups): <p>The updated server groups in the application.</p>
    ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::update_app::UpdateAppOutput::tags): <p>The tags associated with the application.</p>
    /// - On failure, responds with [`SdkError<UpdateAppError>`](crate::operation::update_app::UpdateAppError)
    pub fn update_app(&self) -> crate::operation::update_app::builders::UpdateAppFluentBuilder {
        crate::operation::update_app::builders::UpdateAppFluentBuilder::new(self.handle.clone())
    }
}
