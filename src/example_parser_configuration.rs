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
pub struct VarLenFeatureProto {
    // message fields
    pub dtype: super::types::DataType,
    pub values_output_tensor_name: ::std::string::String,
    pub indices_output_tensor_name: ::std::string::String,
    pub shapes_output_tensor_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VarLenFeatureProto {}

impl VarLenFeatureProto {
    pub fn new() -> VarLenFeatureProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VarLenFeatureProto {
        static mut instance: ::protobuf::lazy::Lazy<VarLenFeatureProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VarLenFeatureProto,
        };
        unsafe {
            instance.get(VarLenFeatureProto::new)
        }
    }

    // .tensorflow.DataType dtype = 1;

    pub fn clear_dtype(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_dtype(&mut self, v: super::types::DataType) {
        self.dtype = v;
    }

    pub fn get_dtype(&self) -> super::types::DataType {
        self.dtype
    }

    fn get_dtype_for_reflect(&self) -> &super::types::DataType {
        &self.dtype
    }

    fn mut_dtype_for_reflect(&mut self) -> &mut super::types::DataType {
        &mut self.dtype
    }

    // string values_output_tensor_name = 2;

    pub fn clear_values_output_tensor_name(&mut self) {
        self.values_output_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_values_output_tensor_name(&mut self, v: ::std::string::String) {
        self.values_output_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_values_output_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.values_output_tensor_name
    }

    // Take field
    pub fn take_values_output_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.values_output_tensor_name, ::std::string::String::new())
    }

    pub fn get_values_output_tensor_name(&self) -> &str {
        &self.values_output_tensor_name
    }

    fn get_values_output_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.values_output_tensor_name
    }

    fn mut_values_output_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.values_output_tensor_name
    }

    // string indices_output_tensor_name = 3;

    pub fn clear_indices_output_tensor_name(&mut self) {
        self.indices_output_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_indices_output_tensor_name(&mut self, v: ::std::string::String) {
        self.indices_output_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_indices_output_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.indices_output_tensor_name
    }

    // Take field
    pub fn take_indices_output_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.indices_output_tensor_name, ::std::string::String::new())
    }

    pub fn get_indices_output_tensor_name(&self) -> &str {
        &self.indices_output_tensor_name
    }

    fn get_indices_output_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.indices_output_tensor_name
    }

    fn mut_indices_output_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.indices_output_tensor_name
    }

    // string shapes_output_tensor_name = 4;

    pub fn clear_shapes_output_tensor_name(&mut self) {
        self.shapes_output_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_shapes_output_tensor_name(&mut self, v: ::std::string::String) {
        self.shapes_output_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shapes_output_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.shapes_output_tensor_name
    }

    // Take field
    pub fn take_shapes_output_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.shapes_output_tensor_name, ::std::string::String::new())
    }

    pub fn get_shapes_output_tensor_name(&self) -> &str {
        &self.shapes_output_tensor_name
    }

    fn get_shapes_output_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.shapes_output_tensor_name
    }

    fn mut_shapes_output_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.shapes_output_tensor_name
    }
}

