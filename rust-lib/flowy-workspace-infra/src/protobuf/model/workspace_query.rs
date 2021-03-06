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
//! Generated file from `workspace_query.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct QueryWorkspaceRequest {
    // message oneof groups
    pub one_of_workspace_id: ::std::option::Option<QueryWorkspaceRequest_oneof_one_of_workspace_id>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a QueryWorkspaceRequest {
    fn default() -> &'a QueryWorkspaceRequest {
        <QueryWorkspaceRequest as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum QueryWorkspaceRequest_oneof_one_of_workspace_id {
    workspace_id(::std::string::String),
}

impl QueryWorkspaceRequest {
    pub fn new() -> QueryWorkspaceRequest {
        ::std::default::Default::default()
    }

    // string workspace_id = 1;


    pub fn get_workspace_id(&self) -> &str {
        match self.one_of_workspace_id {
            ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_workspace_id(&mut self) {
        self.one_of_workspace_id = ::std::option::Option::None;
    }

    pub fn has_workspace_id(&self) -> bool {
        match self.one_of_workspace_id {
            ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_workspace_id(&mut self, v: ::std::string::String) {
        self.one_of_workspace_id = ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(v))
    }

    // Mutable pointer to the field.
    pub fn mut_workspace_id(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(_)) = self.one_of_workspace_id {
        } else {
            self.one_of_workspace_id = ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(::std::string::String::new()));
        }
        match self.one_of_workspace_id {
            ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_workspace_id(&mut self) -> ::std::string::String {
        if self.has_workspace_id() {
            match self.one_of_workspace_id.take() {
                ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for QueryWorkspaceRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_workspace_id = ::std::option::Option::Some(QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.one_of_workspace_id {
            match v {
                &QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.one_of_workspace_id {
            match v {
                &QueryWorkspaceRequest_oneof_one_of_workspace_id::workspace_id(ref v) => {
                    os.write_string(1, v)?;
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

    fn new() -> QueryWorkspaceRequest {
        QueryWorkspaceRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "workspace_id",
                QueryWorkspaceRequest::has_workspace_id,
                QueryWorkspaceRequest::get_workspace_id,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<QueryWorkspaceRequest>(
                "QueryWorkspaceRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static QueryWorkspaceRequest {
        static instance: ::protobuf::rt::LazyV2<QueryWorkspaceRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(QueryWorkspaceRequest::new)
    }
}

impl ::protobuf::Clear for QueryWorkspaceRequest {
    fn clear(&mut self) {
        self.one_of_workspace_id = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryWorkspaceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryWorkspaceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WorkspaceIdentifier {
    // message oneof groups
    pub one_of_workspace_id: ::std::option::Option<WorkspaceIdentifier_oneof_one_of_workspace_id>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WorkspaceIdentifier {
    fn default() -> &'a WorkspaceIdentifier {
        <WorkspaceIdentifier as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum WorkspaceIdentifier_oneof_one_of_workspace_id {
    workspace_id(::std::string::String),
}

impl WorkspaceIdentifier {
    pub fn new() -> WorkspaceIdentifier {
        ::std::default::Default::default()
    }

    // string workspace_id = 1;


    pub fn get_workspace_id(&self) -> &str {
        match self.one_of_workspace_id {
            ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_workspace_id(&mut self) {
        self.one_of_workspace_id = ::std::option::Option::None;
    }

    pub fn has_workspace_id(&self) -> bool {
        match self.one_of_workspace_id {
            ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_workspace_id(&mut self, v: ::std::string::String) {
        self.one_of_workspace_id = ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(v))
    }

    // Mutable pointer to the field.
    pub fn mut_workspace_id(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(_)) = self.one_of_workspace_id {
        } else {
            self.one_of_workspace_id = ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(::std::string::String::new()));
        }
        match self.one_of_workspace_id {
            ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_workspace_id(&mut self) -> ::std::string::String {
        if self.has_workspace_id() {
            match self.one_of_workspace_id.take() {
                ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for WorkspaceIdentifier {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_workspace_id = ::std::option::Option::Some(WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.one_of_workspace_id {
            match v {
                &WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.one_of_workspace_id {
            match v {
                &WorkspaceIdentifier_oneof_one_of_workspace_id::workspace_id(ref v) => {
                    os.write_string(1, v)?;
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

    fn new() -> WorkspaceIdentifier {
        WorkspaceIdentifier::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "workspace_id",
                WorkspaceIdentifier::has_workspace_id,
                WorkspaceIdentifier::get_workspace_id,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<WorkspaceIdentifier>(
                "WorkspaceIdentifier",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static WorkspaceIdentifier {
        static instance: ::protobuf::rt::LazyV2<WorkspaceIdentifier> = ::protobuf::rt::LazyV2::INIT;
        instance.get(WorkspaceIdentifier::new)
    }
}

impl ::protobuf::Clear for WorkspaceIdentifier {
    fn clear(&mut self) {
        self.one_of_workspace_id = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WorkspaceIdentifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WorkspaceIdentifier {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15workspace_query.proto\"S\n\x15QueryWorkspaceRequest\x12#\n\x0cwork\
    space_id\x18\x01\x20\x01(\tH\0R\x0bworkspaceIdB\x15\n\x13one_of_workspac\
    e_id\"Q\n\x13WorkspaceIdentifier\x12#\n\x0cworkspace_id\x18\x01\x20\x01(\
    \tH\0R\x0bworkspaceIdB\x15\n\x13one_of_workspace_idJ\xe6\x01\n\x06\x12\
    \x04\0\0\x07\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\
    \x02\0\x04\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x1d\n\x0b\n\x04\x04\0\
    \x08\0\x12\x03\x03\x04:\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x03\n\x1d\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x03\x208\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\x03\x20&\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03'3\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\x0367\n\n\n\x02\x04\x01\x12\x04\x05\0\x07\x01\n\n\n\
    \x03\x04\x01\x01\x12\x03\x05\x08\x1b\n\x0b\n\x04\x04\x01\x08\0\x12\x03\
    \x06\x04:\n\x0c\n\x05\x04\x01\x08\0\x01\x12\x03\x06\n\x1d\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x06\x208\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x06\x20&\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x06'3\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x0667b\x06proto3\
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
