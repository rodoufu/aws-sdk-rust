// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateUserProficiencies`](crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder::set_instance_id):<br>required: **true**<br><p>The identifier of the Amazon Connect instance. You can find the instance ID in the Amazon Resource Name (ARN) of the instance.</p><br>
    ///   - [`user_id(impl Into<String>)`](crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder::user_id) / [`set_user_id(Option<String>)`](crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder::set_user_id):<br>required: **true**<br><p>The identifier of the user account.</p><br>
    ///   - [`user_proficiencies(UserProficiency)`](crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder::user_proficiencies) / [`set_user_proficiencies(Option<Vec::<UserProficiency>>)`](crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder::set_user_proficiencies):<br>required: **true**<br><p>The proficiencies to be updated for the user. Proficiencies must first be associated to the user. You can do this using AssociateUserProficiencies API.</p><br>
    /// - On success, responds with [`UpdateUserProficienciesOutput`](crate::operation::update_user_proficiencies::UpdateUserProficienciesOutput)
    /// - On failure, responds with [`SdkError<UpdateUserProficienciesError>`](crate::operation::update_user_proficiencies::UpdateUserProficienciesError)
    pub fn update_user_proficiencies(&self) -> crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder {
        crate::operation::update_user_proficiencies::builders::UpdateUserProficienciesFluentBuilder::new(self.handle.clone())
    }
}
