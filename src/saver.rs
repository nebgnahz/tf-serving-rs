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
pub struct SaverDef {
    // message fields
    pub filename_tensor_name: ::std::string::String,
    pub save_tensor_name: ::std::string::String,
    pub restore_op_name: ::std::string::String,
    pub max_to_keep: i32,
    pub sharded: bool,
    pub keep_checkpoint_every_n_hours: f32,
    pub version: SaverDef_CheckpointFormatVersion,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SaverDef {}

impl SaverDef {
    pub fn new() -> SaverDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SaverDef {
        static mut instance: ::protobuf::lazy::Lazy<SaverDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SaverDef,
        };
        unsafe {
            instance.get(SaverDef::new)
        }
    }

    // string filename_tensor_name = 1;

    pub fn clear_filename_tensor_name(&mut self) {
        self.filename_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_filename_tensor_name(&mut self, v: ::std::string::String) {
        self.filename_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.filename_tensor_name
    }

    // Take field
    pub fn take_filename_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.filename_tensor_name, ::std::string::String::new())
    }

    pub fn get_filename_tensor_name(&self) -> &str {
        &self.filename_tensor_name
    }

    fn get_filename_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.filename_tensor_name
    }

    fn mut_filename_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.filename_tensor_name
    }

    // string save_tensor_name = 2;

    pub fn clear_save_tensor_name(&mut self) {
        self.save_tensor_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_save_tensor_name(&mut self, v: ::std::string::String) {
        self.save_tensor_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_tensor_name(&mut self) -> &mut ::std::string::String {
        &mut self.save_tensor_name
    }

    // Take field
    pub fn take_save_tensor_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.save_tensor_name, ::std::string::String::new())
    }

    pub fn get_save_tensor_name(&self) -> &str {
        &self.save_tensor_name
    }

    fn get_save_tensor_name_for_reflect(&self) -> &::std::string::String {
        &self.save_tensor_name
    }

    fn mut_save_tensor_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.save_tensor_name
    }

    // string restore_op_name = 3;

    pub fn clear_restore_op_name(&mut self) {
        self.restore_op_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_restore_op_name(&mut self, v: ::std::string::String) {
        self.restore_op_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_restore_op_name(&mut self) -> &mut ::std::string::String {
        &mut self.restore_op_name
    }

    // Take field
    pub fn take_restore_op_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.restore_op_name, ::std::string::String::new())
    }

    pub fn get_restore_op_name(&self) -> &str {
        &self.restore_op_name
    }

    fn get_restore_op_name_for_reflect(&self) -> &::std::string::String {
        &self.restore_op_name
    }

    fn mut_restore_op_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.restore_op_name
    }

    // int32 max_to_keep = 4;

    pub fn clear_max_to_keep(&mut self) {
        self.max_to_keep = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_to_keep(&mut self, v: i32) {
        self.max_to_keep = v;
    }

    pub fn get_max_to_keep(&self) -> i32 {
        self.max_to_keep
    }

    fn get_max_to_keep_for_reflect(&self) -> &i32 {
        &self.max_to_keep
    }

    fn mut_max_to_keep_for_reflect(&mut self) -> &mut i32 {
        &mut self.max_to_keep
    }

    // bool sharded = 5;

    pub fn clear_sharded(&mut self) {
        self.sharded = false;
    }

    // Param is passed by value, moved
    pub fn set_sharded(&mut self, v: bool) {
        self.sharded = v;
    }

    pub fn get_sharded(&self) -> bool {
        self.sharded
    }

    fn get_sharded_for_reflect(&self) -> &bool {
        &self.sharded
    }

    fn mut_sharded_for_reflect(&mut self) -> &mut bool {
        &mut self.sharded
    }

    // float keep_checkpoint_every_n_hours = 6;

    pub fn clear_keep_checkpoint_every_n_hours(&mut self) {
        self.keep_checkpoint_every_n_hours = 0.;
    }

    // Param is passed by value, moved
    pub fn set_keep_checkpoint_every_n_hours(&mut self, v: f32) {
        self.keep_checkpoint_every_n_hours = v;
    }

    pub fn get_keep_checkpoint_every_n_hours(&self) -> f32 {
        self.keep_checkpoint_every_n_hours
    }

    fn get_keep_checkpoint_every_n_hours_for_reflect(&self) -> &f32 {
        &self.keep_checkpoint_every_n_hours
    }

    fn mut_keep_checkpoint_every_n_hours_for_reflect(&mut self) -> &mut f32 {
        &mut self.keep_checkpoint_every_n_hours
    }

    // .tensorflow.SaverDef.CheckpointFormatVersion version = 7;

    pub fn clear_version(&mut self) {
        self.version = SaverDef_CheckpointFormatVersion::LEGACY;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: SaverDef_CheckpointFormatVersion) {
        self.version = v;
    }

    pub fn get_version(&self) -> SaverDef_CheckpointFormatVersion {
        self.version
    }

    fn get_version_for_reflect(&self) -> &SaverDef_CheckpointFormatVersion {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut SaverDef_CheckpointFormatVersion {
        &mut self.version
    }
}

impl ::protobuf::Message for SaverDef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.filename_tensor_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.save_tensor_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.restore_op_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_to_keep = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.sharded = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.keep_checkpoint_every_n_hours = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.version = tmp;
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
        if !self.filename_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.filename_tensor_name);
        }
        if !self.save_tensor_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.save_tensor_name);
        }
        if !self.restore_op_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.restore_op_name);
        }
        if self.max_to_keep != 0 {
            my_size += ::protobuf::rt::value_size(4, self.max_to_keep, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sharded != false {
            my_size += 2;
        }
        if self.keep_checkpoint_every_n_hours != 0. {
            my_size += 5;
        }
        if self.version != SaverDef_CheckpointFormatVersion::LEGACY {
            my_size += ::protobuf::rt::enum_size(7, self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.filename_tensor_name.is_empty() {
            os.write_string(1, &self.filename_tensor_name)?;
        }
        if !self.save_tensor_name.is_empty() {
            os.write_string(2, &self.save_tensor_name)?;
        }
        if !self.restore_op_name.is_empty() {
            os.write_string(3, &self.restore_op_name)?;
        }
        if self.max_to_keep != 0 {
            os.write_int32(4, self.max_to_keep)?;
        }
        if self.sharded != false {
            os.write_bool(5, self.sharded)?;
        }
        if self.keep_checkpoint_every_n_hours != 0. {
            os.write_float(6, self.keep_checkpoint_every_n_hours)?;
        }
        if self.version != SaverDef_CheckpointFormatVersion::LEGACY {
            os.write_enum(7, self.version.value())?;
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

impl ::protobuf::MessageStatic for SaverDef {
    fn new() -> SaverDef {
        SaverDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<SaverDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename_tensor_name",
                    SaverDef::get_filename_tensor_name_for_reflect,
                    SaverDef::mut_filename_tensor_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "save_tensor_name",
                    SaverDef::get_save_tensor_name_for_reflect,
                    SaverDef::mut_save_tensor_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "restore_op_name",
                    SaverDef::get_restore_op_name_for_reflect,
                    SaverDef::mut_restore_op_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_to_keep",
                    SaverDef::get_max_to_keep_for_reflect,
                    SaverDef::mut_max_to_keep_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sharded",
                    SaverDef::get_sharded_for_reflect,
                    SaverDef::mut_sharded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "keep_checkpoint_every_n_hours",
                    SaverDef::get_keep_checkpoint_every_n_hours_for_reflect,
                    SaverDef::mut_keep_checkpoint_every_n_hours_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SaverDef_CheckpointFormatVersion>>(
                    "version",
                    SaverDef::get_version_for_reflect,
                    SaverDef::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SaverDef>(
                    "SaverDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SaverDef {
    fn clear(&mut self) {
        self.clear_filename_tensor_name();
        self.clear_save_tensor_name();
        self.clear_restore_op_name();
        self.clear_max_to_keep();
        self.clear_sharded();
        self.clear_keep_checkpoint_every_n_hours();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SaverDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SaverDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SaverDef_CheckpointFormatVersion {
    LEGACY = 0,
    V1 = 1,
    V2 = 2,
}

impl ::protobuf::ProtobufEnum for SaverDef_CheckpointFormatVersion {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SaverDef_CheckpointFormatVersion> {
        match value {
            0 => ::std::option::Option::Some(SaverDef_CheckpointFormatVersion::LEGACY),
            1 => ::std::option::Option::Some(SaverDef_CheckpointFormatVersion::V1),
            2 => ::std::option::Option::Some(SaverDef_CheckpointFormatVersion::V2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SaverDef_CheckpointFormatVersion] = &[
            SaverDef_CheckpointFormatVersion::LEGACY,
            SaverDef_CheckpointFormatVersion::V1,
            SaverDef_CheckpointFormatVersion::V2,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<SaverDef_CheckpointFormatVersion>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SaverDef_CheckpointFormatVersion", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SaverDef_CheckpointFormatVersion {
}

impl ::std::default::Default for SaverDef_CheckpointFormatVersion {
    fn default() -> Self {
        SaverDef_CheckpointFormatVersion::LEGACY
    }
}

impl ::protobuf::reflect::ProtobufValue for SaverDef_CheckpointFormatVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$tensorflow/core/protobuf/saver.proto\x12\ntensorflow\"\x89\x03\n\x08S\
    averDef\x120\n\x14filename_tensor_name\x18\x01\x20\x01(\tR\x12filenameTe\
    nsorName\x12(\n\x10save_tensor_name\x18\x02\x20\x01(\tR\x0esaveTensorNam\
    e\x12&\n\x0frestore_op_name\x18\x03\x20\x01(\tR\rrestoreOpName\x12\x1e\n\
    \x0bmax_to_keep\x18\x04\x20\x01(\x05R\tmaxToKeep\x12\x18\n\x07sharded\
    \x18\x05\x20\x01(\x08R\x07sharded\x12@\n\x1dkeep_checkpoint_every_n_hour\
    s\x18\x06\x20\x01(\x02R\x19keepCheckpointEveryNHours\x12F\n\x07version\
    \x18\x07\x20\x01(\x0e2,.tensorflow.SaverDef.CheckpointFormatVersionR\x07\
    version\"5\n\x17CheckpointFormatVersion\x12\n\n\x06LEGACY\x10\0\x12\x06\
    \n\x02V1\x10\x01\x12\x06\n\x02V2\x10\x02B'\n\x13org.tensorflow.utilB\x0b\
    SaverProtosP\x01\xf8\x01\x01J\xdd\x10\n\x06\x12\x04\0\0-\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\
    \x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\
    \x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\
    \x12\x03\x04\0,\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0,\n\x0c\n\x05\
    \x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\
    \x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\
    \x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e+\n\x08\n\x01\
    \x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\
    \x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\
    \x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\
    \x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\
    \x01\x08\x12\x03\x06\0,\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\0,\n\x0c\
    \n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x16+\nH\n\x02\
    \x04\0\x12\x04\t\0-\x01\x1a<\x20Protocol\x20buffer\x20representing\x20th\
    e\x20configuration\x20of\x20a\x20Saver.\n\n\n\n\x03\x04\0\x01\x12\x03\t\
    \x08\x10\nt\n\x04\x04\0\x02\0\x12\x03\x0c\x02\"\x1ag\x20The\x20name\x20o\
    f\x20the\x20tensor\x20in\x20which\x20to\x20specify\x20the\x20filename\
    \x20when\x20saving\x20or\n\x20restoring\x20a\x20model\x20checkpoint.\n\n\
    \r\n\x05\x04\0\x02\0\x04\x12\x04\x0c\x02\t\x12\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\t\x1d\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x20!\nC\n\x04\x04\0\x02\x01\x12\
    \x03\x0f\x02\x1e\x1a6\x20The\x20operation\x20to\x20run\x20when\x20saving\
    \x20a\x20model\x20checkpoint.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0f\
    \x02\x0c\"\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x0f\t\x19\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x0f\x1c\x1d\nF\n\x04\x04\0\x02\x02\x12\x03\x12\x02\x1d\x1a9\x20The\
    \x20operation\x20to\x20run\x20when\x20restoring\x20a\x20model\x20checkpo\
    int.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x12\x02\x0f\x1e\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\x12\t\x18\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x12\x1b\x1c\nX\n\
    \x04\x04\0\x02\x03\x12\x03\x15\x02\x18\x1aK\x20Maximum\x20number\x20of\
    \x20checkpoints\x20to\x20keep.\x20\x20If\x200,\x20no\x20checkpoints\x20a\
    re\x20deleted.\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x15\x02\x12\x1d\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x15\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x15\x08\x13\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x15\
    \x16\x17\nL\n\x04\x04\0\x02\x04\x12\x03\x18\x02\x13\x1a?\x20Shard\x20the\
    \x20save\x20files,\x20one\x20per\x20device\x20that\x20has\x20Variable\
    \x20nodes.\n\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\x18\x02\x15\x18\n\x0c\n\
    \x05\x04\0\x02\x04\x05\x12\x03\x18\x02\x06\n\x0c\n\x05\x04\0\x02\x04\x01\
    \x12\x03\x18\x07\x0e\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x18\x11\x12\n\
    \x90\x02\n\x04\x04\0\x02\x05\x12\x03\x1e\x02*\x1a\x82\x02\x20How\x20ofte\
    n\x20to\x20keep\x20an\x20additional\x20checkpoint.\x20If\x20not\x20speci\
    fied,\x20only\x20the\x20last\n\x20\"max_to_keep\"\x20checkpoints\x20are\
    \x20kept;\x20if\x20specified,\x20in\x20addition\x20to\x20keeping\n\x20th\
    e\x20last\x20\"max_to_keep\"\x20checkpoints,\x20an\x20additional\x20chec\
    kpoint\x20will\x20be\x20kept\n\x20for\x20every\x20n\x20hours\x20of\x20tr\
    aining.\n\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\x1e\x02\x18\x13\n\x0c\n\
    \x05\x04\0\x02\x05\x05\x12\x03\x1e\x02\x07\n\x0c\n\x05\x04\0\x02\x05\x01\
    \x12\x03\x1e\x08%\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x1e()\n\xa1\x02\
    \n\x04\x04\0\x04\0\x12\x04$\x02+\x03\x1a\x92\x02\x20A\x20version\x20numb\
    er\x20that\x20identifies\x20a\x20different\x20on-disk\x20checkpoint\x20f\
    ormat.\n\x20Usually,\x20each\x20subclass\x20of\x20BaseSaverBuilder\x20wo\
    rks\x20with\x20a\x20particular\n\x20version/format.\x20\x20However,\x20i\
    t\x20is\x20possible\x20that\x20the\x20same\x20builder\x20may\x20be\n\x20\
    upgraded\x20to\x20support\x20a\x20newer\x20checkpoint\x20format\x20in\
    \x20the\x20future.\n\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03$\x07\x1e\n(\n\
    \x06\x04\0\x04\0\x02\0\x12\x03&\x04\x0f\x1a\x19\x20Internal\x20legacy\
    \x20format.\n\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03&\x04\n\n\x0e\n\
    \x07\x04\0\x04\0\x02\0\x02\x12\x03&\r\x0e\nY\n\x06\x04\0\x04\0\x02\x01\
    \x12\x03(\x04\x0b\x1aJ\x20Deprecated\x20format:\x20tf.Saver()\x20which\
    \x20works\x20with\x20tensorflow::table::Table.\n\n\x0e\n\x07\x04\0\x04\0\
    \x02\x01\x01\x12\x03(\x04\x06\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\
    \x03(\t\n\n0\n\x06\x04\0\x04\0\x02\x02\x12\x03*\x04\x0b\x1a!\x20Current\
    \x20format:\x20more\x20efficient.\n\n\x0e\n\x07\x04\0\x04\0\x02\x02\x01\
    \x12\x03*\x04\x06\n\x0e\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03*\t\n\n\x0b\
    \n\x04\x04\0\x02\x06\x12\x03,\x02&\n\r\n\x05\x04\0\x02\x06\x04\x12\x04,\
    \x02+\x03\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03,\x02\x19\n\x0c\n\x05\x04\
    \0\x02\x06\x01\x12\x03,\x1a!\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03,$%b\
    \x06proto3\
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
