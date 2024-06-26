// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRun`](crate::operation::delete_run::builders::DeleteRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::delete_run::builders::DeleteRunFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::delete_run::builders::DeleteRunFluentBuilder::set_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) for the run to delete.</p><br>
    /// - On success, responds with [`DeleteRunOutput`](crate::operation::delete_run::DeleteRunOutput)
    /// - On failure, responds with [`SdkError<DeleteRunError>`](crate::operation::delete_run::DeleteRunError)
    pub fn delete_run(&self) -> crate::operation::delete_run::builders::DeleteRunFluentBuilder {
        crate::operation::delete_run::builders::DeleteRunFluentBuilder::new(self.handle.clone())
    }
}