impl ::protobuf::Message for VarLenFeatureProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dtype = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.values_output_tensor_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.indices_output_tensor_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.shapes_output_tensor_name)?;
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
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.dtype);
        }
        if !self.values_output_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.values_output_tensor_name);
        }
        if !self.indices_output_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.indices_output_tensor_name);
        }
        if !self.shapes_output_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.shapes_output_tensor_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(1, self.dtype.value())?;
        }
        if !self.values_output_tensor_name.is_empty() {
            os.write_string(2, &self.values_output_tensor_name)?;
        }
        if !self.indices_output_tensor_name.is_empty() {
            os.write_string(3, &self.indices_output_tensor_name)?;
        }
        if !self.shapes_output_tensor_name.is_empty() {
            os.write_string(4, &self.shapes_output_tensor_name)?;
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

impl ::protobuf::MessageStatic for VarLenFeatureProto {
    fn new() -> VarLenFeatureProto {
        VarLenFeatureProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<VarLenFeatureProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    VarLenFeatureProto::get_dtype_for_reflect,
                    VarLenFeatureProto::mut_dtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "values_output_tensor_name",
                    VarLenFeatureProto::get_values_output_tensor_name_for_reflect,
                    VarLenFeatureProto::mut_values_output_tensor_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "indices_output_tensor_name",
                    VarLenFeatureProto::get_indices_output_tensor_name_for_reflect,
                    VarLenFeatureProto::mut_indices_output_tensor_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "shapes_output_tensor_name",
                    VarLenFeatureProto::get_shapes_output_tensor_name_for_reflect,
                    VarLenFeatureProto::mut_shapes_output_tensor_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VarLenFeatureProto>(
                    "VarLenFeatureProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VarLenFeatureProto {
    fn clear(&mut self) {
        self.clear_dtype();
        self.clear_values_output_tensor_name();
        self.clear_indices_output_tensor_name();
        self.clear_shapes_output_tensor_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VarLenFeatureProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VarLenFeatureProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FixedLenFeatureProto {
    // message fields
    pub dtype: super::types::DataType,
    pub shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    pub default_value: ::protobuf::SingularPtrField<super::tensor::TensorProto>,
    pub values_output_tensor_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FixedLenFeatureProto {}

impl FixedLenFeatureProto {
    pub fn new() -> FixedLenFeatureProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FixedLenFeatureProto {
        static mut instance: ::protobuf::lazy::Lazy<FixedLenFeatureProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FixedLenFeatureProto,
        };
        unsafe {
            instance.get(FixedLenFeatureProto::new)
        }
    }

    // .tensorflow.DataType dtype = 1;

    pub fn clear_dtype(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_dtype(&mut self, v: super::types::DataType) {
        self.dtype = v;
    }

    pub fn get_dtype(&self) -> super::types::DataType {
        self.dtype
    }

    fn get_dtype_for_reflect(&self) -> &super::types::DataType {
        &self.dtype
    }

    fn mut_dtype_for_reflect(&mut self) -> &mut super::types::DataType {
        &mut self.dtype
    }

    // .tensorflow.TensorShapeProto shape = 2;

    pub fn clear_shape(&mut self) {
        self.shape.clear();
    }

    pub fn has_shape(&self) -> bool {
        self.shape.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.shape = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if self.shape.is_none() {
            self.shape.set_default();
        }
        self.shape.as_mut().unwrap()
    }

    // Take field
    pub fn take_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        self.shape.take().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::new())
    }

    pub fn get_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        self.shape.as_ref().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::default_instance())
    }

    fn get_shape_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &self.shape
    }

    fn mut_shape_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &mut self.shape
    }

    // .tensorflow.TensorProto default_value = 3;

    pub fn clear_default_value(&mut self) {
        self.default_value.clear();
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_value(&mut self, v: super::tensor::TensorProto) {
        self.default_value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_value(&mut self) -> &mut super::tensor::TensorProto {
        if self.default_value.is_none() {
            self.default_value.set_default();
        }
        self.default_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_default_value(&mut self) -> super::tensor::TensorProto {
        self.default_value.take().unwrap_or_else(|| super::tensor::TensorProto::new())
    }

    pub fn get_default_value(&self) -> &super::tensor::TensorProto {
        self.default_value.as_ref().unwrap_or_else(|| super::tensor::TensorProto::default_instance())
    }

    fn get_default_value_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor::TensorProto> {
        &self.default_value
    }

    fn mut_default_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor::TensorProto> {
        &mut self.default_value
    }

    // string values_output_tensor_name = 4;

    pub fn clear_values_output_tensor_name(&mut self) {
        self.values_output_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_values_output_tensor_name(&mut self, v: ::std::string::String) {
        self.values_output_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_values_output_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.values_output_tensor_name
    }

    // Take field
    pub fn take_values_output_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.values_output_tensor_name, ::std::string::String::new())
    }

    pub fn get_values_output_tensor_name(&self) -> &str {
        &self.values_output_tensor_name
    }

    fn get_values_output_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.values_output_tensor_name
    }

    fn mut_values_output_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.values_output_tensor_name
    }
}

