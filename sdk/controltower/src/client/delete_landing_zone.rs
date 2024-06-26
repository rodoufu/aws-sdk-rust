// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLandingZone`](crate::operation::delete_landing_zone::builders::DeleteLandingZoneFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`landing_zone_identifier(impl Into<String>)`](crate::operation::delete_landing_zone::builders::DeleteLandingZoneFluentBuilder::landing_zone_identifier) / [`set_landing_zone_identifier(Option<String>)`](crate::operation::delete_landing_zone::builders::DeleteLandingZoneFluentBuilder::set_landing_zone_identifier):<br>required: **true**<br><p>The unique identifier of the landing zone.</p><br>
    /// - On success, responds with [`DeleteLandingZoneOutput`](crate::operation::delete_landing_zone::DeleteLandingZoneOutput) with field(s):
    ///   - [`operation_identifier(String)`](crate::operation::delete_landing_zone::DeleteLandingZoneOutput::operation_identifier): <p>&gt;A unique identifier assigned to a <code>DeleteLandingZone</code> operation. You can use this identifier as an input parameter of <code>GetLandingZoneOperation</code> to check the operation's status.</p>
    /// - On failure, responds with [`SdkError<DeleteLandingZoneError>`](crate::operation::delete_landing_zone::DeleteLandingZoneError)
    pub fn delete_landing_zone(&self) -> crate::operation::delete_landing_zone::builders::DeleteLandingZoneFluentBuilder {
        crate::operation::delete_landing_zone::builders::DeleteLandingZoneFluentBuilder::new(self.handle.clone())
    }
}
