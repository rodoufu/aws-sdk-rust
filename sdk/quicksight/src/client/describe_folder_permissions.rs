// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFolderPermissions`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID for the Amazon Web Services account that contains the folder.</p><br>
    ///   - [`folder_id(impl Into<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::folder_id) / [`set_folder_id(Option<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::set_folder_id):<br>required: **true**<br><p>The ID of the folder.</p><br>
    ///   - [`namespace(impl Into<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::set_namespace):<br>required: **false**<br><p>The namespace of the folder whose permissions you want described.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to be returned per request.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::set_next_token):<br>required: **false**<br><p>A pagination token for the next set of results.</p><br>
    /// - On success, responds with [`DescribeFolderPermissionsOutput`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsOutput) with field(s):
    ///   - [`status(i32)`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`folder_id(Option<String>)`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsOutput::folder_id): <p>The ID of the folder.</p>
    ///   - [`arn(Option<String>)`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsOutput::arn): <p>The Amazon Resource Name (ARN) for the folder.</p>
    ///   - [`permissions(Option<Vec::<ResourcePermission>>)`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsOutput::permissions): <p>Information about the permissions on the folder.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsOutput::next_token): <p>The pagination token for the next set of results, or null if there are no more results.</p>
    /// - On failure, responds with [`SdkError<DescribeFolderPermissionsError>`](crate::operation::describe_folder_permissions::DescribeFolderPermissionsError)
    pub fn describe_folder_permissions(&self) -> crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder {
        crate::operation::describe_folder_permissions::builders::DescribeFolderPermissionsFluentBuilder::new(self.handle.clone())
    }
}
