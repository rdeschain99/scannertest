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
//! Generated file from `sign_up.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct UserSignUpParams {
    // message fields
    pub email: ::std::string::String,
    pub name: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserSignUpParams {
    fn default() -> &'a UserSignUpParams {
        <UserSignUpParams as ::protobuf::Message>::default_instance()
    }
}

impl UserSignUpParams {
    pub fn new() -> UserSignUpParams {
        ::std::default::Default::default()
    }

    // string email = 1;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
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

    // string password = 3;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for UserSignUpParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.password.is_empty() {
            os.write_string(3, &self.password)?;
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

    fn new() -> UserSignUpParams {
        UserSignUpParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &UserSignUpParams| { &m.email },
                |m: &mut UserSignUpParams| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &UserSignUpParams| { &m.name },
                |m: &mut UserSignUpParams| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &UserSignUpParams| { &m.password },
                |m: &mut UserSignUpParams| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserSignUpParams>(
                "UserSignUpParams",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserSignUpParams {
        static instance: ::protobuf::rt::LazyV2<UserSignUpParams> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserSignUpParams::new)
    }
}

impl ::protobuf::Clear for UserSignUpParams {
    fn clear(&mut self) {
        self.email.clear();
        self.name.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserSignUpParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserSignUpParams {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserSignUpRequest {
    // message fields
    pub email: ::std::string::String,
    pub name: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserSignUpRequest {
    fn default() -> &'a UserSignUpRequest {
        <UserSignUpRequest as ::protobuf::Message>::default_instance()
    }
}

impl UserSignUpRequest {
    pub fn new() -> UserSignUpRequest {
        ::std::default::Default::default()
    }

    // string email = 1;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
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

    // string password = 3;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for UserSignUpRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.password.is_empty() {
            os.write_string(3, &self.password)?;
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

    fn new() -> UserSignUpRequest {
        UserSignUpRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &UserSignUpRequest| { &m.email },
                |m: &mut UserSignUpRequest| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &UserSignUpRequest| { &m.name },
                |m: &mut UserSignUpRequest| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &UserSignUpRequest| { &m.password },
                |m: &mut UserSignUpRequest| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserSignUpRequest>(
                "UserSignUpRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserSignUpRequest {
        static instance: ::protobuf::rt::LazyV2<UserSignUpRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserSignUpRequest::new)
    }
}

impl ::protobuf::Clear for UserSignUpRequest {
    fn clear(&mut self) {
        self.email.clear();
        self.name.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserSignUpRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserSignUpRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserSignUpResult {
    // message fields
    pub is_success: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserSignUpResult {
    fn default() -> &'a UserSignUpResult {
        <UserSignUpResult as ::protobuf::Message>::default_instance()
    }
}

impl UserSignUpResult {
    pub fn new() -> UserSignUpResult {
        ::std::default::Default::default()
    }

    // bool is_success = 1;


    pub fn get_is_success(&self) -> bool {
        self.is_success
    }
    pub fn clear_is_success(&mut self) {
        self.is_success = false;
    }

    // Param is passed by value, moved
    pub fn set_is_success(&mut self, v: bool) {
        self.is_success = v;
    }
}

impl ::protobuf::Message for UserSignUpResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_success = tmp;
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
        if self.is_success != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.is_success != false {
            os.write_bool(1, self.is_success)?;
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

    fn new() -> UserSignUpResult {
        UserSignUpResult::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "is_success",
                |m: &UserSignUpResult| { &m.is_success },
                |m: &mut UserSignUpResult| { &mut m.is_success },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserSignUpResult>(
                "UserSignUpResult",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserSignUpResult {
        static instance: ::protobuf::rt::LazyV2<UserSignUpResult> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserSignUpResult::new)
    }
}

impl ::protobuf::Clear for UserSignUpResult {
    fn clear(&mut self) {
        self.is_success = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserSignUpResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserSignUpResult {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsign_up.proto\"X\n\x10UserSignUpParams\x12\x14\n\x05email\x18\x01\
    \x20\x01(\tR\x05email\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\
    \x1a\n\x08password\x18\x03\x20\x01(\tR\x08password\"Y\n\x11UserSignUpReq\
    uest\x12\x14\n\x05email\x18\x01\x20\x01(\tR\x05email\x12\x12\n\x04name\
    \x18\x02\x20\x01(\tR\x04name\x12\x1a\n\x08password\x18\x03\x20\x01(\tR\
    \x08password\"1\n\x10UserSignUpResult\x12\x1d\n\nis_success\x18\x01\x20\
    \x01(\x08R\tisSuccessJ\xdb\x03\n\x06\x12\x04\0\0\x0e\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x06\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x02\x08\x18\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x15\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x03\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x13\x14\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x14\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x0b\x0f\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x12\x13\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\x05\x04\x18\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x04\n\
    \n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x0b\x13\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x05\x16\x17\n\n\n\x02\x04\x01\x12\x04\x07\0\x0b\x01\n\n\
    \n\x03\x04\x01\x01\x12\x03\x07\x08\x19\n\x0b\n\x04\x04\x01\x02\0\x12\x03\
    \x08\x04\x15\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x08\x04\n\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x08\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x08\x13\x14\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\t\x04\x14\n\x0c\n\
    \x05\x04\x01\x02\x01\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\
    \x12\x03\t\x0b\x0f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\t\x12\x13\n\
    \x0b\n\x04\x04\x01\x02\x02\x12\x03\n\x04\x18\n\x0c\n\x05\x04\x01\x02\x02\
    \x05\x12\x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\n\x0b\x13\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\n\x16\x17\n\n\n\x02\x04\x02\x12\
    \x04\x0c\0\x0e\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0c\x08\x18\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03\r\x04\x18\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\r\
    \x04\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\r\t\x13\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03\r\x16\x17b\x06proto3\
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