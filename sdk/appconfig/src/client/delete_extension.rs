// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteExtension`](crate::operation::delete_extension::builders::DeleteExtensionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`extension_identifier(impl Into<String>)`](crate::operation::delete_extension::builders::DeleteExtensionFluentBuilder::extension_identifier) / [`set_extension_identifier(Option<String>)`](crate::operation::delete_extension::builders::DeleteExtensionFluentBuilder::set_extension_identifier):<br>required: **true**<br><p>The name, ID, or Amazon Resource Name (ARN) of the extension you want to delete.</p><br>
    ///   - [`version_number(i32)`](crate::operation::delete_extension::builders::DeleteExtensionFluentBuilder::version_number) / [`set_version_number(Option<i32>)`](crate::operation::delete_extension::builders::DeleteExtensionFluentBuilder::set_version_number):<br>required: **false**<br><p>A specific version of an extension to delete. If omitted, the highest version is deleted.</p><br>
    /// - On success, responds with [`DeleteExtensionOutput`](crate::operation::delete_extension::DeleteExtensionOutput)
    /// - On failure, responds with [`SdkError<DeleteExtensionError>`](crate::operation::delete_extension::DeleteExtensionError)
    pub fn delete_extension(&self) -> crate::operation::delete_extension::builders::DeleteExtensionFluentBuilder {
        crate::operation::delete_extension::builders::DeleteExtensionFluentBuilder::new(self.handle.clone())
    }
}
