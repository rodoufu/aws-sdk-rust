// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetGroupMembershipId`](crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identity_store_id(impl Into<String>)`](crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder::identity_store_id) / [`set_identity_store_id(Option<String>)`](crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder::set_identity_store_id):<br>required: **true**<br><p>The globally unique identifier for the identity store.</p><br>
    ///   - [`group_id(impl Into<String>)`](crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder::group_id) / [`set_group_id(Option<String>)`](crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder::set_group_id):<br>required: **true**<br><p>The identifier for a group in the identity store.</p><br>
    ///   - [`member_id(MemberId)`](crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder::member_id) / [`set_member_id(Option<MemberId>)`](crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder::set_member_id):<br>required: **true**<br><p>An object that contains the identifier of a group member. Setting the <code>UserID</code> field to the specific identifier for a user indicates that the user is a member of the group.</p><br>
    /// - On success, responds with [`GetGroupMembershipIdOutput`](crate::operation::get_group_membership_id::GetGroupMembershipIdOutput) with field(s):
    ///   - [`membership_id(String)`](crate::operation::get_group_membership_id::GetGroupMembershipIdOutput::membership_id): <p>The identifier for a <code>GroupMembership</code> in an identity store.</p>
    ///   - [`identity_store_id(String)`](crate::operation::get_group_membership_id::GetGroupMembershipIdOutput::identity_store_id): <p>The globally unique identifier for the identity store.</p>
    /// - On failure, responds with [`SdkError<GetGroupMembershipIdError>`](crate::operation::get_group_membership_id::GetGroupMembershipIdError)
    pub fn get_group_membership_id(&self) -> crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder {
        crate::operation::get_group_membership_id::builders::GetGroupMembershipIdFluentBuilder::new(self.handle.clone())
    }
}
