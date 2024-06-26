// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLicenseEndpoints`](crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results, or <code>null</code> to start from the beginning.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p><br>
    /// - On success, responds with [`ListLicenseEndpointsOutput`](crate::operation::list_license_endpoints::ListLicenseEndpointsOutput) with field(s):
    ///   - [`license_endpoints(Vec::<LicenseEndpointSummary>)`](crate::operation::list_license_endpoints::ListLicenseEndpointsOutput::license_endpoints): <p>The license endpoints.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_license_endpoints::ListLicenseEndpointsOutput::next_token): <p>If Deadline Cloud returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an HTTP 400 <code>ValidationException</code> error.</p>
    /// - On failure, responds with [`SdkError<ListLicenseEndpointsError>`](crate::operation::list_license_endpoints::ListLicenseEndpointsError)
    pub fn list_license_endpoints(&self) -> crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder {
        crate::operation::list_license_endpoints::builders::ListLicenseEndpointsFluentBuilder::new(self.handle.clone())
    }
}
