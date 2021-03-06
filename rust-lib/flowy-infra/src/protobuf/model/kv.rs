// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `kv.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct KeyValue {
    // message fields
    pub key: ::std::string::String,
    // message oneof groups
    pub one_of_str_value: ::std::option::Option<KeyValue_oneof_one_of_str_value>,
    pub one_of_int_value: ::std::option::Option<KeyValue_oneof_one_of_int_value>,
    pub one_of_float_value: ::std::option::Option<KeyValue_oneof_one_of_float_value>,
    pub one_of_bool_value: ::std::option::Option<KeyValue_oneof_one_of_bool_value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a KeyValue {
    fn default() -> &'a KeyValue {
        <KeyValue as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum KeyValue_oneof_one_of_str_value {
    str_value(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum KeyValue_oneof_one_of_int_value {
    int_value(i64),
}

#[derive(Clone,PartialEq,Debug)]
pub enum KeyValue_oneof_one_of_float_value {
    float_value(f64),
}

#[derive(Clone,PartialEq,Debug)]
pub enum KeyValue_oneof_one_of_bool_value {
    bool_value(bool),
}

impl KeyValue {
    pub fn new() -> KeyValue {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    // string str_value = 2;


    pub fn get_str_value(&self) -> &str {
        match self.one_of_str_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_str_value(&mut self) {
        self.one_of_str_value = ::std::option::Option::None;
    }

    pub fn has_str_value(&self) -> bool {
        match self.one_of_str_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_str_value(&mut self, v: ::std::string::String) {
        self.one_of_str_value = ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_str_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(_)) = self.one_of_str_value {
        } else {
            self.one_of_str_value = ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(::std::string::String::new()));
        }
        match self.one_of_str_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_str_value(&mut self) -> ::std::string::String {
        if self.has_str_value() {
            match self.one_of_str_value.take() {
                ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // int64 int_value = 3;


    pub fn get_int_value(&self) -> i64 {
        match self.one_of_int_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_int_value::int_value(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_int_value(&mut self) {
        self.one_of_int_value = ::std::option::Option::None;
    }

    pub fn has_int_value(&self) -> bool {
        match self.one_of_int_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_int_value::int_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int_value(&mut self, v: i64) {
        self.one_of_int_value = ::std::option::Option::Some(KeyValue_oneof_one_of_int_value::int_value(v))
    }

    // double float_value = 4;


    pub fn get_float_value(&self) -> f64 {
        match self.one_of_float_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_float_value::float_value(v)) => v,
            _ => 0.,
        }
    }
    pub fn clear_float_value(&mut self) {
        self.one_of_float_value = ::std::option::Option::None;
    }

    pub fn has_float_value(&self) -> bool {
        match self.one_of_float_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_float_value::float_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_value(&mut self, v: f64) {
        self.one_of_float_value = ::std::option::Option::Some(KeyValue_oneof_one_of_float_value::float_value(v))
    }

    // bool bool_value = 5;


    pub fn get_bool_value(&self) -> bool {
        match self.one_of_bool_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_bool_value::bool_value(v)) => v,
            _ => false,
        }
    }
    pub fn clear_bool_value(&mut self) {
        self.one_of_bool_value = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        match self.one_of_bool_value {
            ::std::option::Option::Some(KeyValue_oneof_one_of_bool_value::bool_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.one_of_bool_value = ::std::option::Option::Some(KeyValue_oneof_one_of_bool_value::bool_value(v))
    }
}

impl ::protobuf::Message for KeyValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_str_value = ::std::option::Option::Some(KeyValue_oneof_one_of_str_value::str_value(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_int_value = ::std::option::Option::Some(KeyValue_oneof_one_of_int_value::int_value(is.read_int64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_float_value = ::std::option::Option::Some(KeyValue_oneof_one_of_float_value::float_value(is.read_double()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_bool_value = ::std::option::Option::Some(KeyValue_oneof_one_of_bool_value::bool_value(is.read_bool()?));
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_str_value {
            match v {
                &KeyValue_oneof_one_of_str_value::str_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_int_value {
            match v {
                &KeyValue_oneof_one_of_int_value::int_value(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_float_value {
            match v {
                &KeyValue_oneof_one_of_float_value::float_value(v) => {
                    my_size += 9;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_bool_value {
            match v {
                &KeyValue_oneof_one_of_bool_value::bool_value(v) => {
                    my_size += 2;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_str_value {
            match v {
                &KeyValue_oneof_one_of_str_value::str_value(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_int_value {
            match v {
                &KeyValue_oneof_one_of_int_value::int_value(v) => {
                    os.write_int64(3, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_float_value {
            match v {
                &KeyValue_oneof_one_of_float_value::float_value(v) => {
                    os.write_double(4, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_bool_value {
            match v {
                &KeyValue_oneof_one_of_bool_value::bool_value(v) => {
                    os.write_bool(5, v)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> KeyValue {
        KeyValue::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "key",
                |m: &KeyValue| { &m.key },
                |m: &mut KeyValue| { &mut m.key },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "str_value",
                KeyValue::has_str_value,
                KeyValue::get_str_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                "int_value",
                KeyValue::has_int_value,
                KeyValue::get_int_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor::<_>(
                "float_value",
                KeyValue::has_float_value,
                KeyValue::get_float_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                "bool_value",
                KeyValue::has_bool_value,
                KeyValue::get_bool_value,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<KeyValue>(
                "KeyValue",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static KeyValue {
        static instance: ::protobuf::rt::LazyV2<KeyValue> = ::protobuf::rt::LazyV2::INIT;
        instance.get(KeyValue::new)
    }
}

impl ::protobuf::Clear for KeyValue {
    fn clear(&mut self) {
        self.key.clear();
        self.one_of_str_value = ::std::option::Option::None;
        self.one_of_int_value = ::std::option::Option::None;
        self.one_of_float_value = ::std::option::Option::None;
        self.one_of_bool_value = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyValue {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08kv.proto\"\xf1\x01\n\x08KeyValue\x12\x10\n\x03key\x18\x01\x20\x01(\
    \tR\x03key\x12\x1d\n\tstr_value\x18\x02\x20\x01(\tH\0R\x08strValue\x12\
    \x1d\n\tint_value\x18\x03\x20\x01(\x03H\x01R\x08intValue\x12!\n\x0bfloat\
    _value\x18\x04\x20\x01(\x01H\x02R\nfloatValue\x12\x1f\n\nbool_value\x18\
    \x05\x20\x01(\x08H\x03R\tboolValueB\x12\n\x10one_of_str_valueB\x12\n\x10\
    one_of_int_valueB\x14\n\x12one_of_float_valueB\x13\n\x11one_of_bool_valu\
    eJ\xa9\x03\n\x06\x12\x04\0\0\x08\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\
    \n\x02\x04\0\x12\x04\x02\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\
    \x10\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x13\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x0e\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x11\x12\n\x0b\n\x04\x04\0\x08\0\
    \x12\x03\x04\x044\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x04\n\x1a\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x04\x1d2\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x04\x1d#\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04$-\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x0401\n\x0b\n\x04\x04\0\x08\x01\x12\x03\x05\
    \x043\n\x0c\n\x05\x04\0\x08\x01\x01\x12\x03\x05\n\x1a\n\x0b\n\x04\x04\0\
    \x02\x02\x12\x03\x05\x1d1\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x1d\
    \"\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05#,\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x03\x05/0\n\x0b\n\x04\x04\0\x08\x02\x12\x03\x06\x048\n\x0c\n\
    \x05\x04\0\x08\x02\x01\x12\x03\x06\n\x1c\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\x06\x1f6\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x06\x1f%\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03\x06&1\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\
    \x0645\n\x0b\n\x04\x04\0\x08\x03\x12\x03\x07\x044\n\x0c\n\x05\x04\0\x08\
    \x03\x01\x12\x03\x07\n\x1b\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x07\x1e2\n\
    \x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x07\x1e\"\n\x0c\n\x05\x04\0\x02\x04\
    \x01\x12\x03\x07#-\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x0701b\x06proto\
    3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
