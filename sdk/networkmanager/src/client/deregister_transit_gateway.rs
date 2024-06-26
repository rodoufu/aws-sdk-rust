// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterTransitGateway`](crate::operation::deregister_transit_gateway::builders::DeregisterTransitGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::operation::deregister_transit_gateway::builders::DeregisterTransitGatewayFluentBuilder::global_network_id) / [`set_global_network_id(Option<String>)`](crate::operation::deregister_transit_gateway::builders::DeregisterTransitGatewayFluentBuilder::set_global_network_id):<br>required: **true**<br><p>The ID of the global network.</p><br>
    ///   - [`transit_gateway_arn(impl Into<String>)`](crate::operation::deregister_transit_gateway::builders::DeregisterTransitGatewayFluentBuilder::transit_gateway_arn) / [`set_transit_gateway_arn(Option<String>)`](crate::operation::deregister_transit_gateway::builders::DeregisterTransitGatewayFluentBuilder::set_transit_gateway_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the transit gateway.</p><br>
    /// - On success, responds with [`DeregisterTransitGatewayOutput`](crate::operation::deregister_transit_gateway::DeregisterTransitGatewayOutput) with field(s):
    ///   - [`transit_gateway_registration(Option<TransitGatewayRegistration>)`](crate::operation::deregister_transit_gateway::DeregisterTransitGatewayOutput::transit_gateway_registration): <p>The transit gateway registration information.</p>
    /// - On failure, responds with [`SdkError<DeregisterTransitGatewayError>`](crate::operation::deregister_transit_gateway::DeregisterTransitGatewayError)
    pub fn deregister_transit_gateway(&self) -> crate::operation::deregister_transit_gateway::builders::DeregisterTransitGatewayFluentBuilder {
        crate::operation::deregister_transit_gateway::builders::DeregisterTransitGatewayFluentBuilder::new(self.handle.clone())
    }
}
