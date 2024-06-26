// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDatastore`](crate::operation::describe_datastore::builders::DescribeDatastoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`datastore_name(impl Into<String>)`](crate::operation::describe_datastore::builders::DescribeDatastoreFluentBuilder::datastore_name) / [`set_datastore_name(Option<String>)`](crate::operation::describe_datastore::builders::DescribeDatastoreFluentBuilder::set_datastore_name):<br>required: **true**<br><p>The name of the data store</p><br>
    ///   - [`include_statistics(bool)`](crate::operation::describe_datastore::builders::DescribeDatastoreFluentBuilder::include_statistics) / [`set_include_statistics(Option<bool>)`](crate::operation::describe_datastore::builders::DescribeDatastoreFluentBuilder::set_include_statistics):<br>required: **false**<br><p>If true, additional statistical information about the data store is included in the response. This feature can't be used with a data store whose S3 storage is customer-managed.</p><br>
    /// - On success, responds with [`DescribeDatastoreOutput`](crate::operation::describe_datastore::DescribeDatastoreOutput) with field(s):
    ///   - [`datastore(Option<Datastore>)`](crate::operation::describe_datastore::DescribeDatastoreOutput::datastore): <p>Information about the data store.</p>
    ///   - [`statistics(Option<DatastoreStatistics>)`](crate::operation::describe_datastore::DescribeDatastoreOutput::statistics): <p>Additional statistical information about the data store. Included if the <code>includeStatistics</code> parameter is set to <code>true</code> in the request.</p>
    /// - On failure, responds with [`SdkError<DescribeDatastoreError>`](crate::operation::describe_datastore::DescribeDatastoreError)
    pub fn describe_datastore(&self) -> crate::operation::describe_datastore::builders::DescribeDatastoreFluentBuilder {
        crate::operation::describe_datastore::builders::DescribeDatastoreFluentBuilder::new(self.handle.clone())
    }
}
