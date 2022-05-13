// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_complete_snapshot(
    input: &crate::input::CompleteSnapshotInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.changed_blocks_count {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_1);
        let formatted_2 = encoder.encode();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "changed_blocks_count",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-ChangedBlocksCount", header_value);
        }
    }
    if let Some(inner_3) = &input.checksum {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "checksum",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Checksum", header_value);
        }
    }
    if let Some(inner_5) = &input.checksum_algorithm {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "checksum_algorithm",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Checksum-Algorithm", header_value);
        }
    }
    if let Some(inner_7) = &input.checksum_aggregation_method {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "checksum_aggregation_method",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Checksum-Aggregation-Method", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_put_snapshot_block(
    input: &crate::input::PutSnapshotBlockInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_9) = &input.data_length {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_9);
        let formatted_10 = encoder.encode();
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "data_length",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Data-Length", header_value);
        }
    }
    if let Some(inner_11) = &input.progress {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_11);
        let formatted_12 = encoder.encode();
        if !formatted_12.is_empty() {
            let header_value = formatted_12;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "progress",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Progress", header_value);
        }
    }
    if let Some(inner_13) = &input.checksum {
        let formatted_14 = AsRef::<str>::as_ref(inner_13);
        if !formatted_14.is_empty() {
            let header_value = formatted_14;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "checksum",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Checksum", header_value);
        }
    }
    if let Some(inner_15) = &input.checksum_algorithm {
        let formatted_16 = AsRef::<str>::as_ref(inner_15);
        if !formatted_16.is_empty() {
            let header_value = formatted_16;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "checksum_algorithm",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-Checksum-Algorithm", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_payload_get_snapshot_block_get_snapshot_block_output_block_data(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<
    aws_smithy_http::byte_stream::ByteStream,
    crate::error::GetSnapshotBlockError,
> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_get_snapshot_block_get_snapshot_block_output_checksum(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-Checksum").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_snapshot_block_get_snapshot_block_output_checksum_algorithm(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::ChecksumAlgorithm>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-Checksum-Algorithm").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_snapshot_block_get_snapshot_block_output_data_length(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-Data-Length").iter();
    let var_17 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_17.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_17.len()),
        ))
    } else {
        let mut var_17 = var_17;
        Ok(var_17.pop())
    }
}

pub fn deser_header_put_snapshot_block_put_snapshot_block_output_checksum(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-Checksum").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_snapshot_block_put_snapshot_block_output_checksum_algorithm(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::ChecksumAlgorithm>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-Checksum-Algorithm").iter();
    aws_smithy_http::header::one_or_none(headers)
}
