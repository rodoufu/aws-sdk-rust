// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSourceNetwork`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpc_id(impl Into<String>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::vpc_id) / [`set_vpc_id(Option<String>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::set_vpc_id):<br>required: **true**<br><p>Which VPC ID to protect.</p><br>
    ///   - [`origin_account_id(impl Into<String>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::origin_account_id) / [`set_origin_account_id(Option<String>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::set_origin_account_id):<br>required: **true**<br><p>Account containing the VPC to protect.</p><br>
    ///   - [`origin_region(impl Into<String>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::origin_region) / [`set_origin_region(Option<String>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::set_origin_region):<br>required: **true**<br><p>Region containing the VPC to protect.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::set_tags):<br>required: **false**<br><p>A set of tags to be associated with the Source Network resource.</p><br>
    /// - On success, responds with [`CreateSourceNetworkOutput`](crate::operation::create_source_network::CreateSourceNetworkOutput) with field(s):
    ///   - [`source_network_id(Option<String>)`](crate::operation::create_source_network::CreateSourceNetworkOutput::source_network_id): <p>ID of the created Source Network.</p>
    /// - On failure, responds with [`SdkError<CreateSourceNetworkError>`](crate::operation::create_source_network::CreateSourceNetworkError)
    pub fn create_source_network(&self) -> crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder {
        crate::operation::create_source_network::builders::CreateSourceNetworkFluentBuilder::new(self.handle.clone())
    }
}
