// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct OpGenOverride {
    // message fields
    pub name: ::std::string::String,
    pub skip: bool,
    pub hide: bool,
    pub rename_to: ::std::string::String,
    pub alias: ::protobuf::RepeatedField<::std::string::String>,
    pub attr_default: ::protobuf::RepeatedField<OpGenOverride_AttrDefault>,
    pub attr_rename: ::protobuf::RepeatedField<OpGenOverride_Rename>,
    pub input_rename: ::protobuf::RepeatedField<OpGenOverride_Rename>,
    pub output_rename: ::protobuf::RepeatedField<OpGenOverride_Rename>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpGenOverride {}

impl OpGenOverride {
    pub fn new() -> OpGenOverride {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpGenOverride {
        static mut instance: ::protobuf::lazy::Lazy<OpGenOverride> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpGenOverride,
        };
        unsafe {
            instance.get(OpGenOverride::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // bool skip = 2;

    pub fn clear_skip(&mut self) {
        self.skip = false;
    }

    // Param is passed by value, moved
    pub fn set_skip(&mut self, v: bool) {
        self.skip = v;
    }

    pub fn get_skip(&self) -> bool {
        self.skip
    }

    fn get_skip_for_reflect(&self) -> &bool {
        &self.skip
    }

    fn mut_skip_for_reflect(&mut self) -> &mut bool {
        &mut self.skip
    }

    // bool hide = 3;

    pub fn clear_hide(&mut self) {
        self.hide = false;
    }

    // Param is passed by value, moved
    pub fn set_hide(&mut self, v: bool) {
        self.hide = v;
    }

    pub fn get_hide(&self) -> bool {
        self.hide
    }

    fn get_hide_for_reflect(&self) -> &bool {
        &self.hide
    }

    fn mut_hide_for_reflect(&mut self) -> &mut bool {
        &mut self.hide
    }

    // string rename_to = 4;

    pub fn clear_rename_to(&mut self) {
        self.rename_to.clear();
    }

    // Param is passed by value, moved
    pub fn set_rename_to(&mut self, v: ::std::string::String) {
        self.rename_to = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rename_to(&mut self) -> &mut ::std::string::String {
        &mut self.rename_to
    }

    // Take field
    pub fn take_rename_to(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rename_to, ::std::string::String::new())
    }

    pub fn get_rename_to(&self) -> &str {
        &self.rename_to
    }

    fn get_rename_to_for_reflect(&self) -> &::std::string::String {
        &self.rename_to
    }

    fn mut_rename_to_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.rename_to
    }

    // repeated string alias = 5;

    pub fn clear_alias(&mut self) {
        self.alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_alias(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.alias = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alias(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.alias
    }

    // Take field
    pub fn take_alias(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.alias, ::protobuf::RepeatedField::new())
    }

    pub fn get_alias(&self) -> &[::std::string::String] {
        &self.alias
    }

    fn get_alias_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.alias
    }

    fn mut_alias_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.alias
    }

    // repeated .tensorflow.OpGenOverride.AttrDefault attr_default = 6;

    pub fn clear_attr_default(&mut self) {
        self.attr_default.clear();
    }

    // Param is passed by value, moved
    pub fn set_attr_default(&mut self, v: ::protobuf::RepeatedField<OpGenOverride_AttrDefault>) {
        self.attr_default = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attr_default(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_AttrDefault> {
        &mut self.attr_default
    }

    // Take field
    pub fn take_attr_default(&mut self) -> ::protobuf::RepeatedField<OpGenOverride_AttrDefault> {
        ::std::mem::replace(&mut self.attr_default, ::protobuf::RepeatedField::new())
    }

    pub fn get_attr_default(&self) -> &[OpGenOverride_AttrDefault] {
        &self.attr_default
    }

    fn get_attr_default_for_reflect(&self) -> &::protobuf::RepeatedField<OpGenOverride_AttrDefault> {
        &self.attr_default
    }

    fn mut_attr_default_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_AttrDefault> {
        &mut self.attr_default
    }

    // repeated .tensorflow.OpGenOverride.Rename attr_rename = 7;

    pub fn clear_attr_rename(&mut self) {
        self.attr_rename.clear();
    }

    // Param is passed by value, moved
    pub fn set_attr_rename(&mut self, v: ::protobuf::RepeatedField<OpGenOverride_Rename>) {
        self.attr_rename = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attr_rename(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_Rename> {
        &mut self.attr_rename
    }

    // Take field
    pub fn take_attr_rename(&mut self) -> ::protobuf::RepeatedField<OpGenOverride_Rename> {
        ::std::mem::replace(&mut self.attr_rename, ::protobuf::RepeatedField::new())
    }

    pub fn get_attr_rename(&self) -> &[OpGenOverride_Rename] {
        &self.attr_rename
    }

    fn get_attr_rename_for_reflect(&self) -> &::protobuf::RepeatedField<OpGenOverride_Rename> {
        &self.attr_rename
    }

    fn mut_attr_rename_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_Rename> {
        &mut self.attr_rename
    }

    // repeated .tensorflow.OpGenOverride.Rename input_rename = 8;

    pub fn clear_input_rename(&mut self) {
        self.input_rename.clear();
    }

    // Param is passed by value, moved
    pub fn set_input_rename(&mut self, v: ::protobuf::RepeatedField<OpGenOverride_Rename>) {
        self.input_rename = v;
    }

    // Mutable pointer to the field.
    pub fn mut_input_rename(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_Rename> {
        &mut self.input_rename
    }

    // Take field
    pub fn take_input_rename(&mut self) -> ::protobuf::RepeatedField<OpGenOverride_Rename> {
        ::std::mem::replace(&mut self.input_rename, ::protobuf::RepeatedField::new())
    }

    pub fn get_input_rename(&self) -> &[OpGenOverride_Rename] {
        &self.input_rename
    }

    fn get_input_rename_for_reflect(&self) -> &::protobuf::RepeatedField<OpGenOverride_Rename> {
        &self.input_rename
    }

    fn mut_input_rename_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_Rename> {
        &mut self.input_rename
    }

    // repeated .tensorflow.OpGenOverride.Rename output_rename = 9;

    pub fn clear_output_rename(&mut self) {
        self.output_rename.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_rename(&mut self, v: ::protobuf::RepeatedField<OpGenOverride_Rename>) {
        self.output_rename = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_rename(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_Rename> {
        &mut self.output_rename
    }

    // Take field
    pub fn take_output_rename(&mut self) -> ::protobuf::RepeatedField<OpGenOverride_Rename> {
        ::std::mem::replace(&mut self.output_rename, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_rename(&self) -> &[OpGenOverride_Rename] {
        &self.output_rename
    }

    fn get_output_rename_for_reflect(&self) -> &::protobuf::RepeatedField<OpGenOverride_Rename> {
        &self.output_rename
    }

    fn mut_output_rename_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride_Rename> {
        &mut self.output_rename
    }
}

impl ::protobuf::Message for OpGenOverride {
    fn is_initialized(&self) -> bool {
        for v in &self.attr_default {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.attr_rename {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.input_rename {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.output_rename {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.skip = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hide = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rename_to)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.alias)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attr_default)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attr_rename)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.input_rename)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.output_rename)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.skip != false {
            my_size += 2;
        }
        if self.hide != false {
            my_size += 2;
        }
        if !self.rename_to.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.rename_to);
        }
        for value in &self.alias {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.attr_default {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.attr_rename {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.input_rename {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output_rename {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.skip != false {
            os.write_bool(2, self.skip)?;
        }
        if self.hide != false {
            os.write_bool(3, self.hide)?;
        }
        if !self.rename_to.is_empty() {
            os.write_string(4, &self.rename_to)?;
        }
        for v in &self.alias {
            os.write_string(5, &v)?;
        };
        for v in &self.attr_default {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.attr_rename {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.input_rename {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output_rename {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpGenOverride {
    fn new() -> OpGenOverride {
        OpGenOverride::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpGenOverride>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OpGenOverride::get_name_for_reflect,
                    OpGenOverride::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "skip",
                    OpGenOverride::get_skip_for_reflect,
                    OpGenOverride::mut_skip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hide",
                    OpGenOverride::get_hide_for_reflect,
                    OpGenOverride::mut_hide_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rename_to",
                    OpGenOverride::get_rename_to_for_reflect,
                    OpGenOverride::mut_rename_to_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alias",
                    OpGenOverride::get_alias_for_reflect,
                    OpGenOverride::mut_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpGenOverride_AttrDefault>>(
                    "attr_default",
                    OpGenOverride::get_attr_default_for_reflect,
                    OpGenOverride::mut_attr_default_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpGenOverride_Rename>>(
                    "attr_rename",
                    OpGenOverride::get_attr_rename_for_reflect,
                    OpGenOverride::mut_attr_rename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpGenOverride_Rename>>(
                    "input_rename",
                    OpGenOverride::get_input_rename_for_reflect,
                    OpGenOverride::mut_input_rename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpGenOverride_Rename>>(
                    "output_rename",
                    OpGenOverride::get_output_rename_for_reflect,
                    OpGenOverride::mut_output_rename_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpGenOverride>(
                    "OpGenOverride",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpGenOverride {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_skip();
        self.clear_hide();
        self.clear_rename_to();
        self.clear_alias();
        self.clear_attr_default();
        self.clear_attr_rename();
        self.clear_input_rename();
        self.clear_output_rename();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpGenOverride {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpGenOverride {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpGenOverride_AttrDefault {
    // message fields
    pub name: ::std::string::String,
    pub value: ::protobuf::SingularPtrField<super::attr_value::AttrValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpGenOverride_AttrDefault {}

impl OpGenOverride_AttrDefault {
    pub fn new() -> OpGenOverride_AttrDefault {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpGenOverride_AttrDefault {
        static mut instance: ::protobuf::lazy::Lazy<OpGenOverride_AttrDefault> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpGenOverride_AttrDefault,
        };
        unsafe {
            instance.get(OpGenOverride_AttrDefault::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .tensorflow.AttrValue value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: super::attr_value::AttrValue) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut super::attr_value::AttrValue {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> super::attr_value::AttrValue {
        self.value.take().unwrap_or_else(|| super::attr_value::AttrValue::new())
    }

    pub fn get_value(&self) -> &super::attr_value::AttrValue {
        self.value.as_ref().unwrap_or_else(|| super::attr_value::AttrValue::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &mut self.value
    }
}

impl ::protobuf::Message for OpGenOverride_AttrDefault {
    fn is_initialized(&self) -> bool {
        for v in &self.value {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpGenOverride_AttrDefault {
    fn new() -> OpGenOverride_AttrDefault {
        OpGenOverride_AttrDefault::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpGenOverride_AttrDefault>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OpGenOverride_AttrDefault::get_name_for_reflect,
                    OpGenOverride_AttrDefault::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(
                    "value",
                    OpGenOverride_AttrDefault::get_value_for_reflect,
                    OpGenOverride_AttrDefault::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpGenOverride_AttrDefault>(
                    "OpGenOverride_AttrDefault",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpGenOverride_AttrDefault {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpGenOverride_AttrDefault {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpGenOverride_AttrDefault {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpGenOverride_Rename {
    // message fields
    pub from: ::std::string::String,
    pub to: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpGenOverride_Rename {}

impl OpGenOverride_Rename {
    pub fn new() -> OpGenOverride_Rename {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpGenOverride_Rename {
        static mut instance: ::protobuf::lazy::Lazy<OpGenOverride_Rename> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpGenOverride_Rename,
        };
        unsafe {
            instance.get(OpGenOverride_Rename::new)
        }
    }

    // string from = 1;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: ::std::string::String) {
        self.from = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut ::std::string::String {
        &mut self.from
    }

    // Take field
    pub fn take_from(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.from, ::std::string::String::new())
    }

    pub fn get_from(&self) -> &str {
        &self.from
    }

    fn get_from_for_reflect(&self) -> &::std::string::String {
        &self.from
    }

    fn mut_from_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.from
    }

    // string to = 2;

    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: ::std::string::String) {
        self.to = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut ::std::string::String {
        &mut self.to
    }

    // Take field
    pub fn take_to(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.to, ::std::string::String::new())
    }

    pub fn get_to(&self) -> &str {
        &self.to
    }

    fn get_to_for_reflect(&self) -> &::std::string::String {
        &self.to
    }

    fn mut_to_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.to
    }
}

impl ::protobuf::Message for OpGenOverride_Rename {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.from)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.to)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.from.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.from);
        }
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.to);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.from.is_empty() {
            os.write_string(1, &self.from)?;
        }
        if !self.to.is_empty() {
            os.write_string(2, &self.to)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpGenOverride_Rename {
    fn new() -> OpGenOverride_Rename {
        OpGenOverride_Rename::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpGenOverride_Rename>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "from",
                    OpGenOverride_Rename::get_from_for_reflect,
                    OpGenOverride_Rename::mut_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "to",
                    OpGenOverride_Rename::get_to_for_reflect,
                    OpGenOverride_Rename::mut_to_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpGenOverride_Rename>(
                    "OpGenOverride_Rename",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpGenOverride_Rename {
    fn clear(&mut self) {
        self.clear_from();
        self.clear_to();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpGenOverride_Rename {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpGenOverride_Rename {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpGenOverrides {
    // message fields
    pub op: ::protobuf::RepeatedField<OpGenOverride>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpGenOverrides {}

impl OpGenOverrides {
    pub fn new() -> OpGenOverrides {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpGenOverrides {
        static mut instance: ::protobuf::lazy::Lazy<OpGenOverrides> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpGenOverrides,
        };
        unsafe {
            instance.get(OpGenOverrides::new)
        }
    }

    // repeated .tensorflow.OpGenOverride op = 1;

    pub fn clear_op(&mut self) {
        self.op.clear();
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: ::protobuf::RepeatedField<OpGenOverride>) {
        self.op = v;
    }

    // Mutable pointer to the field.
    pub fn mut_op(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride> {
        &mut self.op
    }

    // Take field
    pub fn take_op(&mut self) -> ::protobuf::RepeatedField<OpGenOverride> {
        ::std::mem::replace(&mut self.op, ::protobuf::RepeatedField::new())
    }

    pub fn get_op(&self) -> &[OpGenOverride] {
        &self.op
    }

    fn get_op_for_reflect(&self) -> &::protobuf::RepeatedField<OpGenOverride> {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OpGenOverride> {
        &mut self.op
    }
}

impl ::protobuf::Message for OpGenOverrides {
    fn is_initialized(&self) -> bool {
        for v in &self.op {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.op)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.op {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.op {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpGenOverrides {
    fn new() -> OpGenOverrides {
        OpGenOverrides::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpGenOverrides>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpGenOverride>>(
                    "op",
                    OpGenOverrides::get_op_for_reflect,
                    OpGenOverrides::mut_op_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpGenOverrides>(
                    "OpGenOverrides",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpGenOverrides {
    fn clear(&mut self) {
        self.clear_op();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpGenOverrides {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpGenOverrides {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n0tensorflow/core/framework/op_gen_overrides.proto\x12\ntensorflow\x1a*\
    tensorflow/core/framework/attr_value.proto\"\x95\x04\n\rOpGenOverride\
    \x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x12\n\x04skip\x18\x02\
    \x20\x01(\x08R\x04skip\x12\x12\n\x04hide\x18\x03\x20\x01(\x08R\x04hide\
    \x12\x1b\n\trename_to\x18\x04\x20\x01(\tR\x08renameTo\x12\x14\n\x05alias\
    \x18\x05\x20\x03(\tR\x05alias\x12H\n\x0cattr_default\x18\x06\x20\x03(\
    \x0b2%.tensorflow.OpGenOverride.AttrDefaultR\x0battrDefault\x12A\n\x0bat\
    tr_rename\x18\x07\x20\x03(\x0b2\x20.tensorflow.OpGenOverride.RenameR\nat\
    trRename\x12C\n\x0cinput_rename\x18\x08\x20\x03(\x0b2\x20.tensorflow.OpG\
    enOverride.RenameR\x0binputRename\x12E\n\routput_rename\x18\t\x20\x03(\
    \x0b2\x20.tensorflow.OpGenOverride.RenameR\x0coutputRename\x1aN\n\x0bAtt\
    rDefault\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12+\n\x05value\
    \x18\x02\x20\x01(\x0b2\x15.tensorflow.AttrValueR\x05value\x1a,\n\x06Rena\
    me\x12\x12\n\x04from\x18\x01\x20\x01(\tR\x04from\x12\x0e\n\x02to\x18\x02\
    \x20\x01(\tR\x02to\";\n\x0eOpGenOverrides\x12)\n\x02op\x18\x01\x20\x03(\
    \x0b2\x19.tensorflow.OpGenOverrideR\x02opJ\x96\x17\n\x06\x12\x04\x03\0B\
    \x01\nh\n\x01\x0c\x12\x03\x03\0\x122^\x20Defines\x20the\x20text\x20forma\
    t\x20for\x20adding\x20per-op\x20overrides\x20for\x20client\n\x20language\
    \x20op\x20code\x20generators.\n\n\x08\n\x01\x02\x12\x03\x05\x08\x12\n\t\
    \n\x02\x03\0\x12\x03\x06\x073\n\xd6\x05\n\x02\x04\0\x12\x04\x15\0>\x01\
    \x1a\xc9\x05\x20Used\x20to\x20override\x20the\x20default\x20API\x20&\x20\
    behavior\x20in\x20the\x20generated\x20code\n\x20for\x20client\x20languag\
    es,\x20from\x20what\x20you\x20would\x20get\x20from\x20the\x20OpDef\x20al\
    one.\n\x20This\x20is\x20so\x20we\x20can\x20evolve\x20the\x20API\x20while\
    \x20remaining\x20backwards\n\x20compatible\x20when\x20interpretting\x20o\
    ld\x20graphs.\x20\x20Overrides\x20go\x20in\x20an\n\x20\"op_gen_overrides\
    .pbtxt\"\x20file\x20with\x20a\x20text-format\x20OpGenOverrides\n\x20mess\
    age.\x20\x20Right\x20now\x20these\x20only\x20apply\x20to\x20the\x20C++\
    \x20API.\n\x20TODO(josh11b):\x20In\x20the\x20future\x20there\x20will\x20\
    be\x20a\x20common\x20set\x20of\x20overrides\n\x20and\x20per-client-langu\
    age\x20overrides.\n\n\x20WARNING:\x20Be\x20*very*\x20careful\x20using\
    \x20these\x20features\x20--\x20these\x20overrides\n\x20can\x20change\x20\
    the\x20semantics\x20of\x20existing\x20code.\x20\x20These\x20changes\x20m\
    ay\x20need\n\x20to\x20wait\x20until\x20a\x20major\x20release\x20of\x20Te\
    nsorFlow\x20to\x20avoid\x20breaking\x20our\n\x20compatibility\x20promise\
    s.\n\n\n\n\x03\x04\0\x01\x12\x03\x15\x08\x15\n4\n\x04\x04\0\x02\0\x12\
    \x03\x17\x02\x12\x1a'\x20Name\x20of\x20the\x20op\x20to\x20apply\x20overr\
    ides\x20to.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x17\x02\x15\x17\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x17\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x17\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x17\x10\x11\n|\n\x04\
    \x04\0\x02\x01\x12\x03\x1b\x02\x10\x1ao\x20Do\x20not\x20include\x20this\
    \x20op\x20in\x20the\x20generated\x20API.\n\x20If\x20`skip`\x20is\x20true\
    ,\x20all\x20other\x20overrides\x20are\x20ignored\x20for\x20this\x20op.\n\
    \n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x1b\x02\x17\x12\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x1b\x02\x06\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x1b\x07\x0b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1b\x0e\x0f\nz\n\x04\
    \x04\0\x02\x02\x12\x03\x1f\x02\x10\x1am\x20Hide\x20this\x20op\x20by\x20p\
    utting\x20it\x20into\x20an\x20internal\x20namespace\x20(or\x20whatever\n\
    \x20is\x20appropriate\x20in\x20the\x20target\x20language).\n\n\r\n\x05\
    \x04\0\x02\x02\x04\x12\x04\x1f\x02\x1b\x10\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\x1f\x02\x06\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1f\x07\x0b\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x1f\x0e\x0f\n\x8e\x01\n\x04\x04\0\
    \x02\x03\x12\x03#\x02\x17\x1a\x80\x01\x20Use\x20a\x20different\x20name\
    \x20in\x20the\x20API\x20than\x20the\x20op's\x20name.\x20Note\x20that\n\
    \x20the\x20op's\x20name\x20in\x20`backticks`\x20will\x20also\x20be\x20re\
    placed\x20in\x20the\x20docs.\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04#\x02\
    \x1f\x10\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03#\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x03\x01\x12\x03#\t\x12\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03#\x15\
    \x16\n\x82\x01\n\x04\x04\0\x02\x04\x12\x03'\x02\x1c\x1au\x20Create\x20*a\
    dditional*\x20API\x20endpoints\x20with\x20different\x20names\x20(contras\
    t\n\x20with\x20rename_to,\x20which\x20affects\x20the\x20original\x20name\
    ).\n\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03'\x02\n\n\x0c\n\x05\x04\0\x02\
    \x04\x05\x12\x03'\x0b\x11\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03'\x12\x17\
    \n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03'\x1a\x1b\n\xb8\x02\n\x04\x04\0\
    \x03\0\x12\x04.\x021\x03\x1a\xa9\x02\x20Map\x20the\x20name\x20of\x20an\
    \x20attr\x20to\x20a\x20new\x20default\x20value\x20to\x20use.\x20\x20This\
    \n\x20default\x20will\x20be\x20used\x20when\x20creating\x20new\x20graphs\
    ,\x20as\x20opposed\x20to\x20the\n\x20default\x20in\x20the\x20OpDef,\x20w\
    hich\x20will\x20be\x20used\x20when\x20interpreting\x20old\n\x20GraphDefs\
    .\x20\x20If\x20this\x20attr\x20is\x20also\x20renamed\x20(using\x20attr_r\
    ename\n\x20below),\x20use\x20the\x20original\x20name\x20of\x20the\x20att\
    r.\n\n\x0c\n\x05\x04\0\x03\0\x01\x12\x03.\n\x15\n\r\n\x06\x04\0\x03\0\
    \x02\0\x12\x03/\x04\x14\n\x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04/\x04.\
    \x17\n\x0e\n\x07\x04\0\x03\0\x02\0\x05\x12\x03/\x04\n\n\x0e\n\x07\x04\0\
    \x03\0\x02\0\x01\x12\x03/\x0b\x0f\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\
    \x03/\x12\x13\n\r\n\x06\x04\0\x03\0\x02\x01\x12\x030\x04\x18\n\x0f\n\x07\
    \x04\0\x03\0\x02\x01\x04\x12\x040\x04/\x14\n\x0e\n\x07\x04\0\x03\0\x02\
    \x01\x06\x12\x030\x04\r\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x030\x0e\
    \x13\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x030\x16\x17\n\x0b\n\x04\
    \x04\0\x02\x05\x12\x032\x02(\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x032\x02\
    \n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x032\x0b\x16\n\x0c\n\x05\x04\0\x02\
    \x05\x01\x12\x032\x17#\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x032&'\n\xbd\
    \x01\n\x04\x04\0\x03\x01\x12\x047\x02:\x03\x1a\xae\x01\x20Change\x20the\
    \x20name\x20used\x20to\x20access\x20attrs/inputs/outputs\x20in\x20the\
    \x20API\n\x20from\x20what\x20is\x20used\x20in\x20the\x20GraphDef.\x20\
    \x20Note\x20that\x20these\x20names\x20in\n\x20`backticks`\x20will\x20als\
    o\x20be\x20replaced\x20in\x20the\x20docs.\n\n\x0c\n\x05\x04\0\x03\x01\
    \x01\x12\x037\n\x10\n\r\n\x06\x04\0\x03\x01\x02\0\x12\x038\x04\x14\n\x0f\
    \n\x07\x04\0\x03\x01\x02\0\x04\x12\x048\x047\x12\n\x0e\n\x07\x04\0\x03\
    \x01\x02\0\x05\x12\x038\x04\n\n\x0e\n\x07\x04\0\x03\x01\x02\0\x01\x12\
    \x038\x0b\x0f\n\x0e\n\x07\x04\0\x03\x01\x02\0\x03\x12\x038\x12\x13\n\r\n\
    \x06\x04\0\x03\x01\x02\x01\x12\x039\x04\x12\n\x0f\n\x07\x04\0\x03\x01\
    \x02\x01\x04\x12\x049\x048\x14\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x05\x12\
    \x039\x04\n\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x01\x12\x039\x0b\r\n\x0e\n\
    \x07\x04\0\x03\x01\x02\x01\x03\x12\x039\x10\x11\n\x0b\n\x04\x04\0\x02\
    \x06\x12\x03;\x02\"\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x03;\x02\n\n\x0c\n\
    \x05\x04\0\x02\x06\x06\x12\x03;\x0b\x11\n\x0c\n\x05\x04\0\x02\x06\x01\
    \x12\x03;\x12\x1d\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03;\x20!\n\x0b\n\
    \x04\x04\0\x02\x07\x12\x03<\x02#\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03<\
    \x02\n\n\x0c\n\x05\x04\0\x02\x07\x06\x12\x03<\x0b\x11\n\x0c\n\x05\x04\0\
    \x02\x07\x01\x12\x03<\x12\x1e\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03<!\"\
    \n\x0b\n\x04\x04\0\x02\x08\x12\x03=\x02$\n\x0c\n\x05\x04\0\x02\x08\x04\
    \x12\x03=\x02\n\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03=\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x08\x01\x12\x03=\x12\x1f\n\x0c\n\x05\x04\0\x02\x08\x03\
    \x12\x03=\"#\n\n\n\x02\x04\x01\x12\x04@\0B\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03@\x08\x16\n\x0b\n\x04\x04\x01\x02\0\x12\x03A\x02\x20\n\x0c\n\x05\x04\
    \x01\x02\0\x04\x12\x03A\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03A\x0b\
    \x18\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03A\x19\x1b\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03A\x1e\x1fb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
