// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchUpdatePartition`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::set_catalog_id):<br>required: **false**<br><p>The ID of the catalog in which the partition is to be updated. Currently, this should be the Amazon Web Services account ID.</p><br>
    ///   - [`database_name(impl Into<String>)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::set_database_name):<br>required: **true**<br><p>The name of the metadata database in which the partition is to be updated.</p><br>
    ///   - [`table_name(impl Into<String>)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::set_table_name):<br>required: **true**<br><p>The name of the metadata table in which the partition is to be updated.</p><br>
    ///   - [`entries(BatchUpdatePartitionRequestEntry)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::entries) / [`set_entries(Option<Vec::<BatchUpdatePartitionRequestEntry>>)`](crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::set_entries):<br>required: **true**<br><p>A list of up to 100 <code>BatchUpdatePartitionRequestEntry</code> objects to update.</p><br>
    /// - On success, responds with [`BatchUpdatePartitionOutput`](crate::operation::batch_update_partition::BatchUpdatePartitionOutput) with field(s):
    ///   - [`errors(Option<Vec::<BatchUpdatePartitionFailureEntry>>)`](crate::operation::batch_update_partition::BatchUpdatePartitionOutput::errors): <p>The errors encountered when trying to update the requested partitions. A list of <code>BatchUpdatePartitionFailureEntry</code> objects.</p>
    /// - On failure, responds with [`SdkError<BatchUpdatePartitionError>`](crate::operation::batch_update_partition::BatchUpdatePartitionError)
    pub fn batch_update_partition(&self) -> crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder {
        crate::operation::batch_update_partition::builders::BatchUpdatePartitionFluentBuilder::new(self.handle.clone())
    }
}
