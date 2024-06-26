// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRouteResponse`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::set_api_id):<br>required: **true**<br><p>The API identifier.</p><br>
    ///   - [`model_selection_expression(impl Into<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::model_selection_expression) / [`set_model_selection_expression(Option<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::set_model_selection_expression):<br>required: **false**<br><p>The model selection expression for the route response. Supported only for WebSocket APIs.</p><br>
    ///   - [`response_models(impl Into<String>, impl Into<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::response_models) / [`set_response_models(Option<HashMap::<String, String>>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::set_response_models):<br>required: **false**<br><p>The response models for the route response.</p><br>
    ///   - [`response_parameters(impl Into<String>, ParameterConstraints)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::response_parameters) / [`set_response_parameters(Option<HashMap::<String, ParameterConstraints>>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::set_response_parameters):<br>required: **false**<br><p>The route response parameters.</p><br>
    ///   - [`route_id(impl Into<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::route_id) / [`set_route_id(Option<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::set_route_id):<br>required: **true**<br><p>The route ID.</p><br>
    ///   - [`route_response_key(impl Into<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::route_response_key) / [`set_route_response_key(Option<String>)`](crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::set_route_response_key):<br>required: **true**<br><p>The route response key.</p><br>
    /// - On success, responds with [`CreateRouteResponseOutput`](crate::operation::create_route_response::CreateRouteResponseOutput) with field(s):
    ///   - [`model_selection_expression(Option<String>)`](crate::operation::create_route_response::CreateRouteResponseOutput::model_selection_expression): <p>Represents the model selection expression of a route response. Supported only for WebSocket APIs.</p>
    ///   - [`response_models(Option<HashMap::<String, String>>)`](crate::operation::create_route_response::CreateRouteResponseOutput::response_models): <p>Represents the response models of a route response.</p>
    ///   - [`response_parameters(Option<HashMap::<String, ParameterConstraints>>)`](crate::operation::create_route_response::CreateRouteResponseOutput::response_parameters): <p>Represents the response parameters of a route response.</p>
    ///   - [`route_response_id(Option<String>)`](crate::operation::create_route_response::CreateRouteResponseOutput::route_response_id): <p>Represents the identifier of a route response.</p>
    ///   - [`route_response_key(Option<String>)`](crate::operation::create_route_response::CreateRouteResponseOutput::route_response_key): <p>Represents the route response key of a route response.</p>
    /// - On failure, responds with [`SdkError<CreateRouteResponseError>`](crate::operation::create_route_response::CreateRouteResponseError)
    pub fn create_route_response(&self) -> crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder {
        crate::operation::create_route_response::builders::CreateRouteResponseFluentBuilder::new(self.handle.clone())
    }
}
