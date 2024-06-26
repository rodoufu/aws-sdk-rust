// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SampleChannelData`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_name(impl Into<String>)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::channel_name) / [`set_channel_name(Option<String>)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::set_channel_name):<br>required: **true**<br><p>The name of the channel whose message samples are retrieved.</p><br>
    ///   - [`max_messages(i32)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::max_messages) / [`set_max_messages(Option<i32>)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::set_max_messages):<br>required: **false**<br><p>The number of sample messages to be retrieved. The limit is 10. The default is also 10.</p><br>
    ///   - [`start_time(DateTime)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::set_start_time):<br>required: **false**<br><p>The start of the time window from which sample messages are retrieved.</p><br>
    ///   - [`end_time(DateTime)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::set_end_time):<br>required: **false**<br><p>The end of the time window from which sample messages are retrieved.</p><br>
    /// - On success, responds with [`SampleChannelDataOutput`](crate::operation::sample_channel_data::SampleChannelDataOutput) with field(s):
    ///   - [`payloads(Option<Vec::<Blob>>)`](crate::operation::sample_channel_data::SampleChannelDataOutput::payloads): <p>The list of message samples. Each sample message is returned as a base64-encoded string.</p>
    /// - On failure, responds with [`SdkError<SampleChannelDataError>`](crate::operation::sample_channel_data::SampleChannelDataError)
    pub fn sample_channel_data(&self) -> crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder {
        crate::operation::sample_channel_data::builders::SampleChannelDataFluentBuilder::new(self.handle.clone())
    }
}
