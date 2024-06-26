// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConfiguration`](crate::operation::get_configuration::builders::GetConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_configuration::builders::GetConfigurationFluentBuilder::send) it.
    /// - On success, responds with [`GetConfigurationOutput`](crate::operation::get_configuration::GetConfigurationOutput) with field(s):
    ///   - [`ecr_configuration(Option<EcrConfigurationState>)`](crate::operation::get_configuration::GetConfigurationOutput::ecr_configuration): <p>Specifies how the ECR automated re-scan duration is currently configured for your environment.</p>
    /// - On failure, responds with [`SdkError<GetConfigurationError>`](crate::operation::get_configuration::GetConfigurationError)
    pub fn get_configuration(&self) -> crate::operation::get_configuration::builders::GetConfigurationFluentBuilder {
        crate::operation::get_configuration::builders::GetConfigurationFluentBuilder::new(self.handle.clone())
    }
}
