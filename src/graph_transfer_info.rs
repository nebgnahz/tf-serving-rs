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
pub struct GraphTransferInfo {
    // message fields
    pub node_info: ::protobuf::RepeatedField<GraphTransferInfo_NodeInfo>,
    pub const_node_info: ::protobuf::RepeatedField<GraphTransferInfo_ConstNodeInfo>,
    pub node_input_info: ::protobuf::RepeatedField<GraphTransferInfo_NodeInputInfo>,
    pub node_output_info: ::protobuf::RepeatedField<GraphTransferInfo_NodeOutputInfo>,
    pub graph_input_node_info: ::protobuf::RepeatedField<GraphTransferInfo_GraphInputNodeInfo>,
    pub graph_output_node_info: ::protobuf::RepeatedField<GraphTransferInfo_GraphOutputNodeInfo>,
    pub destination: GraphTransferInfo_Destination,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo {}

impl GraphTransferInfo {
    pub fn new() -> GraphTransferInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo,
        };
        unsafe {
            instance.get(GraphTransferInfo::new)
        }
    }

    // repeated .tensorflow.GraphTransferInfo.NodeInfo node_info = 1;

    pub fn clear_node_info(&mut self) {
        self.node_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_info(&mut self, v: ::protobuf::RepeatedField<GraphTransferInfo_NodeInfo>) {
        self.node_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_info(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeInfo> {
        &mut self.node_info
    }

    // Take field
    pub fn take_node_info(&mut self) -> ::protobuf::RepeatedField<GraphTransferInfo_NodeInfo> {
        ::std::mem::replace(&mut self.node_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_info(&self) -> &[GraphTransferInfo_NodeInfo] {
        &self.node_info
    }

    fn get_node_info_for_reflect(&self) -> &::protobuf::RepeatedField<GraphTransferInfo_NodeInfo> {
        &self.node_info
    }

    fn mut_node_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeInfo> {
        &mut self.node_info
    }

    // repeated .tensorflow.GraphTransferInfo.ConstNodeInfo const_node_info = 2;

    pub fn clear_const_node_info(&mut self) {
        self.const_node_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_const_node_info(&mut self, v: ::protobuf::RepeatedField<GraphTransferInfo_ConstNodeInfo>) {
        self.const_node_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_const_node_info(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_ConstNodeInfo> {
        &mut self.const_node_info
    }

    // Take field
    pub fn take_const_node_info(&mut self) -> ::protobuf::RepeatedField<GraphTransferInfo_ConstNodeInfo> {
        ::std::mem::replace(&mut self.const_node_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_const_node_info(&self) -> &[GraphTransferInfo_ConstNodeInfo] {
        &self.const_node_info
    }

    fn get_const_node_info_for_reflect(&self) -> &::protobuf::RepeatedField<GraphTransferInfo_ConstNodeInfo> {
        &self.const_node_info
    }

    fn mut_const_node_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_ConstNodeInfo> {
        &mut self.const_node_info
    }

    // repeated .tensorflow.GraphTransferInfo.NodeInputInfo node_input_info = 3;

    pub fn clear_node_input_info(&mut self) {
        self.node_input_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_input_info(&mut self, v: ::protobuf::RepeatedField<GraphTransferInfo_NodeInputInfo>) {
        self.node_input_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_input_info(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeInputInfo> {
        &mut self.node_input_info
    }

    // Take field
    pub fn take_node_input_info(&mut self) -> ::protobuf::RepeatedField<GraphTransferInfo_NodeInputInfo> {
        ::std::mem::replace(&mut self.node_input_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_input_info(&self) -> &[GraphTransferInfo_NodeInputInfo] {
        &self.node_input_info
    }

    fn get_node_input_info_for_reflect(&self) -> &::protobuf::RepeatedField<GraphTransferInfo_NodeInputInfo> {
        &self.node_input_info
    }

    fn mut_node_input_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeInputInfo> {
        &mut self.node_input_info
    }

    // repeated .tensorflow.GraphTransferInfo.NodeOutputInfo node_output_info = 4;

    pub fn clear_node_output_info(&mut self) {
        self.node_output_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_output_info(&mut self, v: ::protobuf::RepeatedField<GraphTransferInfo_NodeOutputInfo>) {
        self.node_output_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_output_info(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeOutputInfo> {
        &mut self.node_output_info
    }

    // Take field
    pub fn take_node_output_info(&mut self) -> ::protobuf::RepeatedField<GraphTransferInfo_NodeOutputInfo> {
        ::std::mem::replace(&mut self.node_output_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_output_info(&self) -> &[GraphTransferInfo_NodeOutputInfo] {
        &self.node_output_info
    }

    fn get_node_output_info_for_reflect(&self) -> &::protobuf::RepeatedField<GraphTransferInfo_NodeOutputInfo> {
        &self.node_output_info
    }

    fn mut_node_output_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeOutputInfo> {
        &mut self.node_output_info
    }

    // repeated .tensorflow.GraphTransferInfo.GraphInputNodeInfo graph_input_node_info = 5;

    pub fn clear_graph_input_node_info(&mut self) {
        self.graph_input_node_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_input_node_info(&mut self, v: ::protobuf::RepeatedField<GraphTransferInfo_GraphInputNodeInfo>) {
        self.graph_input_node_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_input_node_info(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_GraphInputNodeInfo> {
        &mut self.graph_input_node_info
    }

    // Take field
    pub fn take_graph_input_node_info(&mut self) -> ::protobuf::RepeatedField<GraphTransferInfo_GraphInputNodeInfo> {
        ::std::mem::replace(&mut self.graph_input_node_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_graph_input_node_info(&self) -> &[GraphTransferInfo_GraphInputNodeInfo] {
        &self.graph_input_node_info
    }

    fn get_graph_input_node_info_for_reflect(&self) -> &::protobuf::RepeatedField<GraphTransferInfo_GraphInputNodeInfo> {
        &self.graph_input_node_info
    }

    fn mut_graph_input_node_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_GraphInputNodeInfo> {
        &mut self.graph_input_node_info
    }

    // repeated .tensorflow.GraphTransferInfo.GraphOutputNodeInfo graph_output_node_info = 6;

    pub fn clear_graph_output_node_info(&mut self) {
        self.graph_output_node_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_output_node_info(&mut self, v: ::protobuf::RepeatedField<GraphTransferInfo_GraphOutputNodeInfo>) {
        self.graph_output_node_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_output_node_info(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_GraphOutputNodeInfo> {
        &mut self.graph_output_node_info
    }

    // Take field
    pub fn take_graph_output_node_info(&mut self) -> ::protobuf::RepeatedField<GraphTransferInfo_GraphOutputNodeInfo> {
        ::std::mem::replace(&mut self.graph_output_node_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_graph_output_node_info(&self) -> &[GraphTransferInfo_GraphOutputNodeInfo] {
        &self.graph_output_node_info
    }

    fn get_graph_output_node_info_for_reflect(&self) -> &::protobuf::RepeatedField<GraphTransferInfo_GraphOutputNodeInfo> {
        &self.graph_output_node_info
    }

    fn mut_graph_output_node_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_GraphOutputNodeInfo> {
        &mut self.graph_output_node_info
    }

    // .tensorflow.GraphTransferInfo.Destination destination = 7;

    pub fn clear_destination(&mut self) {
        self.destination = GraphTransferInfo_Destination::NOP;
    }

    // Param is passed by value, moved
    pub fn set_destination(&mut self, v: GraphTransferInfo_Destination) {
        self.destination = v;
    }

    pub fn get_destination(&self) -> GraphTransferInfo_Destination {
        self.destination
    }

    fn get_destination_for_reflect(&self) -> &GraphTransferInfo_Destination {
        &self.destination
    }

    fn mut_destination_for_reflect(&mut self) -> &mut GraphTransferInfo_Destination {
        &mut self.destination
    }
}

impl ::protobuf::Message for GraphTransferInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.node_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.const_node_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.node_input_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.node_output_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.graph_input_node_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.graph_output_node_info {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_info)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.const_node_info)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_input_info)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_output_info)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.graph_input_node_info)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.graph_output_node_info)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.destination = tmp;
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
        for value in &self.node_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.const_node_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.node_input_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.node_output_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.graph_input_node_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.graph_output_node_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.destination != GraphTransferInfo_Destination::NOP {
            my_size += ::protobuf::rt::enum_size(7, self.destination);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.node_info {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.const_node_info {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.node_input_info {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.node_output_info {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.graph_input_node_info {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.graph_output_node_info {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.destination != GraphTransferInfo_Destination::NOP {
            os.write_enum(7, self.destination.value())?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo {
    fn new() -> GraphTransferInfo {
        GraphTransferInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphTransferInfo_NodeInfo>>(
                    "node_info",
                    GraphTransferInfo::get_node_info_for_reflect,
                    GraphTransferInfo::mut_node_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphTransferInfo_ConstNodeInfo>>(
                    "const_node_info",
                    GraphTransferInfo::get_const_node_info_for_reflect,
                    GraphTransferInfo::mut_const_node_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphTransferInfo_NodeInputInfo>>(
                    "node_input_info",
                    GraphTransferInfo::get_node_input_info_for_reflect,
                    GraphTransferInfo::mut_node_input_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphTransferInfo_NodeOutputInfo>>(
                    "node_output_info",
                    GraphTransferInfo::get_node_output_info_for_reflect,
                    GraphTransferInfo::mut_node_output_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphTransferInfo_GraphInputNodeInfo>>(
                    "graph_input_node_info",
                    GraphTransferInfo::get_graph_input_node_info_for_reflect,
                    GraphTransferInfo::mut_graph_input_node_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphTransferInfo_GraphOutputNodeInfo>>(
                    "graph_output_node_info",
                    GraphTransferInfo::get_graph_output_node_info_for_reflect,
                    GraphTransferInfo::mut_graph_output_node_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<GraphTransferInfo_Destination>>(
                    "destination",
                    GraphTransferInfo::get_destination_for_reflect,
                    GraphTransferInfo::mut_destination_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo>(
                    "GraphTransferInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo {
    fn clear(&mut self) {
        self.clear_node_info();
        self.clear_const_node_info();
        self.clear_node_input_info();
        self.clear_node_output_info();
        self.clear_graph_input_node_info();
        self.clear_graph_output_node_info();
        self.clear_destination();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTransferInfo_NodeInput {
    // message fields
    pub node_id: i32,
    pub output_port: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo_NodeInput {}

impl GraphTransferInfo_NodeInput {
    pub fn new() -> GraphTransferInfo_NodeInput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo_NodeInput {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo_NodeInput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo_NodeInput,
        };
        unsafe {
            instance.get(GraphTransferInfo_NodeInput::new)
        }
    }

    // int32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: i32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> i32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &i32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.node_id
    }

    // int32 output_port = 2;

    pub fn clear_output_port(&mut self) {
        self.output_port = 0;
    }

    // Param is passed by value, moved
    pub fn set_output_port(&mut self, v: i32) {
        self.output_port = v;
    }

    pub fn get_output_port(&self) -> i32 {
        self.output_port
    }

    fn get_output_port_for_reflect(&self) -> &i32 {
        &self.output_port
    }

    fn mut_output_port_for_reflect(&mut self) -> &mut i32 {
        &mut self.output_port
    }
}

impl ::protobuf::Message for GraphTransferInfo_NodeInput {
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
                    let tmp = is.read_int32()?;
                    self.node_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.output_port = tmp;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.output_port != 0 {
            my_size += ::protobuf::rt::value_size(2, self.output_port, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_int32(1, self.node_id)?;
        }
        if self.output_port != 0 {
            os.write_int32(2, self.output_port)?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo_NodeInput {
    fn new() -> GraphTransferInfo_NodeInput {
        GraphTransferInfo_NodeInput::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo_NodeInput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "node_id",
                    GraphTransferInfo_NodeInput::get_node_id_for_reflect,
                    GraphTransferInfo_NodeInput::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "output_port",
                    GraphTransferInfo_NodeInput::get_output_port_for_reflect,
                    GraphTransferInfo_NodeInput::mut_output_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo_NodeInput>(
                    "GraphTransferInfo_NodeInput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo_NodeInput {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_output_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo_NodeInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_NodeInput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTransferInfo_NodeInfo {
    // message fields
    pub name: ::std::string::String,
    pub node_id: i32,
    pub type_name: ::std::string::String,
    pub soc_op_id: i32,
    pub padding_id: i32,
    pub input_count: i32,
    pub output_count: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo_NodeInfo {}

impl GraphTransferInfo_NodeInfo {
    pub fn new() -> GraphTransferInfo_NodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo_NodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo_NodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo_NodeInfo,
        };
        unsafe {
            instance.get(GraphTransferInfo_NodeInfo::new)
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

    // int32 node_id = 2;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: i32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> i32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &i32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.node_id
    }

    // string type_name = 3;

    pub fn clear_type_name(&mut self) {
        self.type_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_name(&mut self, v: ::std::string::String) {
        self.type_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_name(&mut self) -> &mut ::std::string::String {
        &mut self.type_name
    }

    // Take field
    pub fn take_type_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_name, ::std::string::String::new())
    }

    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }

    fn get_type_name_for_reflect(&self) -> &::std::string::String {
        &self.type_name
    }

    fn mut_type_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_name
    }

    // int32 soc_op_id = 4;

    pub fn clear_soc_op_id(&mut self) {
        self.soc_op_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_soc_op_id(&mut self, v: i32) {
        self.soc_op_id = v;
    }

    pub fn get_soc_op_id(&self) -> i32 {
        self.soc_op_id
    }

    fn get_soc_op_id_for_reflect(&self) -> &i32 {
        &self.soc_op_id
    }

    fn mut_soc_op_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.soc_op_id
    }

    // int32 padding_id = 5;

    pub fn clear_padding_id(&mut self) {
        self.padding_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_padding_id(&mut self, v: i32) {
        self.padding_id = v;
    }

    pub fn get_padding_id(&self) -> i32 {
        self.padding_id
    }

    fn get_padding_id_for_reflect(&self) -> &i32 {
        &self.padding_id
    }

    fn mut_padding_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.padding_id
    }

    // int32 input_count = 6;

    pub fn clear_input_count(&mut self) {
        self.input_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_input_count(&mut self, v: i32) {
        self.input_count = v;
    }

    pub fn get_input_count(&self) -> i32 {
        self.input_count
    }

    fn get_input_count_for_reflect(&self) -> &i32 {
        &self.input_count
    }

    fn mut_input_count_for_reflect(&mut self) -> &mut i32 {
        &mut self.input_count
    }

    // int32 output_count = 7;

    pub fn clear_output_count(&mut self) {
        self.output_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_output_count(&mut self, v: i32) {
        self.output_count = v;
    }

    pub fn get_output_count(&self) -> i32 {
        self.output_count
    }

    fn get_output_count_for_reflect(&self) -> &i32 {
        &self.output_count
    }

    fn mut_output_count_for_reflect(&mut self) -> &mut i32 {
        &mut self.output_count
    }
}

impl ::protobuf::Message for GraphTransferInfo_NodeInfo {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int32()?;
                    self.node_id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.soc_op_id = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.padding_id = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.input_count = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.output_count = tmp;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.type_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.type_name);
        }
        if self.soc_op_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.soc_op_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.padding_id != 0 {
            my_size += ::protobuf::rt::value_size(5, self.padding_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.input_count != 0 {
            my_size += ::protobuf::rt::value_size(6, self.input_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.output_count != 0 {
            my_size += ::protobuf::rt::value_size(7, self.output_count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.node_id != 0 {
            os.write_int32(2, self.node_id)?;
        }
        if !self.type_name.is_empty() {
            os.write_string(3, &self.type_name)?;
        }
        if self.soc_op_id != 0 {
            os.write_int32(4, self.soc_op_id)?;
        }
        if self.padding_id != 0 {
            os.write_int32(5, self.padding_id)?;
        }
        if self.input_count != 0 {
            os.write_int32(6, self.input_count)?;
        }
        if self.output_count != 0 {
            os.write_int32(7, self.output_count)?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo_NodeInfo {
    fn new() -> GraphTransferInfo_NodeInfo {
        GraphTransferInfo_NodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo_NodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GraphTransferInfo_NodeInfo::get_name_for_reflect,
                    GraphTransferInfo_NodeInfo::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "node_id",
                    GraphTransferInfo_NodeInfo::get_node_id_for_reflect,
                    GraphTransferInfo_NodeInfo::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_name",
                    GraphTransferInfo_NodeInfo::get_type_name_for_reflect,
                    GraphTransferInfo_NodeInfo::mut_type_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "soc_op_id",
                    GraphTransferInfo_NodeInfo::get_soc_op_id_for_reflect,
                    GraphTransferInfo_NodeInfo::mut_soc_op_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "padding_id",
                    GraphTransferInfo_NodeInfo::get_padding_id_for_reflect,
                    GraphTransferInfo_NodeInfo::mut_padding_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "input_count",
                    GraphTransferInfo_NodeInfo::get_input_count_for_reflect,
                    GraphTransferInfo_NodeInfo::mut_input_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "output_count",
                    GraphTransferInfo_NodeInfo::get_output_count_for_reflect,
                    GraphTransferInfo_NodeInfo::mut_output_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo_NodeInfo>(
                    "GraphTransferInfo_NodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo_NodeInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_node_id();
        self.clear_type_name();
        self.clear_soc_op_id();
        self.clear_padding_id();
        self.clear_input_count();
        self.clear_output_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo_NodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_NodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTransferInfo_ConstNodeInfo {
    // message fields
    pub name: ::std::string::String,
    pub node_id: i32,
    pub shape: ::std::vec::Vec<i64>,
    pub data: ::std::vec::Vec<u8>,
    pub dtype: super::types::DataType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo_ConstNodeInfo {}

impl GraphTransferInfo_ConstNodeInfo {
    pub fn new() -> GraphTransferInfo_ConstNodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo_ConstNodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo_ConstNodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo_ConstNodeInfo,
        };
        unsafe {
            instance.get(GraphTransferInfo_ConstNodeInfo::new)
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

    // int32 node_id = 2;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: i32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> i32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &i32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.node_id
    }

    // repeated int64 shape = 3;

    pub fn clear_shape(&mut self) {
        self.shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: ::std::vec::Vec<i64>) {
        self.shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_shape(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.shape
    }

    // Take field
    pub fn take_shape(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.shape, ::std::vec::Vec::new())
    }

    pub fn get_shape(&self) -> &[i64] {
        &self.shape
    }

    fn get_shape_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.shape
    }

    fn mut_shape_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.shape
    }

    // bytes data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // .tensorflow.DataType dtype = 5;

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
}

impl ::protobuf::Message for GraphTransferInfo_ConstNodeInfo {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int32()?;
                    self.node_id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.shape)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dtype = tmp;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.shape {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.data);
        }
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(5, self.dtype);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.node_id != 0 {
            os.write_int32(2, self.node_id)?;
        }
        for v in &self.shape {
            os.write_int64(3, *v)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(4, &self.data)?;
        }
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(5, self.dtype.value())?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo_ConstNodeInfo {
    fn new() -> GraphTransferInfo_ConstNodeInfo {
        GraphTransferInfo_ConstNodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo_ConstNodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GraphTransferInfo_ConstNodeInfo::get_name_for_reflect,
                    GraphTransferInfo_ConstNodeInfo::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "node_id",
                    GraphTransferInfo_ConstNodeInfo::get_node_id_for_reflect,
                    GraphTransferInfo_ConstNodeInfo::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "shape",
                    GraphTransferInfo_ConstNodeInfo::get_shape_for_reflect,
                    GraphTransferInfo_ConstNodeInfo::mut_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    GraphTransferInfo_ConstNodeInfo::get_data_for_reflect,
                    GraphTransferInfo_ConstNodeInfo::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    GraphTransferInfo_ConstNodeInfo::get_dtype_for_reflect,
                    GraphTransferInfo_ConstNodeInfo::mut_dtype_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo_ConstNodeInfo>(
                    "GraphTransferInfo_ConstNodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo_ConstNodeInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_node_id();
        self.clear_shape();
        self.clear_data();
        self.clear_dtype();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo_ConstNodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_ConstNodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTransferInfo_NodeInputInfo {
    // message fields
    pub node_id: i32,
    pub node_input: ::protobuf::RepeatedField<GraphTransferInfo_NodeInput>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo_NodeInputInfo {}

impl GraphTransferInfo_NodeInputInfo {
    pub fn new() -> GraphTransferInfo_NodeInputInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo_NodeInputInfo {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo_NodeInputInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo_NodeInputInfo,
        };
        unsafe {
            instance.get(GraphTransferInfo_NodeInputInfo::new)
        }
    }

    // int32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: i32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> i32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &i32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.node_id
    }

    // repeated .tensorflow.GraphTransferInfo.NodeInput node_input = 2;

    pub fn clear_node_input(&mut self) {
        self.node_input.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_input(&mut self, v: ::protobuf::RepeatedField<GraphTransferInfo_NodeInput>) {
        self.node_input = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_input(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeInput> {
        &mut self.node_input
    }

    // Take field
    pub fn take_node_input(&mut self) -> ::protobuf::RepeatedField<GraphTransferInfo_NodeInput> {
        ::std::mem::replace(&mut self.node_input, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_input(&self) -> &[GraphTransferInfo_NodeInput] {
        &self.node_input
    }

    fn get_node_input_for_reflect(&self) -> &::protobuf::RepeatedField<GraphTransferInfo_NodeInput> {
        &self.node_input
    }

    fn mut_node_input_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GraphTransferInfo_NodeInput> {
        &mut self.node_input
    }
}

impl ::protobuf::Message for GraphTransferInfo_NodeInputInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.node_input {
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
                    let tmp = is.read_int32()?;
                    self.node_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_input)?;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.node_input {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_int32(1, self.node_id)?;
        }
        for v in &self.node_input {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo_NodeInputInfo {
    fn new() -> GraphTransferInfo_NodeInputInfo {
        GraphTransferInfo_NodeInputInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo_NodeInputInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "node_id",
                    GraphTransferInfo_NodeInputInfo::get_node_id_for_reflect,
                    GraphTransferInfo_NodeInputInfo::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphTransferInfo_NodeInput>>(
                    "node_input",
                    GraphTransferInfo_NodeInputInfo::get_node_input_for_reflect,
                    GraphTransferInfo_NodeInputInfo::mut_node_input_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo_NodeInputInfo>(
                    "GraphTransferInfo_NodeInputInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo_NodeInputInfo {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_node_input();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo_NodeInputInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_NodeInputInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTransferInfo_NodeOutputInfo {
    // message fields
    pub node_id: i32,
    pub max_byte_size: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo_NodeOutputInfo {}

impl GraphTransferInfo_NodeOutputInfo {
    pub fn new() -> GraphTransferInfo_NodeOutputInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo_NodeOutputInfo {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo_NodeOutputInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo_NodeOutputInfo,
        };
        unsafe {
            instance.get(GraphTransferInfo_NodeOutputInfo::new)
        }
    }

    // int32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: i32) {
        self.node_id = v;
    }

    pub fn get_node_id(&self) -> i32 {
        self.node_id
    }

    fn get_node_id_for_reflect(&self) -> &i32 {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.node_id
    }

    // repeated int32 max_byte_size = 2;

    pub fn clear_max_byte_size(&mut self) {
        self.max_byte_size.clear();
    }

    // Param is passed by value, moved
    pub fn set_max_byte_size(&mut self, v: ::std::vec::Vec<i32>) {
        self.max_byte_size = v;
    }

    // Mutable pointer to the field.
    pub fn mut_max_byte_size(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.max_byte_size
    }

    // Take field
    pub fn take_max_byte_size(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.max_byte_size, ::std::vec::Vec::new())
    }

    pub fn get_max_byte_size(&self) -> &[i32] {
        &self.max_byte_size
    }

    fn get_max_byte_size_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.max_byte_size
    }

    fn mut_max_byte_size_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.max_byte_size
    }
}

impl ::protobuf::Message for GraphTransferInfo_NodeOutputInfo {
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
                    let tmp = is.read_int32()?;
                    self.node_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.max_byte_size)?;
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
        if self.node_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.node_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.max_byte_size {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.node_id != 0 {
            os.write_int32(1, self.node_id)?;
        }
        for v in &self.max_byte_size {
            os.write_int32(2, *v)?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo_NodeOutputInfo {
    fn new() -> GraphTransferInfo_NodeOutputInfo {
        GraphTransferInfo_NodeOutputInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo_NodeOutputInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "node_id",
                    GraphTransferInfo_NodeOutputInfo::get_node_id_for_reflect,
                    GraphTransferInfo_NodeOutputInfo::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_byte_size",
                    GraphTransferInfo_NodeOutputInfo::get_max_byte_size_for_reflect,
                    GraphTransferInfo_NodeOutputInfo::mut_max_byte_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo_NodeOutputInfo>(
                    "GraphTransferInfo_NodeOutputInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo_NodeOutputInfo {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_max_byte_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo_NodeOutputInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_NodeOutputInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTransferInfo_GraphInputNodeInfo {
    // message fields
    pub name: ::std::string::String,
    pub shape: ::std::vec::Vec<i64>,
    pub dtype: super::types::DataType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo_GraphInputNodeInfo {}

impl GraphTransferInfo_GraphInputNodeInfo {
    pub fn new() -> GraphTransferInfo_GraphInputNodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo_GraphInputNodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo_GraphInputNodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo_GraphInputNodeInfo,
        };
        unsafe {
            instance.get(GraphTransferInfo_GraphInputNodeInfo::new)
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

    // repeated int64 shape = 2;

    pub fn clear_shape(&mut self) {
        self.shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: ::std::vec::Vec<i64>) {
        self.shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_shape(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.shape
    }

    // Take field
    pub fn take_shape(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.shape, ::std::vec::Vec::new())
    }

    pub fn get_shape(&self) -> &[i64] {
        &self.shape
    }

    fn get_shape_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.shape
    }

    fn mut_shape_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.shape
    }

    // .tensorflow.DataType dtype = 3;

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
}

impl ::protobuf::Message for GraphTransferInfo_GraphInputNodeInfo {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.shape)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dtype = tmp;
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
        for value in &self.shape {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(3, self.dtype);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.shape {
            os.write_int64(2, *v)?;
        };
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(3, self.dtype.value())?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo_GraphInputNodeInfo {
    fn new() -> GraphTransferInfo_GraphInputNodeInfo {
        GraphTransferInfo_GraphInputNodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo_GraphInputNodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GraphTransferInfo_GraphInputNodeInfo::get_name_for_reflect,
                    GraphTransferInfo_GraphInputNodeInfo::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "shape",
                    GraphTransferInfo_GraphInputNodeInfo::get_shape_for_reflect,
                    GraphTransferInfo_GraphInputNodeInfo::mut_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    GraphTransferInfo_GraphInputNodeInfo::get_dtype_for_reflect,
                    GraphTransferInfo_GraphInputNodeInfo::mut_dtype_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo_GraphInputNodeInfo>(
                    "GraphTransferInfo_GraphInputNodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo_GraphInputNodeInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_shape();
        self.clear_dtype();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo_GraphInputNodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_GraphInputNodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphTransferInfo_GraphOutputNodeInfo {
    // message fields
    pub name: ::std::string::String,
    pub shape: ::std::vec::Vec<i64>,
    pub dtype: super::types::DataType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphTransferInfo_GraphOutputNodeInfo {}

impl GraphTransferInfo_GraphOutputNodeInfo {
    pub fn new() -> GraphTransferInfo_GraphOutputNodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphTransferInfo_GraphOutputNodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<GraphTransferInfo_GraphOutputNodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphTransferInfo_GraphOutputNodeInfo,
        };
        unsafe {
            instance.get(GraphTransferInfo_GraphOutputNodeInfo::new)
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

    // repeated int64 shape = 2;

    pub fn clear_shape(&mut self) {
        self.shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: ::std::vec::Vec<i64>) {
        self.shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_shape(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.shape
    }

    // Take field
    pub fn take_shape(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.shape, ::std::vec::Vec::new())
    }

    pub fn get_shape(&self) -> &[i64] {
        &self.shape
    }

    fn get_shape_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.shape
    }

    fn mut_shape_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.shape
    }

    // .tensorflow.DataType dtype = 3;

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
}

impl ::protobuf::Message for GraphTransferInfo_GraphOutputNodeInfo {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.shape)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dtype = tmp;
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
        for value in &self.shape {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(3, self.dtype);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.shape {
            os.write_int64(2, *v)?;
        };
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(3, self.dtype.value())?;
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

impl ::protobuf::MessageStatic for GraphTransferInfo_GraphOutputNodeInfo {
    fn new() -> GraphTransferInfo_GraphOutputNodeInfo {
        GraphTransferInfo_GraphOutputNodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphTransferInfo_GraphOutputNodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GraphTransferInfo_GraphOutputNodeInfo::get_name_for_reflect,
                    GraphTransferInfo_GraphOutputNodeInfo::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "shape",
                    GraphTransferInfo_GraphOutputNodeInfo::get_shape_for_reflect,
                    GraphTransferInfo_GraphOutputNodeInfo::mut_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    GraphTransferInfo_GraphOutputNodeInfo::get_dtype_for_reflect,
                    GraphTransferInfo_GraphOutputNodeInfo::mut_dtype_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphTransferInfo_GraphOutputNodeInfo>(
                    "GraphTransferInfo_GraphOutputNodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphTransferInfo_GraphOutputNodeInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_shape();
        self.clear_dtype();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphTransferInfo_GraphOutputNodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_GraphOutputNodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GraphTransferInfo_Destination {
    NOP = 0,
    HEXAGON = 1,
}

impl ::protobuf::ProtobufEnum for GraphTransferInfo_Destination {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GraphTransferInfo_Destination> {
        match value {
            0 => ::std::option::Option::Some(GraphTransferInfo_Destination::NOP),
            1 => ::std::option::Option::Some(GraphTransferInfo_Destination::HEXAGON),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GraphTransferInfo_Destination] = &[
            GraphTransferInfo_Destination::NOP,
            GraphTransferInfo_Destination::HEXAGON,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<GraphTransferInfo_Destination>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GraphTransferInfo_Destination", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GraphTransferInfo_Destination {
}

impl ::std::default::Default for GraphTransferInfo_Destination {
    fn default() -> Self {
        GraphTransferInfo_Destination::NOP
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphTransferInfo_Destination {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n3tensorflow/core/framework/graph_transfer_info.proto\x12\ntensorflow\
    \x1a%tensorflow/core/framework/types.proto\"\xe5\x0b\n\x11GraphTransferI\
    nfo\x12C\n\tnode_info\x18\x01\x20\x03(\x0b2&.tensorflow.GraphTransferInf\
    o.NodeInfoR\x08nodeInfo\x12S\n\x0fconst_node_info\x18\x02\x20\x03(\x0b2+\
    .tensorflow.GraphTransferInfo.ConstNodeInfoR\rconstNodeInfo\x12S\n\x0fno\
    de_input_info\x18\x03\x20\x03(\x0b2+.tensorflow.GraphTransferInfo.NodeIn\
    putInfoR\rnodeInputInfo\x12V\n\x10node_output_info\x18\x04\x20\x03(\x0b2\
    ,.tensorflow.GraphTransferInfo.NodeOutputInfoR\x0enodeOutputInfo\x12c\n\
    \x15graph_input_node_info\x18\x05\x20\x03(\x0b20.tensorflow.GraphTransfe\
    rInfo.GraphInputNodeInfoR\x12graphInputNodeInfo\x12f\n\x16graph_output_n\
    ode_info\x18\x06\x20\x03(\x0b21.tensorflow.GraphTransferInfo.GraphOutput\
    NodeInfoR\x13graphOutputNodeInfo\x12K\n\x0bdestination\x18\x07\x20\x01(\
    \x0e2).tensorflow.GraphTransferInfo.DestinationR\x0bdestination\x1aE\n\t\
    NodeInput\x12\x17\n\x07node_id\x18\x01\x20\x01(\x05R\x06nodeId\x12\x1f\n\
    \x0boutput_port\x18\x02\x20\x01(\x05R\noutputPort\x1a\xd3\x01\n\x08NodeI\
    nfo\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x17\n\x07node_id\
    \x18\x02\x20\x01(\x05R\x06nodeId\x12\x1b\n\ttype_name\x18\x03\x20\x01(\t\
    R\x08typeName\x12\x1a\n\tsoc_op_id\x18\x04\x20\x01(\x05R\x07socOpId\x12\
    \x1d\n\npadding_id\x18\x05\x20\x01(\x05R\tpaddingId\x12\x1f\n\x0binput_c\
    ount\x18\x06\x20\x01(\x05R\ninputCount\x12!\n\x0coutput_count\x18\x07\
    \x20\x01(\x05R\x0boutputCount\x1a\x92\x01\n\rConstNodeInfo\x12\x12\n\x04\
    name\x18\x01\x20\x01(\tR\x04name\x12\x17\n\x07node_id\x18\x02\x20\x01(\
    \x05R\x06nodeId\x12\x14\n\x05shape\x18\x03\x20\x03(\x03R\x05shape\x12\
    \x12\n\x04data\x18\x04\x20\x01(\x0cR\x04data\x12*\n\x05dtype\x18\x05\x20\
    \x01(\x0e2\x14.tensorflow.DataTypeR\x05dtype\x1ap\n\rNodeInputInfo\x12\
    \x17\n\x07node_id\x18\x01\x20\x01(\x05R\x06nodeId\x12F\n\nnode_input\x18\
    \x02\x20\x03(\x0b2'.tensorflow.GraphTransferInfo.NodeInputR\tnodeInput\
    \x1aM\n\x0eNodeOutputInfo\x12\x17\n\x07node_id\x18\x01\x20\x01(\x05R\x06\
    nodeId\x12\"\n\rmax_byte_size\x18\x02\x20\x03(\x05R\x0bmaxByteSize\x1aj\
    \n\x12GraphInputNodeInfo\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12\x14\n\x05shape\x18\x02\x20\x03(\x03R\x05shape\x12*\n\x05dtype\x18\
    \x03\x20\x01(\x0e2\x14.tensorflow.DataTypeR\x05dtype\x1ak\n\x13GraphOutp\
    utNodeInfo\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05sh\
    ape\x18\x02\x20\x03(\x03R\x05shape\x12*\n\x05dtype\x18\x03\x20\x01(\x0e2\
    \x14.tensorflow.DataTypeR\x05dtype\"#\n\x0bDestination\x12\x07\n\x03NOP\
    \x10\0\x12\x0b\n\x07HEXAGON\x10\x01B7\n\x18org.tensorflow.frameworkB\x16\
    GraphTransferInfoProtoP\x01\xf8\x01\x01J\xa2\x1a\n\x06\x12\x04\0\0C\x02\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\
    \x08\n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\
    \x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\
    \x07\0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\
    \n\x01\x08\x12\x03\x04\07\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\07\n\
    \x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\
    \x01\x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\
    \x03\x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e6\n\x08\
    \n\x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\
    \n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\
    \x02\x03\0\x12\x03\x08\x07.\n\xb8\x01\n\x02\x04\0\x12\x04\r\0C\x01\x1a\
    \xab\x01\x20Protocol\x20buffer\x20representing\x20a\x20handle\x20to\x20a\
    \x20tensorflow\x20resource.\x20Handles\x20are\n\x20not\x20valid\x20acros\
    s\x20executions,\x20but\x20can\x20be\x20serialized\x20back\x20and\x20for\
    th\x20from\x20within\n\x20a\x20single\x20run.\n\n\n\n\x03\x04\0\x01\x12\
    \x03\r\x08\x19\n\x0c\n\x04\x04\0\x04\0\x12\x04\x0e\x02\x11\x03\n\x0c\n\
    \x05\x04\0\x04\0\x01\x12\x03\x0e\x07\x12\n\r\n\x06\x04\0\x04\0\x02\0\x12\
    \x03\x0f\x04\x0c\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\x0f\x04\x07\n\
    \x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03\x0f\n\x0b\n\r\n\x06\x04\0\x04\0\
    \x02\x01\x12\x03\x10\x04\x10\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\
    \x10\x04\x0b\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03\x10\x0e\x0f\n\
    \x0c\n\x04\x04\0\x03\0\x12\x04\x12\x02\x15\x03\n\x0c\n\x05\x04\0\x03\0\
    \x01\x12\x03\x12\n\x13\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03\x13\x04\x16\n\
    \x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04\x13\x04\x12\x15\n\x0e\n\x07\x04\
    \0\x03\0\x02\0\x05\x12\x03\x13\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\
    \x12\x03\x13\n\x11\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\x13\x14\x15\
    \n\r\n\x06\x04\0\x03\0\x02\x01\x12\x03\x14\x04\x1a\n\x0f\n\x07\x04\0\x03\
    \0\x02\x01\x04\x12\x04\x14\x04\x13\x16\n\x0e\n\x07\x04\0\x03\0\x02\x01\
    \x05\x12\x03\x14\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03\x14\n\
    \x15\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03\x14\x18\x19\n\x0c\n\x04\
    \x04\0\x03\x01\x12\x04\x16\x02\x1e\x03\n\x0c\n\x05\x04\0\x03\x01\x01\x12\
    \x03\x16\n\x12\n\r\n\x06\x04\0\x03\x01\x02\0\x12\x03\x17\x04\x14\n\x0f\n\
    \x07\x04\0\x03\x01\x02\0\x04\x12\x04\x17\x04\x16\x14\n\x0e\n\x07\x04\0\
    \x03\x01\x02\0\x05\x12\x03\x17\x04\n\n\x0e\n\x07\x04\0\x03\x01\x02\0\x01\
    \x12\x03\x17\x0b\x0f\n\x0e\n\x07\x04\0\x03\x01\x02\0\x03\x12\x03\x17\x12\
    \x13\n\r\n\x06\x04\0\x03\x01\x02\x01\x12\x03\x18\x04\x16\n\x0f\n\x07\x04\
    \0\x03\x01\x02\x01\x04\x12\x04\x18\x04\x17\x14\n\x0e\n\x07\x04\0\x03\x01\
    \x02\x01\x05\x12\x03\x18\x04\t\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x01\x12\
    \x03\x18\n\x11\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x03\x12\x03\x18\x14\x15\
    \n\r\n\x06\x04\0\x03\x01\x02\x02\x12\x03\x19\x04\x19\n\x0f\n\x07\x04\0\
    \x03\x01\x02\x02\x04\x12\x04\x19\x04\x18\x16\n\x0e\n\x07\x04\0\x03\x01\
    \x02\x02\x05\x12\x03\x19\x04\n\n\x0e\n\x07\x04\0\x03\x01\x02\x02\x01\x12\
    \x03\x19\x0b\x14\n\x0e\n\x07\x04\0\x03\x01\x02\x02\x03\x12\x03\x19\x17\
    \x18\n\r\n\x06\x04\0\x03\x01\x02\x03\x12\x03\x1a\x04\x18\n\x0f\n\x07\x04\
    \0\x03\x01\x02\x03\x04\x12\x04\x1a\x04\x19\x19\n\x0e\n\x07\x04\0\x03\x01\
    \x02\x03\x05\x12\x03\x1a\x04\t\n\x0e\n\x07\x04\0\x03\x01\x02\x03\x01\x12\
    \x03\x1a\n\x13\n\x0e\n\x07\x04\0\x03\x01\x02\x03\x03\x12\x03\x1a\x16\x17\
    \n\r\n\x06\x04\0\x03\x01\x02\x04\x12\x03\x1b\x04\x19\n\x0f\n\x07\x04\0\
    \x03\x01\x02\x04\x04\x12\x04\x1b\x04\x1a\x18\n\x0e\n\x07\x04\0\x03\x01\
    \x02\x04\x05\x12\x03\x1b\x04\t\n\x0e\n\x07\x04\0\x03\x01\x02\x04\x01\x12\
    \x03\x1b\n\x14\n\x0e\n\x07\x04\0\x03\x01\x02\x04\x03\x12\x03\x1b\x17\x18\
    \n\r\n\x06\x04\0\x03\x01\x02\x05\x12\x03\x1c\x04\x1a\n\x0f\n\x07\x04\0\
    \x03\x01\x02\x05\x04\x12\x04\x1c\x04\x1b\x19\n\x0e\n\x07\x04\0\x03\x01\
    \x02\x05\x05\x12\x03\x1c\x04\t\n\x0e\n\x07\x04\0\x03\x01\x02\x05\x01\x12\
    \x03\x1c\n\x15\n\x0e\n\x07\x04\0\x03\x01\x02\x05\x03\x12\x03\x1c\x18\x19\
    \n\r\n\x06\x04\0\x03\x01\x02\x06\x12\x03\x1d\x04\x1b\n\x0f\n\x07\x04\0\
    \x03\x01\x02\x06\x04\x12\x04\x1d\x04\x1c\x1a\n\x0e\n\x07\x04\0\x03\x01\
    \x02\x06\x05\x12\x03\x1d\x04\t\n\x0e\n\x07\x04\0\x03\x01\x02\x06\x01\x12\
    \x03\x1d\n\x16\n\x0e\n\x07\x04\0\x03\x01\x02\x06\x03\x12\x03\x1d\x19\x1a\
    \n\x0c\n\x04\x04\0\x03\x02\x12\x04\x1f\x02%\x03\n\x0c\n\x05\x04\0\x03\
    \x02\x01\x12\x03\x1f\n\x17\n\r\n\x06\x04\0\x03\x02\x02\0\x12\x03\x20\x04\
    \x14\n\x0f\n\x07\x04\0\x03\x02\x02\0\x04\x12\x04\x20\x04\x1f\x19\n\x0e\n\
    \x07\x04\0\x03\x02\x02\0\x05\x12\x03\x20\x04\n\n\x0e\n\x07\x04\0\x03\x02\
    \x02\0\x01\x12\x03\x20\x0b\x0f\n\x0e\n\x07\x04\0\x03\x02\x02\0\x03\x12\
    \x03\x20\x12\x13\n\r\n\x06\x04\0\x03\x02\x02\x01\x12\x03!\x04\x16\n\x0f\
    \n\x07\x04\0\x03\x02\x02\x01\x04\x12\x04!\x04\x20\x14\n\x0e\n\x07\x04\0\
    \x03\x02\x02\x01\x05\x12\x03!\x04\t\n\x0e\n\x07\x04\0\x03\x02\x02\x01\
    \x01\x12\x03!\n\x11\n\x0e\n\x07\x04\0\x03\x02\x02\x01\x03\x12\x03!\x14\
    \x15\n\r\n\x06\x04\0\x03\x02\x02\x02\x12\x03\"\x04\x1d\n\x0e\n\x07\x04\0\
    \x03\x02\x02\x02\x04\x12\x03\"\x04\x0c\n\x0e\n\x07\x04\0\x03\x02\x02\x02\
    \x05\x12\x03\"\r\x12\n\x0e\n\x07\x04\0\x03\x02\x02\x02\x01\x12\x03\"\x13\
    \x18\n\x0e\n\x07\x04\0\x03\x02\x02\x02\x03\x12\x03\"\x1b\x1c\n\r\n\x06\
    \x04\0\x03\x02\x02\x03\x12\x03#\x04\x13\n\x0f\n\x07\x04\0\x03\x02\x02\
    \x03\x04\x12\x04#\x04\"\x1d\n\x0e\n\x07\x04\0\x03\x02\x02\x03\x05\x12\
    \x03#\x04\t\n\x0e\n\x07\x04\0\x03\x02\x02\x03\x01\x12\x03#\n\x0e\n\x0e\n\
    \x07\x04\0\x03\x02\x02\x03\x03\x12\x03#\x11\x12\n\r\n\x06\x04\0\x03\x02\
    \x02\x04\x12\x03$\x04\x17\n\x0f\n\x07\x04\0\x03\x02\x02\x04\x04\x12\x04$\
    \x04#\x13\n\x0e\n\x07\x04\0\x03\x02\x02\x04\x06\x12\x03$\x04\x0c\n\x0e\n\
    \x07\x04\0\x03\x02\x02\x04\x01\x12\x03$\r\x12\n\x0e\n\x07\x04\0\x03\x02\
    \x02\x04\x03\x12\x03$\x15\x16\n\x0c\n\x04\x04\0\x03\x03\x12\x04&\x02)\
    \x03\n\x0c\n\x05\x04\0\x03\x03\x01\x12\x03&\n\x17\n\r\n\x06\x04\0\x03\
    \x03\x02\0\x12\x03'\x04\x16\n\x0f\n\x07\x04\0\x03\x03\x02\0\x04\x12\x04'\
    \x04&\x19\n\x0e\n\x07\x04\0\x03\x03\x02\0\x05\x12\x03'\x04\t\n\x0e\n\x07\
    \x04\0\x03\x03\x02\0\x01\x12\x03'\n\x11\n\x0e\n\x07\x04\0\x03\x03\x02\0\
    \x03\x12\x03'\x14\x15\n\r\n\x06\x04\0\x03\x03\x02\x01\x12\x03(\x04&\n\
    \x0e\n\x07\x04\0\x03\x03\x02\x01\x04\x12\x03(\x04\x0c\n\x0e\n\x07\x04\0\
    \x03\x03\x02\x01\x06\x12\x03(\r\x16\n\x0e\n\x07\x04\0\x03\x03\x02\x01\
    \x01\x12\x03(\x17!\n\x0e\n\x07\x04\0\x03\x03\x02\x01\x03\x12\x03($%\n\
    \x0c\n\x04\x04\0\x03\x04\x12\x04*\x02-\x03\n\x0c\n\x05\x04\0\x03\x04\x01\
    \x12\x03*\n\x18\n\r\n\x06\x04\0\x03\x04\x02\0\x12\x03+\x04\x16\n\x0f\n\
    \x07\x04\0\x03\x04\x02\0\x04\x12\x04+\x04*\x1a\n\x0e\n\x07\x04\0\x03\x04\
    \x02\0\x05\x12\x03+\x04\t\n\x0e\n\x07\x04\0\x03\x04\x02\0\x01\x12\x03+\n\
    \x11\n\x0e\n\x07\x04\0\x03\x04\x02\0\x03\x12\x03+\x14\x15\n\r\n\x06\x04\
    \0\x03\x04\x02\x01\x12\x03,\x04%\n\x0e\n\x07\x04\0\x03\x04\x02\x01\x04\
    \x12\x03,\x04\x0c\n\x0e\n\x07\x04\0\x03\x04\x02\x01\x05\x12\x03,\r\x12\n\
    \x0e\n\x07\x04\0\x03\x04\x02\x01\x01\x12\x03,\x13\x20\n\x0e\n\x07\x04\0\
    \x03\x04\x02\x01\x03\x12\x03,#$\n\x0c\n\x04\x04\0\x03\x05\x12\x04.\x022\
    \x03\n\x0c\n\x05\x04\0\x03\x05\x01\x12\x03.\n\x1c\n\r\n\x06\x04\0\x03\
    \x05\x02\0\x12\x03/\x04\x14\n\x0f\n\x07\x04\0\x03\x05\x02\0\x04\x12\x04/\
    \x04.\x1e\n\x0e\n\x07\x04\0\x03\x05\x02\0\x05\x12\x03/\x04\n\n\x0e\n\x07\
    \x04\0\x03\x05\x02\0\x01\x12\x03/\x0b\x0f\n\x0e\n\x07\x04\0\x03\x05\x02\
    \0\x03\x12\x03/\x12\x13\n\r\n\x06\x04\0\x03\x05\x02\x01\x12\x030\x04\x1d\
    \n\x0e\n\x07\x04\0\x03\x05\x02\x01\x04\x12\x030\x04\x0c\n\x0e\n\x07\x04\
    \0\x03\x05\x02\x01\x05\x12\x030\r\x12\n\x0e\n\x07\x04\0\x03\x05\x02\x01\
    \x01\x12\x030\x13\x18\n\x0e\n\x07\x04\0\x03\x05\x02\x01\x03\x12\x030\x1b\
    \x1c\n\r\n\x06\x04\0\x03\x05\x02\x02\x12\x031\x04\x17\n\x0f\n\x07\x04\0\
    \x03\x05\x02\x02\x04\x12\x041\x040\x1d\n\x0e\n\x07\x04\0\x03\x05\x02\x02\
    \x06\x12\x031\x04\x0c\n\x0e\n\x07\x04\0\x03\x05\x02\x02\x01\x12\x031\r\
    \x12\n\x0e\n\x07\x04\0\x03\x05\x02\x02\x03\x12\x031\x15\x16\n\x0c\n\x04\
    \x04\0\x03\x06\x12\x044\x028\x03\n\x0c\n\x05\x04\0\x03\x06\x01\x12\x034\
    \n\x1d\n\r\n\x06\x04\0\x03\x06\x02\0\x12\x035\x04\x14\n\x0f\n\x07\x04\0\
    \x03\x06\x02\0\x04\x12\x045\x044\x1f\n\x0e\n\x07\x04\0\x03\x06\x02\0\x05\
    \x12\x035\x04\n\n\x0e\n\x07\x04\0\x03\x06\x02\0\x01\x12\x035\x0b\x0f\n\
    \x0e\n\x07\x04\0\x03\x06\x02\0\x03\x12\x035\x12\x13\n\r\n\x06\x04\0\x03\
    \x06\x02\x01\x12\x036\x04\x1d\n\x0e\n\x07\x04\0\x03\x06\x02\x01\x04\x12\
    \x036\x04\x0c\n\x0e\n\x07\x04\0\x03\x06\x02\x01\x05\x12\x036\r\x12\n\x0e\
    \n\x07\x04\0\x03\x06\x02\x01\x01\x12\x036\x13\x18\n\x0e\n\x07\x04\0\x03\
    \x06\x02\x01\x03\x12\x036\x1b\x1c\n\r\n\x06\x04\0\x03\x06\x02\x02\x12\
    \x037\x04\x17\n\x0f\n\x07\x04\0\x03\x06\x02\x02\x04\x12\x047\x046\x1d\n\
    \x0e\n\x07\x04\0\x03\x06\x02\x02\x06\x12\x037\x04\x0c\n\x0e\n\x07\x04\0\
    \x03\x06\x02\x02\x01\x12\x037\r\x12\n\x0e\n\x07\x04\0\x03\x06\x02\x02\
    \x03\x12\x037\x15\x16\n\x0b\n\x04\x04\0\x02\0\x12\x03:\x02\"\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03:\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03:\x0b\
    \x13\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03:\x14\x1d\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03:\x20!\n\x0b\n\x04\x04\0\x02\x01\x12\x03;\x02-\n\x0c\n\x05\
    \x04\0\x02\x01\x04\x12\x03;\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03;\
    \x0b\x18\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03;\x19(\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03;+,\n\x0b\n\x04\x04\0\x02\x02\x12\x03<\x02-\n\x0c\n\
    \x05\x04\0\x02\x02\x04\x12\x03<\x02\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\
    \x03<\x0b\x18\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03<\x19(\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03<+,\n\x0b\n\x04\x04\0\x02\x03\x12\x03=\x02/\n\
    \x0c\n\x05\x04\0\x02\x03\x04\x12\x03=\x02\n\n\x0c\n\x05\x04\0\x02\x03\
    \x06\x12\x03=\x0b\x19\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03=\x1a*\n\x0c\
    \n\x05\x04\0\x02\x03\x03\x12\x03=-.\n9\n\x04\x04\0\x02\x04\x12\x03?\x028\
    \x1a,\x20Input\x20Node\x20parameters\x20of\x20transferred\x20graph\n\n\
    \x0c\n\x05\x04\0\x02\x04\x04\x12\x03?\x02\n\n\x0c\n\x05\x04\0\x02\x04\
    \x06\x12\x03?\x0b\x1d\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03?\x1e3\n\x0c\
    \n\x05\x04\0\x02\x04\x03\x12\x03?67\n\x0b\n\x04\x04\0\x02\x05\x12\x03@\
    \x02:\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03@\x02\n\n\x0c\n\x05\x04\0\x02\
    \x05\x06\x12\x03@\x0b\x1e\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03@\x1f5\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x03@89\n,\n\x04\x04\0\x02\x06\x12\x03B\
    \x02\x1e\x1a\x1f\x20Destination\x20of\x20graph\x20transfer\n\n\r\n\x05\
    \x04\0\x02\x06\x04\x12\x04B\x02@:\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03B\
    \x02\r\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03B\x0e\x19\n\x0c\n\x05\x04\0\
    \x02\x06\x03\x12\x03B\x1c\x1db\x06proto3\
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
