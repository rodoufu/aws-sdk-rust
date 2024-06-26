// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAssetModelCompositeModel`](crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`asset_model_id(impl Into<String>)`](crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder::asset_model_id) / [`set_asset_model_id(Option<String>)`](crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder::set_asset_model_id):<br>required: **true**<br><p>The ID of the asset model, in UUID format.</p><br>
    ///   - [`asset_model_composite_model_id(impl Into<String>)`](crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder::asset_model_composite_model_id) / [`set_asset_model_composite_model_id(Option<String>)`](crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder::set_asset_model_composite_model_id):<br>required: **true**<br><p>The ID of a composite model on this asset model.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p><br>
    /// - On success, responds with [`DeleteAssetModelCompositeModelOutput`](crate::operation::delete_asset_model_composite_model::DeleteAssetModelCompositeModelOutput) with field(s):
    ///   - [`asset_model_status(Option<AssetModelStatus>)`](crate::operation::delete_asset_model_composite_model::DeleteAssetModelCompositeModelOutput::asset_model_status): <p>Contains current status information for an asset model. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/asset-and-model-states.html">Asset and model states</a> in the <i>IoT SiteWise User Guide</i>.</p>
    /// - On failure, responds with [`SdkError<DeleteAssetModelCompositeModelError>`](crate::operation::delete_asset_model_composite_model::DeleteAssetModelCompositeModelError)
    pub fn delete_asset_model_composite_model(
        &self,
    ) -> crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder {
        crate::operation::delete_asset_model_composite_model::builders::DeleteAssetModelCompositeModelFluentBuilder::new(self.handle.clone())
    }
}
