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
pub struct RemoteFusedGraphExecuteInfo {
    // message fields
    pub remote_graph: ::protobuf::SingularPtrField<super::graph::GraphDef>,
    pub graph_input_node_name: ::protobuf::RepeatedField<::std::string::String>,
    pub graph_output_node_name: ::protobuf::RepeatedField<::std::string::String>,
    pub executor_name: ::std::string::String,
    pub serialized_executor_parameters: ::std::vec::Vec<u8>,
    pub default_graph_input_tensor_shape: ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>,
    pub default_graph_output_tensor_shape: ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoteFusedGraphExecuteInfo {}

impl RemoteFusedGraphExecuteInfo {
    pub fn new() -> RemoteFusedGraphExecuteInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoteFusedGraphExecuteInfo {
        static mut instance: ::protobuf::lazy::Lazy<RemoteFusedGraphExecuteInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoteFusedGraphExecuteInfo,
        };
        unsafe {
            instance.get(RemoteFusedGraphExecuteInfo::new)
        }
    }

    // .tensorflow.GraphDef remote_graph = 1;

    pub fn clear_remote_graph(&mut self) {
        self.remote_graph.clear();
    }

    pub fn has_remote_graph(&self) -> bool {
        self.remote_graph.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remote_graph(&mut self, v: super::graph::GraphDef) {
        self.remote_graph = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remote_graph(&mut self) -> &mut super::graph::GraphDef {
        if self.remote_graph.is_none() {
            self.remote_graph.set_default();
        }
        self.remote_graph.as_mut().unwrap()
    }

    // Take field
    pub fn take_remote_graph(&mut self) -> super::graph::GraphDef {
        self.remote_graph.take().unwrap_or_else(|| super::graph::GraphDef::new())
    }

    pub fn get_remote_graph(&self) -> &super::graph::GraphDef {
        self.remote_graph.as_ref().unwrap_or_else(|| super::graph::GraphDef::default_instance())
    }

    fn get_remote_graph_for_reflect(&self) -> &::protobuf::SingularPtrField<super::graph::GraphDef> {
        &self.remote_graph
    }

    fn mut_remote_graph_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::graph::GraphDef> {
        &mut self.remote_graph
    }

    // repeated string graph_input_node_name = 2;

    pub fn clear_graph_input_node_name(&mut self) {
        self.graph_input_node_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_input_node_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.graph_input_node_name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_input_node_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.graph_input_node_name
    }

    // Take field
    pub fn take_graph_input_node_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.graph_input_node_name, ::protobuf::RepeatedField::new())
    }

    pub fn get_graph_input_node_name(&self) -> &[::std::string::String] {
        &self.graph_input_node_name
    }

