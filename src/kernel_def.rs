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
pub struct KernelDef {
    // message fields
    pub op: ::std::string::String,
    pub device_type: ::std::string::String,
    pub constraint: ::protobuf::RepeatedField<KernelDef_AttrConstraint>,
    pub host_memory_arg: ::protobuf::RepeatedField<::std::string::String>,
    pub label: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KernelDef {}

impl KernelDef {
    pub fn new() -> KernelDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KernelDef {
        static mut instance: ::protobuf::lazy::Lazy<KernelDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KernelDef,
        };
        unsafe {
            instance.get(KernelDef::new)
        }
    }

    // string op = 1;

    pub fn clear_op(&mut self) {
        self.op.clear();
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: ::std::string::String) {
        self.op = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_op(&mut self) -> &mut ::std::string::String {
        &mut self.op
    }

    // Take field
    pub fn take_op(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.op, ::std::string::String::new())
    }

    pub fn get_op(&self) -> &str {
        &self.op
    }

    fn get_op_for_reflect(&self) -> &::std::string::String {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.op
    }

    // string device_type = 2;

    pub fn clear_device_type(&mut self) {
        self.device_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_device_type(&mut self, v: ::std::string::String) {
        self.device_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_type(&mut self) -> &mut ::std::string::String {
        &mut self.device_type
    }

    // Take field
    pub fn take_device_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.device_type, ::std::string::String::new())
    }

    pub fn get_device_type(&self) -> &str {
        &self.device_type
    }

    fn get_device_type_for_reflect(&self) -> &::std::string::String {
        &self.device_type
    }

    fn mut_device_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.device_type
    }

    // repeated .tensorflow.KernelDef.AttrConstraint constraint = 3;

    pub fn clear_constraint(&mut self) {
        self.constraint.clear();
    }

    // Param is passed by value, moved
    pub fn set_constraint(&mut self, v: ::protobuf::RepeatedField<KernelDef_AttrConstraint>) {
        self.constraint = v;
    }

    // Mutable pointer to the field.
    pub fn mut_constraint(&mut self) -> &mut ::protobuf::RepeatedField<KernelDef_AttrConstraint> {
        &mut self.constraint
    }

    // Take field
    pub fn take_constraint(&mut self) -> ::protobuf::RepeatedField<KernelDef_AttrConstraint> {
        ::std::mem::replace(&mut self.constraint, ::protobuf::RepeatedField::new())
    }

    pub fn get_constraint(&self) -> &[KernelDef_AttrConstraint] {
        &self.constraint
    }

    fn get_constraint_for_reflect(&self) -> &::protobuf::RepeatedField<KernelDef_AttrConstraint> {
        &self.constraint
    }

    fn mut_constraint_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KernelDef_AttrConstraint> {
        &mut self.constraint
    }

    // repeated string host_memory_arg = 4;

    pub fn clear_host_memory_arg(&mut self) {
        self.host_memory_arg.clear();
    }

    // Param is passed by value, moved
    pub fn set_host_memory_arg(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.host_memory_arg = v;
    }

    // Mutable pointer to the field.
    pub fn mut_host_memory_arg(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.host_memory_arg
    }

    // Take field
    pub fn take_host_memory_arg(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.host_memory_arg, ::protobuf::RepeatedField::new())
    }

    pub fn get_host_memory_arg(&self) -> &[::std::string::String] {
        &self.host_memory_arg
    }

    fn get_host_memory_arg_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.host_memory_arg
    }

    fn mut_host_memory_arg_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.host_memory_arg
    }

    // string label = 5;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.label, ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    fn get_label_for_reflect(&self) -> &::std::string::String {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.label
    }
}

