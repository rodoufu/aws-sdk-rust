// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTemplate`](crate::operation::create_template::builders::CreateTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_name(impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::template_name) / [`set_template_name(Option<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_template_name):<br>required: **true**<br><p>The name of the migration workflow template.</p><br>
    ///   - [`template_description(impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::template_description) / [`set_template_description(Option<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_template_description):<br>required: **false**<br><p>A description of the migration workflow template.</p><br>
    ///   - [`template_source(TemplateSource)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::template_source) / [`set_template_source(Option<TemplateSource>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_template_source):<br>required: **true**<br><p>The source of the migration workflow template.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://smithy.io/2.0/spec/behavior-traits.html#idempotencytoken-trait">Idempotency</a> in the Smithy documentation.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_template::builders::CreateTemplateFluentBuilder::set_tags):<br>required: **false**<br><p>The tags to add to the migration workflow template.</p><br>
    /// - On success, responds with [`CreateTemplateOutput`](crate::operation::create_template::CreateTemplateOutput) with field(s):
    ///   - [`template_id(Option<String>)`](crate::operation::create_template::CreateTemplateOutput::template_id): <p>The ID of the migration workflow template.</p>
    ///   - [`template_arn(Option<String>)`](crate::operation::create_template::CreateTemplateOutput::template_arn): <p>The Amazon Resource Name (ARN) of the migration workflow template. The format for an Migration Hub Orchestrator template ARN is <code>arn:aws:migrationhub-orchestrator:region:account:template/template-abcd1234</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::create_template::CreateTemplateOutput::tags): <p>The tags added to the migration workflow template.</p>
    /// - On failure, responds with [`SdkError<CreateTemplateError>`](crate::operation::create_template::CreateTemplateError)
    pub fn create_template(&self) -> crate::operation::create_template::builders::CreateTemplateFluentBuilder {
        crate::operation::create_template::builders::CreateTemplateFluentBuilder::new(self.handle.clone())
    }
}