    fn get_graph_input_node_name_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.graph_input_node_name
    }

    fn mut_graph_input_node_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.graph_input_node_name
    }

    // repeated string graph_output_node_name = 3;

    pub fn clear_graph_output_node_name(&mut self) {
        self.graph_output_node_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_output_node_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.graph_output_node_name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_output_node_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.graph_output_node_name
    }

    // Take field
    pub fn take_graph_output_node_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.graph_output_node_name, ::protobuf::RepeatedField::new())
    }

    pub fn get_graph_output_node_name(&self) -> &[::std::string::String] {
        &self.graph_output_node_name
    }

    fn get_graph_output_node_name_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.graph_output_node_name
    }

    fn mut_graph_output_node_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.graph_output_node_name
    }

    // string executor_name = 4;

    pub fn clear_executor_name(&mut self) {
        self.executor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_executor_name(&mut self, v: ::std::string::String) {
        self.executor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_name(&mut self) -> &mut ::std::string::String {
        &mut self.executor_name
    }

    // Take field
    pub fn take_executor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.executor_name, ::std::string::String::new())
    }

    pub fn get_executor_name(&self) -> &str {
        &self.executor_name
    }

    fn get_executor_name_for_reflect(&self) -> &::std::string::String {
        &self.executor_name
    }

    fn mut_executor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.executor_name
    }

    // bytes serialized_executor_parameters = 5;

    pub fn clear_serialized_executor_parameters(&mut self) {
        self.serialized_executor_parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_serialized_executor_parameters(&mut self, v: ::std::vec::Vec<u8>) {
        self.serialized_executor_parameters = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialized_executor_parameters(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.serialized_executor_parameters
    }

    // Take field
    pub fn take_serialized_executor_parameters(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.serialized_executor_parameters, ::std::vec::Vec::new())
    }

    pub fn get_serialized_executor_parameters(&self) -> &[u8] {
        &self.serialized_executor_parameters
    }

    fn get_serialized_executor_parameters_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.serialized_executor_parameters
    }

    fn mut_serialized_executor_parameters_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.serialized_executor_parameters
    }

    // repeated .tensorflow.RemoteFusedGraphExecuteInfo.TensorShapeTypeProto default_graph_input_tensor_shape = 6;

    pub fn clear_default_graph_input_tensor_shape(&mut self) {
        self.default_graph_input_tensor_shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_default_graph_input_tensor_shape(&mut self, v: ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>) {
        self.default_graph_input_tensor_shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_default_graph_input_tensor_shape(&mut self) -> &mut ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        &mut self.default_graph_input_tensor_shape
    }

    // Take field
    pub fn take_default_graph_input_tensor_shape(&mut self) -> ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        ::std::mem::replace(&mut self.default_graph_input_tensor_shape, ::protobuf::RepeatedField::new())
    }

    pub fn get_default_graph_input_tensor_shape(&self) -> &[RemoteFusedGraphExecuteInfo_TensorShapeTypeProto] {
        &self.default_graph_input_tensor_shape
    }

    fn get_default_graph_input_tensor_shape_for_reflect(&self) -> &::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        &self.default_graph_input_tensor_shape
    }

    fn mut_default_graph_input_tensor_shape_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        &mut self.default_graph_input_tensor_shape
    }

    // repeated .tensorflow.RemoteFusedGraphExecuteInfo.TensorShapeTypeProto default_graph_output_tensor_shape = 7;

    pub fn clear_default_graph_output_tensor_shape(&mut self) {
        self.default_graph_output_tensor_shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_default_graph_output_tensor_shape(&mut self, v: ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>) {
        self.default_graph_output_tensor_shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_default_graph_output_tensor_shape(&mut self) -> &mut ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        &mut self.default_graph_output_tensor_shape
    }

    // Take field
    pub fn take_default_graph_output_tensor_shape(&mut self) -> ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        ::std::mem::replace(&mut self.default_graph_output_tensor_shape, ::protobuf::RepeatedField::new())
    }

    pub fn get_default_graph_output_tensor_shape(&self) -> &[RemoteFusedGraphExecuteInfo_TensorShapeTypeProto] {
        &self.default_graph_output_tensor_shape
    }

    fn get_default_graph_output_tensor_shape_for_reflect(&self) -> &::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        &self.default_graph_output_tensor_shape
    }

    fn mut_default_graph_output_tensor_shape_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> {
        &mut self.default_graph_output_tensor_shape
    }
}

