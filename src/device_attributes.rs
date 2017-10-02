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
pub struct DeviceLocality {
    // message fields
    pub bus_id: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeviceLocality {}

impl DeviceLocality {
    pub fn new() -> DeviceLocality {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeviceLocality {
        static mut instance: ::protobuf::lazy::Lazy<DeviceLocality> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeviceLocality,
        };
        unsafe {
            instance.get(DeviceLocality::new)
        }
    }

    // int32 bus_id = 1;

    pub fn clear_bus_id(&mut self) {
        self.bus_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_bus_id(&mut self, v: i32) {
        self.bus_id = v;
    }

    pub fn get_bus_id(&self) -> i32 {
        self.bus_id
    }

    fn get_bus_id_for_reflect(&self) -> &i32 {
        &self.bus_id
    }

    fn mut_bus_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.bus_id
    }
}

impl ::protobuf::Message for DeviceLocality {
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
                    self.bus_id = tmp;
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
        if self.bus_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.bus_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.bus_id != 0 {
            os.write_int32(1, self.bus_id)?;
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

impl ::protobuf::MessageStatic for DeviceLocality {
    fn new() -> DeviceLocality {
        DeviceLocality::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeviceLocality>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "bus_id",
                    DeviceLocality::get_bus_id_for_reflect,
                    DeviceLocality::mut_bus_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeviceLocality>(
                    "DeviceLocality",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeviceLocality {
    fn clear(&mut self) {
        self.clear_bus_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeviceLocality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceLocality {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeviceAttributes {
    // message fields
    pub name: ::std::string::String,
    pub device_type: ::std::string::String,
    pub memory_limit: i64,
    pub locality: ::protobuf::SingularPtrField<DeviceLocality>,
    pub incarnation: u64,
    pub physical_device_desc: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeviceAttributes {}

impl DeviceAttributes {
    pub fn new() -> DeviceAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeviceAttributes {
        static mut instance: ::protobuf::lazy::Lazy<DeviceAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeviceAttributes,
        };
        unsafe {
            instance.get(DeviceAttributes::new)
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

    // int64 memory_limit = 4;

    pub fn clear_memory_limit(&mut self) {
        self.memory_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_memory_limit(&mut self, v: i64) {
        self.memory_limit = v;
    }

    pub fn get_memory_limit(&self) -> i64 {
        self.memory_limit
    }

    fn get_memory_limit_for_reflect(&self) -> &i64 {
        &self.memory_limit
    }

    fn mut_memory_limit_for_reflect(&mut self) -> &mut i64 {
        &mut self.memory_limit
    }

    // .tensorflow.DeviceLocality locality = 5;

    pub fn clear_locality(&mut self) {
        self.locality.clear();
    }

    pub fn has_locality(&self) -> bool {
        self.locality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locality(&mut self, v: DeviceLocality) {
        self.locality = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locality(&mut self) -> &mut DeviceLocality {
        if self.locality.is_none() {
            self.locality.set_default();
        }
        self.locality.as_mut().unwrap()
    }

    // Take field
    pub fn take_locality(&mut self) -> DeviceLocality {
        self.locality.take().unwrap_or_else(|| DeviceLocality::new())
    }

    pub fn get_locality(&self) -> &DeviceLocality {
        self.locality.as_ref().unwrap_or_else(|| DeviceLocality::default_instance())
    }

    fn get_locality_for_reflect(&self) -> &::protobuf::SingularPtrField<DeviceLocality> {
        &self.locality
    }

    fn mut_locality_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeviceLocality> {
        &mut self.locality
    }

    // fixed64 incarnation = 6;

    pub fn clear_incarnation(&mut self) {
        self.incarnation = 0;
    }

    // Param is passed by value, moved
    pub fn set_incarnation(&mut self, v: u64) {
        self.incarnation = v;
    }

    pub fn get_incarnation(&self) -> u64 {
        self.incarnation
    }

    fn get_incarnation_for_reflect(&self) -> &u64 {
        &self.incarnation
    }

    fn mut_incarnation_for_reflect(&mut self) -> &mut u64 {
        &mut self.incarnation
    }

    // string physical_device_desc = 7;

    pub fn clear_physical_device_desc(&mut self) {
        self.physical_device_desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_physical_device_desc(&mut self, v: ::std::string::String) {
        self.physical_device_desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_physical_device_desc(&mut self) -> &mut ::std::string::String {
        &mut self.physical_device_desc
    }

    // Take field
    pub fn take_physical_device_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.physical_device_desc, ::std::string::String::new())
    }

    pub fn get_physical_device_desc(&self) -> &str {
        &self.physical_device_desc
    }

    fn get_physical_device_desc_for_reflect(&self) -> &::std::string::String {
        &self.physical_device_desc
    }

    fn mut_physical_device_desc_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.physical_device_desc
    }
}

impl ::protobuf::Message for DeviceAttributes {
    fn is_initialized(&self) -> bool {
        for v in &self.locality {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device_type)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.memory_limit = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locality)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.incarnation = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.physical_device_desc)?;
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
        if !self.device_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.device_type);
        }
        if self.memory_limit != 0 {
            my_size += ::protobuf::rt::value_size(4, self.memory_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.locality.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.incarnation != 0 {
            my_size += 9;
        }
        if !self.physical_device_desc.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.physical_device_desc);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.device_type.is_empty() {
            os.write_string(2, &self.device_type)?;
        }
        if self.memory_limit != 0 {
            os.write_int64(4, self.memory_limit)?;
        }
        if let Some(ref v) = self.locality.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.incarnation != 0 {
            os.write_fixed64(6, self.incarnation)?;
        }
        if !self.physical_device_desc.is_empty() {
            os.write_string(7, &self.physical_device_desc)?;
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

impl ::protobuf::MessageStatic for DeviceAttributes {
    fn new() -> DeviceAttributes {
        DeviceAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeviceAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DeviceAttributes::get_name_for_reflect,
                    DeviceAttributes::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_type",
                    DeviceAttributes::get_device_type_for_reflect,
                    DeviceAttributes::mut_device_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "memory_limit",
                    DeviceAttributes::get_memory_limit_for_reflect,
                    DeviceAttributes::mut_memory_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeviceLocality>>(
                    "locality",
                    DeviceAttributes::get_locality_for_reflect,
                    DeviceAttributes::mut_locality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "incarnation",
                    DeviceAttributes::get_incarnation_for_reflect,
                    DeviceAttributes::mut_incarnation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "physical_device_desc",
                    DeviceAttributes::get_physical_device_desc_for_reflect,
                    DeviceAttributes::mut_physical_device_desc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeviceAttributes>(
                    "DeviceAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeviceAttributes {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_device_type();
        self.clear_memory_limit();
        self.clear_locality();
        self.clear_incarnation();
        self.clear_physical_device_desc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeviceAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n1tensorflow/core/framework/device_attributes.proto\x12\ntensorflow\"'\
    \n\x0eDeviceLocality\x12\x15\n\x06bus_id\x18\x01\x20\x01(\x05R\x05busId\
    \"\xf6\x01\n\x10DeviceAttributes\x12\x12\n\x04name\x18\x01\x20\x01(\tR\
    \x04name\x12\x1f\n\x0bdevice_type\x18\x02\x20\x01(\tR\ndeviceType\x12!\n\
    \x0cmemory_limit\x18\x04\x20\x01(\x03R\x0bmemoryLimit\x126\n\x08locality\
    \x18\x05\x20\x01(\x0b2\x1a.tensorflow.DeviceLocalityR\x08locality\x12\
    \x20\n\x0bincarnation\x18\x06\x20\x01(\x06R\x0bincarnation\x120\n\x14phy\
    sical_device_desc\x18\x07\x20\x01(\tR\x12physicalDeviceDescB7\n\x18org.t\
    ensorflow.frameworkB\x16DeviceAttributesProtosP\x01\xf8\x01\x01J\xa1\x0b\
    \n\x06\x12\x04\0\0\"\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\
    \xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\
    \x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\
    \x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\
    \x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\07\n\x0b\n\x04\x08\
    \xe7\x07\x01\x12\x03\x04\07\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x04\
    \x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\
    \x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\
    \x01\x07\x12\x03\x04\x1e6\n\x08\n\x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\
    \x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\
    \x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\
    \x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\
    \x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\
    \x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\
    \x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\
    \x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\
    \x07\x03\x07\x12\x03\x06\x160\n\n\n\x02\x04\0\x12\x04\x08\0\x0c\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x08\x08\x16\n\x89\x01\n\x04\x04\0\x02\0\x12\x03\
    \x0b\x02\x13\x1a|\x20Optional\x20bus\x20locality\x20of\x20device.\x20\
    \x20Default\x20value\x20of\x200\x20means\n\x20no\x20specific\x20locality\
    .\x20\x20Specific\x20localities\x20are\x20indexed\x20from\x201.\n\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04\x0b\x02\x08\x18\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\x0b\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\x08\x0e\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b\x11\x12\n\n\n\x02\x04\x01\x12\x04\
    \x0e\0\"\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0e\x08\x18\nC\n\x04\x04\x01\
    \x02\0\x12\x03\x10\x02\x12\x1a6\x20Fully\x20specified\x20name\x20of\x20t\
    he\x20device\x20within\x20a\x20cluster.\n\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04\x10\x02\x0e\x1a\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x10\x02\
    \x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x10\t\r\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\x10\x10\x11\n4\n\x04\x04\x01\x02\x01\x12\x03\x13\x02\
    \x19\x1a'\x20String\x20representation\x20of\x20device_type.\n\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04\x13\x02\x10\x12\n\x0c\n\x05\x04\x01\x02\x01\
    \x05\x12\x03\x13\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x13\t\
    \x14\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x13\x17\x18\n2\n\x04\x04\
    \x01\x02\x02\x12\x03\x16\x02\x19\x1a%\x20Memory\x20capacity\x20of\x20dev\
    ice\x20in\x20bytes.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x16\x02\x13\
    \x19\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x16\x02\x07\n\x0c\n\x05\x04\
    \x01\x02\x02\x01\x12\x03\x16\x08\x14\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\
    \x03\x16\x17\x18\no\n\x04\x04\x01\x02\x03\x12\x03\x1a\x02\x1e\x1ab\x20Pl\
    atform-specific\x20data\x20about\x20device\x20that\x20may\x20be\x20usefu\
    l\n\x20for\x20supporting\x20efficient\x20data\x20transfers.\n\n\r\n\x05\
    \x04\x01\x02\x03\x04\x12\x04\x1a\x02\x16\x19\n\x0c\n\x05\x04\x01\x02\x03\
    \x06\x12\x03\x1a\x02\x10\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x1a\x11\
    \x19\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x1a\x1c\x1d\ny\n\x04\x04\
    \x01\x02\x04\x12\x03\x1e\x02\x1a\x1al\x20A\x20device\x20is\x20assigned\
    \x20a\x20global\x20unique\x20number\x20each\x20time\x20it\x20is\n\x20ini\
    tialized.\x20\"incarnation\"\x20should\x20never\x20be\x200.\n\n\r\n\x05\
    \x04\x01\x02\x04\x04\x12\x04\x1e\x02\x1a\x1e\n\x0c\n\x05\x04\x01\x02\x04\
    \x05\x12\x03\x1e\x02\t\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\x1e\n\x15\
    \n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x1e\x18\x19\nU\n\x04\x04\x01\
    \x02\x05\x12\x03!\x02\"\x1aH\x20String\x20representation\x20of\x20the\
    \x20physical\x20device\x20that\x20this\x20device\x20maps\x20to.\n\n\r\n\
    \x05\x04\x01\x02\x05\x04\x12\x04!\x02\x1e\x1a\n\x0c\n\x05\x04\x01\x02\
    \x05\x05\x12\x03!\x02\x08\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03!\t\x1d\
    \n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03!\x20!b\x06proto3\
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
