// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DetachElasticLoadBalancer`](crate::operation::detach_elastic_load_balancer::builders::DetachElasticLoadBalancerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`elastic_load_balancer_name(impl Into<String>)`](crate::operation::detach_elastic_load_balancer::builders::DetachElasticLoadBalancerFluentBuilder::elastic_load_balancer_name) / [`set_elastic_load_balancer_name(Option<String>)`](crate::operation::detach_elastic_load_balancer::builders::DetachElasticLoadBalancerFluentBuilder::set_elastic_load_balancer_name):<br>required: **true**<br><p>The Elastic Load Balancing instance's name.</p><br>
    ///   - [`layer_id(impl Into<String>)`](crate::operation::detach_elastic_load_balancer::builders::DetachElasticLoadBalancerFluentBuilder::layer_id) / [`set_layer_id(Option<String>)`](crate::operation::detach_elastic_load_balancer::builders::DetachElasticLoadBalancerFluentBuilder::set_layer_id):<br>required: **true**<br><p>The ID of the layer that the Elastic Load Balancing instance is attached to.</p><br>
    /// - On success, responds with [`DetachElasticLoadBalancerOutput`](crate::operation::detach_elastic_load_balancer::DetachElasticLoadBalancerOutput)
    /// - On failure, responds with [`SdkError<DetachElasticLoadBalancerError>`](crate::operation::detach_elastic_load_balancer::DetachElasticLoadBalancerError)
    pub fn detach_elastic_load_balancer(&self) -> crate::operation::detach_elastic_load_balancer::builders::DetachElasticLoadBalancerFluentBuilder {
        crate::operation::detach_elastic_load_balancer::builders::DetachElasticLoadBalancerFluentBuilder::new(self.handle.clone())
    }
}