impl ::protobuf::Message for RemoteFusedGraphExecuteInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.remote_graph {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.default_graph_input_tensor_shape {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.default_graph_output_tensor_shape {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.remote_graph)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.graph_input_node_name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.graph_output_node_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.executor_name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.serialized_executor_parameters)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.default_graph_input_tensor_shape)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.default_graph_output_tensor_shape)?;
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
        if let Some(ref v) = self.remote_graph.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.graph_input_node_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.graph_output_node_name {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if !self.executor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.executor_name);
        }
        if !self.serialized_executor_parameters.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.serialized_executor_parameters);
        }
        for value in &self.default_graph_input_tensor_shape {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.default_graph_output_tensor_shape {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.remote_graph.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.graph_input_node_name {
            os.write_string(2, &v)?;
        };
        for v in &self.graph_output_node_name {
            os.write_string(3, &v)?;
        };
        if !self.executor_name.is_empty() {
            os.write_string(4, &self.executor_name)?;
        }
        if !self.serialized_executor_parameters.is_empty() {
            os.write_bytes(5, &self.serialized_executor_parameters)?;
        }
        for v in &self.default_graph_input_tensor_shape {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.default_graph_output_tensor_shape {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RemoteFusedGraphExecuteInfo {
    fn new() -> RemoteFusedGraphExecuteInfo {
        RemoteFusedGraphExecuteInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoteFusedGraphExecuteInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::graph::GraphDef>>(
                    "remote_graph",
                    RemoteFusedGraphExecuteInfo::get_remote_graph_for_reflect,
                    RemoteFusedGraphExecuteInfo::mut_remote_graph_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "graph_input_node_name",
                    RemoteFusedGraphExecuteInfo::get_graph_input_node_name_for_reflect,
                    RemoteFusedGraphExecuteInfo::mut_graph_input_node_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "graph_output_node_name",
                    RemoteFusedGraphExecuteInfo::get_graph_output_node_name_for_reflect,
                    RemoteFusedGraphExecuteInfo::mut_graph_output_node_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "executor_name",
                    RemoteFusedGraphExecuteInfo::get_executor_name_for_reflect,
                    RemoteFusedGraphExecuteInfo::mut_executor_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "serialized_executor_parameters",
                    RemoteFusedGraphExecuteInfo::get_serialized_executor_parameters_for_reflect,
                    RemoteFusedGraphExecuteInfo::mut_serialized_executor_parameters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>>(
                    "default_graph_input_tensor_shape",
                    RemoteFusedGraphExecuteInfo::get_default_graph_input_tensor_shape_for_reflect,
                    RemoteFusedGraphExecuteInfo::mut_default_graph_input_tensor_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>>(
                    "default_graph_output_tensor_shape",
                    RemoteFusedGraphExecuteInfo::get_default_graph_output_tensor_shape_for_reflect,
                    RemoteFusedGraphExecuteInfo::mut_default_graph_output_tensor_shape_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoteFusedGraphExecuteInfo>(
                    "RemoteFusedGraphExecuteInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoteFusedGraphExecuteInfo {
    fn clear(&mut self) {
        self.clear_remote_graph();
        self.clear_graph_input_node_name();
        self.clear_graph_output_node_name();
        self.clear_executor_name();
        self.clear_serialized_executor_parameters();
        self.clear_default_graph_input_tensor_shape();
        self.clear_default_graph_output_tensor_shape();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoteFusedGraphExecuteInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoteFusedGraphExecuteInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
    // message fields
    pub dtype: super::types::DataType,
    pub shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {}

impl RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
    pub fn new() -> RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoteFusedGraphExecuteInfo_TensorShapeTypeProto,
        };
        unsafe {
            instance.get(RemoteFusedGraphExecuteInfo_TensorShapeTypeProto::new)
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
}

impl ::protobuf::Message for RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
    fn is_initialized(&self) -> bool {
        for v in &self.shape {
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

impl ::protobuf::MessageStatic for RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
    fn new() -> RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
        RemoteFusedGraphExecuteInfo_TensorShapeTypeProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    RemoteFusedGraphExecuteInfo_TensorShapeTypeProto::get_dtype_for_reflect,
                    RemoteFusedGraphExecuteInfo_TensorShapeTypeProto::mut_dtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "shape",
                    RemoteFusedGraphExecuteInfo_TensorShapeTypeProto::get_shape_for_reflect,
                    RemoteFusedGraphExecuteInfo_TensorShapeTypeProto::mut_shape_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoteFusedGraphExecuteInfo_TensorShapeTypeProto>(
                    "RemoteFusedGraphExecuteInfo_TensorShapeTypeProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
    fn clear(&mut self) {
        self.clear_dtype();
        self.clear_shape();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoteFusedGraphExecuteInfo_TensorShapeTypeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RemoteFusedGraphExecuteInfo_NodeType {
    UNUSED = 0,
    GRAPH_INPUT = 1,
    GRAPH_OUTPUT = 2,
    FUSED_NODE = 3,
    BORDER_INPUT = 4,
    BORDER_OUTPUT = 5,
}

impl ::protobuf::ProtobufEnum for RemoteFusedGraphExecuteInfo_NodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RemoteFusedGraphExecuteInfo_NodeType> {
        match value {
            0 => ::std::option::Option::Some(RemoteFusedGraphExecuteInfo_NodeType::UNUSED),
            1 => ::std::option::Option::Some(RemoteFusedGraphExecuteInfo_NodeType::GRAPH_INPUT),
            2 => ::std::option::Option::Some(RemoteFusedGraphExecuteInfo_NodeType::GRAPH_OUTPUT),
            3 => ::std::option::Option::Some(RemoteFusedGraphExecuteInfo_NodeType::FUSED_NODE),
            4 => ::std::option::Option::Some(RemoteFusedGraphExecuteInfo_NodeType::BORDER_INPUT),
            5 => ::std::option::Option::Some(RemoteFusedGraphExecuteInfo_NodeType::BORDER_OUTPUT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RemoteFusedGraphExecuteInfo_NodeType] = &[
            RemoteFusedGraphExecuteInfo_NodeType::UNUSED,
            RemoteFusedGraphExecuteInfo_NodeType::GRAPH_INPUT,
            RemoteFusedGraphExecuteInfo_NodeType::GRAPH_OUTPUT,
            RemoteFusedGraphExecuteInfo_NodeType::FUSED_NODE,
            RemoteFusedGraphExecuteInfo_NodeType::BORDER_INPUT,
            RemoteFusedGraphExecuteInfo_NodeType::BORDER_OUTPUT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RemoteFusedGraphExecuteInfo_NodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RemoteFusedGraphExecuteInfo_NodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RemoteFusedGraphExecuteInfo_NodeType {
}

impl ::std::default::Default for RemoteFusedGraphExecuteInfo_NodeType {
    fn default() -> Self {
        RemoteFusedGraphExecuteInfo_NodeType::UNUSED
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoteFusedGraphExecuteInfo_NodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n?tensorflow/core/framework/remote_fused_graph_execute_info.proto\x12\n\
    tensorflow\x1a%tensorflow/core/framework/graph.proto\x1a,tensorflow/core\
    /framework/tensor_shape.proto\x1a%tensorflow/core/framework/types.proto\
    \"\xa1\x06\n\x1bRemoteFusedGraphExecuteInfo\x127\n\x0cremote_graph\x18\
    \x01\x20\x01(\x0b2\x14.tensorflow.GraphDefR\x0bremoteGraph\x121\n\x15gra\
    ph_input_node_name\x18\x02\x20\x03(\tR\x12graphInputNodeName\x123\n\x16g\
    raph_output_node_name\x18\x03\x20\x03(\tR\x13graphOutputNodeName\x12#\n\
    \rexecutor_name\x18\x04\x20\x01(\tR\x0cexecutorName\x12D\n\x1eserialized\
    _executor_parameters\x18\x05\x20\x01(\x0cR\x1cserializedExecutorParamete\
    rs\x12\x84\x01\n\x20default_graph_input_tensor_shape\x18\x06\x20\x03(\
    \x0b2<.tensorflow.RemoteFusedGraphExecuteInfo.TensorShapeTypeProtoR\x1cd\
    efaultGraphInputTensorShape\x12\x86\x01\n!default_graph_output_tensor_sh\
    ape\x18\x07\x20\x03(\x0b2<.tensorflow.RemoteFusedGraphExecuteInfo.Tensor\
    ShapeTypeProtoR\x1ddefaultGraphOutputTensorShape\x1av\n\x14TensorShapeTy\
    peProto\x12*\n\x05dtype\x18\x01\x20\x01(\x0e2\x14.tensorflow.DataTypeR\
    \x05dtype\x122\n\x05shape\x18\x02\x20\x01(\x0b2\x1c.tensorflow.TensorSha\
    peProtoR\x05shape\"n\n\x08NodeType\x12\n\n\x06UNUSED\x10\0\x12\x0f\n\x0b\
    GRAPH_INPUT\x10\x01\x12\x10\n\x0cGRAPH_OUTPUT\x10\x02\x12\x0e\n\nFUSED_N\
    ODE\x10\x03\x12\x10\n\x0cBORDER_INPUT\x10\x04\x12\x11\n\rBORDER_OUTPUT\
    \x10\x05BA\n\x18org.tensorflow.frameworkB\x20RemoteFusedGraphExecuteInfo\
    ProtoP\x01\xf8\x01\x01J\xde\x0f\n\x06\x12\x04\0\06\x02\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\
    \x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\
    \x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\
    \x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\
    \n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\
    \x03\x04\0A\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0A\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\
    \x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e@\n\x08\n\x01\x08\
    \x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\
    \x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\
    \x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\
    \x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\
    \x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\
    \x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\
    \x03\0\x12\x03\x08\x07.\n\t\n\x02\x03\x01\x12\x03\t\x075\n\t\n\x02\x03\
    \x02\x12\x03\n\x07.\n\xb8\x01\n\x02\x04\0\x12\x04\x0f\06\x01\x1a\xab\x01\
    \x20Protocol\x20buffer\x20representing\x20a\x20handle\x20to\x20a\x20tens\
    orflow\x20resource.\x20Handles\x20are\n\x20not\x20valid\x20across\x20exe\
    cutions,\x20but\x20can\x20be\x20serialized\x20back\x20and\x20forth\x20fr\
    om\x20within\n\x20a\x20single\x20run.\n\n\n\n\x03\x04\0\x01\x12\x03\x0f\
    \x08#\n\x0c\n\x04\x04\0\x04\0\x12\x04\x10\x02\x17\x03\n\x0c\n\x05\x04\0\
    \x04\0\x01\x12\x03\x10\x07\x0f\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03\x11\
    \x04\x0f\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\x11\x04\n\n\x0e\n\x07\
    \x04\0\x04\0\x02\0\x02\x12\x03\x11\r\x0e\n\r\n\x06\x04\0\x04\0\x02\x01\
    \x12\x03\x12\x04\x14\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\x12\x04\
    \x0f\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03\x12\x12\x13\n\r\n\x06\
    \x04\0\x04\0\x02\x02\x12\x03\x13\x04\x15\n\x0e\n\x07\x04\0\x04\0\x02\x02\
    \x01\x12\x03\x13\x04\x10\n\x0e\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03\x13\
    \x13\x14\n\r\n\x06\x04\0\x04\0\x02\x03\x12\x03\x14\x04\x13\n\x0e\n\x07\
    \x04\0\x04\0\x02\x03\x01\x12\x03\x14\x04\x0e\n\x0e\n\x07\x04\0\x04\0\x02\
    \x03\x02\x12\x03\x14\x11\x12\n\r\n\x06\x04\0\x04\0\x02\x04\x12\x03\x15\
    \x04\x15\n\x0e\n\x07\x04\0\x04\0\x02\x04\x01\x12\x03\x15\x04\x10\n\x0e\n\
    \x07\x04\0\x04\0\x02\x04\x02\x12\x03\x15\x13\x14\n\r\n\x06\x04\0\x04\0\
    \x02\x05\x12\x03\x16\x04\x16\n\x0e\n\x07\x04\0\x04\0\x02\x05\x01\x12\x03\
    \x16\x04\x11\n\x0e\n\x07\x04\0\x04\0\x02\x05\x02\x12\x03\x16\x14\x15\n\
    \x0c\n\x04\x04\0\x03\0\x12\x04\x19\x02\x1c\x03\n\x0c\n\x05\x04\0\x03\0\
    \x01\x12\x03\x19\n\x1e\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03\x1a\x04\x17\n\
    \x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04\x1a\x04\x19\x20\n\x0e\n\x07\x04\
    \0\x03\0\x02\0\x06\x12\x03\x1a\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\
    \x12\x03\x1a\r\x12\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\x1a\x15\x16\
    \n\r\n\x06\x04\0\x03\0\x02\x01\x12\x03\x1b\x04\x1f\n\x0f\n\x07\x04\0\x03\
    \0\x02\x01\x04\x12\x04\x1b\x04\x1a\x17\n\x0e\n\x07\x04\0\x03\0\x02\x01\
    \x06\x12\x03\x1b\x04\x14\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03\x1b\
    \x15\x1a\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03\x1b\x1d\x1e\n)\n\
    \x04\x04\0\x02\0\x12\x03\x1f\x02\x1c\x1a\x1c\x20Definition\x20of\x20remo\
    te\x20graph\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x1f\x02\x1c\x03\n\x0c\n\
    \x05\x04\0\x02\0\x06\x12\x03\x1f\x02\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x1f\x0b\x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1f\x1a\x1b\n1\n\
    \x04\x04\0\x02\x01\x12\x03\"\x02,\x1a$\x20Remote\x20fused\x20graph\x20in\
    put\x20node\x20name\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\"\x02\n\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\"\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\"\x12'\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\"*+\n2\n\x04\
    \x04\0\x02\x02\x12\x03%\x02-\x1a%\x20Remote\x20fused\x20graph\x20output\
    \x20node\x20name\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03%\x02\n\n\x0c\n\
    \x05\x04\0\x02\x02\x05\x12\x03%\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03%\x12(\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03%+,\n\x1e\n\x04\x04\
    \0\x02\x03\x12\x03(\x02\x1b\x1a\x11\x20Executor's\x20name\n\n\r\n\x05\
    \x04\0\x02\x03\x04\x12\x04(\x02%-\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03(\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03(\t\x16\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03(\x19\x1a\n9\n\x04\x04\0\x02\x04\x12\x03+\x02+\x1a,\
    \x20Optional:\x20Parameters\x20given\x20to\x20the\x20executor\n\n\r\n\
    \x05\x04\0\x02\x04\x04\x12\x04+\x02(\x1b\n\x0c\n\x05\x04\0\x02\x04\x05\
    \x12\x03+\x02\x07\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03+\x08&\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03+)*\nf\n\x04\x04\0\x02\x05\x12\x03/\x02E\
    \x1aY\x20Optional:\x20Default\x20graph\x20input\x20tensor\x20shape\x20us\
    ed\x20to\x20allocate\x20memory\n\x20before\x20executing\x20op\n\n\x0c\n\
    \x05\x04\0\x02\x05\x04\x12\x03/\x02\n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\
    \x03/\x0b\x1f\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03/\x20@\n\x0c\n\x05\
    \x04\0\x02\x05\x03\x12\x03/CD\n\xbd\x01\n\x04\x04\0\x02\x06\x12\x035\x02\
    F\x1a\xaf\x01\x20Optional:\x20Default\x20graph\x20input\x20tensor\x20sha\
    pe\x20used\x20to\x20allocate\x20memory\n\x20before\x20executing\x20op\n\
    \x20TODO(satok):\x20Remote\x20output\x20tensor\x20shape\x20once\x20shape\
    \x20information\x20is\x20stored\n\x20in\x20NodeDef\n\n\x0c\n\x05\x04\0\
    \x02\x06\x04\x12\x035\x02\n\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x035\x0b\
    \x1f\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x035\x20A\n\x0c\n\x05\x04\0\x02\
    \x06\x03\x12\x035DEb\x06proto3\
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
