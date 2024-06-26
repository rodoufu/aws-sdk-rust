// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDataLakeExceptionSubscription`](crate::operation::delete_data_lake_exception_subscription::builders::DeleteDataLakeExceptionSubscriptionFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::delete_data_lake_exception_subscription::builders::DeleteDataLakeExceptionSubscriptionFluentBuilder::send) it.
    /// - On success, responds with [`DeleteDataLakeExceptionSubscriptionOutput`](crate::operation::delete_data_lake_exception_subscription::DeleteDataLakeExceptionSubscriptionOutput)
    /// - On failure, responds with [`SdkError<DeleteDataLakeExceptionSubscriptionError>`](crate::operation::delete_data_lake_exception_subscription::DeleteDataLakeExceptionSubscriptionError)
    pub fn delete_data_lake_exception_subscription(
        &self,
    ) -> crate::operation::delete_data_lake_exception_subscription::builders::DeleteDataLakeExceptionSubscriptionFluentBuilder {
        crate::operation::delete_data_lake_exception_subscription::builders::DeleteDataLakeExceptionSubscriptionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
