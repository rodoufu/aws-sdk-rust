// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendInvites`](crate::operation::send_invites::builders::SendInvitesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`space_id(impl Into<String>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::space_id) / [`set_space_id(Option<String>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::set_space_id):<br>required: **true**<br><p>The ID of the private re:Post.</p><br>
    ///   - [`accessor_ids(impl Into<String>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::accessor_ids) / [`set_accessor_ids(Option<Vec::<String>>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::set_accessor_ids):<br>required: **true**<br><p>The array of identifiers for the users and groups.</p><br>
    ///   - [`title(impl Into<String>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::title) / [`set_title(Option<String>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::set_title):<br>required: **true**<br><p>The title of the invite.</p><br>
    ///   - [`body(impl Into<String>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::body) / [`set_body(Option<String>)`](crate::operation::send_invites::builders::SendInvitesFluentBuilder::set_body):<br>required: **true**<br><p>The body of the invite.</p><br>
    /// - On success, responds with [`SendInvitesOutput`](crate::operation::send_invites::SendInvitesOutput)
    /// - On failure, responds with [`SdkError<SendInvitesError>`](crate::operation::send_invites::SendInvitesError)
    pub fn send_invites(&self) -> crate::operation::send_invites::builders::SendInvitesFluentBuilder {
        crate::operation::send_invites::builders::SendInvitesFluentBuilder::new(self.handle.clone())
    }
}
