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
pub struct SummaryDescription {
    // message fields
    pub type_hint: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SummaryDescription {}

impl SummaryDescription {
    pub fn new() -> SummaryDescription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SummaryDescription {
        static mut instance: ::protobuf::lazy::Lazy<SummaryDescription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SummaryDescription,
        };
        unsafe {
            instance.get(SummaryDescription::new)
        }
    }

    // string type_hint = 1;

    pub fn clear_type_hint(&mut self) {
        self.type_hint.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_hint(&mut self, v: ::std::string::String) {
        self.type_hint = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_hint(&mut self) -> &mut ::std::string::String {
        &mut self.type_hint
    }

    // Take field
    pub fn take_type_hint(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_hint, ::std::string::String::new())
    }

    pub fn get_type_hint(&self) -> &str {
        &self.type_hint
    }

    fn get_type_hint_for_reflect(&self) -> &::std::string::String {
        &self.type_hint
    }

    fn mut_type_hint_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_hint
    }
}

impl ::protobuf::Message for SummaryDescription {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_hint)?;
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
        if !self.type_hint.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.type_hint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.type_hint.is_empty() {
            os.write_string(1, &self.type_hint)?;
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

impl ::protobuf::MessageStatic for SummaryDescription {
    fn new() -> SummaryDescription {
        SummaryDescription::new()
    }

    fn descriptor_static(_: ::std::option::Option<SummaryDescription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_hint",
                    SummaryDescription::get_type_hint_for_reflect,
                    SummaryDescription::mut_type_hint_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SummaryDescription>(
                    "SummaryDescription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SummaryDescription {
    fn clear(&mut self) {
        self.clear_type_hint();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SummaryDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SummaryDescription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HistogramProto {
    // message fields
    pub min: f64,
    pub max: f64,
    pub num: f64,
    pub sum: f64,
    pub sum_squares: f64,
    pub bucket_limit: ::std::vec::Vec<f64>,
    pub bucket: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HistogramProto {}

impl HistogramProto {
    pub fn new() -> HistogramProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HistogramProto {
        static mut instance: ::protobuf::lazy::Lazy<HistogramProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HistogramProto,
        };
        unsafe {
            instance.get(HistogramProto::new)
        }
    }

    // double min = 1;

    pub fn clear_min(&mut self) {
        self.min = 0.;
    }

    // Param is passed by value, moved
    pub fn set_min(&mut self, v: f64) {
        self.min = v;
    }

    pub fn get_min(&self) -> f64 {
        self.min
    }

    fn get_min_for_reflect(&self) -> &f64 {
        &self.min
    }

    fn mut_min_for_reflect(&mut self) -> &mut f64 {
        &mut self.min
    }

    // double max = 2;

    pub fn clear_max(&mut self) {
        self.max = 0.;
    }

    // Param is passed by value, moved
    pub fn set_max(&mut self, v: f64) {
        self.max = v;
    }

    pub fn get_max(&self) -> f64 {
        self.max
    }

    fn get_max_for_reflect(&self) -> &f64 {
        &self.max
    }

    fn mut_max_for_reflect(&mut self) -> &mut f64 {
        &mut self.max
    }

    // double num = 3;

    pub fn clear_num(&mut self) {
        self.num = 0.;
    }

    // Param is passed by value, moved
    pub fn set_num(&mut self, v: f64) {
        self.num = v;
    }

    pub fn get_num(&self) -> f64 {
        self.num
    }

    fn get_num_for_reflect(&self) -> &f64 {
        &self.num
    }

    fn mut_num_for_reflect(&mut self) -> &mut f64 {
        &mut self.num
    }

    // double sum = 4;

    pub fn clear_sum(&mut self) {
        self.sum = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: f64) {
        self.sum = v;
    }

    pub fn get_sum(&self) -> f64 {
        self.sum
    }

    fn get_sum_for_reflect(&self) -> &f64 {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut f64 {
        &mut self.sum
    }

    // double sum_squares = 5;

    pub fn clear_sum_squares(&mut self) {
        self.sum_squares = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sum_squares(&mut self, v: f64) {
        self.sum_squares = v;
    }

    pub fn get_sum_squares(&self) -> f64 {
        self.sum_squares
    }

    fn get_sum_squares_for_reflect(&self) -> &f64 {
        &self.sum_squares
    }

    fn mut_sum_squares_for_reflect(&mut self) -> &mut f64 {
        &mut self.sum_squares
    }

    // repeated double bucket_limit = 6;

    pub fn clear_bucket_limit(&mut self) {
        self.bucket_limit.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket_limit(&mut self, v: ::std::vec::Vec<f64>) {
        self.bucket_limit = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket_limit(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.bucket_limit
    }

    // Take field
    pub fn take_bucket_limit(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.bucket_limit, ::std::vec::Vec::new())
    }

    pub fn get_bucket_limit(&self) -> &[f64] {
        &self.bucket_limit
    }

    fn get_bucket_limit_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.bucket_limit
    }

    fn mut_bucket_limit_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.bucket_limit
    }

    // repeated double bucket = 7;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<f64>) {
        self.bucket = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.bucket
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.bucket, ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[f64] {
        &self.bucket
    }

    fn get_bucket_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.bucket
    }
}

impl ::protobuf::Message for HistogramProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.min = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.max = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.num = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sum = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sum_squares = tmp;
                },
                6 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.bucket_limit)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.bucket)?;
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
        if self.min != 0. {
            my_size += 9;
        }
        if self.max != 0. {
            my_size += 9;
        }
        if self.num != 0. {
            my_size += 9;
        }
        if self.sum != 0. {
            my_size += 9;
        }
        if self.sum_squares != 0. {
            my_size += 9;
        }
        if !self.bucket_limit.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.bucket_limit.len() as u32) + (self.bucket_limit.len() * 8) as u32;
        }
        if !self.bucket.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.bucket.len() as u32) + (self.bucket.len() * 8) as u32;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.min != 0. {
            os.write_double(1, self.min)?;
        }
        if self.max != 0. {
            os.write_double(2, self.max)?;
        }
        if self.num != 0. {
            os.write_double(3, self.num)?;
        }
        if self.sum != 0. {
            os.write_double(4, self.sum)?;
        }
        if self.sum_squares != 0. {
            os.write_double(5, self.sum_squares)?;
        }
        if !self.bucket_limit.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.bucket_limit.len() * 8) as u32)?;
            for v in &self.bucket_limit {
                os.write_double_no_tag(*v)?;
            };
        }
        if !self.bucket.is_empty() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.bucket.len() * 8) as u32)?;
            for v in &self.bucket {
                os.write_double_no_tag(*v)?;
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

impl ::protobuf::MessageStatic for HistogramProto {
    fn new() -> HistogramProto {
        HistogramProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<HistogramProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "min",
                    HistogramProto::get_min_for_reflect,
                    HistogramProto::mut_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "max",
                    HistogramProto::get_max_for_reflect,
                    HistogramProto::mut_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "num",
                    HistogramProto::get_num_for_reflect,
                    HistogramProto::mut_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "sum",
                    HistogramProto::get_sum_for_reflect,
                    HistogramProto::mut_sum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "sum_squares",
                    HistogramProto::get_sum_squares_for_reflect,
                    HistogramProto::mut_sum_squares_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "bucket_limit",
                    HistogramProto::get_bucket_limit_for_reflect,
                    HistogramProto::mut_bucket_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "bucket",
                    HistogramProto::get_bucket_for_reflect,
                    HistogramProto::mut_bucket_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HistogramProto>(
                    "HistogramProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HistogramProto {
    fn clear(&mut self) {
        self.clear_min();
        self.clear_max();
        self.clear_num();
        self.clear_sum();
        self.clear_sum_squares();
        self.clear_bucket_limit();
        self.clear_bucket();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HistogramProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HistogramProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SummaryMetadata {
    // message fields
    pub plugin_data: ::protobuf::SingularPtrField<SummaryMetadata_PluginData>,
    pub display_name: ::std::string::String,
    pub summary_description: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SummaryMetadata {}

impl SummaryMetadata {
    pub fn new() -> SummaryMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SummaryMetadata {
        static mut instance: ::protobuf::lazy::Lazy<SummaryMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SummaryMetadata,
        };
        unsafe {
            instance.get(SummaryMetadata::new)
        }
    }

    // .tensorflow.SummaryMetadata.PluginData plugin_data = 1;

    pub fn clear_plugin_data(&mut self) {
        self.plugin_data.clear();
    }

    pub fn has_plugin_data(&self) -> bool {
        self.plugin_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plugin_data(&mut self, v: SummaryMetadata_PluginData) {
        self.plugin_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plugin_data(&mut self) -> &mut SummaryMetadata_PluginData {
        if self.plugin_data.is_none() {
            self.plugin_data.set_default();
        }
        self.plugin_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_plugin_data(&mut self) -> SummaryMetadata_PluginData {
        self.plugin_data.take().unwrap_or_else(|| SummaryMetadata_PluginData::new())
    }

    pub fn get_plugin_data(&self) -> &SummaryMetadata_PluginData {
        self.plugin_data.as_ref().unwrap_or_else(|| SummaryMetadata_PluginData::default_instance())
    }

    fn get_plugin_data_for_reflect(&self) -> &::protobuf::SingularPtrField<SummaryMetadata_PluginData> {
        &self.plugin_data
    }

    fn mut_plugin_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SummaryMetadata_PluginData> {
        &mut self.plugin_data
    }

    // string display_name = 2;

    pub fn clear_display_name(&mut self) {
        self.display_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_display_name(&mut self, v: ::std::string::String) {
        self.display_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_display_name(&mut self) -> &mut ::std::string::String {
        &mut self.display_name
    }

    // Take field
    pub fn take_display_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.display_name, ::std::string::String::new())
    }

    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    fn get_display_name_for_reflect(&self) -> &::std::string::String {
        &self.display_name
    }

    fn mut_display_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.display_name
    }

    // string summary_description = 3;

    pub fn clear_summary_description(&mut self) {
        self.summary_description.clear();
    }

    // Param is passed by value, moved
    pub fn set_summary_description(&mut self, v: ::std::string::String) {
        self.summary_description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary_description(&mut self) -> &mut ::std::string::String {
        &mut self.summary_description
    }

    // Take field
    pub fn take_summary_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.summary_description, ::std::string::String::new())
    }

    pub fn get_summary_description(&self) -> &str {
        &self.summary_description
    }

    fn get_summary_description_for_reflect(&self) -> &::std::string::String {
        &self.summary_description
    }

    fn mut_summary_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.summary_description
    }
}

impl ::protobuf::Message for SummaryMetadata {
    fn is_initialized(&self) -> bool {
        for v in &self.plugin_data {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.plugin_data)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.display_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.summary_description)?;
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
        if let Some(ref v) = self.plugin_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.display_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.display_name);
        }
        if !self.summary_description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.summary_description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.plugin_data.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.display_name.is_empty() {
            os.write_string(2, &self.display_name)?;
        }
        if !self.summary_description.is_empty() {
            os.write_string(3, &self.summary_description)?;
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

impl ::protobuf::MessageStatic for SummaryMetadata {
    fn new() -> SummaryMetadata {
        SummaryMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<SummaryMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SummaryMetadata_PluginData>>(
                    "plugin_data",
                    SummaryMetadata::get_plugin_data_for_reflect,
                    SummaryMetadata::mut_plugin_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "display_name",
                    SummaryMetadata::get_display_name_for_reflect,
                    SummaryMetadata::mut_display_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "summary_description",
                    SummaryMetadata::get_summary_description_for_reflect,
                    SummaryMetadata::mut_summary_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SummaryMetadata>(
                    "SummaryMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SummaryMetadata {
    fn clear(&mut self) {
        self.clear_plugin_data();
        self.clear_display_name();
        self.clear_summary_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SummaryMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SummaryMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SummaryMetadata_PluginData {
    // message fields
    pub plugin_name: ::std::string::String,
    pub content: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SummaryMetadata_PluginData {}

impl SummaryMetadata_PluginData {
    pub fn new() -> SummaryMetadata_PluginData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SummaryMetadata_PluginData {
        static mut instance: ::protobuf::lazy::Lazy<SummaryMetadata_PluginData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SummaryMetadata_PluginData,
        };
        unsafe {
            instance.get(SummaryMetadata_PluginData::new)
        }
    }

    // string plugin_name = 1;

    pub fn clear_plugin_name(&mut self) {
        self.plugin_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_plugin_name(&mut self, v: ::std::string::String) {
        self.plugin_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plugin_name(&mut self) -> &mut ::std::string::String {
        &mut self.plugin_name
    }

    // Take field
    pub fn take_plugin_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.plugin_name, ::std::string::String::new())
    }

    pub fn get_plugin_name(&self) -> &str {
        &self.plugin_name
    }

    fn get_plugin_name_for_reflect(&self) -> &::std::string::String {
        &self.plugin_name
    }

    fn mut_plugin_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.plugin_name
    }

    // string content = 2;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }
}

impl ::protobuf::Message for SummaryMetadata_PluginData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.plugin_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
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
        if !self.plugin_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.plugin_name);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.plugin_name.is_empty() {
            os.write_string(1, &self.plugin_name)?;
        }
        if !self.content.is_empty() {
            os.write_string(2, &self.content)?;
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

impl ::protobuf::MessageStatic for SummaryMetadata_PluginData {
    fn new() -> SummaryMetadata_PluginData {
        SummaryMetadata_PluginData::new()
    }

    fn descriptor_static(_: ::std::option::Option<SummaryMetadata_PluginData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "plugin_name",
                    SummaryMetadata_PluginData::get_plugin_name_for_reflect,
                    SummaryMetadata_PluginData::mut_plugin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    SummaryMetadata_PluginData::get_content_for_reflect,
                    SummaryMetadata_PluginData::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SummaryMetadata_PluginData>(
                    "SummaryMetadata_PluginData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SummaryMetadata_PluginData {
    fn clear(&mut self) {
        self.clear_plugin_name();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SummaryMetadata_PluginData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SummaryMetadata_PluginData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Summary {
    // message fields
    pub value: ::protobuf::RepeatedField<Summary_Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Summary {}

impl Summary {
    pub fn new() -> Summary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Summary {
        static mut instance: ::protobuf::lazy::Lazy<Summary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Summary,
        };
        unsafe {
            instance.get(Summary::new)
        }
    }

    // repeated .tensorflow.Summary.Value value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::protobuf::RepeatedField<Summary_Value>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::protobuf::RepeatedField<Summary_Value> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::protobuf::RepeatedField<Summary_Value> {
        ::std::mem::replace(&mut self.value, ::protobuf::RepeatedField::new())
    }

    pub fn get_value(&self) -> &[Summary_Value] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::protobuf::RepeatedField<Summary_Value> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Summary_Value> {
        &mut self.value
    }
}

impl ::protobuf::Message for Summary {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.value)?;
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
        for value in &self.value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.value {
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

impl ::protobuf::MessageStatic for Summary {
    fn new() -> Summary {
        Summary::new()
    }

    fn descriptor_static(_: ::std::option::Option<Summary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Summary_Value>>(
                    "value",
                    Summary::get_value_for_reflect,
                    Summary::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Summary>(
                    "Summary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Summary {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Summary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Summary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Summary_Image {
    // message fields
    pub height: i32,
    pub width: i32,
    pub colorspace: i32,
    pub encoded_image_string: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Summary_Image {}

impl Summary_Image {
    pub fn new() -> Summary_Image {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Summary_Image {
        static mut instance: ::protobuf::lazy::Lazy<Summary_Image> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Summary_Image,
        };
        unsafe {
            instance.get(Summary_Image::new)
        }
    }

    // int32 height = 1;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = v;
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &i32 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut i32 {
        &mut self.height
    }

    // int32 width = 2;

    pub fn clear_width(&mut self) {
        self.width = 0;
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = v;
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    fn get_width_for_reflect(&self) -> &i32 {
        &self.width
    }

    fn mut_width_for_reflect(&mut self) -> &mut i32 {
        &mut self.width
    }

    // int32 colorspace = 3;

    pub fn clear_colorspace(&mut self) {
        self.colorspace = 0;
    }

    // Param is passed by value, moved
    pub fn set_colorspace(&mut self, v: i32) {
        self.colorspace = v;
    }

    pub fn get_colorspace(&self) -> i32 {
        self.colorspace
    }

    fn get_colorspace_for_reflect(&self) -> &i32 {
        &self.colorspace
    }

    fn mut_colorspace_for_reflect(&mut self) -> &mut i32 {
        &mut self.colorspace
    }

    // bytes encoded_image_string = 4;

    pub fn clear_encoded_image_string(&mut self) {
        self.encoded_image_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_encoded_image_string(&mut self, v: ::std::vec::Vec<u8>) {
        self.encoded_image_string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encoded_image_string(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.encoded_image_string
    }

    // Take field
    pub fn take_encoded_image_string(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.encoded_image_string, ::std::vec::Vec::new())
    }

    pub fn get_encoded_image_string(&self) -> &[u8] {
        &self.encoded_image_string
    }

    fn get_encoded_image_string_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.encoded_image_string
    }

    fn mut_encoded_image_string_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.encoded_image_string
    }
}

impl ::protobuf::Message for Summary_Image {
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
                    self.height = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.width = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.colorspace = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.encoded_image_string)?;
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
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.width != 0 {
            my_size += ::protobuf::rt::value_size(2, self.width, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.colorspace != 0 {
            my_size += ::protobuf::rt::value_size(3, self.colorspace, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.encoded_image_string.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.encoded_image_string);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_int32(1, self.height)?;
        }
        if self.width != 0 {
            os.write_int32(2, self.width)?;
        }
        if self.colorspace != 0 {
            os.write_int32(3, self.colorspace)?;
        }
        if !self.encoded_image_string.is_empty() {
            os.write_bytes(4, &self.encoded_image_string)?;
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

impl ::protobuf::MessageStatic for Summary_Image {
    fn new() -> Summary_Image {
        Summary_Image::new()
    }

    fn descriptor_static(_: ::std::option::Option<Summary_Image>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "height",
                    Summary_Image::get_height_for_reflect,
                    Summary_Image::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "width",
                    Summary_Image::get_width_for_reflect,
                    Summary_Image::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "colorspace",
                    Summary_Image::get_colorspace_for_reflect,
                    Summary_Image::mut_colorspace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encoded_image_string",
                    Summary_Image::get_encoded_image_string_for_reflect,
                    Summary_Image::mut_encoded_image_string_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Summary_Image>(
                    "Summary_Image",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Summary_Image {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_width();
        self.clear_colorspace();
        self.clear_encoded_image_string();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Summary_Image {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Summary_Image {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Summary_Audio {
    // message fields
    pub sample_rate: f32,
    pub num_channels: i64,
    pub length_frames: i64,
    pub encoded_audio_string: ::std::vec::Vec<u8>,
    pub content_type: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Summary_Audio {}

impl Summary_Audio {
    pub fn new() -> Summary_Audio {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Summary_Audio {
        static mut instance: ::protobuf::lazy::Lazy<Summary_Audio> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Summary_Audio,
        };
        unsafe {
            instance.get(Summary_Audio::new)
        }
    }

    // float sample_rate = 1;

    pub fn clear_sample_rate(&mut self) {
        self.sample_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sample_rate(&mut self, v: f32) {
        self.sample_rate = v;
    }

    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }

    fn get_sample_rate_for_reflect(&self) -> &f32 {
        &self.sample_rate
    }

    fn mut_sample_rate_for_reflect(&mut self) -> &mut f32 {
        &mut self.sample_rate
    }

    // int64 num_channels = 2;

    pub fn clear_num_channels(&mut self) {
        self.num_channels = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_channels(&mut self, v: i64) {
        self.num_channels = v;
    }

    pub fn get_num_channels(&self) -> i64 {
        self.num_channels
    }

    fn get_num_channels_for_reflect(&self) -> &i64 {
        &self.num_channels
    }

    fn mut_num_channels_for_reflect(&mut self) -> &mut i64 {
        &mut self.num_channels
    }

    // int64 length_frames = 3;

    pub fn clear_length_frames(&mut self) {
        self.length_frames = 0;
    }

    // Param is passed by value, moved
    pub fn set_length_frames(&mut self, v: i64) {
        self.length_frames = v;
    }

    pub fn get_length_frames(&self) -> i64 {
        self.length_frames
    }

    fn get_length_frames_for_reflect(&self) -> &i64 {
        &self.length_frames
    }

    fn mut_length_frames_for_reflect(&mut self) -> &mut i64 {
        &mut self.length_frames
    }

    // bytes encoded_audio_string = 4;

    pub fn clear_encoded_audio_string(&mut self) {
        self.encoded_audio_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_encoded_audio_string(&mut self, v: ::std::vec::Vec<u8>) {
        self.encoded_audio_string = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encoded_audio_string(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.encoded_audio_string
    }

    // Take field
    pub fn take_encoded_audio_string(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.encoded_audio_string, ::std::vec::Vec::new())
    }

    pub fn get_encoded_audio_string(&self) -> &[u8] {
        &self.encoded_audio_string
    }

    fn get_encoded_audio_string_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.encoded_audio_string
    }

    fn mut_encoded_audio_string_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.encoded_audio_string
    }

    // string content_type = 5;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        &mut self.content_type
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content_type, ::std::string::String::new())
    }

    pub fn get_content_type(&self) -> &str {
        &self.content_type
    }

    fn get_content_type_for_reflect(&self) -> &::std::string::String {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content_type
    }
}

impl ::protobuf::Message for Summary_Audio {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.sample_rate = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.num_channels = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.length_frames = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.encoded_audio_string)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content_type)?;
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
        if self.sample_rate != 0. {
            my_size += 5;
        }
        if self.num_channels != 0 {
            my_size += ::protobuf::rt::value_size(2, self.num_channels, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.length_frames != 0 {
            my_size += ::protobuf::rt::value_size(3, self.length_frames, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.encoded_audio_string.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.encoded_audio_string);
        }
        if !self.content_type.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.content_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.sample_rate != 0. {
            os.write_float(1, self.sample_rate)?;
        }
        if self.num_channels != 0 {
            os.write_int64(2, self.num_channels)?;
        }
        if self.length_frames != 0 {
            os.write_int64(3, self.length_frames)?;
        }
        if !self.encoded_audio_string.is_empty() {
            os.write_bytes(4, &self.encoded_audio_string)?;
        }
        if !self.content_type.is_empty() {
            os.write_string(5, &self.content_type)?;
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

impl ::protobuf::MessageStatic for Summary_Audio {
    fn new() -> Summary_Audio {
        Summary_Audio::new()
    }

    fn descriptor_static(_: ::std::option::Option<Summary_Audio>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "sample_rate",
                    Summary_Audio::get_sample_rate_for_reflect,
                    Summary_Audio::mut_sample_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "num_channels",
                    Summary_Audio::get_num_channels_for_reflect,
                    Summary_Audio::mut_num_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "length_frames",
                    Summary_Audio::get_length_frames_for_reflect,
                    Summary_Audio::mut_length_frames_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encoded_audio_string",
                    Summary_Audio::get_encoded_audio_string_for_reflect,
                    Summary_Audio::mut_encoded_audio_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content_type",
                    Summary_Audio::get_content_type_for_reflect,
                    Summary_Audio::mut_content_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Summary_Audio>(
                    "Summary_Audio",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Summary_Audio {
    fn clear(&mut self) {
        self.clear_sample_rate();
        self.clear_num_channels();
        self.clear_length_frames();
        self.clear_encoded_audio_string();
        self.clear_content_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Summary_Audio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Summary_Audio {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Summary_Value {
    // message fields
    pub node_name: ::std::string::String,
    pub tag: ::std::string::String,
    pub metadata: ::protobuf::SingularPtrField<SummaryMetadata>,
    // message oneof groups
    value: ::std::option::Option<Summary_Value_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Summary_Value {}

#[derive(Clone,PartialEq)]
pub enum Summary_Value_oneof_value {
    simple_value(f32),
    obsolete_old_style_histogram(::std::vec::Vec<u8>),
    image(Summary_Image),
    histo(HistogramProto),
    audio(Summary_Audio),
    tensor(super::tensor::TensorProto),
}

impl Summary_Value {
    pub fn new() -> Summary_Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Summary_Value {
        static mut instance: ::protobuf::lazy::Lazy<Summary_Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Summary_Value,
        };
        unsafe {
            instance.get(Summary_Value::new)
        }
    }

    // string node_name = 7;

    pub fn clear_node_name(&mut self) {
        self.node_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_name(&mut self, v: ::std::string::String) {
        self.node_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_name(&mut self) -> &mut ::std::string::String {
        &mut self.node_name
    }

    // Take field
    pub fn take_node_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node_name, ::std::string::String::new())
    }

    pub fn get_node_name(&self) -> &str {
        &self.node_name
    }

    fn get_node_name_for_reflect(&self) -> &::std::string::String {
        &self.node_name
    }

    fn mut_node_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.node_name
    }

    // string tag = 1;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        &mut self.tag
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tag, ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    fn get_tag_for_reflect(&self) -> &::std::string::String {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tag
    }

    // .tensorflow.SummaryMetadata metadata = 9;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: SummaryMetadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut SummaryMetadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> SummaryMetadata {
        self.metadata.take().unwrap_or_else(|| SummaryMetadata::new())
    }

    pub fn get_metadata(&self) -> &SummaryMetadata {
        self.metadata.as_ref().unwrap_or_else(|| SummaryMetadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<SummaryMetadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SummaryMetadata> {
        &mut self.metadata
    }

    // float simple_value = 2;

    pub fn clear_simple_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_simple_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::simple_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_simple_value(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(Summary_Value_oneof_value::simple_value(v))
    }

    pub fn get_simple_value(&self) -> f32 {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::simple_value(v)) => v,
            _ => 0.,
        }
    }

    // bytes obsolete_old_style_histogram = 3;

    pub fn clear_obsolete_old_style_histogram(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_obsolete_old_style_histogram(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_obsolete_old_style_histogram(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(v))
    }

    // Mutable pointer to the field.
    pub fn mut_obsolete_old_style_histogram(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(::std::vec::Vec::new()));
        }
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_obsolete_old_style_histogram(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_obsolete_old_style_histogram() {
            match self.value.take() {
                ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_obsolete_old_style_histogram(&self) -> &[u8] {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(ref v)) => v,
            _ => &[],
        }
    }

    // .tensorflow.Summary.Image image = 4;

    pub fn clear_image(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_image(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::image(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: Summary_Image) {
        self.value = ::std::option::Option::Some(Summary_Value_oneof_value::image(v))
    }

    // Mutable pointer to the field.
    pub fn mut_image(&mut self) -> &mut Summary_Image {
        if let ::std::option::Option::Some(Summary_Value_oneof_value::image(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Summary_Value_oneof_value::image(Summary_Image::new()));
        }
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::image(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_image(&mut self) -> Summary_Image {
        if self.has_image() {
            match self.value.take() {
                ::std::option::Option::Some(Summary_Value_oneof_value::image(v)) => v,
                _ => panic!(),
            }
        } else {
            Summary_Image::new()
        }
    }

    pub fn get_image(&self) -> &Summary_Image {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::image(ref v)) => v,
            _ => Summary_Image::default_instance(),
        }
    }

    // .tensorflow.HistogramProto histo = 5;

    pub fn clear_histo(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_histo(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::histo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_histo(&mut self, v: HistogramProto) {
        self.value = ::std::option::Option::Some(Summary_Value_oneof_value::histo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_histo(&mut self) -> &mut HistogramProto {
        if let ::std::option::Option::Some(Summary_Value_oneof_value::histo(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Summary_Value_oneof_value::histo(HistogramProto::new()));
        }
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::histo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_histo(&mut self) -> HistogramProto {
        if self.has_histo() {
            match self.value.take() {
                ::std::option::Option::Some(Summary_Value_oneof_value::histo(v)) => v,
                _ => panic!(),
            }
        } else {
            HistogramProto::new()
        }
    }

    pub fn get_histo(&self) -> &HistogramProto {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::histo(ref v)) => v,
            _ => HistogramProto::default_instance(),
        }
    }

    // .tensorflow.Summary.Audio audio = 6;

    pub fn clear_audio(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_audio(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::audio(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_audio(&mut self, v: Summary_Audio) {
        self.value = ::std::option::Option::Some(Summary_Value_oneof_value::audio(v))
    }

    // Mutable pointer to the field.
    pub fn mut_audio(&mut self) -> &mut Summary_Audio {
        if let ::std::option::Option::Some(Summary_Value_oneof_value::audio(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Summary_Value_oneof_value::audio(Summary_Audio::new()));
        }
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::audio(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_audio(&mut self) -> Summary_Audio {
        if self.has_audio() {
            match self.value.take() {
                ::std::option::Option::Some(Summary_Value_oneof_value::audio(v)) => v,
                _ => panic!(),
            }
        } else {
            Summary_Audio::new()
        }
    }

    pub fn get_audio(&self) -> &Summary_Audio {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::audio(ref v)) => v,
            _ => Summary_Audio::default_instance(),
        }
    }

    // .tensorflow.TensorProto tensor = 8;

    pub fn clear_tensor(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_tensor(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::tensor(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tensor(&mut self, v: super::tensor::TensorProto) {
        self.value = ::std::option::Option::Some(Summary_Value_oneof_value::tensor(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tensor(&mut self) -> &mut super::tensor::TensorProto {
        if let ::std::option::Option::Some(Summary_Value_oneof_value::tensor(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Summary_Value_oneof_value::tensor(super::tensor::TensorProto::new()));
        }
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::tensor(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tensor(&mut self) -> super::tensor::TensorProto {
        if self.has_tensor() {
            match self.value.take() {
                ::std::option::Option::Some(Summary_Value_oneof_value::tensor(v)) => v,
                _ => panic!(),
            }
        } else {
            super::tensor::TensorProto::new()
        }
    }

    pub fn get_tensor(&self) -> &super::tensor::TensorProto {
        match self.value {
            ::std::option::Option::Some(Summary_Value_oneof_value::tensor(ref v)) => v,
            _ => super::tensor::TensorProto::default_instance(),
        }
    }
}

impl ::protobuf::Message for Summary_Value {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(Summary_Value_oneof_value::image(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Summary_Value_oneof_value::histo(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Summary_Value_oneof_value::audio(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Summary_Value_oneof_value::tensor(ref v)) = self.value {
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
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node_name)?;
                },
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tag)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Summary_Value_oneof_value::simple_value(is.read_float()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Summary_Value_oneof_value::obsolete_old_style_histogram(is.read_bytes()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Summary_Value_oneof_value::image(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Summary_Value_oneof_value::histo(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Summary_Value_oneof_value::audio(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Summary_Value_oneof_value::tensor(is.read_message()?));
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
        if !self.node_name.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.node_name);
        }
        if !self.tag.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tag);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Summary_Value_oneof_value::simple_value(v) => {
                    my_size += 5;
                },
                &Summary_Value_oneof_value::obsolete_old_style_histogram(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(3, &v);
                },
                &Summary_Value_oneof_value::image(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Summary_Value_oneof_value::histo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Summary_Value_oneof_value::audio(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Summary_Value_oneof_value::tensor(ref v) => {
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
        if !self.node_name.is_empty() {
            os.write_string(7, &self.node_name)?;
        }
        if !self.tag.is_empty() {
            os.write_string(1, &self.tag)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Summary_Value_oneof_value::simple_value(v) => {
                    os.write_float(2, v)?;
                },
                &Summary_Value_oneof_value::obsolete_old_style_histogram(ref v) => {
                    os.write_bytes(3, v)?;
                },
                &Summary_Value_oneof_value::image(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Summary_Value_oneof_value::histo(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Summary_Value_oneof_value::audio(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Summary_Value_oneof_value::tensor(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Summary_Value {
    fn new() -> Summary_Value {
        Summary_Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<Summary_Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node_name",
                    Summary_Value::get_node_name_for_reflect,
                    Summary_Value::mut_node_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    Summary_Value::get_tag_for_reflect,
                    Summary_Value::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SummaryMetadata>>(
                    "metadata",
                    Summary_Value::get_metadata_for_reflect,
                    Summary_Value::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor::<_>(
                    "simple_value",
                    Summary_Value::has_simple_value,
                    Summary_Value::get_simple_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "obsolete_old_style_histogram",
                    Summary_Value::has_obsolete_old_style_histogram,
                    Summary_Value::get_obsolete_old_style_histogram,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Summary_Image>(
                    "image",
                    Summary_Value::has_image,
                    Summary_Value::get_image,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, HistogramProto>(
                    "histo",
                    Summary_Value::has_histo,
                    Summary_Value::get_histo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Summary_Audio>(
                    "audio",
                    Summary_Value::has_audio,
                    Summary_Value::get_audio,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::tensor::TensorProto>(
                    "tensor",
                    Summary_Value::has_tensor,
                    Summary_Value::get_tensor,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Summary_Value>(
                    "Summary_Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Summary_Value {
    fn clear(&mut self) {
        self.clear_node_name();
        self.clear_tag();
        self.clear_metadata();
        self.clear_simple_value();
        self.clear_obsolete_old_style_histogram();
        self.clear_image();
        self.clear_histo();
        self.clear_audio();
        self.clear_tensor();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Summary_Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Summary_Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'tensorflow/core/framework/summary.proto\x12\ntensorflow\x1a&tensorflo\
    w/core/framework/tensor.proto\"1\n\x12SummaryDescription\x12\x1b\n\ttype\
    _hint\x18\x01\x20\x01(\tR\x08typeHint\"\xbc\x01\n\x0eHistogramProto\x12\
    \x10\n\x03min\x18\x01\x20\x01(\x01R\x03min\x12\x10\n\x03max\x18\x02\x20\
    \x01(\x01R\x03max\x12\x10\n\x03num\x18\x03\x20\x01(\x01R\x03num\x12\x10\
    \n\x03sum\x18\x04\x20\x01(\x01R\x03sum\x12\x1f\n\x0bsum_squares\x18\x05\
    \x20\x01(\x01R\nsumSquares\x12%\n\x0cbucket_limit\x18\x06\x20\x03(\x01R\
    \x0bbucketLimitB\x02\x10\x01\x12\x1a\n\x06bucket\x18\x07\x20\x03(\x01R\
    \x06bucketB\x02\x10\x01\"\xf7\x01\n\x0fSummaryMetadata\x12G\n\x0bplugin_\
    data\x18\x01\x20\x01(\x0b2&.tensorflow.SummaryMetadata.PluginDataR\nplug\
    inData\x12!\n\x0cdisplay_name\x18\x02\x20\x01(\tR\x0bdisplayName\x12/\n\
    \x13summary_description\x18\x03\x20\x01(\tR\x12summaryDescription\x1aG\n\
    \nPluginData\x12\x1f\n\x0bplugin_name\x18\x01\x20\x01(\tR\npluginName\
    \x12\x18\n\x07content\x18\x02\x20\x01(\tR\x07content\"\xbc\x06\n\x07Summ\
    ary\x12/\n\x05value\x18\x01\x20\x03(\x0b2\x19.tensorflow.Summary.ValueR\
    \x05value\x1a\x87\x01\n\x05Image\x12\x16\n\x06height\x18\x01\x20\x01(\
    \x05R\x06height\x12\x14\n\x05width\x18\x02\x20\x01(\x05R\x05width\x12\
    \x1e\n\ncolorspace\x18\x03\x20\x01(\x05R\ncolorspace\x120\n\x14encoded_i\
    mage_string\x18\x04\x20\x01(\x0cR\x12encodedImageString\x1a\xc5\x01\n\
    \x05Audio\x12\x1f\n\x0bsample_rate\x18\x01\x20\x01(\x02R\nsampleRate\x12\
    !\n\x0cnum_channels\x18\x02\x20\x01(\x03R\x0bnumChannels\x12#\n\rlength_\
    frames\x18\x03\x20\x01(\x03R\x0clengthFrames\x120\n\x14encoded_audio_str\
    ing\x18\x04\x20\x01(\x0cR\x12encodedAudioString\x12!\n\x0ccontent_type\
    \x18\x05\x20\x01(\tR\x0bcontentType\x1a\xad\x03\n\x05Value\x12\x1b\n\tno\
    de_name\x18\x07\x20\x01(\tR\x08nodeName\x12\x10\n\x03tag\x18\x01\x20\x01\
    (\tR\x03tag\x127\n\x08metadata\x18\t\x20\x01(\x0b2\x1b.tensorflow.Summar\
    yMetadataR\x08metadata\x12#\n\x0csimple_value\x18\x02\x20\x01(\x02H\0R\
    \x0bsimpleValue\x12A\n\x1cobsolete_old_style_histogram\x18\x03\x20\x01(\
    \x0cH\0R\x19obsoleteOldStyleHistogram\x121\n\x05image\x18\x04\x20\x01(\
    \x0b2\x19.tensorflow.Summary.ImageH\0R\x05image\x122\n\x05histo\x18\x05\
    \x20\x01(\x0b2\x1a.tensorflow.HistogramProtoH\0R\x05histo\x121\n\x05audi\
    o\x18\x06\x20\x01(\x0b2\x19.tensorflow.Summary.AudioH\0R\x05audio\x121\n\
    \x06tensor\x18\x08\x20\x01(\x0b2\x17.tensorflow.TensorProtoH\0R\x06tenso\
    rB\x07\n\x05valueB.\n\x18org.tensorflow.frameworkB\rSummaryProtosP\x01\
    \xf8\x01\x01J\xfb*\n\x06\x12\x04\0\0{\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\x03\0\
    \x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\
    \0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\
    \x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\
    \x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\0.\n\
    \x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0.\n\x0c\n\x05\x08\xe7\x07\x01\
    \x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\
    \x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\
    \x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e-\n\x08\n\x01\x08\x12\x03\x05\0\
    \"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\
    \x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\
    \x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\
    \n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\
    \01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\
    \x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\
    \x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\
    \n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\x03\0\x12\x03\x08\
    \x07/\n?\n\x02\x04\0\x12\x04\x0b\0\x0f\x01\x1a3\x20Metadata\x20associate\
    d\x20with\x20a\x20series\x20of\x20Summary\x20data\n\n\n\n\x03\x04\0\x01\
    \x12\x03\x0b\x08\x1a\n\x8c\x01\n\x04\x04\0\x02\0\x12\x03\x0e\x02\x17\x1a\
    \x7f\x20Hint\x20on\x20how\x20plugins\x20should\x20process\x20the\x20data\
    \x20in\x20this\x20series.\n\x20Supported\x20values\x20include\x20\"scala\
    r\",\x20\"histogram\",\x20\"image\",\x20\"audio\"\n\n\r\n\x05\x04\0\x02\
    \0\x04\x12\x04\x0e\x02\x0b\x1c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0e\
    \x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0e\t\x12\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x0e\x15\x16\nZ\n\x02\x04\x01\x12\x04\x13\0!\x01\x1aN\
    \x20Serialization\x20format\x20for\x20histogram\x20module\x20in\n\x20cor\
    e/lib/histogram/histogram.h\n\n\n\n\x03\x04\x01\x01\x12\x03\x13\x08\x16\
    \n\x0b\n\x04\x04\x01\x02\0\x12\x03\x14\x02\x11\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04\x14\x02\x13\x18\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x14\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x14\t\x0c\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x14\x0f\x10\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\
    \x15\x02\x11\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x15\x02\x14\x11\n\x0c\
    \n\x05\x04\x01\x02\x01\x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03\x15\t\x0c\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x15\
    \x0f\x10\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x16\x02\x11\n\r\n\x05\x04\
    \x01\x02\x02\x04\x12\x04\x16\x02\x15\x11\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\x16\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x16\t\x0c\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x16\x0f\x10\n\x0b\n\x04\x04\x01\
    \x02\x03\x12\x03\x17\x02\x11\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\x17\
    \x02\x16\x11\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x17\x02\x08\n\x0c\n\
    \x05\x04\x01\x02\x03\x01\x12\x03\x17\t\x0c\n\x0c\n\x05\x04\x01\x02\x03\
    \x03\x12\x03\x17\x0f\x10\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x18\x02\x19\
    \n\r\n\x05\x04\x01\x02\x04\x04\x12\x04\x18\x02\x17\x11\n\x0c\n\x05\x04\
    \x01\x02\x04\x05\x12\x03\x18\x02\x08\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\
    \x03\x18\t\x14\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x18\x17\x18\n\xf5\
    \x01\n\x04\x04\x01\x02\x05\x12\x03\x1f\x023\x1a\xe7\x01\x20Parallel\x20a\
    rrays\x20encoding\x20the\x20bucket\x20boundaries\x20and\x20the\x20bucket\
    \x20values.\n\x20bucket(i)\x20is\x20the\x20count\x20for\x20the\x20bucket\
    \x20i.\x20\x20The\x20range\x20for\n\x20a\x20bucket\x20is:\n\x20\x20\x20i\
    \x20==\x200:\x20\x20-DBL_MAX\x20..\x20bucket_limit(0)\n\x20\x20\x20i\x20\
    !=\x200:\x20\x20bucket_limit(i-1)\x20..\x20bucket_limit(i)\n\n\x0c\n\x05\
    \x04\x01\x02\x05\x04\x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x01\x02\x05\x05\
    \x12\x03\x1f\x0b\x11\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x1f\x12\x1e\
    \n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03\x1f!\"\n\x0c\n\x05\x04\x01\x02\
    \x05\x08\x12\x03\x1f#2\n\x0f\n\x08\x04\x01\x02\x05\x08\xe7\x07\0\x12\x03\
    \x1f$1\n\x10\n\t\x04\x01\x02\x05\x08\xe7\x07\0\x02\x12\x03\x1f$*\n\x11\n\
    \n\x04\x01\x02\x05\x08\xe7\x07\0\x02\0\x12\x03\x1f$*\n\x12\n\x0b\x04\x01\
    \x02\x05\x08\xe7\x07\0\x02\0\x01\x12\x03\x1f$*\n\x10\n\t\x04\x01\x02\x05\
    \x08\xe7\x07\0\x03\x12\x03\x1f-1\n\x0b\n\x04\x04\x01\x02\x06\x12\x03\x20\
    \x02-\n\x0c\n\x05\x04\x01\x02\x06\x04\x12\x03\x20\x02\n\n\x0c\n\x05\x04\
    \x01\x02\x06\x05\x12\x03\x20\x0b\x11\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\
    \x03\x20\x12\x18\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03\x20\x1b\x1c\n\
    \x0c\n\x05\x04\x01\x02\x06\x08\x12\x03\x20\x1d,\n\x0f\n\x08\x04\x01\x02\
    \x06\x08\xe7\x07\0\x12\x03\x20\x1e+\n\x10\n\t\x04\x01\x02\x06\x08\xe7\
    \x07\0\x02\x12\x03\x20\x1e$\n\x11\n\n\x04\x01\x02\x06\x08\xe7\x07\0\x02\
    \0\x12\x03\x20\x1e$\n\x12\n\x0b\x04\x01\x02\x06\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x20\x1e$\n\x10\n\t\x04\x01\x02\x06\x08\xe7\x07\0\x03\x12\x03\
    \x20'+\n{\n\x02\x04\x02\x12\x04%\07\x01\x1ao\x20A\x20SummaryMetadata\x20\
    encapsulates\x20information\x20on\x20which\x20plugins\x20are\x20able\x20\
    to\x20make\n\x20use\x20of\x20a\x20certain\x20summary\x20value.\n\n\n\n\
    \x03\x04\x02\x01\x12\x03%\x08\x17\n\x0c\n\x04\x04\x02\x03\0\x12\x04&\x02\
    -\x03\n\x0c\n\x05\x04\x02\x03\0\x01\x12\x03&\n\x14\n>\n\x06\x04\x02\x03\
    \0\x02\0\x12\x03(\x04\x1b\x1a/\x20The\x20name\x20of\x20the\x20plugin\x20\
    this\x20data\x20pertains\x20to.\n\n\x0f\n\x07\x04\x02\x03\0\x02\0\x04\
    \x12\x04(\x04&\x16\n\x0e\n\x07\x04\x02\x03\0\x02\0\x05\x12\x03(\x04\n\n\
    \x0e\n\x07\x04\x02\x03\0\x02\0\x01\x12\x03(\x0b\x16\n\x0e\n\x07\x04\x02\
    \x03\0\x02\0\x03\x12\x03(\x19\x1a\n\x7f\n\x06\x04\x02\x03\0\x02\x01\x12\
    \x03,\x04\x17\x1ap\x20The\x20content\x20to\x20store\x20for\x20the\x20plu\
    gin.\x20The\x20best\x20practice\x20is\x20for\x20this\x20to\x20be\n\x20a\
    \x20binary\x20serialized\x20protocol\x20buffer.\n\n\x0f\n\x07\x04\x02\
    \x03\0\x02\x01\x04\x12\x04,\x04(\x1b\n\x0e\n\x07\x04\x02\x03\0\x02\x01\
    \x05\x12\x03,\x04\n\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x03,\x0b\
    \x12\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03,\x15\x16\nD\n\x04\x04\
    \x02\x02\0\x12\x030\x02\x1d\x1a7\x20Data\x20that\x20associates\x20a\x20s\
    ummary\x20with\x20a\x20certain\x20plugin.\n\n\r\n\x05\x04\x02\x02\0\x04\
    \x12\x040\x02-\x03\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x030\x02\x0c\n\x0c\
    \n\x05\x04\x02\x02\0\x01\x12\x030\r\x18\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x030\x1b\x1c\n7\n\x04\x04\x02\x02\x01\x12\x033\x02\x1a\x1a*\x20Disp\
    lay\x20name\x20for\x20viewing\x20in\x20TensorBoard.\n\n\r\n\x05\x04\x02\
    \x02\x01\x04\x12\x043\x020\x1d\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x033\
    \x02\x08\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x033\t\x15\n\x0c\n\x05\x04\
    \x02\x02\x01\x03\x12\x033\x18\x19\nY\n\x04\x04\x02\x02\x02\x12\x036\x02!\
    \x1aL\x20Longform\x20readable\x20description\x20of\x20the\x20summary\x20\
    sequence.\x20Markdown\x20supported.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\
    \x046\x023\x1a\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x036\x02\x08\n\x0c\n\
    \x05\x04\x02\x02\x02\x01\x12\x036\t\x1c\n\x0c\n\x05\x04\x02\x02\x02\x03\
    \x12\x036\x1f\x20\n\x96\x02\n\x02\x04\x03\x12\x04?\0{\x01\x1a\x89\x02\
    \x20A\x20Summary\x20is\x20a\x20set\x20of\x20named\x20values\x20to\x20be\
    \x20displayed\x20by\x20the\n\x20visualizer.\n\n\x20Summaries\x20are\x20p\
    roduced\x20regularly\x20during\x20training,\x20as\x20controlled\x20by\n\
    \x20the\x20\"summary_interval_secs\"\x20attribute\x20of\x20the\x20traini\
    ng\x20operation.\n\x20Summaries\x20are\x20also\x20produced\x20at\x20the\
    \x20end\x20of\x20an\x20evaluation.\n\n\n\n\x03\x04\x03\x01\x12\x03?\x08\
    \x0f\n\x0c\n\x04\x04\x03\x03\0\x12\x04@\x02O\x03\n\x0c\n\x05\x04\x03\x03\
    \0\x01\x12\x03@\n\x0f\n)\n\x06\x04\x03\x03\0\x02\0\x12\x03B\x04\x15\x1a\
    \x1a\x20Dimensions\x20of\x20the\x20image.\n\n\x0f\n\x07\x04\x03\x03\0\
    \x02\0\x04\x12\x04B\x04@\x11\n\x0e\n\x07\x04\x03\x03\0\x02\0\x05\x12\x03\
    B\x04\t\n\x0e\n\x07\x04\x03\x03\0\x02\0\x01\x12\x03B\n\x10\n\x0e\n\x07\
    \x04\x03\x03\0\x02\0\x03\x12\x03B\x13\x14\n\r\n\x06\x04\x03\x03\0\x02\
    \x01\x12\x03C\x04\x14\n\x0f\n\x07\x04\x03\x03\0\x02\x01\x04\x12\x04C\x04\
    B\x15\n\x0e\n\x07\x04\x03\x03\0\x02\x01\x05\x12\x03C\x04\t\n\x0e\n\x07\
    \x04\x03\x03\0\x02\x01\x01\x12\x03C\n\x0f\n\x0e\n\x07\x04\x03\x03\0\x02\
    \x01\x03\x12\x03C\x12\x13\n\x8c\x01\n\x06\x04\x03\x03\0\x02\x02\x12\x03K\
    \x04\x19\x1a}\x20Valid\x20colorspace\x20values\x20are\n\x20\x20\x201\x20\
    -\x20grayscale\n\x20\x20\x202\x20-\x20grayscale\x20+\x20alpha\n\x20\x20\
    \x203\x20-\x20RGB\n\x20\x20\x204\x20-\x20RGBA\n\x20\x20\x205\x20-\x20DIG\
    ITAL_YUV\n\x20\x20\x206\x20-\x20BGRA\n\n\x0f\n\x07\x04\x03\x03\0\x02\x02\
    \x04\x12\x04K\x04C\x14\n\x0e\n\x07\x04\x03\x03\0\x02\x02\x05\x12\x03K\
    \x04\t\n\x0e\n\x07\x04\x03\x03\0\x02\x02\x01\x12\x03K\n\x14\n\x0e\n\x07\
    \x04\x03\x03\0\x02\x02\x03\x12\x03K\x17\x18\nz\n\x06\x04\x03\x03\0\x02\
    \x03\x12\x03N\x04#\x1ak\x20Image\x20data\x20in\x20encoded\x20format.\x20\
    \x20All\x20image\x20formats\x20supported\x20by\n\x20image_codec::CoderUt\
    il\x20can\x20be\x20stored\x20here.\n\n\x0f\n\x07\x04\x03\x03\0\x02\x03\
    \x04\x12\x04N\x04K\x19\n\x0e\n\x07\x04\x03\x03\0\x02\x03\x05\x12\x03N\
    \x04\t\n\x0e\n\x07\x04\x03\x03\0\x02\x03\x01\x12\x03N\n\x1e\n\x0e\n\x07\
    \x04\x03\x03\0\x02\x03\x03\x12\x03N!\"\n\x0c\n\x04\x04\x03\x03\x01\x12\
    \x04Q\x02\\\x03\n\x0c\n\x05\x04\x03\x03\x01\x01\x12\x03Q\n\x0f\n0\n\x06\
    \x04\x03\x03\x01\x02\0\x12\x03S\x04\x1a\x1a!\x20Sample\x20rate\x20of\x20\
    the\x20audio\x20in\x20Hz.\n\n\x0f\n\x07\x04\x03\x03\x01\x02\0\x04\x12\
    \x04S\x04Q\x11\n\x0e\n\x07\x04\x03\x03\x01\x02\0\x05\x12\x03S\x04\t\n\
    \x0e\n\x07\x04\x03\x03\x01\x02\0\x01\x12\x03S\n\x15\n\x0e\n\x07\x04\x03\
    \x03\x01\x02\0\x03\x12\x03S\x18\x19\n-\n\x06\x04\x03\x03\x01\x02\x01\x12\
    \x03U\x04\x1b\x1a\x1e\x20Number\x20of\x20channels\x20of\x20audio.\n\n\
    \x0f\n\x07\x04\x03\x03\x01\x02\x01\x04\x12\x04U\x04S\x1a\n\x0e\n\x07\x04\
    \x03\x03\x01\x02\x01\x05\x12\x03U\x04\t\n\x0e\n\x07\x04\x03\x03\x01\x02\
    \x01\x01\x12\x03U\n\x16\n\x0e\n\x07\x04\x03\x03\x01\x02\x01\x03\x12\x03U\
    \x19\x1a\nE\n\x06\x04\x03\x03\x01\x02\x02\x12\x03W\x04\x1c\x1a6\x20Lengt\
    h\x20of\x20the\x20audio\x20in\x20frames\x20(samples\x20per\x20channel).\
    \n\n\x0f\n\x07\x04\x03\x03\x01\x02\x02\x04\x12\x04W\x04U\x1b\n\x0e\n\x07\
    \x04\x03\x03\x01\x02\x02\x05\x12\x03W\x04\t\n\x0e\n\x07\x04\x03\x03\x01\
    \x02\x02\x01\x12\x03W\n\x17\n\x0e\n\x07\x04\x03\x03\x01\x02\x02\x03\x12\
    \x03W\x1a\x1b\na\n\x06\x04\x03\x03\x01\x02\x03\x12\x03Z\x04#\x1aR\x20Enc\
    oded\x20audio\x20data\x20and\x20its\x20associated\x20RFC\x202045\x20cont\
    ent\x20type\x20(e.g.\n\x20\"audio/wav\").\n\n\x0f\n\x07\x04\x03\x03\x01\
    \x02\x03\x04\x12\x04Z\x04W\x1c\n\x0e\n\x07\x04\x03\x03\x01\x02\x03\x05\
    \x12\x03Z\x04\t\n\x0e\n\x07\x04\x03\x03\x01\x02\x03\x01\x12\x03Z\n\x1e\n\
    \x0e\n\x07\x04\x03\x03\x01\x02\x03\x03\x12\x03Z!\"\n\r\n\x06\x04\x03\x03\
    \x01\x02\x04\x12\x03[\x04\x1c\n\x0f\n\x07\x04\x03\x03\x01\x02\x04\x04\
    \x12\x04[\x04Z#\n\x0e\n\x07\x04\x03\x03\x01\x02\x04\x05\x12\x03[\x04\n\n\
    \x0e\n\x07\x04\x03\x03\x01\x02\x04\x01\x12\x03[\x0b\x17\n\x0e\n\x07\x04\
    \x03\x03\x01\x02\x04\x03\x12\x03[\x1a\x1b\n\x0c\n\x04\x04\x03\x03\x02\
    \x12\x04^\x02w\x03\n\x0c\n\x05\x04\x03\x03\x02\x01\x12\x03^\n\x0f\n>\n\
    \x06\x04\x03\x03\x02\x02\0\x12\x03`\x04\x19\x1a/\x20This\x20field\x20is\
    \x20deprecated\x20and\x20will\x20not\x20be\x20set.\n\n\x0f\n\x07\x04\x03\
    \x03\x02\x02\0\x04\x12\x04`\x04^\x11\n\x0e\n\x07\x04\x03\x03\x02\x02\0\
    \x05\x12\x03`\x04\n\n\x0e\n\x07\x04\x03\x03\x02\x02\0\x01\x12\x03`\x0b\
    \x14\n\x0e\n\x07\x04\x03\x03\x02\x02\0\x03\x12\x03`\x17\x18\n\xbf\x01\n\
    \x06\x04\x03\x03\x02\x02\x01\x12\x03e\x04\x13\x1a\xaf\x01\x20Tag\x20name\
    \x20for\x20the\x20data.\x20Used\x20by\x20TensorBoard\x20plugins\x20to\
    \x20organize\x20data.\x20Tags\n\x20are\x20often\x20organized\x20by\x20sc\
    ope\x20(which\x20contains\x20slashes\x20to\x20convey\n\x20hierarchy).\
    \x20For\x20example:\x20foo/bar/0\n\n\x0f\n\x07\x04\x03\x03\x02\x02\x01\
    \x04\x12\x04e\x04`\x19\n\x0e\n\x07\x04\x03\x03\x02\x02\x01\x05\x12\x03e\
    \x04\n\n\x0e\n\x07\x04\x03\x03\x02\x02\x01\x01\x12\x03e\x0b\x0e\n\x0e\n\
    \x07\x04\x03\x03\x02\x02\x01\x03\x12\x03e\x11\x12\n\xef\x02\n\x06\x04\
    \x03\x03\x02\x02\x02\x12\x03l\x04!\x1a\xdf\x02\x20Contains\x20metadata\
    \x20on\x20the\x20summary\x20value\x20such\x20as\x20which\x20plugins\x20m\
    ay\x20use\x20it.\n\x20Take\x20note\x20that\x20many\x20summary\x20values\
    \x20may\x20lack\x20a\x20metadata\x20field.\x20This\x20is\n\x20because\
    \x20the\x20FileWriter\x20only\x20keeps\x20a\x20metadata\x20object\x20on\
    \x20the\x20first\x20summary\n\x20value\x20with\x20a\x20certain\x20tag\
    \x20for\x20each\x20tag.\x20TensorBoard\x20then\x20remembers\x20which\n\
    \x20tags\x20are\x20associated\x20with\x20which\x20plugins.\x20This\x20sa\
    ves\x20space.\n\n\x0f\n\x07\x04\x03\x03\x02\x02\x02\x04\x12\x04l\x04e\
    \x13\n\x0e\n\x07\x04\x03\x03\x02\x02\x02\x06\x12\x03l\x04\x13\n\x0e\n\
    \x07\x04\x03\x03\x02\x02\x02\x01\x12\x03l\x14\x1c\n\x0e\n\x07\x04\x03\
    \x03\x02\x02\x02\x03\x12\x03l\x1f\x20\n0\n\x06\x04\x03\x03\x02\x08\0\x12\
    \x04o\x04v\x05\x1a\x20\x20Value\x20associated\x20with\x20the\x20tag.\n\n\
    \x0e\n\x07\x04\x03\x03\x02\x08\0\x01\x12\x03o\n\x0f\n\r\n\x06\x04\x03\
    \x03\x02\x02\x03\x12\x03p\x06\x1d\n\x0e\n\x07\x04\x03\x03\x02\x02\x03\
    \x05\x12\x03p\x06\x0b\n\x0e\n\x07\x04\x03\x03\x02\x02\x03\x01\x12\x03p\
    \x0c\x18\n\x0e\n\x07\x04\x03\x03\x02\x02\x03\x03\x12\x03p\x1b\x1c\n\r\n\
    \x06\x04\x03\x03\x02\x02\x04\x12\x03q\x06-\n\x0e\n\x07\x04\x03\x03\x02\
    \x02\x04\x05\x12\x03q\x06\x0b\n\x0e\n\x07\x04\x03\x03\x02\x02\x04\x01\
    \x12\x03q\x0c(\n\x0e\n\x07\x04\x03\x03\x02\x02\x04\x03\x12\x03q+,\n\r\n\
    \x06\x04\x03\x03\x02\x02\x05\x12\x03r\x06\x16\n\x0e\n\x07\x04\x03\x03\
    \x02\x02\x05\x06\x12\x03r\x06\x0b\n\x0e\n\x07\x04\x03\x03\x02\x02\x05\
    \x01\x12\x03r\x0c\x11\n\x0e\n\x07\x04\x03\x03\x02\x02\x05\x03\x12\x03r\
    \x14\x15\n\r\n\x06\x04\x03\x03\x02\x02\x06\x12\x03s\x06\x1f\n\x0e\n\x07\
    \x04\x03\x03\x02\x02\x06\x06\x12\x03s\x06\x14\n\x0e\n\x07\x04\x03\x03\
    \x02\x02\x06\x01\x12\x03s\x15\x1a\n\x0e\n\x07\x04\x03\x03\x02\x02\x06\
    \x03\x12\x03s\x1d\x1e\n\r\n\x06\x04\x03\x03\x02\x02\x07\x12\x03t\x06\x16\
    \n\x0e\n\x07\x04\x03\x03\x02\x02\x07\x06\x12\x03t\x06\x0b\n\x0e\n\x07\
    \x04\x03\x03\x02\x02\x07\x01\x12\x03t\x0c\x11\n\x0e\n\x07\x04\x03\x03\
    \x02\x02\x07\x03\x12\x03t\x14\x15\n\r\n\x06\x04\x03\x03\x02\x02\x08\x12\
    \x03u\x06\x1d\n\x0e\n\x07\x04\x03\x03\x02\x02\x08\x06\x12\x03u\x06\x11\n\
    \x0e\n\x07\x04\x03\x03\x02\x02\x08\x01\x12\x03u\x12\x18\n\x0e\n\x07\x04\
    \x03\x03\x02\x02\x08\x03\x12\x03u\x1b\x1c\n-\n\x04\x04\x03\x02\0\x12\x03\
    z\x02\x1b\x1a\x20\x20Set\x20of\x20values\x20for\x20the\x20summary.\n\n\
    \x0c\n\x05\x04\x03\x02\0\x04\x12\x03z\x02\n\n\x0c\n\x05\x04\x03\x02\0\
    \x06\x12\x03z\x0b\x10\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03z\x11\x16\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03z\x19\x1ab\x06proto3\
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
