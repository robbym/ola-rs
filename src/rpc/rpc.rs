// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct RpcMessage {
    // message fields
    field_type: ::std::option::Option<Type>,
    id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    buffer: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcMessage {}

impl RpcMessage {
    pub fn new() -> RpcMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcMessage {
        static mut instance: ::protobuf::lazy::Lazy<RpcMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcMessage,
        };
        unsafe {
            instance.get(|| {
                RpcMessage {
                    field_type: ::std::option::Option::None,
                    id: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    buffer: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .ola.rpc.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Type {
        self.field_type.unwrap_or(Type::REQUEST)
    }

    // optional uint32 id = 2;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes buffer = 4;

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn has_buffer(&self) -> bool {
        self.buffer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer(&mut self, v: ::std::vec::Vec<u8>) {
        self.buffer = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buffer(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.buffer.is_none() {
            self.buffer.set_default();
        };
        self.buffer.as_mut().unwrap()
    }

    // Take field
    pub fn take_buffer(&mut self) -> ::std::vec::Vec<u8> {
        self.buffer.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_buffer(&self) -> &[u8] {
        match self.buffer.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpcMessage {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.buffer));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.buffer.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.id {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.buffer.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RpcMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpcMessage {
    fn new() -> RpcMessage {
        RpcMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    RpcMessage::has_field_type,
                    RpcMessage::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    RpcMessage::has_id,
                    RpcMessage::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    RpcMessage::has_name,
                    RpcMessage::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "buffer",
                    RpcMessage::has_buffer,
                    RpcMessage::get_buffer,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcMessage>(
                    "RpcMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcMessage {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.clear_name();
        self.clear_buffer();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpcMessage {
    fn eq(&self, other: &RpcMessage) -> bool {
        self.field_type == other.field_type &&
        self.id == other.id &&
        self.name == other.name &&
        self.buffer == other.buffer &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpcMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Type {
    REQUEST = 1,
    RESPONSE = 2,
    RESPONSE_CANCEL = 3,
    RESPONSE_FAILED = 4,
    RESPONSE_NOT_IMPLEMENTED = 5,
    DISCONNECT = 6,
    DESCRIPTOR_REQUEST = 7,
    DESCRIPTOR_RESPONSE = 8,
    REQUEST_CANCEL = 9,
    STREAM_REQUEST = 10,
}

impl ::protobuf::ProtobufEnum for Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Type> {
        match value {
            1 => ::std::option::Option::Some(Type::REQUEST),
            2 => ::std::option::Option::Some(Type::RESPONSE),
            3 => ::std::option::Option::Some(Type::RESPONSE_CANCEL),
            4 => ::std::option::Option::Some(Type::RESPONSE_FAILED),
            5 => ::std::option::Option::Some(Type::RESPONSE_NOT_IMPLEMENTED),
            6 => ::std::option::Option::Some(Type::DISCONNECT),
            7 => ::std::option::Option::Some(Type::DESCRIPTOR_REQUEST),
            8 => ::std::option::Option::Some(Type::DESCRIPTOR_RESPONSE),
            9 => ::std::option::Option::Some(Type::REQUEST_CANCEL),
            10 => ::std::option::Option::Some(Type::STREAM_REQUEST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Type] = &[
            Type::REQUEST,
            Type::RESPONSE,
            Type::RESPONSE_CANCEL,
            Type::RESPONSE_FAILED,
            Type::RESPONSE_NOT_IMPLEMENTED,
            Type::DISCONNECT,
            Type::DESCRIPTOR_REQUEST,
            Type::DESCRIPTOR_RESPONSE,
            Type::REQUEST_CANCEL,
            Type::STREAM_REQUEST,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Type {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x52, 0x70, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x07, 0x6f, 0x6c, 0x61,
    0x2e, 0x72, 0x70, 0x63, 0x22, 0x53, 0x0a, 0x0a, 0x52, 0x70, 0x63, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x1b, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x0d, 0x2e, 0x6f, 0x6c, 0x61, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x66,
    0x66, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x2a, 0xd2, 0x01, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x01, 0x12,
    0x0c, 0x0a, 0x08, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x45, 0x10, 0x02, 0x12, 0x13, 0x0a,
    0x0f, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x45, 0x5f, 0x43, 0x41, 0x4e, 0x43, 0x45, 0x4c,
    0x10, 0x03, 0x12, 0x13, 0x0a, 0x0f, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x45, 0x5f, 0x46,
    0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x04, 0x12, 0x1c, 0x0a, 0x18, 0x52, 0x45, 0x53, 0x50, 0x4f,
    0x4e, 0x53, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x49, 0x4d, 0x50, 0x4c, 0x45, 0x4d, 0x45, 0x4e,
    0x54, 0x45, 0x44, 0x10, 0x05, 0x12, 0x0e, 0x0a, 0x0a, 0x44, 0x49, 0x53, 0x43, 0x4f, 0x4e, 0x4e,
    0x45, 0x43, 0x54, 0x10, 0x06, 0x12, 0x16, 0x0a, 0x12, 0x44, 0x45, 0x53, 0x43, 0x52, 0x49, 0x50,
    0x54, 0x4f, 0x52, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x07, 0x12, 0x17, 0x0a,
    0x13, 0x44, 0x45, 0x53, 0x43, 0x52, 0x49, 0x50, 0x54, 0x4f, 0x52, 0x5f, 0x52, 0x45, 0x53, 0x50,
    0x4f, 0x4e, 0x53, 0x45, 0x10, 0x08, 0x12, 0x12, 0x0a, 0x0e, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53,
    0x54, 0x5f, 0x43, 0x41, 0x4e, 0x43, 0x45, 0x4c, 0x10, 0x09, 0x12, 0x12, 0x0a, 0x0e, 0x53, 0x54,
    0x52, 0x45, 0x41, 0x4d, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x0a, 0x4a, 0xc7,
    0x06, 0x0a, 0x06, 0x12, 0x04, 0x19, 0x00, 0x2d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x19, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x1b, 0x00, 0x26, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x05, 0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1c, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x1c, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x1e, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x1f, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1f, 0x02,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x1f, 0x14, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x20, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x20, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x20, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x21, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x21, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x21, 0x0f,
    0x10, 0x0a, 0x1e, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x22, 0x02, 0x19, 0x22, 0x11,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x22, 0x02, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x22, 0x17, 0x18, 0x0a, 0x1e, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x23, 0x02, 0x1a, 0x22, 0x11, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x23, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x23, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x08, 0x12, 0x03, 0x24, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x24, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x24,
    0x13, 0x14, 0x0a, 0x3c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x25, 0x02, 0x16, 0x22,
    0x2f, 0x20, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x77, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74,
    0x20, 0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x25, 0x02, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x25, 0x13, 0x15, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x28, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x28, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x2a, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x12, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x2b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x2b, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x2b, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x2c, 0x1a, 0x1b,
];

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