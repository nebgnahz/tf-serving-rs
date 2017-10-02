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
pub struct Example {
    // message fields
    pub features: ::protobuf::SingularPtrField<super::feature::Features>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Example {}

impl Example {
    pub fn new() -> Example {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Example {
        static mut instance: ::protobuf::lazy::Lazy<Example> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Example,
        };
        unsafe {
            instance.get(Example::new)
        }
    }

    // .tensorflow.Features features = 1;

    pub fn clear_features(&mut self) {
        self.features.clear();
    }

    pub fn has_features(&self) -> bool {
        self.features.is_some()
    }

    // Param is passed by value, moved
    pub fn set_features(&mut self, v: super::feature::Features) {
        self.features = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_features(&mut self) -> &mut super::feature::Features {
        if self.features.is_none() {
            self.features.set_default();
        }
        self.features.as_mut().unwrap()
    }

    // Take field
    pub fn take_features(&mut self) -> super::feature::Features {
        self.features.take().unwrap_or_else(|| super::feature::Features::new())
    }

    pub fn get_features(&self) -> &super::feature::Features {
        self.features.as_ref().unwrap_or_else(|| super::feature::Features::default_instance())
    }

    fn get_features_for_reflect(&self) -> &::protobuf::SingularPtrField<super::feature::Features> {
        &self.features
    }

    fn mut_features_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::feature::Features> {
        &mut self.features
    }
}

impl ::protobuf::Message for Example {
    fn is_initialized(&self) -> bool {
        for v in &self.features {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.features)?;
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
        if let Some(ref v) = self.features.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.features.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Example {
    fn new() -> Example {
        Example::new()
    }

    fn descriptor_static(_: ::std::option::Option<Example>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::feature::Features>>(
                    "features",
                    Example::get_features_for_reflect,
                    Example::mut_features_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Example>(
                    "Example",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Example {
    fn clear(&mut self) {
        self.clear_features();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Example {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Example {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SequenceExample {
    // message fields
    pub context: ::protobuf::SingularPtrField<super::feature::Features>,
    pub feature_lists: ::protobuf::SingularPtrField<super::feature::FeatureLists>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SequenceExample {}

impl SequenceExample {
    pub fn new() -> SequenceExample {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SequenceExample {
        static mut instance: ::protobuf::lazy::Lazy<SequenceExample> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SequenceExample,
        };
        unsafe {
            instance.get(SequenceExample::new)
        }
    }

    // .tensorflow.Features context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: super::feature::Features) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut super::feature::Features {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> super::feature::Features {
        self.context.take().unwrap_or_else(|| super::feature::Features::new())
    }

    pub fn get_context(&self) -> &super::feature::Features {
        self.context.as_ref().unwrap_or_else(|| super::feature::Features::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<super::feature::Features> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::feature::Features> {
        &mut self.context
    }

    // .tensorflow.FeatureLists feature_lists = 2;

    pub fn clear_feature_lists(&mut self) {
        self.feature_lists.clear();
    }

    pub fn has_feature_lists(&self) -> bool {
        self.feature_lists.is_some()
    }

    // Param is passed by value, moved
    pub fn set_feature_lists(&mut self, v: super::feature::FeatureLists) {
        self.feature_lists = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_feature_lists(&mut self) -> &mut super::feature::FeatureLists {
        if self.feature_lists.is_none() {
            self.feature_lists.set_default();
        }
        self.feature_lists.as_mut().unwrap()
    }

    // Take field
    pub fn take_feature_lists(&mut self) -> super::feature::FeatureLists {
        self.feature_lists.take().unwrap_or_else(|| super::feature::FeatureLists::new())
    }

    pub fn get_feature_lists(&self) -> &super::feature::FeatureLists {
        self.feature_lists.as_ref().unwrap_or_else(|| super::feature::FeatureLists::default_instance())
    }

    fn get_feature_lists_for_reflect(&self) -> &::protobuf::SingularPtrField<super::feature::FeatureLists> {
        &self.feature_lists
    }

    fn mut_feature_lists_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::feature::FeatureLists> {
        &mut self.feature_lists
    }
}

impl ::protobuf::Message for SequenceExample {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.feature_lists {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.feature_lists)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.feature_lists.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.feature_lists.as_ref() {
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

impl ::protobuf::MessageStatic for SequenceExample {
    fn new() -> SequenceExample {
        SequenceExample::new()
    }

    fn descriptor_static(_: ::std::option::Option<SequenceExample>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::feature::Features>>(
                    "context",
                    SequenceExample::get_context_for_reflect,
                    SequenceExample::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::feature::FeatureLists>>(
                    "feature_lists",
                    SequenceExample::get_feature_lists_for_reflect,
                    SequenceExample::mut_feature_lists_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SequenceExample>(
                    "SequenceExample",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SequenceExample {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_feature_lists();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SequenceExample {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SequenceExample {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n0tensorflow/tensorflow/core/example/example.proto\x12\ntensorflow\x1a%\
    tensorflow/core/example/feature.proto\";\n\x07Example\x120\n\x08features\
    \x18\x01\x20\x01(\x0b2\x14.tensorflow.FeaturesR\x08features\"\x80\x01\n\
    \x0fSequenceExample\x12.\n\x07context\x18\x01\x20\x01(\x0b2\x14.tensorfl\
    ow.FeaturesR\x07context\x12=\n\rfeature_lists\x18\x02\x20\x01(\x0b2\x18.\
    tensorflow.FeatureListsR\x0cfeatureListsB,\n\x16org.tensorflow.exampleB\
    \rExampleProtosP\x01\xf8\x01\x01J\x85N\n\x07\x12\x05\x02\0\xa6\x02\x02\n\
    s\n\x01\x0c\x12\x03\x02\0\x12\x1ai\x20Protocol\x20messages\x20for\x20des\
    cribing\x20input\x20data\x20Examples\x20for\x20machine\x20learning\n\x20\
    model\x20training\x20or\x20inference.\n\n\t\n\x02\x03\0\x12\x03\x04\x07.\
    \n\x08\n\x01\x08\x12\x03\x05\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x05\
    \0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x05\x07\x17\n\r\n\x06\x08\
    \xe7\x07\0\x02\0\x12\x03\x05\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x05\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x05\x1a\x1e\n\
    \x08\n\x01\x08\x12\x03\x06\0.\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x06\0.\
    \n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x06\x07\x1b\n\r\n\x06\x08\xe7\
    \x07\x01\x02\0\x12\x03\x06\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\
    \x12\x03\x06\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x06\x1e-\n\
    \x08\n\x01\x08\x12\x03\x07\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x07\0\
    \"\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x07\x07\x1a\n\r\n\x06\x08\xe7\
    \x07\x02\x02\0\x12\x03\x07\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\
    \x12\x03\x07\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x07\x1d!\n\
    \x08\n\x01\x08\x12\x03\x08\0/\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x08\0/\
    \n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x08\x07\x13\n\r\n\x06\x08\xe7\
    \x07\x03\x02\0\x12\x03\x08\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\
    \x12\x03\x08\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x08\x16.\n\
    \x08\n\x01\x02\x12\x03\n\x08\x12\n\xaa\x16\n\x02\x04\0\x12\x04W\0Y\x012\
    \x9d\x16\x20An\x20Example\x20is\x20a\x20mostly-normalized\x20data\x20for\
    mat\x20for\x20storing\x20data\x20for\n\x20training\x20and\x20inference.\
    \x20\x20It\x20contains\x20a\x20key-value\x20store\x20(features);\x20wher\
    e\n\x20each\x20key\x20(string)\x20maps\x20to\x20a\x20Feature\x20message\
    \x20(which\x20is\x20oneof\x20packed\x20BytesList,\n\x20FloatList,\x20or\
    \x20Int64List).\x20\x20This\x20flexible\x20and\x20compact\x20format\x20a\
    llows\x20the\n\x20storage\x20of\x20large\x20amounts\x20of\x20typed\x20da\
    ta,\x20but\x20requires\x20that\x20the\x20data\x20shape\n\x20and\x20use\
    \x20be\x20determined\x20by\x20the\x20configuration\x20files\x20and\x20pa\
    rsers\x20that\x20are\x20used\x20to\n\x20read\x20and\x20write\x20this\x20\
    format.\x20\x20That\x20is,\x20the\x20Example\x20is\x20mostly\x20*not*\
    \x20a\n\x20self-describing\x20format.\x20\x20In\x20TensorFlow,\x20Exampl\
    es\x20are\x20read\x20in\x20row-major\n\x20format,\x20so\x20any\x20config\
    uration\x20that\x20describes\x20data\x20with\x20rank-2\x20or\x20above\n\
    \x20should\x20keep\x20this\x20in\x20mind.\x20\x20For\x20example,\x20to\
    \x20store\x20an\x20M\x20x\x20N\x20matrix\x20of\x20Bytes,\n\x20the\x20Byt\
    esList\x20must\x20contain\x20M*N\x20bytes,\x20with\x20M\x20rows\x20of\
    \x20N\x20contiguous\x20values\n\x20each.\x20\x20That\x20is,\x20the\x20By\
    tesList\x20value\x20must\x20store\x20the\x20matrix\x20as:\n\x20\x20\x20\
    \x20\x20....\x20row\x200\x20....\x20....\x20row\x201\x20....\x20//\x20..\
    .........\x20\x20//\x20...\x20row\x20M-1\x20....\n\n\x20An\x20Example\
    \x20for\x20a\x20movie\x20recommendation\x20application:\n\x20\x20\x20fea\
    tures\x20{\n\x20\x20\x20\x20\x20feature\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20key:\x20\"age\"\n\x20\x20\x20\x20\x20\x20\x20value\x20{\x20float_lis\
    t\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x2029.0\n\x20\x20\x20\
    \x20\x20\x20\x20}}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20feature\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20key:\x20\"movie\"\n\x20\x20\x20\x20\
    \x20\x20\x20value\x20{\x20bytes_list\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20value:\x20\"The\x20Shawshank\x20Redemption\"\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20value:\x20\"Fight\x20Club\"\n\x20\x20\x20\x20\x20\
    \x20\x20}}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20feature\x20{\n\
    \x20\x20\x20\x20\x20\x20\x20key:\x20\"movie_ratings\"\n\x20\x20\x20\x20\
    \x20\x20\x20value\x20{\x20float_list\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20value:\x209.0\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x209.7\
    \n\x20\x20\x20\x20\x20\x20\x20}}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20\
    \x20\x20feature\x20{\n\x20\x20\x20\x20\x20\x20\x20key:\x20\"suggestion\"\
    \n\x20\x20\x20\x20\x20\x20\x20value\x20{\x20bytes_list\x20{\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20value:\x20\"Inception\"\n\x20\x20\x20\x20\
    \x20\x20\x20}}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20#\x20Note\x20\
    that\x20this\x20feature\x20exists\x20to\x20be\x20used\x20as\x20a\x20labe\
    l\x20in\x20training.\n\x20\x20\x20\x20\x20#\x20E.g.,\x20if\x20training\
    \x20a\x20logistic\x20regression\x20model\x20to\x20predict\x20purchase\n\
    \x20\x20\x20\x20\x20#\x20probability\x20in\x20our\x20learning\x20tool\
    \x20we\x20would\x20set\x20the\x20label\x20feature\x20to\n\x20\x20\x20\
    \x20\x20#\x20\"suggestion_purchased\".\n\x20\x20\x20\x20\x20feature\x20{\
    \n\x20\x20\x20\x20\x20\x20\x20key:\x20\"suggestion_purchased\"\n\x20\x20\
    \x20\x20\x20\x20\x20value\x20{\x20float_list\x20{\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20value:\x201.0\n\x20\x20\x20\x20\x20\x20\x20}}\n\x20\x20\
    \x20\x20\x20}\n\x20\x20\x20\x20\x20#\x20Similar\x20to\x20\"suggestion_pu\
    rchased\"\x20above\x20this\x20feature\x20exists\x20to\x20be\x20used\n\
    \x20\x20\x20\x20\x20#\x20as\x20a\x20label\x20in\x20training.\n\x20\x20\
    \x20\x20\x20#\x20E.g.,\x20if\x20training\x20a\x20linear\x20regression\
    \x20model\x20to\x20predict\x20purchase\n\x20\x20\x20\x20\x20#\x20price\
    \x20in\x20our\x20learning\x20tool\x20we\x20would\x20set\x20the\x20label\
    \x20feature\x20to\n\x20\x20\x20\x20\x20#\x20\"purchase_price\".\n\x20\
    \x20\x20\x20\x20feature\x20{\n\x20\x20\x20\x20\x20\x20\x20key:\x20\"purc\
    hase_price\"\n\x20\x20\x20\x20\x20\x20\x20value\x20{\x20float_list\x20{\
    \n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x209.99\n\x20\x20\x20\x20\
    \x20\x20\x20}}\n\x20\x20\x20\x20\x20}\n\x20\x20}\n\n\x20A\x20conformant\
    \x20Example\x20data\x20set\x20obeys\x20the\x20following\x20conventions:\
    \n\x20\x20\x20-\x20If\x20a\x20Feature\x20K\x20exists\x20in\x20one\x20exa\
    mple\x20with\x20data\x20type\x20T,\x20it\x20must\x20be\x20of\n\x20\x20\
    \x20\x20\x20\x20\x20type\x20T\x20in\x20all\x20other\x20examples\x20when\
    \x20present.\x20It\x20may\x20be\x20omitted.\n\x20\x20\x20-\x20The\x20num\
    ber\x20of\x20instances\x20of\x20Feature\x20K\x20list\x20data\x20may\x20v\
    ary\x20across\x20examples,\n\x20\x20\x20\x20\x20\x20\x20depending\x20on\
    \x20the\x20requirements\x20of\x20the\x20model.\n\x20\x20\x20-\x20If\x20a\
    \x20Feature\x20K\x20doesn't\x20exist\x20in\x20an\x20example,\x20a\x20K-s\
    pecific\x20default\x20will\x20be\n\x20\x20\x20\x20\x20\x20\x20used,\x20i\
    f\x20configured.\n\x20\x20\x20-\x20If\x20a\x20Feature\x20K\x20exists\x20\
    in\x20an\x20example\x20but\x20contains\x20no\x20items,\x20the\x20intent\
    \n\x20\x20\x20\x20\x20\x20\x20is\x20considered\x20to\x20be\x20an\x20empt\
    y\x20tensor\x20and\x20no\x20default\x20will\x20be\x20used.\n\n\n\n\x03\
    \x04\0\x01\x12\x03W\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03X\x02\x18\n\r\
    \n\x05\x04\0\x02\0\x04\x12\x04X\x02W\x11\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03X\x02\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03X\x0b\x13\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03X\x16\x17\n\x832\n\x02\x04\x01\x12\x06\xa3\x02\0\xa6\
    \x02\x012\xf41\x20A\x20SequenceExample\x20is\x20an\x20Example\x20represe\
    nting\x20one\x20or\x20more\x20sequences,\x20and\n\x20some\x20context.\
    \x20\x20The\x20context\x20contains\x20features\x20which\x20apply\x20to\
    \x20the\x20entire\n\x20example.\x20The\x20feature_lists\x20contain\x20a\
    \x20key,\x20value\x20map\x20where\x20each\x20key\x20is\n\x20associated\
    \x20with\x20a\x20repeated\x20set\x20of\x20Features\x20(a\x20FeatureList)\
    .\n\x20A\x20FeatureList\x20thus\x20represents\x20the\x20values\x20of\x20\
    a\x20feature\x20identified\x20by\x20its\x20key\n\x20over\x20time\x20/\
    \x20frames.\n\n\x20Below\x20is\x20a\x20SequenceExample\x20for\x20a\x20mo\
    vie\x20recommendation\x20application\x20recording\x20a\n\x20sequence\x20\
    of\x20ratings\x20by\x20a\x20user.\x20The\x20time-independent\x20features\
    \x20(\"locale\",\n\x20\"age\",\x20\"favorites\")\x20describing\x20the\
    \x20user\x20are\x20part\x20of\x20the\x20context.\x20The\x20sequence\n\
    \x20of\x20movies\x20the\x20user\x20rated\x20are\x20part\x20of\x20the\x20\
    feature_lists.\x20For\x20each\x20movie\x20in\x20the\n\x20sequence\x20we\
    \x20have\x20information\x20on\x20its\x20name\x20and\x20actors\x20and\x20\
    the\x20user's\x20rating.\n\x20This\x20information\x20is\x20recorded\x20i\
    n\x20three\x20separate\x20feature_list(s).\n\x20In\x20the\x20example\x20\
    below\x20there\x20are\x20only\x20two\x20movies.\x20All\x20three\x20featu\
    re_list(s),\n\x20namely\x20\"movie_ratings\",\x20\"movie_names\",\x20and\
    \x20\"actors\"\x20have\x20a\x20feature\x20value\x20for\n\x20both\x20movi\
    es.\x20Note,\x20that\x20\"actors\"\x20is\x20itself\x20a\x20bytes_list\
    \x20with\x20multiple\n\x20strings\x20per\x20movie.\n\n\x20context:\x20{\
    \n\x20\x20\x20feature:\x20{\n\x20\x20\x20\x20\x20key\x20\x20:\x20\"local\
    e\"\n\x20\x20\x20\x20\x20value:\x20{\n\x20\x20\x20\x20\x20\x20\x20bytes_\
    list:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\x20\"pt_BR\"\
    \x20]\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20}\n\x20\x20\x20feature:\x20{\n\x20\x20\x20\x20\x20key\x20\x20:\x20\"\
    age\"\n\x20\x20\x20\x20\x20value:\x20{\n\x20\x20\x20\x20\x20\x20\x20floa\
    t_list:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\x2019.0\
    \x20]\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20}\n\x20\x20\x20feature:\x20{\n\x20\x20\x20\x20\x20key\x20\x20:\x20\"\
    favorites\"\n\x20\x20\x20\x20\x20value:\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20bytes_list:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\
    \x20\"Majesty\x20Rose\",\x20\"Savannah\x20Outen\",\x20\"One\x20Direction\
    \"\x20]\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20}\n\x20}\n\x20feature_lists:\x20{\n\x20\x20\x20feature_list:\x20{\n\
    \x20\x20\x20\x20\x20key\x20\x20:\x20\"movie_ratings\"\n\x20\x20\x20\x20\
    \x20value:\x20{\n\x20\x20\x20\x20\x20\x20\x20feature:\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20float_list:\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20value:\x20[\x204.5\x20]\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20}\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20fe\
    ature:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20float_list:\x20{\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\x205.0\x20]\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20}\n\x20\x20\x20}\n\x20\x20\x20feature_list:\x20{\n\x20\
    \x20\x20\x20\x20key\x20\x20:\x20\"movie_names\"\n\x20\x20\x20\x20\x20val\
    ue:\x20{\n\x20\x20\x20\x20\x20\x20\x20feature:\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20bytes_list:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20value:\x20[\x20\"The\x20Shawshank\x20Redemption\"\x20]\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20\x20\x20feature:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20bytes_list:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\
    \x20[\x20\"Fight\x20Club\"\x20]\n\x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\
    \x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20}\n\
    \x20\x20\x20feature_list:\x20{\n\x20\x20\x20\x20\x20key\x20\x20:\x20\"ac\
    tors\"\n\x20\x20\x20\x20\x20value:\x20{\n\x20\x20\x20\x20\x20\x20\x20fea\
    ture:\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20bytes_list:\x20{\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\x20\"Tim\x20Robbins\
    \",\x20\"Morgan\x20Freeman\"\x20]\n\x20\x20\x20\x20\x20\x20\x20\x20\x20}\
    \n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20feature:\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20bytes_list:\x20{\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20value:\x20[\x20\"Brad\x20Pitt\",\x20\
    \"Edward\x20Norton\",\x20\"Helena\x20Bonham\x20Carter\"\x20]\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20\x20\x20}\n\x20\x20\x20}\n\x20}\n\n\x20A\x20conformant\x20SequenceEx\
    ample\x20data\x20set\x20obeys\x20the\x20following\x20conventions:\n\n\
    \x20Context:\n\x20\x20\x20-\x20All\x20conformant\x20context\x20features\
    \x20K\x20must\x20obey\x20the\x20same\x20conventions\x20as\n\x20\x20\x20\
    \x20\x20a\x20conformant\x20Example's\x20features\x20(see\x20above).\n\
    \x20Feature\x20lists:\n\x20\x20\x20-\x20A\x20FeatureList\x20L\x20may\x20\
    be\x20missing\x20in\x20an\x20example;\x20it\x20is\x20up\x20to\x20the\n\
    \x20\x20\x20\x20\x20parser\x20configuration\x20to\x20determine\x20if\x20\
    this\x20is\x20allowed\x20or\x20considered\n\x20\x20\x20\x20\x20an\x20emp\
    ty\x20list\x20(zero\x20length).\n\x20\x20\x20-\x20If\x20a\x20FeatureList\
    \x20L\x20exists,\x20it\x20may\x20be\x20empty\x20(zero\x20length).\n\x20\
    \x20\x20-\x20If\x20a\x20FeatureList\x20L\x20is\x20non-empty,\x20all\x20f\
    eatures\x20within\x20the\x20FeatureList\n\x20\x20\x20\x20\x20must\x20hav\
    e\x20the\x20same\x20data\x20type\x20T.\x20Even\x20across\x20SequenceExam\
    ples,\x20the\x20type\x20T\n\x20\x20\x20\x20\x20of\x20the\x20FeatureList\
    \x20identified\x20by\x20the\x20same\x20key\x20must\x20be\x20the\x20same.\
    \x20An\x20entry\n\x20\x20\x20\x20\x20without\x20any\x20values\x20may\x20\
    serve\x20as\x20an\x20empty\x20feature.\n\x20\x20\x20-\x20If\x20a\x20Feat\
    ureList\x20L\x20is\x20non-empty,\x20it\x20is\x20up\x20to\x20the\x20parse\
    r\x20configuration\n\x20\x20\x20\x20\x20to\x20determine\x20if\x20all\x20\
    features\x20within\x20the\x20FeatureList\x20must\n\x20\x20\x20\x20\x20ha\
    ve\x20the\x20same\x20size.\x20\x20The\x20same\x20holds\x20for\x20this\
    \x20FeatureList\x20across\x20multiple\n\x20\x20\x20\x20\x20examples.\n\n\
    \x20Examples\x20of\x20conformant\x20and\x20non-conformant\x20examples'\
    \x20FeatureLists:\n\n\x20Conformant\x20FeatureLists:\n\x20\x20\x20\x20fe\
    ature_lists:\x20{\x20feature_list:\x20{\n\x20\x20\x20\x20\x20\x20key:\
    \x20\"movie_ratings\"\n\x20\x20\x20\x20\x20\x20value:\x20{\x20feature:\
    \x20{\x20float_list:\x20{\x20value:\x20[\x204.5\x20]\x20}\x20}\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20feature:\x20{\x20flo\
    at_list:\x20{\x20value:\x20[\x205.0\x20]\x20}\x20}\x20}\n\x20\x20\x20\
    \x20}\x20}\n\n\x20Non-conformant\x20FeatureLists\x20(mismatched\x20types\
    ):\n\x20\x20\x20\x20feature_lists:\x20{\x20feature_list:\x20{\n\x20\x20\
    \x20\x20\x20\x20key:\x20\"movie_ratings\"\n\x20\x20\x20\x20\x20\x20value\
    :\x20{\x20feature:\x20{\x20float_list:\x20{\x20value:\x20[\x204.5\x20]\
    \x20}\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    feature:\x20{\x20int64_list:\x20{\x20value:\x20[\x205\x20]\x20}\x20}\x20\
    }\n\x20\x20\x20\x20}\x20}\n\n\x20Conditionally\x20conformant\x20FeatureL\
    ists,\x20the\x20parser\x20configuration\x20determines\n\x20if\x20the\x20\
    feature\x20sizes\x20must\x20match:\n\x20\x20\x20\x20feature_lists:\x20{\
    \x20feature_list:\x20{\n\x20\x20\x20\x20\x20\x20key:\x20\"movie_ratings\
    \"\n\x20\x20\x20\x20\x20\x20value:\x20{\x20feature:\x20{\x20float_list:\
    \x20{\x20value:\x20[\x204.5\x20]\x20}\x20}\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20feature:\x20{\x20float_list:\x20{\x20val\
    ue:\x20[\x205.0,\x206.0\x20]\x20}\x20}\x20}\n\x20\x20\x20\x20}\x20}\n\n\
    \x20Conformant\x20pair\x20of\x20SequenceExample\n\x20\x20\x20\x20feature\
    _lists:\x20{\x20feature_list:\x20{\n\x20\x20\x20\x20\x20\x20key:\x20\"mo\
    vie_ratings\"\n\x20\x20\x20\x20\x20\x20value:\x20{\x20feature:\x20{\x20f\
    loat_list:\x20{\x20value:\x20[\x204.5\x20]\x20}\x20}\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20feature:\x20{\x20float_list:\
    \x20{\x20value:\x20[\x205.0\x20]\x20}\x20}\x20}\n\x20\x20\x20\x20}\x20}\
    \n\x20and:\n\x20\x20\x20\x20feature_lists:\x20{\x20feature_list:\x20{\n\
    \x20\x20\x20\x20\x20\x20key:\x20\"movie_ratings\"\n\x20\x20\x20\x20\x20\
    \x20value:\x20{\x20feature:\x20{\x20float_list:\x20{\x20value:\x20[\x204\
    .5\x20]\x20}\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20feature:\x20{\x20float_list:\x20{\x20value:\x20[\x205.0\x20]\x20\
    }\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20feat\
    ure:\x20{\x20float_list:\x20{\x20value:\x20[\x202.0\x20]\x20}\x20}\x20}\
    \n\x20\x20\x20\x20}\x20}\n\n\x20Conformant\x20pair\x20of\x20SequenceExam\
    ple\n\x20\x20\x20\x20feature_lists:\x20{\x20feature_list:\x20{\n\x20\x20\
    \x20\x20\x20\x20key:\x20\"movie_ratings\"\n\x20\x20\x20\x20\x20\x20value\
    :\x20{\x20feature:\x20{\x20float_list:\x20{\x20value:\x20[\x204.5\x20]\
    \x20}\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    feature:\x20{\x20float_list:\x20{\x20value:\x20[\x205.0\x20]\x20}\x20}\
    \x20}\n\x20\x20\x20\x20}\x20}\n\x20and:\n\x20\x20\x20\x20feature_lists:\
    \x20{\x20feature_list:\x20{\n\x20\x20\x20\x20\x20\x20key:\x20\"movie_rat\
    ings\"\n\x20\x20\x20\x20\x20\x20value:\x20{\x20}\n\x20\x20\x20\x20}\x20}\
    \n\n\x20Conditionally\x20conformant\x20pair\x20of\x20SequenceExample,\
    \x20the\x20parser\x20configuration\n\x20determines\x20if\x20the\x20secon\
    d\x20feature_lists\x20is\x20consistent\x20(zero-length)\x20or\n\x20inval\
    id\x20(missing\x20\"movie_ratings\"):\n\x20\x20\x20\x20feature_lists:\
    \x20{\x20feature_list:\x20{\n\x20\x20\x20\x20\x20\x20key:\x20\"movie_rat\
    ings\"\n\x20\x20\x20\x20\x20\x20value:\x20{\x20feature:\x20{\x20float_li\
    st:\x20{\x20value:\x20[\x204.5\x20]\x20}\x20}\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20feature:\x20{\x20float_list:\x20{\
    \x20value:\x20[\x205.0\x20]\x20}\x20}\x20}\n\x20\x20\x20\x20}\x20}\n\x20\
    and:\n\x20\x20\x20\x20feature_lists:\x20{\x20}\n\n\x20Non-conformant\x20\
    pair\x20of\x20SequenceExample\x20(mismatched\x20types)\n\x20\x20\x20\x20\
    feature_lists:\x20{\x20feature_list:\x20{\n\x20\x20\x20\x20\x20\x20key:\
    \x20\"movie_ratings\"\n\x20\x20\x20\x20\x20\x20value:\x20{\x20feature:\
    \x20{\x20float_list:\x20{\x20value:\x20[\x204.5\x20]\x20}\x20}\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20feature:\x20{\x20flo\
    at_list:\x20{\x20value:\x20[\x205.0\x20]\x20}\x20}\x20}\n\x20\x20\x20\
    \x20}\x20}\n\x20and:\n\x20\x20\x20\x20feature_lists:\x20{\x20feature_lis\
    t:\x20{\n\x20\x20\x20\x20\x20\x20key:\x20\"movie_ratings\"\n\x20\x20\x20\
    \x20\x20\x20value:\x20{\x20feature:\x20{\x20int64_list:\x20{\x20value:\
    \x20[\x204\x20]\x20}\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20feature:\x20{\x20int64_list:\x20{\x20value:\x20[\x205\
    \x20]\x20}\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20feature:\x20{\x20int64_list:\x20{\x20value:\x20[\x202\x20]\x20}\
    \x20}\x20}\n\x20\x20\x20\x20}\x20}\n\n\x20Conditionally\x20conformant\
    \x20pair\x20of\x20SequenceExample;\x20the\x20parser\x20configuration\n\
    \x20determines\x20if\x20the\x20feature\x20sizes\x20must\x20match:\n\x20\
    \x20\x20\x20feature_lists:\x20{\x20feature_list:\x20{\n\x20\x20\x20\x20\
    \x20\x20key:\x20\"movie_ratings\"\n\x20\x20\x20\x20\x20\x20value:\x20{\
    \x20feature:\x20{\x20float_list:\x20{\x20value:\x20[\x204.5\x20]\x20}\
    \x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20featu\
    re:\x20{\x20float_list:\x20{\x20value:\x20[\x205.0\x20]\x20}\x20}\x20}\n\
    \x20\x20\x20\x20}\x20}\n\x20and:\n\x20\x20\x20\x20feature_lists:\x20{\
    \x20feature_list:\x20{\n\x20\x20\x20\x20\x20\x20key:\x20\"movie_ratings\
    \"\n\x20\x20\x20\x20\x20\x20value:\x20{\x20feature:\x20{\x20float_list:\
    \x20{\x20value:\x20[\x204.0\x20]\x20}\x20}\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20feature:\x20{\x20float_list:\x20{\x20val\
    ue:\x20[\x205.0,\x203.0\x20]\x20}\x20}\n\x20\x20\x20\x20}\x20}\n\n\x0b\n\
    \x03\x04\x01\x01\x12\x04\xa3\x02\x08\x17\n\x0c\n\x04\x04\x01\x02\0\x12\
    \x04\xa4\x02\x02\x17\n\x0f\n\x05\x04\x01\x02\0\x04\x12\x06\xa4\x02\x02\
    \xa3\x02\x19\n\r\n\x05\x04\x01\x02\0\x06\x12\x04\xa4\x02\x02\n\n\r\n\x05\
    \x04\x01\x02\0\x01\x12\x04\xa4\x02\x0b\x12\n\r\n\x05\x04\x01\x02\0\x03\
    \x12\x04\xa4\x02\x15\x16\n\x0c\n\x04\x04\x01\x02\x01\x12\x04\xa5\x02\x02\
    !\n\x0f\n\x05\x04\x01\x02\x01\x04\x12\x06\xa5\x02\x02\xa4\x02\x17\n\r\n\
    \x05\x04\x01\x02\x01\x06\x12\x04\xa5\x02\x02\x0e\n\r\n\x05\x04\x01\x02\
    \x01\x01\x12\x04\xa5\x02\x0f\x1c\n\r\n\x05\x04\x01\x02\x01\x03\x12\x04\
    \xa5\x02\x1f\x20b\x06proto3\
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