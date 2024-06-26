// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_session_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_session::UpdateSessionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tag_filter {
        #[allow(unused_mut)]
        let mut object_3 = object.key("tagFilter").start_object();
        crate::protocol_serde::shape_tag_filter::ser_tag_filter(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
