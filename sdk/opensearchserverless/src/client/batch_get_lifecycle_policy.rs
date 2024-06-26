// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetLifecyclePolicy`](crate::operation::batch_get_lifecycle_policy::builders::BatchGetLifecyclePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifiers(LifecyclePolicyIdentifier)`](crate::operation::batch_get_lifecycle_policy::builders::BatchGetLifecyclePolicyFluentBuilder::identifiers) / [`set_identifiers(Option<Vec::<LifecyclePolicyIdentifier>>)`](crate::operation::batch_get_lifecycle_policy::builders::BatchGetLifecyclePolicyFluentBuilder::set_identifiers):<br>required: **true**<br><p>The unique identifiers of policy types and policy names.</p><br>
    /// - On success, responds with [`BatchGetLifecyclePolicyOutput`](crate::operation::batch_get_lifecycle_policy::BatchGetLifecyclePolicyOutput) with field(s):
    ///   - [`lifecycle_policy_details(Option<Vec::<LifecyclePolicyDetail>>)`](crate::operation::batch_get_lifecycle_policy::BatchGetLifecyclePolicyOutput::lifecycle_policy_details): <p>A list of lifecycle policies matched to the input policy name and policy type.</p>
    ///   - [`lifecycle_policy_error_details(Option<Vec::<LifecyclePolicyErrorDetail>>)`](crate::operation::batch_get_lifecycle_policy::BatchGetLifecyclePolicyOutput::lifecycle_policy_error_details): <p>A list of lifecycle policy names and policy types for which retrieval failed.</p>
    /// - On failure, responds with [`SdkError<BatchGetLifecyclePolicyError>`](crate::operation::batch_get_lifecycle_policy::BatchGetLifecyclePolicyError)
    pub fn batch_get_lifecycle_policy(&self) -> crate::operation::batch_get_lifecycle_policy::builders::BatchGetLifecyclePolicyFluentBuilder {
        crate::operation::batch_get_lifecycle_policy::builders::BatchGetLifecyclePolicyFluentBuilder::new(self.handle.clone())
    }
}
