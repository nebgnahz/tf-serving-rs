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
pub struct TensorSliceProto {
    // message fields
    pub extent: ::protobuf::RepeatedField<TensorSliceProto_Extent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorSliceProto {}

impl TensorSliceProto {
    pub fn new() -> TensorSliceProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorSliceProto {
        static mut instance: ::protobuf::lazy::Lazy<TensorSliceProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorSliceProto,
        };
        unsafe {
            instance.get(TensorSliceProto::new)
        }
    }

    // repeated .tensorflow.TensorSliceProto.Extent extent = 1;

    pub fn clear_extent(&mut self) {
        self.extent.clear();
    }

    // Param is passed by value, moved
    pub fn set_extent(&mut self, v: ::protobuf::RepeatedField<TensorSliceProto_Extent>) {
        self.extent = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extent(&mut self) -> &mut ::protobuf::RepeatedField<TensorSliceProto_Extent> {
        &mut self.extent
    }

    // Take field
    pub fn take_extent(&mut self) -> ::protobuf::RepeatedField<TensorSliceProto_Extent> {
        ::std::mem::replace(&mut self.extent, ::protobuf::RepeatedField::new())
    }

    pub fn get_extent(&self) -> &[TensorSliceProto_Extent] {
        &self.extent
    }

    fn get_extent_for_reflect(&self) -> &::protobuf::RepeatedField<TensorSliceProto_Extent> {
        &self.extent
    }

    fn mut_extent_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TensorSliceProto_Extent> {
        &mut self.extent
    }
}

impl ::protobuf::Message for TensorSliceProto {
    fn is_initialized(&self) -> bool {
        for v in &self.extent {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extent)?;
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
        for value in &self.extent {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.extent {
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

impl ::protobuf::MessageStatic for TensorSliceProto {
    fn new() -> TensorSliceProto {
        TensorSliceProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorSliceProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TensorSliceProto_Extent>>(
                    "extent",
                    TensorSliceProto::get_extent_for_reflect,
                    TensorSliceProto::mut_extent_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorSliceProto>(
                    "TensorSliceProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorSliceProto {
    fn clear(&mut self) {
        self.clear_extent();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorSliceProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorSliceProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TensorSliceProto_Extent {
    // message fields
    pub start: i64,
    // message oneof groups
    has_length: ::std::option::Option<TensorSliceProto_Extent_oneof_has_length>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorSliceProto_Extent {}

#[derive(Clone,PartialEq)]
pub enum TensorSliceProto_Extent_oneof_has_length {
    length(i64),
}

impl TensorSliceProto_Extent {
    pub fn new() -> TensorSliceProto_Extent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorSliceProto_Extent {
        static mut instance: ::protobuf::lazy::Lazy<TensorSliceProto_Extent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorSliceProto_Extent,
        };
        unsafe {
            instance.get(TensorSliceProto_Extent::new)
        }
    }

    // int64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = 0;
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: i64) {
        self.start = v;
    }

    pub fn get_start(&self) -> i64 {
        self.start
    }

    fn get_start_for_reflect(&self) -> &i64 {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut i64 {
        &mut self.start
    }

    // int64 length = 2;

    pub fn clear_length(&mut self) {
        self.has_length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        match self.has_length {
            ::std::option::Option::Some(TensorSliceProto_Extent_oneof_has_length::length(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: i64) {
        self.has_length = ::std::option::Option::Some(TensorSliceProto_Extent_oneof_has_length::length(v))
    }

    pub fn get_length(&self) -> i64 {
        match self.has_length {
            ::std::option::Option::Some(TensorSliceProto_Extent_oneof_has_length::length(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for TensorSliceProto_Extent {
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
                    let tmp = is.read_int64()?;
                    self.start = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.has_length = ::std::option::Option::Some(TensorSliceProto_Extent_oneof_has_length::length(is.read_int64()?));
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
        if self.start != 0 {
            my_size += ::protobuf::rt::value_size(1, self.start, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.has_length {
            match v {
                &TensorSliceProto_Extent_oneof_has_length::length(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.start != 0 {
            os.write_int64(1, self.start)?;
        }
        if let ::std::option::Option::Some(ref v) = self.has_length {
            match v {
                &TensorSliceProto_Extent_oneof_has_length::length(v) => {
                    os.write_int64(2, v)?;
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

impl ::protobuf::MessageStatic for TensorSliceProto_Extent {
    fn new() -> TensorSliceProto_Extent {
        TensorSliceProto_Extent::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorSliceProto_Extent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start",
                    TensorSliceProto_Extent::get_start_for_reflect,
                    TensorSliceProto_Extent::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "length",
                    TensorSliceProto_Extent::has_length,
                    TensorSliceProto_Extent::get_length,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorSliceProto_Extent>(
                    "TensorSliceProto_Extent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorSliceProto_Extent {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorSliceProto_Extent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorSliceProto_Extent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,tensorflow/core/framework/tensor_slice.proto\x12\ntensorflow\"\x97\
    \x01\n\x10TensorSliceProto\x12;\n\x06extent\x18\x01\x20\x03(\x0b2#.tenso\
    rflow.TensorSliceProto.ExtentR\x06extent\x1aF\n\x06Extent\x12\x14\n\x05s\
    tart\x18\x01\x20\x01(\x03R\x05start\x12\x18\n\x06length\x18\x02\x20\x01(\
    \x03H\0R\x06lengthB\x0c\n\nhas_lengthB2\n\x18org.tensorflow.frameworkB\
    \x11TensorSliceProtosP\x01\xf8\x01\x01J\xbe\x0b\n\x06\x12\x04\x02\0$\x02\
    \n;\n\x01\x0c\x12\x03\x02\0\x1221\x20Protocol\x20buffer\x20representing\
    \x20slices\x20of\x20a\x20tensor\n\n\x08\n\x01\x08\x12\x03\x03\0\x1f\n\
    \x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\x17\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\02\n\x0b\
    \n\x04\x08\xe7\x07\x01\x12\x03\x04\02\n\x0c\n\x05\x08\xe7\x07\x01\x02\
    \x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\x1b\
    \n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\x05\
    \x08\xe7\x07\x01\x07\x12\x03\x04\x1e1\n\x08\n\x01\x08\x12\x03\x05\0\"\n\
    \x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\x02\
    \x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\x07\
    \x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\
    \x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\01\
    \n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\x03\
    \x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\x07\
    \x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\n\
    \x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\x08\n\x01\x02\x12\x03\x08\
    \x08\x12\nP\n\x02\x04\0\x12\x04\x0b\0$\x01\x1aD\x20Can\x20only\x20be\x20\
    interpreted\x20if\x20you\x20know\x20the\x20corresponding\x20TensorShape.\
    \n\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x18\n\xa0\x01\n\x04\x04\0\x03\0\
    \x12\x04\r\x02\x1c\x03\x1a'\x20Extent\x20of\x20the\x20slice\x20in\x20one\
    \x20dimension.\n\"i\x20Either\x20both\x20or\x20no\x20attributes\x20must\
    \x20be\x20set.\x20\x20When\x20no\x20attribute\x20is\x20set\n\x20means:\
    \x20All\x20data\x20in\x20that\x20dimension.\n\n\x0c\n\x05\x04\0\x03\0\
    \x01\x12\x03\r\n\x10\n9\n\x06\x04\0\x03\0\x02\0\x12\x03\x12\x04\x14\x1a*\
    \x20Start\x20index\x20of\x20the\x20slice,\x20starting\x20at\x200.\n\n\
    \x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04\x12\x04\r\x12\n\x0e\n\x07\x04\0\
    \x03\0\x02\0\x05\x12\x03\x12\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\
    \x03\x12\n\x0f\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\x12\x12\x13\n\
    \xa7\x02\n\x06\x04\0\x03\0\x08\0\x12\x04\x19\x04\x1b\x05\x1a\x96\x02\x20\
    Length\x20of\x20the\x20slice:\x20if\x20the\x20length\x20is\x20missing\
    \x20or\x20-1\x20we\x20will\n\x20interpret\x20this\x20as\x20\"everything\
    \x20in\x20this\x20dimension\".\x20\x20We\x20use\n\x20\"oneof\"\x20to\x20\
    preserve\x20information\x20about\x20whether\x20the\x20length\x20is\n\x20\
    present\x20without\x20changing\x20the\x20serialization\x20format\x20from\
    \x20the\n\x20prior\x20proto2\x20version\x20of\x20this\x20proto.\n\n\x0e\
    \n\x07\x04\0\x03\0\x08\0\x01\x12\x03\x19\n\x14\n\r\n\x06\x04\0\x03\0\x02\
    \x01\x12\x03\x1a\x06\x17\n\x0e\n\x07\x04\0\x03\0\x02\x01\x05\x12\x03\x1a\
    \x06\x0b\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03\x1a\x0c\x12\n\x0e\n\
    \x07\x04\0\x03\0\x02\x01\x03\x12\x03\x1a\x15\x16\n\xe8\x01\n\x04\x04\0\
    \x02\0\x12\x03#\x02\x1d\x1a\xda\x01\x20Extent\x20of\x20the\x20slice\x20i\
    n\x20all\x20tensor\x20dimensions.\n\n\x20Must\x20have\x20one\x20entry\
    \x20for\x20each\x20of\x20the\x20dimension\x20of\x20the\x20tensor\x20that\
    \x20this\n\x20slice\x20belongs\x20to.\x20\x20The\x20order\x20of\x20sizes\
    \x20is\x20the\x20same\x20as\x20the\x20order\x20of\n\x20dimensions\x20in\
    \x20the\x20TensorShape.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03#\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03#\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03#\x12\x18\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03#\x1b\x1cb\x06proto\
    3\
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
