// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIntegration`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::set_rest_api_id):<br>required: **true**<br><p>The string identifier of the associated RestApi.</p><br>
    ///   - [`resource_id(impl Into<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::set_resource_id):<br>required: **true**<br><p>Specifies a delete integration request's resource identifier.</p><br>
    ///   - [`http_method(impl Into<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::http_method) / [`set_http_method(Option<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::set_http_method):<br>required: **true**<br><p>Specifies a delete integration request's HTTP method.</p><br>
    /// - On success, responds with [`DeleteIntegrationOutput`](crate::operation::delete_integration::DeleteIntegrationOutput)
    /// - On failure, responds with [`SdkError<DeleteIntegrationError>`](crate::operation::delete_integration::DeleteIntegrationError)
    pub fn delete_integration(&self) -> crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder {
        crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::new(self.handle.clone())
    }
}