impl ::protobuf::Message for KernelDef {
    fn is_initialized(&self) -> bool {
        for v in &self.constraint {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.op)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device_type)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.constraint)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.host_memory_arg)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.label)?;
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
        if !self.op.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.op);
        }
        if !self.device_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.device_type);
        }
        for value in &self.constraint {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.host_memory_arg {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if !self.label.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.label);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.op.is_empty() {
            os.write_string(1, &self.op)?;
        }
        if !self.device_type.is_empty() {
            os.write_string(2, &self.device_type)?;
        }
        for v in &self.constraint {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.host_memory_arg {
            os.write_string(4, &v)?;
        };
        if !self.label.is_empty() {
            os.write_string(5, &self.label)?;
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

impl ::protobuf::MessageStatic for KernelDef {
    fn new() -> KernelDef {
        KernelDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<KernelDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "op",
                    KernelDef::get_op_for_reflect,
                    KernelDef::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_type",
                    KernelDef::get_device_type_for_reflect,
                    KernelDef::mut_device_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KernelDef_AttrConstraint>>(
                    "constraint",
                    KernelDef::get_constraint_for_reflect,
                    KernelDef::mut_constraint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host_memory_arg",
                    KernelDef::get_host_memory_arg_for_reflect,
                    KernelDef::mut_host_memory_arg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "label",
                    KernelDef::get_label_for_reflect,
                    KernelDef::mut_label_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KernelDef>(
                    "KernelDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KernelDef {
    fn clear(&mut self) {
        self.clear_op();
        self.clear_device_type();
        self.clear_constraint();
        self.clear_host_memory_arg();
        self.clear_label();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KernelDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KernelDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KernelDef_AttrConstraint {
    // message fields
    pub name: ::std::string::String,
    pub allowed_values: ::protobuf::SingularPtrField<super::attr_value::AttrValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KernelDef_AttrConstraint {}

impl KernelDef_AttrConstraint {
    pub fn new() -> KernelDef_AttrConstraint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KernelDef_AttrConstraint {
        static mut instance: ::protobuf::lazy::Lazy<KernelDef_AttrConstraint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KernelDef_AttrConstraint,
        };
        unsafe {
            instance.get(KernelDef_AttrConstraint::new)
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

    // .tensorflow.AttrValue allowed_values = 2;

    pub fn clear_allowed_values(&mut self) {
        self.allowed_values.clear();
    }

    pub fn has_allowed_values(&self) -> bool {
        self.allowed_values.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed_values(&mut self, v: super::attr_value::AttrValue) {
        self.allowed_values = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allowed_values(&mut self) -> &mut super::attr_value::AttrValue {
        if self.allowed_values.is_none() {
            self.allowed_values.set_default();
        }
        self.allowed_values.as_mut().unwrap()
    }

    // Take field
    pub fn take_allowed_values(&mut self) -> super::attr_value::AttrValue {
        self.allowed_values.take().unwrap_or_else(|| super::attr_value::AttrValue::new())
    }

    pub fn get_allowed_values(&self) -> &super::attr_value::AttrValue {
        self.allowed_values.as_ref().unwrap_or_else(|| super::attr_value::AttrValue::default_instance())
    }

    fn get_allowed_values_for_reflect(&self) -> &::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &self.allowed_values
    }

    fn mut_allowed_values_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::attr_value::AttrValue> {
        &mut self.allowed_values
    }
}

impl ::protobuf::Message for KernelDef_AttrConstraint {
    fn is_initialized(&self) -> bool {
        for v in &self.allowed_values {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allowed_values)?;
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
        if let Some(ref v) = self.allowed_values.as_ref() {
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
        if let Some(ref v) = self.allowed_values.as_ref() {
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

impl ::protobuf::MessageStatic for KernelDef_AttrConstraint {
    fn new() -> KernelDef_AttrConstraint {
        KernelDef_AttrConstraint::new()
    }

    fn descriptor_static(_: ::std::option::Option<KernelDef_AttrConstraint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    KernelDef_AttrConstraint::get_name_for_reflect,
                    KernelDef_AttrConstraint::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(
                    "allowed_values",
                    KernelDef_AttrConstraint::get_allowed_values_for_reflect,
                    KernelDef_AttrConstraint::mut_allowed_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KernelDef_AttrConstraint>(
                    "KernelDef_AttrConstraint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KernelDef_AttrConstraint {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_allowed_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KernelDef_AttrConstraint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KernelDef_AttrConstraint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*tensorflow/core/framework/kernel_def.proto\x12\ntensorflow\x1a*tensor\
    flow/core/framework/attr_value.proto\"\xa4\x02\n\tKernelDef\x12\x0e\n\
    \x02op\x18\x01\x20\x01(\tR\x02op\x12\x1f\n\x0bdevice_type\x18\x02\x20\
    \x01(\tR\ndeviceType\x12D\n\nconstraint\x18\x03\x20\x03(\x0b2$.tensorflo\
    w.KernelDef.AttrConstraintR\nconstraint\x12&\n\x0fhost_memory_arg\x18\
    \x04\x20\x03(\tR\rhostMemoryArg\x12\x14\n\x05label\x18\x05\x20\x01(\tR\
    \x05label\x1ab\n\x0eAttrConstraint\x12\x12\n\x04name\x18\x01\x20\x01(\tR\
    \x04name\x12<\n\x0eallowed_values\x18\x02\x20\x01(\x0b2\x15.tensorflow.A\
    ttrValueR\rallowedValuesB0\n\x18org.tensorflow.frameworkB\x0fKernelDefPr\
    otosP\x01\xf8\x01\x01J\x86\x0b\n\x06\x12\x04\0\0#\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\
    \x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\
    \x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\
    \x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\
    \n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\
    \x03\x04\00\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\00\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\
    \x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e/\n\x08\n\x01\x08\
    \x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\
    \x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\
    \x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\
    \x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\
    \x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\
    \x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\
    \x03\0\x12\x03\x08\x073\n\n\n\x02\x04\0\x12\x04\n\0#\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\n\x08\x11\n,\n\x04\x04\0\x02\0\x12\x03\x0c\x02\x10\x1a\x1f\
    \x20Must\x20match\x20the\x20name\x20of\x20an\x20Op.\n\n\r\n\x05\x04\0\
    \x02\0\x04\x12\x04\x0c\x02\n\x13\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0c\
    \x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\t\x0b\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x0c\x0e\x0f\n2\n\x04\x04\0\x02\x01\x12\x03\x0f\x02\
    \x19\x1a%\x20Type\x20of\x20device\x20this\x20kernel\x20runs\x20on.\n\n\r\
    \n\x05\x04\0\x02\x01\x04\x12\x04\x0f\x02\x0c\x10\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0f\t\
    \x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0f\x17\x18\n\x0c\n\x04\x04\0\
    \x03\0\x12\x04\x11\x02\x18\x03\n\x0c\n\x05\x04\0\x03\0\x01\x12\x03\x11\n\
    \x18\n-\n\x06\x04\0\x03\0\x02\0\x12\x03\x13\x04\x14\x1a\x1e\x20Name\x20o\
    f\x20an\x20attr\x20from\x20the\x20Op.\n\n\x0f\n\x07\x04\0\x03\0\x02\0\
    \x04\x12\x04\x13\x04\x11\x1a\n\x0e\n\x07\x04\0\x03\0\x02\0\x05\x12\x03\
    \x13\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x03\x13\x0b\x0f\n\x0e\n\
    \x07\x04\0\x03\0\x02\0\x03\x12\x03\x13\x12\x13\n\x92\x01\n\x06\x04\0\x03\
    \0\x02\x01\x12\x03\x17\x04!\x1a\x82\x01\x20A\x20list\x20of\x20values\x20\
    that\x20this\x20kernel\x20supports\x20for\x20this\x20attr.\n\x20Like\x20\
    OpDef.AttrDef.allowed_values,\x20except\x20for\x20kernels\x20instead\x20\
    of\x20Ops.\n\n\x0f\n\x07\x04\0\x03\0\x02\x01\x04\x12\x04\x17\x04\x13\x14\
    \n\x0e\n\x07\x04\0\x03\0\x02\x01\x06\x12\x03\x17\x04\r\n\x0e\n\x07\x04\0\
    \x03\0\x02\x01\x01\x12\x03\x17\x0e\x1c\n\x0e\n\x07\x04\0\x03\0\x02\x01\
    \x03\x12\x03\x17\x1f\x20\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x19\x02)\n\
    \x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x19\x02\n\n\x0c\n\x05\x04\0\x02\x02\
    \x06\x12\x03\x19\x0b\x19\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x19\x1a$\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x19'(\ni\n\x04\x04\0\x02\x03\x12\
    \x03\x1d\x02&\x1a\\\x20Names\x20of\x20the\x20Op's\x20input_/output_args\
    \x20that\x20reside\x20in\x20host\x20memory\n\x20instead\x20of\x20device\
    \x20memory.\n\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x1d\x02\n\n\x0c\n\
    \x05\x04\0\x02\x03\x05\x12\x03\x1d\x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x03\x1d\x12!\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x1d$%\n\xa5\x01\
    \n\x04\x04\0\x02\x04\x12\x03\"\x02\x13\x1a\x97\x01\x20This\x20allows\x20\
    experimental\x20kernels\x20to\x20be\x20registered\x20for\x20an\x20op\x20\
    that\n\x20won't\x20be\x20used\x20unless\x20the\x20user\x20specifies\x20a\
    \x20\"_kernel\"\x20attr\x20with\n\x20value\x20matching\x20this.\n\n\r\n\
    \x05\x04\0\x02\x04\x04\x12\x04\"\x02\x1d&\n\x0c\n\x05\x04\0\x02\x04\x05\
    \x12\x03\"\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\"\t\x0e\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03\"\x11\x12b\x06proto3\
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
