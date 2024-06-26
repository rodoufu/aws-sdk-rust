// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelSubscription`](crate::operation::cancel_subscription::builders::CancelSubscriptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_identifier(impl Into<String>)`](crate::operation::cancel_subscription::builders::CancelSubscriptionFluentBuilder::domain_identifier) / [`set_domain_identifier(Option<String>)`](crate::operation::cancel_subscription::builders::CancelSubscriptionFluentBuilder::set_domain_identifier):<br>required: **true**<br><p>The unique identifier of the Amazon DataZone domain where the subscription request is being cancelled.</p><br>
    ///   - [`identifier(impl Into<String>)`](crate::operation::cancel_subscription::builders::CancelSubscriptionFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::cancel_subscription::builders::CancelSubscriptionFluentBuilder::set_identifier):<br>required: **true**<br><p>The unique identifier of the subscription that is being cancelled.</p><br>
    /// - On success, responds with [`CancelSubscriptionOutput`](crate::operation::cancel_subscription::CancelSubscriptionOutput) with field(s):
    ///   - [`id(String)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::id): <p>The identifier of the subscription.</p>
    ///   - [`created_by(String)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::created_by): <p>Specifies the Amazon DataZone user who is cancelling the subscription.</p>
    ///   - [`updated_by(Option<String>)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::updated_by): <p>The Amazon DataZone user that cancelled the subscription.</p>
    ///   - [`domain_id(String)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::domain_id): <p>The unique identifier of the Amazon DataZone domain where the subscription is being cancelled.</p>
    ///   - [`status(SubscriptionStatus)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::status): <p>The status of the request to cancel the subscription.</p>
    ///   - [`created_at(DateTime)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::created_at): <p>The timestamp that specifies when the request to cancel the subscription was created.</p>
    ///   - [`updated_at(DateTime)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::updated_at): <p>The timestamp that specifies when the subscription was cancelled.</p>
    ///   - [`subscribed_principal(Option<SubscribedPrincipal>)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::subscribed_principal): <p>The Amazon DataZone user who is made a subscriber to the specified asset by the subscription that is being cancelled.</p>
    ///   - [`subscribed_listing(Option<SubscribedListing>)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::subscribed_listing): <p>The asset to which a subscription is being cancelled.</p>
    ///   - [`subscription_request_id(Option<String>)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::subscription_request_id): <p>The unique ID of the subscripton request for the subscription that is being cancelled.</p>
    ///   - [`retain_permissions(Option<bool>)`](crate::operation::cancel_subscription::CancelSubscriptionOutput::retain_permissions): <p>Specifies whether the permissions to the asset are retained after the subscription is cancelled.</p>
    /// - On failure, responds with [`SdkError<CancelSubscriptionError>`](crate::operation::cancel_subscription::CancelSubscriptionError)
    pub fn cancel_subscription(&self) -> crate::operation::cancel_subscription::builders::CancelSubscriptionFluentBuilder {
        crate::operation::cancel_subscription::builders::CancelSubscriptionFluentBuilder::new(self.handle.clone())
    }
}
