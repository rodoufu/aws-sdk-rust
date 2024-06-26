// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendTaskFailure`](crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_token(impl Into<String>)`](crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder::task_token) / [`set_task_token(Option<String>)`](crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder::set_task_token):<br>required: **true**<br><p>The token that represents this task. Task tokens are generated by Step Functions when tasks are assigned to a worker, or in the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/input-output-contextobject.html">context object</a> when a workflow enters a task state. See <code>GetActivityTaskOutput$taskToken</code>.</p><br>
    ///   - [`error(impl Into<String>)`](crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder::error) / [`set_error(Option<String>)`](crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder::set_error):<br>required: **false**<br><p>The error code of the failure.</p><br>
    ///   - [`cause(impl Into<String>)`](crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder::cause) / [`set_cause(Option<String>)`](crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder::set_cause):<br>required: **false**<br><p>A more detailed explanation of the cause of the failure.</p><br>
    /// - On success, responds with [`SendTaskFailureOutput`](crate::operation::send_task_failure::SendTaskFailureOutput)
    /// - On failure, responds with [`SdkError<SendTaskFailureError>`](crate::operation::send_task_failure::SendTaskFailureError)
    pub fn send_task_failure(&self) -> crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder {
        crate::operation::send_task_failure::builders::SendTaskFailureFluentBuilder::new(self.handle.clone())
    }
}
