// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartDataSourceIntrospection`](crate::operation::start_data_source_introspection::builders::StartDataSourceIntrospectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rds_data_api_config(RdsDataApiConfig)`](crate::operation::start_data_source_introspection::builders::StartDataSourceIntrospectionFluentBuilder::rds_data_api_config) / [`set_rds_data_api_config(Option<RdsDataApiConfig>)`](crate::operation::start_data_source_introspection::builders::StartDataSourceIntrospectionFluentBuilder::set_rds_data_api_config):<br>required: **false**<br><p>The <code>rdsDataApiConfig</code> object data.</p><br>
    /// - On success, responds with [`StartDataSourceIntrospectionOutput`](crate::operation::start_data_source_introspection::StartDataSourceIntrospectionOutput) with field(s):
    ///   - [`introspection_id(Option<String>)`](crate::operation::start_data_source_introspection::StartDataSourceIntrospectionOutput::introspection_id): <p>The introspection ID. Each introspection contains a unique ID that can be used to reference the instrospection record.</p>
    ///   - [`introspection_status(Option<DataSourceIntrospectionStatus>)`](crate::operation::start_data_source_introspection::StartDataSourceIntrospectionOutput::introspection_status): <p>The status of the introspection during creation. By default, when a new instrospection has been created, the status will be set to <code>PROCESSING</code>. Once the operation has been completed, the status will change to <code>SUCCESS</code> or <code>FAILED</code> depending on how the data was parsed. A <code>FAILED</code> operation will return an error and its details as an <code>introspectionStatusDetail</code>.</p>
    ///   - [`introspection_status_detail(Option<String>)`](crate::operation::start_data_source_introspection::StartDataSourceIntrospectionOutput::introspection_status_detail): <p>The error detail field. When a <code>FAILED</code> <code>introspectionStatus</code> is returned, the <code>introspectionStatusDetail</code> will also return the exact error that was generated during the operation.</p>
    /// - On failure, responds with [`SdkError<StartDataSourceIntrospectionError>`](crate::operation::start_data_source_introspection::StartDataSourceIntrospectionError)
    pub fn start_data_source_introspection(
        &self,
    ) -> crate::operation::start_data_source_introspection::builders::StartDataSourceIntrospectionFluentBuilder {
        crate::operation::start_data_source_introspection::builders::StartDataSourceIntrospectionFluentBuilder::new(self.handle.clone())
    }
}
