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
pub struct MemoryLogStep {
    // message fields
    pub step_id: i64,
    pub handle: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemoryLogStep {}

impl MemoryLogStep {
    pub fn new() -> MemoryLogStep {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemoryLogStep {
        static mut instance: ::protobuf::lazy::Lazy<MemoryLogStep> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemoryLogStep,
        };
        unsafe {
            instance.get(MemoryLogStep::new)
        }
    }

    // int64 step_id = 1;

    pub fn clear_step_id(&mut self) {
        self.step_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_step_id(&mut self, v: i64) {
        self.step_id = v;
    }

    pub fn get_step_id(&self) -> i64 {
        self.step_id
    }

    fn get_step_id_for_reflect(&self) -> &i64 {
        &self.step_id
    }

    fn mut_step_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.step_id
    }

    // string handle = 2;

    pub fn clear_handle(&mut self) {
        self.handle.clear();
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: ::std::string::String) {
        self.handle = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_handle(&mut self) -> &mut ::std::string::String {
        &mut self.handle
    }

    // Take field
    pub fn take_handle(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.handle, ::std::string::String::new())
    }

    pub fn get_handle(&self) -> &str {
        &self.handle
    }

    fn get_handle_for_reflect(&self) -> &::std::string::String {
        &self.handle
    }

    fn mut_handle_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.handle
    }
}