impl ::protobuf::Message for FixedLenFeatureProto {
    fn is_initialized(&self) -> bool {
        for v in &self.shape {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.default_value {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dtype = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shape)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.default_value)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.values_output_tensor_name)?;
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
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.dtype);
        }
        if let Some(ref v) = self.shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.default_value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.values_output_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.values_output_tensor_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(1, self.dtype.value())?;
        }
        if let Some(ref v) = self.shape.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.default_value.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.values_output_tensor_name.is_empty() {
            os.write_string(4, &self.values_output_tensor_name)?;
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

impl ::protobuf::MessageStatic for FixedLenFeatureProto {
    fn new() -> FixedLenFeatureProto {
        FixedLenFeatureProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FixedLenFeatureProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    FixedLenFeatureProto::get_dtype_for_reflect,
                    FixedLenFeatureProto::mut_dtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "shape",
                    FixedLenFeatureProto::get_shape_for_reflect,
                    FixedLenFeatureProto::mut_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor::TensorProto>>(
                    "default_value",
                    FixedLenFeatureProto::get_default_value_for_reflect,
                    FixedLenFeatureProto::mut_default_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "values_output_tensor_name",
                    FixedLenFeatureProto::get_values_output_tensor_name_for_reflect,
                    FixedLenFeatureProto::mut_values_output_tensor_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FixedLenFeatureProto>(
                    "FixedLenFeatureProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FixedLenFeatureProto {
    fn clear(&mut self) {
        self.clear_dtype();
        self.clear_shape();
        self.clear_default_value();
        self.clear_values_output_tensor_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FixedLenFeatureProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FixedLenFeatureProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeatureConfiguration {
    // message oneof groups
    config: ::std::option::Option<FeatureConfiguration_oneof_config>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeatureConfiguration {}

#[derive(Clone,PartialEq)]
pub enum FeatureConfiguration_oneof_config {
    fixed_len_feature(FixedLenFeatureProto),
    var_len_feature(VarLenFeatureProto),
}

impl FeatureConfiguration {
    pub fn new() -> FeatureConfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeatureConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<FeatureConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeatureConfiguration,
        };
        unsafe {
            instance.get(FeatureConfiguration::new)
        }
    }

    // .tensorflow.FixedLenFeatureProto fixed_len_feature = 1;

    pub fn clear_fixed_len_feature(&mut self) {
        self.config = ::std::option::Option::None;
    }

    pub fn has_fixed_len_feature(&self) -> bool {
        match self.config {
            ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fixed_len_feature(&mut self, v: FixedLenFeatureProto) {
        self.config = ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(v))
    }

    // Mutable pointer to the field.
    pub fn mut_fixed_len_feature(&mut self) -> &mut FixedLenFeatureProto {
        if let ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(_)) = self.config {
        } else {
            self.config = ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(FixedLenFeatureProto::new()));
        }
        match self.config {
            ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_fixed_len_feature(&mut self) -> FixedLenFeatureProto {
        if self.has_fixed_len_feature() {
            match self.config.take() {
                ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(v)) => v,
                _ => panic!(),
            }
        } else {
            FixedLenFeatureProto::new()
        }
    }

    pub fn get_fixed_len_feature(&self) -> &FixedLenFeatureProto {
        match self.config {
            ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(ref v)) => v,
            _ => FixedLenFeatureProto::default_instance(),
        }
    }

    // .tensorflow.VarLenFeatureProto var_len_feature = 2;

    pub fn clear_var_len_feature(&mut self) {
        self.config = ::std::option::Option::None;
    }

    pub fn has_var_len_feature(&self) -> bool {
        match self.config {
            ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_var_len_feature(&mut self, v: VarLenFeatureProto) {
        self.config = ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(v))
    }

    // Mutable pointer to the field.
    pub fn mut_var_len_feature(&mut self) -> &mut VarLenFeatureProto {
        if let ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(_)) = self.config {
        } else {
            self.config = ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(VarLenFeatureProto::new()));
        }
        match self.config {
            ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_var_len_feature(&mut self) -> VarLenFeatureProto {
        if self.has_var_len_feature() {
            match self.config.take() {
                ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(v)) => v,
                _ => panic!(),
            }
        } else {
            VarLenFeatureProto::new()
        }
    }

    pub fn get_var_len_feature(&self) -> &VarLenFeatureProto {
        match self.config {
            ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(ref v)) => v,
            _ => VarLenFeatureProto::default_instance(),
        }
    }
}

impl ::protobuf::Message for FeatureConfiguration {
    fn is_initialized(&self) -> bool {
        if let Some(FeatureConfiguration_oneof_config::fixed_len_feature(ref v)) = self.config {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(FeatureConfiguration_oneof_config::var_len_feature(ref v)) = self.config {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.config = ::std::option::Option::Some(FeatureConfiguration_oneof_config::fixed_len_feature(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.config = ::std::option::Option::Some(FeatureConfiguration_oneof_config::var_len_feature(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.config {
            match v {
                &FeatureConfiguration_oneof_config::fixed_len_feature(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &FeatureConfiguration_oneof_config::var_len_feature(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.config {
            match v {
                &FeatureConfiguration_oneof_config::fixed_len_feature(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &FeatureConfiguration_oneof_config::var_len_feature(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for FeatureConfiguration {
    fn new() -> FeatureConfiguration {
        FeatureConfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeatureConfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, FixedLenFeatureProto>(
                    "fixed_len_feature",
                    FeatureConfiguration::has_fixed_len_feature,
                    FeatureConfiguration::get_fixed_len_feature,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, VarLenFeatureProto>(
                    "var_len_feature",
                    FeatureConfiguration::has_var_len_feature,
                    FeatureConfiguration::get_var_len_feature,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeatureConfiguration>(
                    "FeatureConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeatureConfiguration {
    fn clear(&mut self) {
        self.clear_fixed_len_feature();
        self.clear_var_len_feature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExampleParserConfiguration {
    // message fields
    pub feature_map: ::std::collections::HashMap<::std::string::String, FeatureConfiguration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExampleParserConfiguration {}

impl ExampleParserConfiguration {
    pub fn new() -> ExampleParserConfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExampleParserConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<ExampleParserConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExampleParserConfiguration,
        };
        unsafe {
            instance.get(ExampleParserConfiguration::new)
        }
    }

    // repeated .tensorflow.ExampleParserConfiguration.FeatureMapEntry feature_map = 1;

    pub fn clear_feature_map(&mut self) {
        self.feature_map.clear();
    }

    // Param is passed by value, moved
    pub fn set_feature_map(&mut self, v: ::std::collections::HashMap<::std::string::String, FeatureConfiguration>) {
        self.feature_map = v;
    }

    // Mutable pointer to the field.
    pub fn mut_feature_map(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, FeatureConfiguration> {
        &mut self.feature_map
    }

    // Take field
    pub fn take_feature_map(&mut self) -> ::std::collections::HashMap<::std::string::String, FeatureConfiguration> {
        ::std::mem::replace(&mut self.feature_map, ::std::collections::HashMap::new())
    }

    pub fn get_feature_map(&self) -> &::std::collections::HashMap<::std::string::String, FeatureConfiguration> {
        &self.feature_map
    }

    fn get_feature_map_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, FeatureConfiguration> {
        &self.feature_map
    }

    fn mut_feature_map_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, FeatureConfiguration> {
        &mut self.feature_map
    }
}

impl ::protobuf::Message for ExampleParserConfiguration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureConfiguration>>(wire_type, is, &mut self.feature_map)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureConfiguration>>(1, &self.feature_map);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureConfiguration>>(1, &self.feature_map, os)?;
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

impl ::protobuf::MessageStatic for ExampleParserConfiguration {
    fn new() -> ExampleParserConfiguration {
        ExampleParserConfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExampleParserConfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FeatureConfiguration>>(
                    "feature_map",
                    ExampleParserConfiguration::get_feature_map_for_reflect,
                    ExampleParserConfiguration::mut_feature_map_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExampleParserConfiguration>(
                    "ExampleParserConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExampleParserConfiguration {
    fn clear(&mut self) {
        self.clear_feature_map();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExampleParserConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExampleParserConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n:tensorflow/core/example/example_parser_configuration.proto\x12\ntenso\
    rflow\x1a,tensorflow/core/framework/tensor_shape.proto\x1a&tensorflow/co\
    re/framework/tensor.proto\x1a%tensorflow/core/framework/types.proto\"\
    \xf3\x01\n\x12VarLenFeatureProto\x12*\n\x05dtype\x18\x01\x20\x01(\x0e2\
    \x14.tensorflow.DataTypeR\x05dtype\x129\n\x19values_output_tensor_name\
    \x18\x02\x20\x01(\tR\x16valuesOutputTensorName\x12;\n\x1aindices_output_\
    tensor_name\x18\x03\x20\x01(\tR\x17indicesOutputTensorName\x129\n\x19sha\
    pes_output_tensor_name\x18\x04\x20\x01(\tR\x16shapesOutputTensorName\"\
    \xef\x01\n\x14FixedLenFeatureProto\x12*\n\x05dtype\x18\x01\x20\x01(\x0e2\
    \x14.tensorflow.DataTypeR\x05dtype\x122\n\x05shape\x18\x02\x20\x01(\x0b2\
    \x1c.tensorflow.TensorShapeProtoR\x05shape\x12<\n\rdefault_value\x18\x03\
    \x20\x01(\x0b2\x17.tensorflow.TensorProtoR\x0cdefaultValue\x129\n\x19val\
    ues_output_tensor_name\x18\x04\x20\x01(\tR\x16valuesOutputTensorName\"\
    \xba\x01\n\x14FeatureConfiguration\x12N\n\x11fixed_len_feature\x18\x01\
    \x20\x01(\x0b2\x20.tensorflow.FixedLenFeatureProtoH\0R\x0ffixedLenFeatur\
    e\x12H\n\x0fvar_len_feature\x18\x02\x20\x01(\x0b2\x1e.tensorflow.VarLenF\
    eatureProtoH\0R\rvarLenFeatureB\x08\n\x06config\"\xd6\x01\n\x1aExamplePa\
    rserConfiguration\x12W\n\x0bfeature_map\x18\x01\x20\x03(\x0b26.tensorflo\
    w.ExampleParserConfiguration.FeatureMapEntryR\nfeatureMap\x1a_\n\x0fFeat\
    ureMapEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x126\n\x05value\
    \x18\x02\x20\x01(\x0b2\x20.tensorflow.FeatureConfigurationR\x05value:\
    \x028\x01B?\n\x16org.tensorflow.exampleB\x20ExampleParserConfigurationPr\
    otosP\x01\xf8\x01\x01J\xb3\n\n\x06\x12\x04\x02\0%\x02\nV\n\x01\x0c\x12\
    \x03\x02\0\x122L\x20Protocol\x20messages\x20for\x20describing\x20the\x20\
    configuration\x20of\x20the\x20ExampleParserOp.\n\n\x08\n\x01\x08\x12\x03\
    \x04\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x04\0\x1f\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x04\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x04\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x04\x07\x17\n\
    \x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x04\x1a\x1e\n\x08\n\x01\x08\x12\x03\
    \x05\0A\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x05\0A\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x05\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x05\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x05\x07\x1b\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x05\x1e@\n\x08\n\x01\x08\x12\x03\
    \x06\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x06\0\"\n\x0c\n\x05\x08\xe7\
    \x07\x02\x02\x12\x03\x06\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\
    \x06\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x06\x07\x1a\n\
    \x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x06\x1d!\n\x08\n\x01\x08\x12\x03\
    \x07\0/\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x07\0/\n\x0c\n\x05\x08\xe7\
    \x07\x03\x02\x12\x03\x07\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\
    \x07\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x07\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x07\x16.\n\x08\n\x01\x02\x12\x03\
    \x08\x08\x12\n\t\n\x02\x03\0\x12\x03\n\x075\n\t\n\x02\x03\x01\x12\x03\
    \x0b\x07/\n\t\n\x02\x03\x02\x12\x03\x0c\x07.\n\n\n\x02\x04\0\x12\x04\x0e\
    \0\x13\x01\n\n\n\x03\x04\0\x01\x12\x03\x0e\x08\x1a\n\x0b\n\x04\x04\0\x02\
    \0\x12\x03\x0f\x02\x20\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0f\x02\x0e\x1c\
    \n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0f\x02\x15\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x0f\x16\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0f\x1e\x1f\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\x10\x02'\n\r\n\x05\x04\0\x02\x01\x04\
    \x12\x04\x10\x02\x0f\x20\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x10\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x10\t\"\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x10%&\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x11\x02(\n\r\n\
    \x05\x04\0\x02\x02\x04\x12\x04\x11\x02\x10'\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x11\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x11\t#\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x11&'\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\x12\x02'\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x12\x02\x11(\n\x0c\n\
    \x05\x04\0\x02\x03\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x03\x12\t\"\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x12%&\n\n\n\x02\
    \x04\x01\x12\x04\x15\0\x1a\x01\n\n\n\x03\x04\x01\x01\x12\x03\x15\x08\x1c\
    \n\x0b\n\x04\x04\x01\x02\0\x12\x03\x16\x02\x20\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04\x16\x02\x15\x1e\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x16\
    \x02\x15\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x16\x16\x1b\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x16\x1e\x1f\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03\x17\x02(\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x17\x02\x16\x20\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x17\x02\x1d\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x17\x1e#\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x17&'\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x18\x02+\n\r\n\x05\x04\x01\
    \x02\x02\x04\x12\x04\x18\x02\x17(\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\
    \x03\x18\x02\x18\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x18\x19&\n\x0c\
    \n\x05\x04\x01\x02\x02\x03\x12\x03\x18)*\n\x0b\n\x04\x04\x01\x02\x03\x12\
    \x03\x19\x02'\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\x19\x02\x18+\n\x0c\n\
    \x05\x04\x01\x02\x03\x05\x12\x03\x19\x02\x08\n\x0c\n\x05\x04\x01\x02\x03\
    \x01\x12\x03\x19\t\"\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x19%&\n\n\n\
    \x02\x04\x02\x12\x04\x1c\0!\x01\n\n\n\x03\x04\x02\x01\x12\x03\x1c\x08\
    \x1c\n\x0c\n\x04\x04\x02\x08\0\x12\x04\x1d\x02\x20\x03\n\x0c\n\x05\x04\
    \x02\x08\0\x01\x12\x03\x1d\x08\x0e\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1e\
    \x04/\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x1e\x04\x18\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03\x1e\x19*\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\
    \x1e-.\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x1f\x04+\n\x0c\n\x05\x04\x02\
    \x02\x01\x06\x12\x03\x1f\x04\x16\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\
    \x1f\x17&\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x1f)*\n\n\n\x02\x04\
    \x03\x12\x04#\0%\x01\n\n\n\x03\x04\x03\x01\x12\x03#\x08\"\n\x0b\n\x04\
    \x04\x03\x02\0\x12\x03$\x024\n\r\n\x05\x04\x03\x02\0\x04\x12\x04$\x02#$\
    \n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03$\x02#\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x03$$/\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03$23b\x06proto3\
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
