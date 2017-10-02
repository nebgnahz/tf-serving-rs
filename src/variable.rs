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
pub struct VariableDef {
    // message fields
    pub variable_name: ::std::string::String,
    pub initial_value_name: ::std::string::String,
    pub initializer_name: ::std::string::String,
    pub snapshot_name: ::std::string::String,
    pub save_slice_info_def: ::protobuf::SingularPtrField<SaveSliceInfoDef>,
    pub is_resource: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VariableDef {}

impl VariableDef {
    pub fn new() -> VariableDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VariableDef {
        static mut instance: ::protobuf::lazy::Lazy<VariableDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VariableDef,
        };
        unsafe {
            instance.get(VariableDef::new)
        }
    }

    // string variable_name = 1;

    pub fn clear_variable_name(&mut self) {
        self.variable_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_variable_name(&mut self, v: ::std::string::String) {
        self.variable_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_variable_name(&mut self) -> &mut ::std::string::String {
        &mut self.variable_name
    }

    // Take field
    pub fn take_variable_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.variable_name, ::std::string::String::new())
    }

    pub fn get_variable_name(&self) -> &str {
        &self.variable_name
    }

    fn get_variable_name_for_reflect(&self) -> &::std::string::String {
        &self.variable_name
    }

    fn mut_variable_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.variable_name
    }

    // string initial_value_name = 6;

    pub fn clear_initial_value_name(&mut self) {
        self.initial_value_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_initial_value_name(&mut self, v: ::std::string::String) {
        self.initial_value_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_initial_value_name(&mut self) -> &mut ::std::string::String {
        &mut self.initial_value_name
    }

    // Take field
    pub fn take_initial_value_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.initial_value_name, ::std::string::String::new())
    }

    pub fn get_initial_value_name(&self) -> &str {
        &self.initial_value_name
    }

    fn get_initial_value_name_for_reflect(&self) -> &::std::string::String {
        &self.initial_value_name
    }

    fn mut_initial_value_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.initial_value_name
    }

    // string initializer_name = 2;

    pub fn clear_initializer_name(&mut self) {
        self.initializer_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_initializer_name(&mut self, v: ::std::string::String) {
        self.initializer_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_initializer_name(&mut self) -> &mut ::std::string::String {
        &mut self.initializer_name
    }

    // Take field
    pub fn take_initializer_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.initializer_name, ::std::string::String::new())
    }

    pub fn get_initializer_name(&self) -> &str {
        &self.initializer_name
    }

    fn get_initializer_name_for_reflect(&self) -> &::std::string::String {
        &self.initializer_name
    }

    fn mut_initializer_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.initializer_name
    }

    // string snapshot_name = 3;

    pub fn clear_snapshot_name(&mut self) {
        self.snapshot_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_snapshot_name(&mut self, v: ::std::string::String) {
        self.snapshot_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshot_name(&mut self) -> &mut ::std::string::String {
        &mut self.snapshot_name
    }

    // Take field
    pub fn take_snapshot_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.snapshot_name, ::std::string::String::new())
    }

    pub fn get_snapshot_name(&self) -> &str {
        &self.snapshot_name
    }

    fn get_snapshot_name_for_reflect(&self) -> &::std::string::String {
        &self.snapshot_name
    }

    fn mut_snapshot_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.snapshot_name
    }

    // .tensorflow.SaveSliceInfoDef save_slice_info_def = 4;

    pub fn clear_save_slice_info_def(&mut self) {
        self.save_slice_info_def.clear();
    }

    pub fn has_save_slice_info_def(&self) -> bool {
        self.save_slice_info_def.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_slice_info_def(&mut self, v: SaveSliceInfoDef) {
        self.save_slice_info_def = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_slice_info_def(&mut self) -> &mut SaveSliceInfoDef {
        if self.save_slice_info_def.is_none() {
            self.save_slice_info_def.set_default();
        }
        self.save_slice_info_def.as_mut().unwrap()
    }

    // Take field
    pub fn take_save_slice_info_def(&mut self) -> SaveSliceInfoDef {
        self.save_slice_info_def.take().unwrap_or_else(|| SaveSliceInfoDef::new())
    }

    pub fn get_save_slice_info_def(&self) -> &SaveSliceInfoDef {
        self.save_slice_info_def.as_ref().unwrap_or_else(|| SaveSliceInfoDef::default_instance())
    }

    fn get_save_slice_info_def_for_reflect(&self) -> &::protobuf::SingularPtrField<SaveSliceInfoDef> {
        &self.save_slice_info_def
    }

    fn mut_save_slice_info_def_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SaveSliceInfoDef> {
        &mut self.save_slice_info_def
    }

    // bool is_resource = 5;

    pub fn clear_is_resource(&mut self) {
        self.is_resource = false;
    }

    // Param is passed by value, moved
    pub fn set_is_resource(&mut self, v: bool) {
        self.is_resource = v;
    }

    pub fn get_is_resource(&self) -> bool {
        self.is_resource
    }

    fn get_is_resource_for_reflect(&self) -> &bool {
        &self.is_resource
    }

    fn mut_is_resource_for_reflect(&mut self) -> &mut bool {
        &mut self.is_resource
    }
}

impl ::protobuf::Message for VariableDef {
    fn is_initialized(&self) -> bool {
        for v in &self.save_slice_info_def {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.variable_name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.initial_value_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.initializer_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.snapshot_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.save_slice_info_def)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_resource = tmp;
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
        if !self.variable_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.variable_name);
        }
        if !self.initial_value_name.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.initial_value_name);
        }
        if !self.initializer_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.initializer_name);
        }
        if !self.snapshot_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.snapshot_name);
        }
        if let Some(ref v) = self.save_slice_info_def.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.is_resource != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.variable_name.is_empty() {
            os.write_string(1, &self.variable_name)?;
        }
        if !self.initial_value_name.is_empty() {
            os.write_string(6, &self.initial_value_name)?;
        }
        if !self.initializer_name.is_empty() {
            os.write_string(2, &self.initializer_name)?;
        }
        if !self.snapshot_name.is_empty() {
            os.write_string(3, &self.snapshot_name)?;
        }
        if let Some(ref v) = self.save_slice_info_def.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.is_resource != false {
            os.write_bool(5, self.is_resource)?;
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

impl ::protobuf::MessageStatic for VariableDef {
    fn new() -> VariableDef {
        VariableDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<VariableDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "variable_name",
                    VariableDef::get_variable_name_for_reflect,
                    VariableDef::mut_variable_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "initial_value_name",
                    VariableDef::get_initial_value_name_for_reflect,
                    VariableDef::mut_initial_value_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "initializer_name",
                    VariableDef::get_initializer_name_for_reflect,
                    VariableDef::mut_initializer_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "snapshot_name",
                    VariableDef::get_snapshot_name_for_reflect,
                    VariableDef::mut_snapshot_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SaveSliceInfoDef>>(
                    "save_slice_info_def",
                    VariableDef::get_save_slice_info_def_for_reflect,
                    VariableDef::mut_save_slice_info_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_resource",
                    VariableDef::get_is_resource_for_reflect,
                    VariableDef::mut_is_resource_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VariableDef>(
                    "VariableDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VariableDef {
    fn clear(&mut self) {
        self.clear_variable_name();
        self.clear_initial_value_name();
        self.clear_initializer_name();
        self.clear_snapshot_name();
        self.clear_save_slice_info_def();
        self.clear_is_resource();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VariableDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VariableDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SaveSliceInfoDef {
    // message fields
    pub full_name: ::std::string::String,
    pub full_shape: ::std::vec::Vec<i64>,
    pub var_offset: ::std::vec::Vec<i64>,
    pub var_shape: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SaveSliceInfoDef {}

impl SaveSliceInfoDef {
    pub fn new() -> SaveSliceInfoDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SaveSliceInfoDef {
        static mut instance: ::protobuf::lazy::Lazy<SaveSliceInfoDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SaveSliceInfoDef,
        };
        unsafe {
            instance.get(SaveSliceInfoDef::new)
        }
    }

    // string full_name = 1;

    pub fn clear_full_name(&mut self) {
        self.full_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_full_name(&mut self, v: ::std::string::String) {
        self.full_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_full_name(&mut self) -> &mut ::std::string::String {
        &mut self.full_name
    }

    // Take field
    pub fn take_full_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.full_name, ::std::string::String::new())
    }

    pub fn get_full_name(&self) -> &str {
        &self.full_name
    }

    fn get_full_name_for_reflect(&self) -> &::std::string::String {
        &self.full_name
    }

    fn mut_full_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.full_name
    }

    // repeated int64 full_shape = 2;

    pub fn clear_full_shape(&mut self) {
        self.full_shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_full_shape(&mut self, v: ::std::vec::Vec<i64>) {
        self.full_shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_full_shape(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.full_shape
    }

    // Take field
    pub fn take_full_shape(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.full_shape, ::std::vec::Vec::new())
    }

    pub fn get_full_shape(&self) -> &[i64] {
        &self.full_shape
    }

    fn get_full_shape_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.full_shape
    }

    fn mut_full_shape_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.full_shape
    }

    // repeated int64 var_offset = 3;

    pub fn clear_var_offset(&mut self) {
        self.var_offset.clear();
    }

    // Param is passed by value, moved
    pub fn set_var_offset(&mut self, v: ::std::vec::Vec<i64>) {
        self.var_offset = v;
    }

    // Mutable pointer to the field.
    pub fn mut_var_offset(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.var_offset
    }

    // Take field
    pub fn take_var_offset(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.var_offset, ::std::vec::Vec::new())
    }

    pub fn get_var_offset(&self) -> &[i64] {
        &self.var_offset
    }

    fn get_var_offset_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.var_offset
    }

    fn mut_var_offset_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.var_offset
    }

    // repeated int64 var_shape = 4;

    pub fn clear_var_shape(&mut self) {
        self.var_shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_var_shape(&mut self, v: ::std::vec::Vec<i64>) {
        self.var_shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_var_shape(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.var_shape
    }

    // Take field
    pub fn take_var_shape(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.var_shape, ::std::vec::Vec::new())
    }

    pub fn get_var_shape(&self) -> &[i64] {
        &self.var_shape
    }

    fn get_var_shape_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.var_shape
    }

    fn mut_var_shape_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.var_shape
    }
}

