// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListHapgs`](crate::operation::list_hapgs::builders::ListHapgsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_hapgs::builders::ListHapgsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_hapgs::builders::ListHapgsFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>NextToken</code> value from a previous call to <code>ListHapgs</code>. Pass null if this is the first call.</p><br>
    /// - On success, responds with [`ListHapgsOutput`](crate::operation::list_hapgs::ListHapgsOutput) with field(s):
    ///   - [`hapg_list(Vec::<String>)`](crate::operation::list_hapgs::ListHapgsOutput::hapg_list): <p>The list of high-availability partition groups.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_hapgs::ListHapgsOutput::next_token): <p>If not null, more results are available. Pass this value to <code>ListHapgs</code> to retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<ListHapgsError>`](crate::operation::list_hapgs::ListHapgsError)
    #[deprecated(note = "This API is deprecated.")]
    pub fn list_hapgs(&self) -> crate::operation::list_hapgs::builders::ListHapgsFluentBuilder {
        crate::operation::list_hapgs::builders::ListHapgsFluentBuilder::new(self.handle.clone())
    }
}
