// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetFleets`](crate::operation::batch_get_fleets::builders::BatchGetFleetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`names(impl Into<String>)`](crate::operation::batch_get_fleets::builders::BatchGetFleetsFluentBuilder::names) / [`set_names(Option<Vec::<String>>)`](crate::operation::batch_get_fleets::builders::BatchGetFleetsFluentBuilder::set_names):<br>required: **true**<br><p>The names or ARNs of the compute fleets.</p><br>
    /// - On success, responds with [`BatchGetFleetsOutput`](crate::operation::batch_get_fleets::BatchGetFleetsOutput) with field(s):
    ///   - [`fleets(Option<Vec::<Fleet>>)`](crate::operation::batch_get_fleets::BatchGetFleetsOutput::fleets): <p>Information about the requested compute fleets.</p>
    ///   - [`fleets_not_found(Option<Vec::<String>>)`](crate::operation::batch_get_fleets::BatchGetFleetsOutput::fleets_not_found): <p>The names of compute fleets for which information could not be found.</p>
    /// - On failure, responds with [`SdkError<BatchGetFleetsError>`](crate::operation::batch_get_fleets::BatchGetFleetsError)
    pub fn batch_get_fleets(&self) -> crate::operation::batch_get_fleets::builders::BatchGetFleetsFluentBuilder {
        crate::operation::batch_get_fleets::builders::BatchGetFleetsFluentBuilder::new(self.handle.clone())
    }
}
