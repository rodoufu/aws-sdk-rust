// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopCisSession`](crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`scan_job_id(impl Into<String>)`](crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder::scan_job_id) / [`set_scan_job_id(Option<String>)`](crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder::set_scan_job_id):<br>required: **true**<br><p>A unique identifier for the scan job.</p><br>
    ///   - [`session_token(impl Into<String>)`](crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder::session_token) / [`set_session_token(Option<String>)`](crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder::set_session_token):<br>required: **true**<br><p>The unique token that identifies the CIS session.</p><br>
    ///   - [`message(StopCisSessionMessage)`](crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder::message) / [`set_message(Option<StopCisSessionMessage>)`](crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder::set_message):<br>required: **true**<br><p>The stop CIS session message.</p><br>
    /// - On success, responds with [`StopCisSessionOutput`](crate::operation::stop_cis_session::StopCisSessionOutput)
    /// - On failure, responds with [`SdkError<StopCisSessionError>`](crate::operation::stop_cis_session::StopCisSessionError)
    pub fn stop_cis_session(&self) -> crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder {
        crate::operation::stop_cis_session::builders::StopCisSessionFluentBuilder::new(self.handle.clone())
    }
}
