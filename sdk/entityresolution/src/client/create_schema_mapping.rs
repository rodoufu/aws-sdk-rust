// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSchemaMapping`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`schema_name(impl Into<String>)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::schema_name) / [`set_schema_name(Option<String>)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::set_schema_name):<br>required: **true**<br><p>The name of the schema. There can't be multiple <code>SchemaMappings</code> with the same name.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::set_description):<br>required: **false**<br><p>A description of the schema.</p><br>
    ///   - [`mapped_input_fields(SchemaInputAttribute)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::mapped_input_fields) / [`set_mapped_input_fields(Option<Vec::<SchemaInputAttribute>>)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::set_mapped_input_fields):<br>required: **true**<br><p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::set_tags):<br>required: **false**<br><p>The tags used to organize, track, or control access for this resource.</p><br>
    /// - On success, responds with [`CreateSchemaMappingOutput`](crate::operation::create_schema_mapping::CreateSchemaMappingOutput) with field(s):
    ///   - [`schema_name(String)`](crate::operation::create_schema_mapping::CreateSchemaMappingOutput::schema_name): <p>The name of the schema.</p>
    ///   - [`schema_arn(String)`](crate::operation::create_schema_mapping::CreateSchemaMappingOutput::schema_arn): <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>SchemaMapping</code>.</p>
    ///   - [`description(String)`](crate::operation::create_schema_mapping::CreateSchemaMappingOutput::description): <p>A description of the schema.</p>
    ///   - [`mapped_input_fields(Vec::<SchemaInputAttribute>)`](crate::operation::create_schema_mapping::CreateSchemaMappingOutput::mapped_input_fields): <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p>
    /// - On failure, responds with [`SdkError<CreateSchemaMappingError>`](crate::operation::create_schema_mapping::CreateSchemaMappingError)
    pub fn create_schema_mapping(&self) -> crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder {
        crate::operation::create_schema_mapping::builders::CreateSchemaMappingFluentBuilder::new(self.handle.clone())
    }
}
