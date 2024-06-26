// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLifecyclePolicies`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Filter)`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::set_filters):<br>required: **false**<br><p>Streamline results based on one of the following values: <code>Name</code>, <code>Status</code>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum items to return in a request.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to specify where to start paginating. This is the nextToken from a previously truncated response.</p><br>
    /// - On success, responds with [`ListLifecyclePoliciesOutput`](crate::operation::list_lifecycle_policies::ListLifecyclePoliciesOutput) with field(s):
    ///   - [`lifecycle_policy_summary_list(Option<Vec::<LifecyclePolicySummary>>)`](crate::operation::list_lifecycle_policies::ListLifecyclePoliciesOutput::lifecycle_policy_summary_list): <p>A list of lifecycle policies in your Amazon Web Services account that meet the criteria specified in the request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_lifecycle_policies::ListLifecyclePoliciesOutput::next_token): <p>The next token used for paginated responses. When this field isn't empty, there are additional elements that the service hasn't included in this request. Use this token with the next request to retrieve additional objects.</p>
    /// - On failure, responds with [`SdkError<ListLifecyclePoliciesError>`](crate::operation::list_lifecycle_policies::ListLifecyclePoliciesError)
    pub fn list_lifecycle_policies(&self) -> crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder {
        crate::operation::list_lifecycle_policies::builders::ListLifecyclePoliciesFluentBuilder::new(self.handle.clone())
    }
}
