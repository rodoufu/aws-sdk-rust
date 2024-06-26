// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTable`](crate::operation::delete_table::builders::DeleteTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::set_catalog_id):<br>required: **false**<br><p>The ID of the Data Catalog where the table resides. If none is provided, the Amazon Web Services account ID is used by default.</p><br>
    ///   - [`database_name(impl Into<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::set_database_name):<br>required: **true**<br><p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::set_name):<br>required: **true**<br><p>The name of the table to be deleted. For Hive compatibility, this name is entirely lowercase.</p><br>
    ///   - [`transaction_id(impl Into<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::transaction_id) / [`set_transaction_id(Option<String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::set_transaction_id):<br>required: **false**<br><p>The transaction ID at which to delete the table contents.</p><br>
    /// - On success, responds with [`DeleteTableOutput`](crate::operation::delete_table::DeleteTableOutput)
    /// - On failure, responds with [`SdkError<DeleteTableError>`](crate::operation::delete_table::DeleteTableError)
    pub fn delete_table(&self) -> crate::operation::delete_table::builders::DeleteTableFluentBuilder {
        crate::operation::delete_table::builders::DeleteTableFluentBuilder::new(self.handle.clone())
    }
}
