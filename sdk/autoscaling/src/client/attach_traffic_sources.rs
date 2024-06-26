// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AttachTrafficSources`](crate::operation::attach_traffic_sources::builders::AttachTrafficSourcesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::operation::attach_traffic_sources::builders::AttachTrafficSourcesFluentBuilder::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::operation::attach_traffic_sources::builders::AttachTrafficSourcesFluentBuilder::set_auto_scaling_group_name):<br>required: **true**<br><p>The name of the Auto Scaling group.</p><br>
    ///   - [`traffic_sources(TrafficSourceIdentifier)`](crate::operation::attach_traffic_sources::builders::AttachTrafficSourcesFluentBuilder::traffic_sources) / [`set_traffic_sources(Option<Vec::<TrafficSourceIdentifier>>)`](crate::operation::attach_traffic_sources::builders::AttachTrafficSourcesFluentBuilder::set_traffic_sources):<br>required: **true**<br><p>The unique identifiers of one or more traffic sources. You can specify up to 10 traffic sources.</p><br>
    /// - On success, responds with [`AttachTrafficSourcesOutput`](crate::operation::attach_traffic_sources::AttachTrafficSourcesOutput)
    /// - On failure, responds with [`SdkError<AttachTrafficSourcesError>`](crate::operation::attach_traffic_sources::AttachTrafficSourcesError)
    pub fn attach_traffic_sources(&self) -> crate::operation::attach_traffic_sources::builders::AttachTrafficSourcesFluentBuilder {
        crate::operation::attach_traffic_sources::builders::AttachTrafficSourcesFluentBuilder::new(self.handle.clone())
    }
}
