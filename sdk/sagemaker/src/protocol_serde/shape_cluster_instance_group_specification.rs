// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cluster_instance_group_specification(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ClusterInstanceGroupSpecification,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_count {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.instance_group_name {
        object.key("InstanceGroupName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_type {
        object.key("InstanceType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.life_cycle_config {
        #[allow(unused_mut)]
        let mut object_5 = object.key("LifeCycleConfig").start_object();
        crate::protocol_serde::shape_cluster_life_cycle_config::ser_cluster_life_cycle_config(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.execution_role {
        object.key("ExecutionRole").string(var_6.as_str());
    }
    if let Some(var_7) = &input.threads_per_core {
        object.key("ThreadsPerCore").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    Ok(())
}
