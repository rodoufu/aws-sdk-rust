// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopComposition`](crate::operation::stop_composition::builders::StopCompositionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::stop_composition::builders::StopCompositionFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::stop_composition::builders::StopCompositionFluentBuilder::set_arn):<br>required: **true**<br><p>ARN of the Composition.</p><br>
    /// - On success, responds with [`StopCompositionOutput`](crate::operation::stop_composition::StopCompositionOutput)
    /// - On failure, responds with [`SdkError<StopCompositionError>`](crate::operation::stop_composition::StopCompositionError)
    pub fn stop_composition(&self) -> crate::operation::stop_composition::builders::StopCompositionFluentBuilder {
        crate::operation::stop_composition::builders::StopCompositionFluentBuilder::new(self.handle.clone())
    }
}