impl ::protobuf::Message for MemoryLogStep {
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
                    self.step_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.handle)?;
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
        if self.step_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.step_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.handle.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.handle);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.step_id != 0 {
            os.write_int64(1, self.step_id)?;
        }
        if !self.handle.is_empty() {
            os.write_string(2, &self.handle)?;
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

impl ::protobuf::MessageStatic for MemoryLogStep {
    fn new() -> MemoryLogStep {
        MemoryLogStep::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemoryLogStep>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "step_id",
                    MemoryLogStep::get_step_id_for_reflect,
                    MemoryLogStep::mut_step_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "handle",
                    MemoryLogStep::get_handle_for_reflect,
                    MemoryLogStep::mut_handle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemoryLogStep>(
                    "MemoryLogStep",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemoryLogStep {
    fn clear(&mut self) {
        self.clear_step_id();
        self.clear_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemoryLogStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemoryLogStep {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemoryLogTensorAllocation {
    // message fields
    pub step_id: i64,
    pub kernel_name: ::std::string::String,
    pub tensor: ::protobuf::SingularPtrField<super::tensor_description::TensorDescription>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemoryLogTensorAllocation {}

impl MemoryLogTensorAllocation {
    pub fn new() -> MemoryLogTensorAllocation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemoryLogTensorAllocation {
        static mut instance: ::protobuf::lazy::Lazy<MemoryLogTensorAllocation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemoryLogTensorAllocation,
        };
        unsafe {
            instance.get(MemoryLogTensorAllocation::new)
        }
    }

    // int64 step_id = 1;

    pub fn clear_step_id(&mut self) {
        self.step_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_step_id(&mut self, v: i64) {
        self.step_id = v;
    }

    pub fn get_step_id(&self) -> i64 {
        self.step_id
    }

    fn get_step_id_for_reflect(&self) -> &i64 {
        &self.step_id
    }

    fn mut_step_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.step_id
    }

    // string kernel_name = 2;

    pub fn clear_kernel_name(&mut self) {
        self.kernel_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_kernel_name(&mut self, v: ::std::string::String) {
        self.kernel_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kernel_name(&mut self) -> &mut ::std::string::String {
        &mut self.kernel_name
    }

    // Take field
    pub fn take_kernel_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.kernel_name, ::std::string::String::new())
    }

    pub fn get_kernel_name(&self) -> &str {
        &self.kernel_name
    }

    fn get_kernel_name_for_reflect(&self) -> &::std::string::String {
        &self.kernel_name
    }

    fn mut_kernel_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.kernel_name
    }

    // .tensorflow.TensorDescription tensor = 3;

    pub fn clear_tensor(&mut self) {
        self.tensor.clear();
    }

    pub fn has_tensor(&self) -> bool {
        self.tensor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor(&mut self, v: super::tensor_description::TensorDescription) {
        self.tensor = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor(&mut self) -> &mut super::tensor_description::TensorDescription {
        if self.tensor.is_none() {
            self.tensor.set_default();
        }
        self.tensor.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor(&mut self) -> super::tensor_description::TensorDescription {
        self.tensor.take().unwrap_or_else(|| super::tensor_description::TensorDescription::new())
    }

    pub fn get_tensor(&self) -> &super::tensor_description::TensorDescription {
        self.tensor.as_ref().unwrap_or_else(|| super::tensor_description::TensorDescription::default_instance())
    }

    fn get_tensor_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_description::TensorDescription> {
        &self.tensor
    }

    fn mut_tensor_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_description::TensorDescription> {
        &mut self.tensor
    }
}

impl ::protobuf::Message for MemoryLogTensorAllocation {
    fn is_initialized(&self) -> bool {
        for v in &self.tensor {
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
                    let tmp = is.read_int64()?;
                    self.step_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.kernel_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor)?;
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
        if self.step_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.step_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.kernel_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.kernel_name);
        }
        if let Some(ref v) = self.tensor.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.step_id != 0 {
            os.write_int64(1, self.step_id)?;
        }
        if !self.kernel_name.is_empty() {
            os.write_string(2, &self.kernel_name)?;
        }
        if let Some(ref v) = self.tensor.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MemoryLogTensorAllocation {
    fn new() -> MemoryLogTensorAllocation {
        MemoryLogTensorAllocation::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemoryLogTensorAllocation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "step_id",
                    MemoryLogTensorAllocation::get_step_id_for_reflect,
                    MemoryLogTensorAllocation::mut_step_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "kernel_name",
                    MemoryLogTensorAllocation::get_kernel_name_for_reflect,
                    MemoryLogTensorAllocation::mut_kernel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_description::TensorDescription>>(
                    "tensor",
                    MemoryLogTensorAllocation::get_tensor_for_reflect,
                    MemoryLogTensorAllocation::mut_tensor_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemoryLogTensorAllocation>(
                    "MemoryLogTensorAllocation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemoryLogTensorAllocation {
    fn clear(&mut self) {
        self.clear_step_id();
        self.clear_kernel_name();
        self.clear_tensor();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemoryLogTensorAllocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemoryLogTensorAllocation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemoryLogTensorDeallocation {
    // message fields
    pub allocation_id: i64,
    pub allocator_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemoryLogTensorDeallocation {}

impl MemoryLogTensorDeallocation {
    pub fn new() -> MemoryLogTensorDeallocation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemoryLogTensorDeallocation {
        static mut instance: ::protobuf::lazy::Lazy<MemoryLogTensorDeallocation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemoryLogTensorDeallocation,
        };
        unsafe {
            instance.get(MemoryLogTensorDeallocation::new)
        }
    }

    // int64 allocation_id = 1;

    pub fn clear_allocation_id(&mut self) {
        self.allocation_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_allocation_id(&mut self, v: i64) {
        self.allocation_id = v;
    }

    pub fn get_allocation_id(&self) -> i64 {
        self.allocation_id
    }

    fn get_allocation_id_for_reflect(&self) -> &i64 {
        &self.allocation_id
    }

    fn mut_allocation_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.allocation_id
    }

    // string allocator_name = 2;

    pub fn clear_allocator_name(&mut self) {
        self.allocator_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_allocator_name(&mut self, v: ::std::string::String) {
        self.allocator_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allocator_name(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }

    // Take field
    pub fn take_allocator_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.allocator_name, ::std::string::String::new())
    }

    pub fn get_allocator_name(&self) -> &str {
        &self.allocator_name
    }

    fn get_allocator_name_for_reflect(&self) -> &::std::string::String {
        &self.allocator_name
    }

    fn mut_allocator_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }
}

impl ::protobuf::Message for MemoryLogTensorDeallocation {
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
                    self.allocation_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allocator_name)?;
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
        if self.allocation_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.allocation_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.allocator_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.allocator_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.allocation_id != 0 {
            os.write_int64(1, self.allocation_id)?;
        }
        if !self.allocator_name.is_empty() {
            os.write_string(2, &self.allocator_name)?;
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

impl ::protobuf::MessageStatic for MemoryLogTensorDeallocation {
    fn new() -> MemoryLogTensorDeallocation {
        MemoryLogTensorDeallocation::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemoryLogTensorDeallocation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "allocation_id",
                    MemoryLogTensorDeallocation::get_allocation_id_for_reflect,
                    MemoryLogTensorDeallocation::mut_allocation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allocator_name",
                    MemoryLogTensorDeallocation::get_allocator_name_for_reflect,
                    MemoryLogTensorDeallocation::mut_allocator_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemoryLogTensorDeallocation>(
                    "MemoryLogTensorDeallocation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemoryLogTensorDeallocation {
    fn clear(&mut self) {
        self.clear_allocation_id();
        self.clear_allocator_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemoryLogTensorDeallocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemoryLogTensorDeallocation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemoryLogTensorOutput {
    // message fields
    pub step_id: i64,
    pub kernel_name: ::std::string::String,
    pub index: i32,
    pub tensor: ::protobuf::SingularPtrField<super::tensor_description::TensorDescription>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemoryLogTensorOutput {}

impl MemoryLogTensorOutput {
    pub fn new() -> MemoryLogTensorOutput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemoryLogTensorOutput {
        static mut instance: ::protobuf::lazy::Lazy<MemoryLogTensorOutput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemoryLogTensorOutput,
        };
        unsafe {
            instance.get(MemoryLogTensorOutput::new)
        }
    }

    // int64 step_id = 1;

    pub fn clear_step_id(&mut self) {
        self.step_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_step_id(&mut self, v: i64) {
        self.step_id = v;
    }

    pub fn get_step_id(&self) -> i64 {
        self.step_id
    }

    fn get_step_id_for_reflect(&self) -> &i64 {
        &self.step_id
    }

    fn mut_step_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.step_id
    }

    // string kernel_name = 2;

    pub fn clear_kernel_name(&mut self) {
        self.kernel_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_kernel_name(&mut self, v: ::std::string::String) {
        self.kernel_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kernel_name(&mut self) -> &mut ::std::string::String {
        &mut self.kernel_name
    }

    // Take field
    pub fn take_kernel_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.kernel_name, ::std::string::String::new())
    }

    pub fn get_kernel_name(&self) -> &str {
        &self.kernel_name
    }

    fn get_kernel_name_for_reflect(&self) -> &::std::string::String {
        &self.kernel_name
    }

    fn mut_kernel_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.kernel_name
    }

    // int32 index = 3;

    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i32) {
        self.index = v;
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    fn get_index_for_reflect(&self) -> &i32 {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut i32 {
        &mut self.index
    }

    // .tensorflow.TensorDescription tensor = 4;

    pub fn clear_tensor(&mut self) {
        self.tensor.clear();
    }

    pub fn has_tensor(&self) -> bool {
        self.tensor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor(&mut self, v: super::tensor_description::TensorDescription) {
        self.tensor = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor(&mut self) -> &mut super::tensor_description::TensorDescription {
        if self.tensor.is_none() {
            self.tensor.set_default();
        }
        self.tensor.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor(&mut self) -> super::tensor_description::TensorDescription {
        self.tensor.take().unwrap_or_else(|| super::tensor_description::TensorDescription::new())
    }

    pub fn get_tensor(&self) -> &super::tensor_description::TensorDescription {
        self.tensor.as_ref().unwrap_or_else(|| super::tensor_description::TensorDescription::default_instance())
    }

    fn get_tensor_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_description::TensorDescription> {
        &self.tensor
    }

    fn mut_tensor_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_description::TensorDescription> {
        &mut self.tensor
    }
}

impl ::protobuf::Message for MemoryLogTensorOutput {
    fn is_initialized(&self) -> bool {
        for v in &self.tensor {
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
                    let tmp = is.read_int64()?;
                    self.step_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.kernel_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.index = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor)?;
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
        if self.step_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.step_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.kernel_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.kernel_name);
        }
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(3, self.index, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.tensor.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.step_id != 0 {
            os.write_int64(1, self.step_id)?;
        }
        if !self.kernel_name.is_empty() {
            os.write_string(2, &self.kernel_name)?;
        }
        if self.index != 0 {
            os.write_int32(3, self.index)?;
        }
        if let Some(ref v) = self.tensor.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MemoryLogTensorOutput {
    fn new() -> MemoryLogTensorOutput {
        MemoryLogTensorOutput::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemoryLogTensorOutput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "step_id",
                    MemoryLogTensorOutput::get_step_id_for_reflect,
                    MemoryLogTensorOutput::mut_step_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "kernel_name",
                    MemoryLogTensorOutput::get_kernel_name_for_reflect,
                    MemoryLogTensorOutput::mut_kernel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "index",
                    MemoryLogTensorOutput::get_index_for_reflect,
                    MemoryLogTensorOutput::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_description::TensorDescription>>(
                    "tensor",
                    MemoryLogTensorOutput::get_tensor_for_reflect,
                    MemoryLogTensorOutput::mut_tensor_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemoryLogTensorOutput>(
                    "MemoryLogTensorOutput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemoryLogTensorOutput {
    fn clear(&mut self) {
        self.clear_step_id();
        self.clear_kernel_name();
        self.clear_index();
        self.clear_tensor();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemoryLogTensorOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemoryLogTensorOutput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemoryLogRawAllocation {
    // message fields
    pub step_id: i64,
    pub operation: ::std::string::String,
    pub num_bytes: i64,
    pub ptr: u64,
    pub allocation_id: i64,
    pub allocator_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemoryLogRawAllocation {}

impl MemoryLogRawAllocation {
    pub fn new() -> MemoryLogRawAllocation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemoryLogRawAllocation {
        static mut instance: ::protobuf::lazy::Lazy<MemoryLogRawAllocation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemoryLogRawAllocation,
        };
        unsafe {
            instance.get(MemoryLogRawAllocation::new)
        }
    }

    // int64 step_id = 1;

    pub fn clear_step_id(&mut self) {
        self.step_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_step_id(&mut self, v: i64) {
        self.step_id = v;
    }

    pub fn get_step_id(&self) -> i64 {
        self.step_id
    }

    fn get_step_id_for_reflect(&self) -> &i64 {
        &self.step_id
    }

    fn mut_step_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.step_id
    }

    // string operation = 2;

    pub fn clear_operation(&mut self) {
        self.operation.clear();
    }

    // Param is passed by value, moved
    pub fn set_operation(&mut self, v: ::std::string::String) {
        self.operation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operation(&mut self) -> &mut ::std::string::String {
        &mut self.operation
    }

    // Take field
    pub fn take_operation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.operation, ::std::string::String::new())
    }

    pub fn get_operation(&self) -> &str {
        &self.operation
    }

    fn get_operation_for_reflect(&self) -> &::std::string::String {
        &self.operation
    }

    fn mut_operation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.operation
    }

    // int64 num_bytes = 3;

    pub fn clear_num_bytes(&mut self) {
        self.num_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_bytes(&mut self, v: i64) {
        self.num_bytes = v;
    }

    pub fn get_num_bytes(&self) -> i64 {
        self.num_bytes
    }

    fn get_num_bytes_for_reflect(&self) -> &i64 {
        &self.num_bytes
    }

    fn mut_num_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.num_bytes
    }

    // uint64 ptr = 4;

    pub fn clear_ptr(&mut self) {
        self.ptr = 0;
    }

    // Param is passed by value, moved
    pub fn set_ptr(&mut self, v: u64) {
        self.ptr = v;
    }

    pub fn get_ptr(&self) -> u64 {
        self.ptr
    }

    fn get_ptr_for_reflect(&self) -> &u64 {
        &self.ptr
    }

    fn mut_ptr_for_reflect(&mut self) -> &mut u64 {
        &mut self.ptr
    }

    // int64 allocation_id = 5;

    pub fn clear_allocation_id(&mut self) {
        self.allocation_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_allocation_id(&mut self, v: i64) {
        self.allocation_id = v;
    }

    pub fn get_allocation_id(&self) -> i64 {
        self.allocation_id
    }

    fn get_allocation_id_for_reflect(&self) -> &i64 {
        &self.allocation_id
    }

    fn mut_allocation_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.allocation_id
    }

    // string allocator_name = 6;

    pub fn clear_allocator_name(&mut self) {
        self.allocator_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_allocator_name(&mut self, v: ::std::string::String) {
        self.allocator_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allocator_name(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }

    // Take field
    pub fn take_allocator_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.allocator_name, ::std::string::String::new())
    }

    pub fn get_allocator_name(&self) -> &str {
        &self.allocator_name
    }

    fn get_allocator_name_for_reflect(&self) -> &::std::string::String {
        &self.allocator_name
    }

    fn mut_allocator_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }
}

impl ::protobuf::Message for MemoryLogRawAllocation {
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
                    self.step_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.operation)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.num_bytes = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ptr = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.allocation_id = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allocator_name)?;
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
        if self.step_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.step_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.operation.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.operation);
        }
        if self.num_bytes != 0 {
            my_size += ::protobuf::rt::value_size(3, self.num_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ptr != 0 {
            my_size += ::protobuf::rt::value_size(4, self.ptr, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.allocation_id != 0 {
            my_size += ::protobuf::rt::value_size(5, self.allocation_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.allocator_name.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.allocator_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.step_id != 0 {
            os.write_int64(1, self.step_id)?;
        }
        if !self.operation.is_empty() {
            os.write_string(2, &self.operation)?;
        }
        if self.num_bytes != 0 {
            os.write_int64(3, self.num_bytes)?;
        }
        if self.ptr != 0 {
            os.write_uint64(4, self.ptr)?;
        }
        if self.allocation_id != 0 {
            os.write_int64(5, self.allocation_id)?;
        }
        if !self.allocator_name.is_empty() {
            os.write_string(6, &self.allocator_name)?;
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

impl ::protobuf::MessageStatic for MemoryLogRawAllocation {
    fn new() -> MemoryLogRawAllocation {
        MemoryLogRawAllocation::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemoryLogRawAllocation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "step_id",
                    MemoryLogRawAllocation::get_step_id_for_reflect,
                    MemoryLogRawAllocation::mut_step_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "operation",
                    MemoryLogRawAllocation::get_operation_for_reflect,
                    MemoryLogRawAllocation::mut_operation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "num_bytes",
                    MemoryLogRawAllocation::get_num_bytes_for_reflect,
                    MemoryLogRawAllocation::mut_num_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ptr",
                    MemoryLogRawAllocation::get_ptr_for_reflect,
                    MemoryLogRawAllocation::mut_ptr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "allocation_id",
                    MemoryLogRawAllocation::get_allocation_id_for_reflect,
                    MemoryLogRawAllocation::mut_allocation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allocator_name",
                    MemoryLogRawAllocation::get_allocator_name_for_reflect,
                    MemoryLogRawAllocation::mut_allocator_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemoryLogRawAllocation>(
                    "MemoryLogRawAllocation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemoryLogRawAllocation {
    fn clear(&mut self) {
        self.clear_step_id();
        self.clear_operation();
        self.clear_num_bytes();
        self.clear_ptr();
        self.clear_allocation_id();
        self.clear_allocator_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemoryLogRawAllocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemoryLogRawAllocation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemoryLogRawDeallocation {
    // message fields
    pub step_id: i64,
    pub operation: ::std::string::String,
    pub allocation_id: i64,
    pub allocator_name: ::std::string::String,
    pub deferred: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemoryLogRawDeallocation {}

impl MemoryLogRawDeallocation {
    pub fn new() -> MemoryLogRawDeallocation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemoryLogRawDeallocation {
        static mut instance: ::protobuf::lazy::Lazy<MemoryLogRawDeallocation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemoryLogRawDeallocation,
        };
        unsafe {
            instance.get(MemoryLogRawDeallocation::new)
        }
    }

    // int64 step_id = 1;

    pub fn clear_step_id(&mut self) {
        self.step_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_step_id(&mut self, v: i64) {
        self.step_id = v;
    }

    pub fn get_step_id(&self) -> i64 {
        self.step_id
    }

    fn get_step_id_for_reflect(&self) -> &i64 {
        &self.step_id
    }

    fn mut_step_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.step_id
    }

    // string operation = 2;

    pub fn clear_operation(&mut self) {
        self.operation.clear();
    }

    // Param is passed by value, moved
    pub fn set_operation(&mut self, v: ::std::string::String) {
        self.operation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operation(&mut self) -> &mut ::std::string::String {
        &mut self.operation
    }

    // Take field
    pub fn take_operation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.operation, ::std::string::String::new())
    }

    pub fn get_operation(&self) -> &str {
        &self.operation
    }

    fn get_operation_for_reflect(&self) -> &::std::string::String {
        &self.operation
    }

    fn mut_operation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.operation
    }

    // int64 allocation_id = 3;

    pub fn clear_allocation_id(&mut self) {
        self.allocation_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_allocation_id(&mut self, v: i64) {
        self.allocation_id = v;
    }

    pub fn get_allocation_id(&self) -> i64 {
        self.allocation_id
    }

    fn get_allocation_id_for_reflect(&self) -> &i64 {
        &self.allocation_id
    }

    fn mut_allocation_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.allocation_id
    }

    // string allocator_name = 4;

    pub fn clear_allocator_name(&mut self) {
        self.allocator_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_allocator_name(&mut self, v: ::std::string::String) {
        self.allocator_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allocator_name(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }

    // Take field
    pub fn take_allocator_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.allocator_name, ::std::string::String::new())
    }

    pub fn get_allocator_name(&self) -> &str {
        &self.allocator_name
    }

    fn get_allocator_name_for_reflect(&self) -> &::std::string::String {
        &self.allocator_name
    }

    fn mut_allocator_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }

    // bool deferred = 5;

    pub fn clear_deferred(&mut self) {
        self.deferred = false;
    }

    // Param is passed by value, moved
    pub fn set_deferred(&mut self, v: bool) {
        self.deferred = v;
    }

    pub fn get_deferred(&self) -> bool {
        self.deferred
    }

    fn get_deferred_for_reflect(&self) -> &bool {
        &self.deferred
    }

    fn mut_deferred_for_reflect(&mut self) -> &mut bool {
        &mut self.deferred
    }
}

impl ::protobuf::Message for MemoryLogRawDeallocation {
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
                    self.step_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.operation)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.allocation_id = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allocator_name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deferred = tmp;
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
        if self.step_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.step_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.operation.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.operation);
        }
        if self.allocation_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.allocation_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.allocator_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.allocator_name);
        }
        if self.deferred != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.step_id != 0 {
            os.write_int64(1, self.step_id)?;
        }
        if !self.operation.is_empty() {
            os.write_string(2, &self.operation)?;
        }
        if self.allocation_id != 0 {
            os.write_int64(3, self.allocation_id)?;
        }
        if !self.allocator_name.is_empty() {
            os.write_string(4, &self.allocator_name)?;
        }
        if self.deferred != false {
            os.write_bool(5, self.deferred)?;
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

impl ::protobuf::MessageStatic for MemoryLogRawDeallocation {
    fn new() -> MemoryLogRawDeallocation {
        MemoryLogRawDeallocation::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemoryLogRawDeallocation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "step_id",
                    MemoryLogRawDeallocation::get_step_id_for_reflect,
                    MemoryLogRawDeallocation::mut_step_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "operation",
                    MemoryLogRawDeallocation::get_operation_for_reflect,
                    MemoryLogRawDeallocation::mut_operation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "allocation_id",
                    MemoryLogRawDeallocation::get_allocation_id_for_reflect,
                    MemoryLogRawDeallocation::mut_allocation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allocator_name",
                    MemoryLogRawDeallocation::get_allocator_name_for_reflect,
                    MemoryLogRawDeallocation::mut_allocator_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deferred",
                    MemoryLogRawDeallocation::get_deferred_for_reflect,
                    MemoryLogRawDeallocation::mut_deferred_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemoryLogRawDeallocation>(
                    "MemoryLogRawDeallocation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemoryLogRawDeallocation {
    fn clear(&mut self) {
        self.clear_step_id();
        self.clear_operation();
        self.clear_allocation_id();
        self.clear_allocator_name();
        self.clear_deferred();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemoryLogRawDeallocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemoryLogRawDeallocation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*tensorflow/core/framework/log_memory.proto\x12\ntensorflow\x1a2tensor\
    flow/core/framework/tensor_description.proto\"@\n\rMemoryLogStep\x12\x17\
    \n\x07step_id\x18\x01\x20\x01(\x03R\x06stepId\x12\x16\n\x06handle\x18\
    \x02\x20\x01(\tR\x06handle\"\x8c\x01\n\x19MemoryLogTensorAllocation\x12\
    \x17\n\x07step_id\x18\x01\x20\x01(\x03R\x06stepId\x12\x1f\n\x0bkernel_na\
    me\x18\x02\x20\x01(\tR\nkernelName\x125\n\x06tensor\x18\x03\x20\x01(\x0b\
    2\x1d.tensorflow.TensorDescriptionR\x06tensor\"i\n\x1bMemoryLogTensorDea\
    llocation\x12#\n\rallocation_id\x18\x01\x20\x01(\x03R\x0callocationId\
    \x12%\n\x0eallocator_name\x18\x02\x20\x01(\tR\rallocatorName\"\x9e\x01\n\
    \x15MemoryLogTensorOutput\x12\x17\n\x07step_id\x18\x01\x20\x01(\x03R\x06\
    stepId\x12\x1f\n\x0bkernel_name\x18\x02\x20\x01(\tR\nkernelName\x12\x14\
    \n\x05index\x18\x03\x20\x01(\x05R\x05index\x125\n\x06tensor\x18\x04\x20\
    \x01(\x0b2\x1d.tensorflow.TensorDescriptionR\x06tensor\"\xca\x01\n\x16Me\
    moryLogRawAllocation\x12\x17\n\x07step_id\x18\x01\x20\x01(\x03R\x06stepI\
    d\x12\x1c\n\toperation\x18\x02\x20\x01(\tR\toperation\x12\x1b\n\tnum_byt\
    es\x18\x03\x20\x01(\x03R\x08numBytes\x12\x10\n\x03ptr\x18\x04\x20\x01(\
    \x04R\x03ptr\x12#\n\rallocation_id\x18\x05\x20\x01(\x03R\x0callocationId\
    \x12%\n\x0eallocator_name\x18\x06\x20\x01(\tR\rallocatorName\"\xb9\x01\n\
    \x18MemoryLogRawDeallocation\x12\x17\n\x07step_id\x18\x01\x20\x01(\x03R\
    \x06stepId\x12\x1c\n\toperation\x18\x02\x20\x01(\tR\toperation\x12#\n\ra\
    llocation_id\x18\x03\x20\x01(\x03R\x0callocationId\x12%\n\x0eallocator_n\
    ame\x18\x04\x20\x01(\tR\rallocatorName\x12\x1a\n\x08deferred\x18\x05\x20\
    \x01(\x08R\x08deferredB0\n\x18org.tensorflow.frameworkB\x0fLogMemoryProt\
    osP\x01\xf8\x01\x01J\xdc\x18\n\x06\x12\x04\0\0\\\x02\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\
    \x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\
    \x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\
    \x04\00\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\00\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e/\n\x08\n\x01\x08\x12\x03\
    \x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\
    \x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\
    \x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\
    \x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\
    \x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\
    \x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\
    \x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\x03\0\x12\x03\
    \x08\x07;\n\n\n\x02\x04\0\x12\x04\n\0\x10\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\n\x08\x15\n&\n\x04\x04\0\x02\0\x12\x03\x0c\x02\x14\x1a\x19\x20Proce\
    ss-unique\x20step\x20id.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0c\x02\n\
    \x17\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0c\x02\x07\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x0c\x08\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\
    \x12\x13\nC\n\x04\x04\0\x02\x01\x12\x03\x0f\x02\x14\x1a6\x20Handle\x20de\
    scribing\x20the\x20feeds\x20and\x20fetches\x20of\x20the\x20step.\n\n\r\n\
    \x05\x04\0\x02\x01\x04\x12\x04\x0f\x02\x0c\x14\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0f\t\x0f\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0f\x12\x13\n\n\n\x02\x04\x01\x12\
    \x04\x12\0\x1c\x01\n\n\n\x03\x04\x01\x01\x12\x03\x12\x08!\n&\n\x04\x04\
    \x01\x02\0\x12\x03\x14\x02\x14\x1a\x19\x20Process-unique\x20step\x20id.\
    \n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x14\x02\x12#\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03\x14\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x14\
    \x08\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x14\x12\x13\nl\n\x04\x04\
    \x01\x02\x01\x12\x03\x18\x02\x19\x1a_\x20Name\x20of\x20the\x20kernel\x20\
    making\x20the\x20allocation\x20as\x20set\x20in\x20GraphDef,\n\x20e.g.,\
    \x20\"affine2/weights/Assign\".\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\
    \x18\x02\x14\x14\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x18\x02\x08\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x18\t\x14\n\x0c\n\x05\x04\x01\x02\
    \x01\x03\x12\x03\x18\x17\x18\n(\n\x04\x04\x01\x02\x02\x12\x03\x1b\x02\
    \x1f\x1a\x1b\x20Allocated\x20tensor\x20details.\n\n\r\n\x05\x04\x01\x02\
    \x02\x04\x12\x04\x1b\x02\x18\x19\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\
    \x1b\x02\x13\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1b\x14\x1a\n\x0c\n\
    \x05\x04\x01\x02\x02\x03\x12\x03\x1b\x1d\x1e\n\n\n\x02\x04\x02\x12\x04\
    \x1e\0%\x01\n\n\n\x03\x04\x02\x01\x12\x03\x1e\x08#\ng\n\x04\x04\x02\x02\
    \0\x12\x03!\x02\x1a\x1aZ\x20Id\x20of\x20the\x20tensor\x20buffer\x20being\
    \x20deallocated,\x20used\x20to\x20match\x20to\x20a\n\x20corresponding\
    \x20allocation.\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04!\x02\x1e%\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03!\x02\x07\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03!\x08\x15\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03!\x18\x19\n*\n\
    \x04\x04\x02\x02\x01\x12\x03$\x02\x1c\x1a\x1d\x20Name\x20of\x20the\x20al\
    locator\x20used.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04$\x02!\x1a\n\x0c\
    \n\x05\x04\x02\x02\x01\x05\x12\x03$\x02\x08\n\x0c\n\x05\x04\x02\x02\x01\
    \x01\x12\x03$\t\x17\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03$\x1a\x1b\n\n\
    \n\x02\x04\x03\x12\x04'\04\x01\n\n\n\x03\x04\x03\x01\x12\x03'\x08\x1d\n&\
    \n\x04\x04\x03\x02\0\x12\x03)\x02\x14\x1a\x19\x20Process-unique\x20step\
    \x20id.\n\n\r\n\x05\x04\x03\x02\0\x04\x12\x04)\x02'\x1f\n\x0c\n\x05\x04\
    \x03\x02\0\x05\x12\x03)\x02\x07\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03)\
    \x08\x0f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03)\x12\x13\nj\n\x04\x04\x03\
    \x02\x01\x12\x03-\x02\x19\x1a]\x20Name\x20of\x20the\x20kernel\x20produci\
    ng\x20an\x20output\x20as\x20set\x20in\x20GraphDef,\x20e.g.,\n\x20\"affin\
    e2/weights/Assign\".\n\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04-\x02)\x14\n\
    \x0c\n\x05\x04\x03\x02\x01\x05\x12\x03-\x02\x08\n\x0c\n\x05\x04\x03\x02\
    \x01\x01\x12\x03-\t\x14\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03-\x17\x18\
    \n-\n\x04\x04\x03\x02\x02\x12\x030\x02\x12\x1a\x20\x20Index\x20of\x20the\
    \x20output\x20being\x20set.\n\n\r\n\x05\x04\x03\x02\x02\x04\x12\x040\x02\
    -\x19\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x030\x02\x07\n\x0c\n\x05\x04\
    \x03\x02\x02\x01\x12\x030\x08\r\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x030\
    \x10\x11\n%\n\x04\x04\x03\x02\x03\x12\x033\x02\x1f\x1a\x18\x20Output\x20\
    tensor\x20details.\n\n\r\n\x05\x04\x03\x02\x03\x04\x12\x043\x020\x12\n\
    \x0c\n\x05\x04\x03\x02\x03\x06\x12\x033\x02\x13\n\x0c\n\x05\x04\x03\x02\
    \x03\x01\x12\x033\x14\x1a\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x033\x1d\
    \x1e\n\n\n\x02\x04\x04\x12\x046\0I\x01\n\n\n\x03\x04\x04\x01\x12\x036\
    \x08\x1e\n&\n\x04\x04\x04\x02\0\x12\x038\x02\x14\x1a\x19\x20Process-uniq\
    ue\x20step\x20id.\n\n\r\n\x05\x04\x04\x02\0\x04\x12\x048\x026\x20\n\x0c\
    \n\x05\x04\x04\x02\0\x05\x12\x038\x02\x07\n\x0c\n\x05\x04\x04\x02\0\x01\
    \x12\x038\x08\x0f\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x038\x12\x13\n;\n\
    \x04\x04\x04\x02\x01\x12\x03;\x02\x17\x1a.\x20Name\x20of\x20the\x20opera\
    tion\x20making\x20the\x20allocation.\n\n\r\n\x05\x04\x04\x02\x01\x04\x12\
    \x04;\x028\x14\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03;\x02\x08\n\x0c\n\
    \x05\x04\x04\x02\x01\x01\x12\x03;\t\x12\n\x0c\n\x05\x04\x04\x02\x01\x03\
    \x12\x03;\x15\x16\n1\n\x04\x04\x04\x02\x02\x12\x03>\x02\x16\x1a$\x20Numb\
    er\x20of\x20bytes\x20in\x20the\x20allocation.\n\n\r\n\x05\x04\x04\x02\
    \x02\x04\x12\x04>\x02;\x17\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03>\x02\
    \x07\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03>\x08\x11\n\x0c\n\x05\x04\
    \x04\x02\x02\x03\x12\x03>\x14\x15\n)\n\x04\x04\x04\x02\x03\x12\x03A\x02\
    \x11\x1a\x1c\x20Address\x20of\x20the\x20allocation.\n\n\r\n\x05\x04\x04\
    \x02\x03\x04\x12\x04A\x02>\x16\n\x0c\n\x05\x04\x04\x02\x03\x05\x12\x03A\
    \x02\x08\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03A\t\x0c\n\x0c\n\x05\x04\
    \x04\x02\x03\x03\x12\x03A\x0f\x10\ng\n\x04\x04\x04\x02\x04\x12\x03E\x02\
    \x1a\x1aZ\x20Id\x20of\x20the\x20tensor\x20buffer\x20being\x20allocated,\
    \x20used\x20to\x20match\x20to\x20a\n\x20corresponding\x20deallocation.\n\
    \n\r\n\x05\x04\x04\x02\x04\x04\x12\x04E\x02A\x11\n\x0c\n\x05\x04\x04\x02\
    \x04\x05\x12\x03E\x02\x07\n\x0c\n\x05\x04\x04\x02\x04\x01\x12\x03E\x08\
    \x15\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x03E\x18\x19\n*\n\x04\x04\x04\
    \x02\x05\x12\x03H\x02\x1c\x1a\x1d\x20Name\x20of\x20the\x20allocator\x20u\
    sed.\n\n\r\n\x05\x04\x04\x02\x05\x04\x12\x04H\x02E\x1a\n\x0c\n\x05\x04\
    \x04\x02\x05\x05\x12\x03H\x02\x08\n\x0c\n\x05\x04\x04\x02\x05\x01\x12\
    \x03H\t\x17\n\x0c\n\x05\x04\x04\x02\x05\x03\x12\x03H\x1a\x1b\n\n\n\x02\
    \x04\x05\x12\x04K\0\\\x01\n\n\n\x03\x04\x05\x01\x12\x03K\x08\x20\n&\n\
    \x04\x04\x05\x02\0\x12\x03M\x02\x14\x1a\x19\x20Process-unique\x20step\
    \x20id.\n\n\r\n\x05\x04\x05\x02\0\x04\x12\x04M\x02K\"\n\x0c\n\x05\x04\
    \x05\x02\0\x05\x12\x03M\x02\x07\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03M\
    \x08\x0f\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03M\x12\x13\n=\n\x04\x04\x05\
    \x02\x01\x12\x03P\x02\x17\x1a0\x20Name\x20of\x20the\x20operation\x20maki\
    ng\x20the\x20deallocation.\n\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04P\x02M\
    \x14\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03P\x02\x08\n\x0c\n\x05\x04\
    \x05\x02\x01\x01\x12\x03P\t\x12\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03P\
    \x15\x16\ng\n\x04\x04\x05\x02\x02\x12\x03T\x02\x1a\x1aZ\x20Id\x20of\x20t\
    he\x20tensor\x20buffer\x20being\x20deallocated,\x20used\x20to\x20match\
    \x20to\x20a\n\x20corresponding\x20allocation.\n\n\r\n\x05\x04\x05\x02\
    \x02\x04\x12\x04T\x02P\x17\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x03T\x02\
    \x07\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03T\x08\x15\n\x0c\n\x05\x04\
    \x05\x02\x02\x03\x12\x03T\x18\x19\n*\n\x04\x04\x05\x02\x03\x12\x03W\x02\
    \x1c\x1a\x1d\x20Name\x20of\x20the\x20allocator\x20used.\n\n\r\n\x05\x04\
    \x05\x02\x03\x04\x12\x04W\x02T\x1a\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\
    \x03W\x02\x08\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x03W\t\x17\n\x0c\n\x05\
    \x04\x05\x02\x03\x03\x12\x03W\x1a\x1b\nu\n\x04\x04\x05\x02\x04\x12\x03[\
    \x02\x14\x1ah\x20True\x20if\x20the\x20deallocation\x20is\x20queued\x20an\
    d\x20will\x20be\x20performed\x20later,\n\x20e.g.\x20for\x20GPU\x20lazy\
    \x20freeing\x20of\x20buffers.\n\n\r\n\x05\x04\x05\x02\x04\x04\x12\x04[\
    \x02W\x1c\n\x0c\n\x05\x04\x05\x02\x04\x05\x12\x03[\x02\x06\n\x0c\n\x05\
    \x04\x05\x02\x04\x01\x12\x03[\x07\x0f\n\x0c\n\x05\x04\x05\x02\x04\x03\
    \x12\x03[\x12\x13b\x06proto3\
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
