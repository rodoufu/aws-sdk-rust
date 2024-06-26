// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateSourceServers`](crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder::set_application_id):<br>required: **true**<br><p>Application ID.</p><br>
    ///   - [`source_server_ids(impl Into<String>)`](crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder::source_server_ids) / [`set_source_server_ids(Option<Vec::<String>>)`](crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder::set_source_server_ids):<br>required: **true**<br><p>Source server IDs list.</p><br>
    ///   - [`account_id(impl Into<String>)`](crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder::set_account_id):<br>required: **false**<br><p>Account ID.</p><br>
    /// - On success, responds with [`DisassociateSourceServersOutput`](crate::operation::disassociate_source_servers::DisassociateSourceServersOutput)
    /// - On failure, responds with [`SdkError<DisassociateSourceServersError>`](crate::operation::disassociate_source_servers::DisassociateSourceServersError)
    pub fn disassociate_source_servers(&self) -> crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder {
        crate::operation::disassociate_source_servers::builders::DisassociateSourceServersFluentBuilder::new(self.handle.clone())
    }
}
