// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateKeys`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`kvs_arn(impl Into<String>)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::kvs_arn) / [`set_kvs_arn(Option<String>)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::set_kvs_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the Key Value Store.</p><br>
    ///   - [`if_match(impl Into<String>)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::if_match) / [`set_if_match(Option<String>)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::set_if_match):<br>required: **true**<br><p>The current version (ETag) of the Key Value Store that you are updating keys of, which you can get using DescribeKeyValueStore.</p><br>
    ///   - [`puts(PutKeyRequestListItem)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::puts) / [`set_puts(Option<Vec::<PutKeyRequestListItem>>)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::set_puts):<br>required: **false**<br><p>List of key value pairs to put.</p><br>
    ///   - [`deletes(DeleteKeyRequestListItem)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::deletes) / [`set_deletes(Option<Vec::<DeleteKeyRequestListItem>>)`](crate::operation::update_keys::builders::UpdateKeysFluentBuilder::set_deletes):<br>required: **false**<br><p>List of keys to delete.</p><br>
    /// - On success, responds with [`UpdateKeysOutput`](crate::operation::update_keys::UpdateKeysOutput) with field(s):
    ///   - [`item_count(i32)`](crate::operation::update_keys::UpdateKeysOutput::item_count): <p>Number of key value pairs in the Key Value Store after the successful update.</p>
    ///   - [`total_size_in_bytes(i64)`](crate::operation::update_keys::UpdateKeysOutput::total_size_in_bytes): <p>Total size of the Key Value Store after the successful update, in bytes.</p>
    ///   - [`e_tag(String)`](crate::operation::update_keys::UpdateKeysOutput::e_tag): <p>The current version identifier of the Key Value Store after the successful update.</p>
    /// - On failure, responds with [`SdkError<UpdateKeysError>`](crate::operation::update_keys::UpdateKeysError)
    pub fn update_keys(&self) -> crate::operation::update_keys::builders::UpdateKeysFluentBuilder {
        crate::operation::update_keys::builders::UpdateKeysFluentBuilder::new(self.handle.clone())
    }
}
