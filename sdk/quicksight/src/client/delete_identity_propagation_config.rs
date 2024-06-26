// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIdentityPropagationConfig`](crate::operation::delete_identity_propagation_config::builders::DeleteIdentityPropagationConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::delete_identity_propagation_config::builders::DeleteIdentityPropagationConfigFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::delete_identity_propagation_config::builders::DeleteIdentityPropagationConfigFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID of the Amazon Web Services account that you want to delete an identity propagation configuration from.</p><br>
    ///   - [`service(ServiceType)`](crate::operation::delete_identity_propagation_config::builders::DeleteIdentityPropagationConfigFluentBuilder::service) / [`set_service(Option<ServiceType>)`](crate::operation::delete_identity_propagation_config::builders::DeleteIdentityPropagationConfigFluentBuilder::set_service):<br>required: **true**<br><p>The name of the Amazon Web Services service that you want to delete the associated access scopes and authorized targets from.</p><br>
    /// - On success, responds with [`DeleteIdentityPropagationConfigOutput`](crate::operation::delete_identity_propagation_config::DeleteIdentityPropagationConfigOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::delete_identity_propagation_config::DeleteIdentityPropagationConfigOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::delete_identity_propagation_config::DeleteIdentityPropagationConfigOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<DeleteIdentityPropagationConfigError>`](crate::operation::delete_identity_propagation_config::DeleteIdentityPropagationConfigError)
    pub fn delete_identity_propagation_config(
        &self,
    ) -> crate::operation::delete_identity_propagation_config::builders::DeleteIdentityPropagationConfigFluentBuilder {
        crate::operation::delete_identity_propagation_config::builders::DeleteIdentityPropagationConfigFluentBuilder::new(self.handle.clone())
    }
}
