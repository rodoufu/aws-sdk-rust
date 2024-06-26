// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteWorkflow`](crate::operation::delete_workflow::builders::DeleteWorkflowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workflow_build_version_arn(impl Into<String>)`](crate::operation::delete_workflow::builders::DeleteWorkflowFluentBuilder::workflow_build_version_arn) / [`set_workflow_build_version_arn(Option<String>)`](crate::operation::delete_workflow::builders::DeleteWorkflowFluentBuilder::set_workflow_build_version_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the workflow resource to delete.</p><br>
    /// - On success, responds with [`DeleteWorkflowOutput`](crate::operation::delete_workflow::DeleteWorkflowOutput) with field(s):
    ///   - [`workflow_build_version_arn(Option<String>)`](crate::operation::delete_workflow::DeleteWorkflowOutput::workflow_build_version_arn): <p>The ARN of the workflow resource that this request deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteWorkflowError>`](crate::operation::delete_workflow::DeleteWorkflowError)
    pub fn delete_workflow(&self) -> crate::operation::delete_workflow::builders::DeleteWorkflowFluentBuilder {
        crate::operation::delete_workflow::builders::DeleteWorkflowFluentBuilder::new(self.handle.clone())
    }
}