impl ::protobuf::Message for SaveSliceInfoDef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.full_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.full_shape)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.var_offset)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.var_shape)?;
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
        if !self.full_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.full_name);
        }
        for value in &self.full_shape {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.var_offset {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.var_shape {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.full_name.is_empty() {
            os.write_string(1, &self.full_name)?;
        }
        for v in &self.full_shape {
            os.write_int64(2, *v)?;
        };
        for v in &self.var_offset {
            os.write_int64(3, *v)?;
        };
        for v in &self.var_shape {
            os.write_int64(4, *v)?;
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

impl ::protobuf::MessageStatic for SaveSliceInfoDef {
    fn new() -> SaveSliceInfoDef {
        SaveSliceInfoDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<SaveSliceInfoDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "full_name",
                    SaveSliceInfoDef::get_full_name_for_reflect,
                    SaveSliceInfoDef::mut_full_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "full_shape",
                    SaveSliceInfoDef::get_full_shape_for_reflect,
                    SaveSliceInfoDef::mut_full_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "var_offset",
                    SaveSliceInfoDef::get_var_offset_for_reflect,
                    SaveSliceInfoDef::mut_var_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "var_shape",
                    SaveSliceInfoDef::get_var_shape_for_reflect,
                    SaveSliceInfoDef::mut_var_shape_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SaveSliceInfoDef>(
                    "SaveSliceInfoDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SaveSliceInfoDef {
    fn clear(&mut self) {
        self.clear_full_name();
        self.clear_full_shape();
        self.clear_var_offset();
        self.clear_var_shape();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SaveSliceInfoDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SaveSliceInfoDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(tensorflow/core/framework/variable.proto\x12\ntensorflow\"\x9e\x02\n\
    \x0bVariableDef\x12#\n\rvariable_name\x18\x01\x20\x01(\tR\x0cvariableNam\
    e\x12,\n\x12initial_value_name\x18\x06\x20\x01(\tR\x10initialValueName\
    \x12)\n\x10initializer_name\x18\x02\x20\x01(\tR\x0finitializerName\x12#\
    \n\rsnapshot_name\x18\x03\x20\x01(\tR\x0csnapshotName\x12K\n\x13save_sli\
    ce_info_def\x18\x04\x20\x01(\x0b2\x1c.tensorflow.SaveSliceInfoDefR\x10sa\
    veSliceInfoDef\x12\x1f\n\x0bis_resource\x18\x05\x20\x01(\x08R\nisResourc\
    e\"\x8a\x01\n\x10SaveSliceInfoDef\x12\x1b\n\tfull_name\x18\x01\x20\x01(\
    \tR\x08fullName\x12\x1d\n\nfull_shape\x18\x02\x20\x03(\x03R\tfullShape\
    \x12\x1d\n\nvar_offset\x18\x03\x20\x03(\x03R\tvarOffset\x12\x1b\n\tvar_s\
    hape\x18\x04\x20\x03(\x03R\x08varShapeB/\n\x18org.tensorflow.frameworkB\
    \x0eVariableProtosP\x01\xf8\x01\x01J\xac\x0c\n\x06\x12\x04\0\0&\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\
    \n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\
    \n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\
    \0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\
    \x01\x08\x12\x03\x04\0/\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0/\n\x0c\
    \n\x05\x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\
    \x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\
    \x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e.\n\x08\n\
    \x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\
    \n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n6\n\
    \x02\x04\0\x12\x04\t\0\x1b\x01\x1a*\x20Protocol\x20buffer\x20representin\
    g\x20a\x20Variable.\n\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x13\n+\n\x04\x04\
    \0\x02\0\x12\x03\x0b\x02\x1b\x1a\x1e\x20Name\x20of\x20the\x20variable\
    \x20tensor.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0b\x02\t\x15\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x0b\t\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b\x19\x1a\nG\n\x04\
    \x04\0\x02\x01\x12\x03\x0e\x02\x20\x1a:\x20Name\x20of\x20the\x20tensor\
    \x20holding\x20the\x20variable's\x20initial\x20value.\n\n\r\n\x05\x04\0\
    \x02\x01\x04\x12\x04\x0e\x02\x0b\x1b\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x0e\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0e\t\x1b\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x0e\x1e\x1f\n*\n\x04\x04\0\x02\x02\x12\
    \x03\x11\x02\x1e\x1a\x1d\x20Name\x20of\x20the\x20initializer\x20op.\n\n\
    \r\n\x05\x04\0\x02\x02\x04\x12\x04\x11\x02\x0e\x20\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\x11\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x11\t\
    \x19\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x11\x1c\x1d\n+\n\x04\x04\0\
    \x02\x03\x12\x03\x14\x02\x1b\x1a\x1e\x20Name\x20of\x20the\x20snapshot\
    \x20tensor.\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x14\x02\x11\x1e\n\x0c\
    \n\x05\x04\0\x02\x03\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03\x14\t\x16\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x14\x19\x1a\
    \nK\n\x04\x04\0\x02\x04\x12\x03\x17\x02+\x1a>\x20Support\x20for\x20savin\
    g\x20variables\x20as\x20slices\x20of\x20a\x20larger\x20variable.\n\n\r\n\
    \x05\x04\0\x02\x04\x04\x12\x04\x17\x02\x14\x1b\n\x0c\n\x05\x04\0\x02\x04\
    \x06\x12\x03\x17\x02\x12\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x17\x13&\
    \n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x17)*\n?\n\x04\x04\0\x02\x05\x12\
    \x03\x1a\x02\x17\x1a2\x20Whether\x20to\x20represent\x20this\x20as\x20a\
    \x20ResourceVariable.\n\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\x1a\x02\x17+\
    \n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x1a\x02\x06\n\x0c\n\x05\x04\0\x02\
    \x05\x01\x12\x03\x1a\x07\x12\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x1a\
    \x15\x16\n\n\n\x02\x04\x01\x12\x04\x1d\0&\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03\x1d\x08\x18\nB\n\x04\x04\x01\x02\0\x12\x03\x1f\x02\x17\x1a5\x20Name\
    \x20of\x20the\x20full\x20variable\x20of\x20which\x20this\x20is\x20a\x20s\
    lice.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x1f\x02\x1d\x1a\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x1f\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x1f\t\x12\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1f\x15\x16\n*\n\
    \x04\x04\x01\x02\x01\x12\x03!\x02\x20\x1a\x1d\x20Shape\x20of\x20the\x20f\
    ull\x20variable.\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03!\x02\n\n\x0c\
    \n\x05\x04\x01\x02\x01\x05\x12\x03!\x0b\x10\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03!\x11\x1b\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03!\x1e\x1f\n\
    >\n\x04\x04\x01\x02\x02\x12\x03#\x02\x20\x1a1\x20Offset\x20of\x20this\
    \x20variable\x20into\x20the\x20full\x20variable.\n\n\x0c\n\x05\x04\x01\
    \x02\x02\x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03#\x0b\
    \x10\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03#\x11\x1b\n\x0c\n\x05\x04\
    \x01\x02\x02\x03\x12\x03#\x1e\x1f\n&\n\x04\x04\x01\x02\x03\x12\x03%\x02\
    \x1f\x1a\x19\x20Shape\x20of\x20this\x20variable.\n\n\x0c\n\x05\x04\x01\
    \x02\x03\x04\x12\x03%\x02\n\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03%\x0b\
    \x10\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03%\x11\x1a\n\x0c\n\x05\x04\
    \x01\x02\x03\x03\x12\x03%\x1d\x1eb\x06proto3\
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
