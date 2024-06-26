// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateStorageLocation`](crate::operation::create_storage_location::builders::CreateStorageLocationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::create_storage_location::builders::CreateStorageLocationFluentBuilder::send) it.
    /// - On success, responds with [`CreateStorageLocationOutput`](crate::operation::create_storage_location::CreateStorageLocationOutput) with field(s):
    ///   - [`s3_bucket(Option<String>)`](crate::operation::create_storage_location::CreateStorageLocationOutput::s3_bucket): <p>The name of the Amazon S3 bucket created.</p>
    /// - On failure, responds with [`SdkError<CreateStorageLocationError>`](crate::operation::create_storage_location::CreateStorageLocationError)
    pub fn create_storage_location(&self) -> crate::operation::create_storage_location::builders::CreateStorageLocationFluentBuilder {
        crate::operation::create_storage_location::builders::CreateStorageLocationFluentBuilder::new(self.handle.clone())
    }
}
