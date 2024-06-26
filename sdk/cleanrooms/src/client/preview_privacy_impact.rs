// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PreviewPrivacyImpact`](crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactFluentBuilder::set_membership_identifier):<br>required: **true**<br><p>A unique identifier for one of your memberships for a collaboration. Accepts a membership ID.</p><br>
    ///   - [`parameters(PreviewPrivacyImpactParametersInput)`](crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactFluentBuilder::parameters) / [`set_parameters(Option<PreviewPrivacyImpactParametersInput>)`](crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactFluentBuilder::set_parameters):<br>required: **true**<br><p>Specifies the desired epsilon and noise parameters to preview.</p><br>
    /// - On success, responds with [`PreviewPrivacyImpactOutput`](crate::operation::preview_privacy_impact::PreviewPrivacyImpactOutput) with field(s):
    ///   - [`privacy_impact(Option<PrivacyImpact>)`](crate::operation::preview_privacy_impact::PreviewPrivacyImpactOutput::privacy_impact): <p>An estimate of the number of aggregation functions that the member who can query can run given the epsilon and noise parameters. This does not change the privacy budget.</p>
    /// - On failure, responds with [`SdkError<PreviewPrivacyImpactError>`](crate::operation::preview_privacy_impact::PreviewPrivacyImpactError)
    pub fn preview_privacy_impact(&self) -> crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactFluentBuilder {
        crate::operation::preview_privacy_impact::builders::PreviewPrivacyImpactFluentBuilder::new(self.handle.clone())
    }
}
