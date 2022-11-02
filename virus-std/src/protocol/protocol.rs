#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaKeyValue {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(oneof="meta_key_value::Value", tags="2, 3, 4")]
    pub value: ::core::option::Option<meta_key_value::Value>,
}
/// Nested message and enum types in `MetaKeyValue`.
pub mod meta_key_value {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="2")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag="3")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag="4")]
        Integer(i64),
    }
}
/// "VIRUS" + meta_length + message_length + meta + message
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaData {
    #[prost(int32, tag="1")]
    pub role: i32,
    #[prost(string, tag="2")]
    pub service_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub method_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub seq_id: u64,
    #[prost(string, tag="5")]
    pub virus_version: ::prost::alloc::string::String,
    #[prost(int32, tag="6")]
    pub compress_type: i32,
    #[prost(int32, tag="7")]
    pub serialize_type: i32,
    #[prost(int32, tag="8")]
    pub message_type: i32,
    #[prost(int32, tag="9")]
    pub origin_size: i32,
    #[prost(int32, tag="10")]
    pub compressed_size: i32,
    #[prost(message, repeated, tag="11")]
    pub values: ::prost::alloc::vec::Vec<MetaKeyValue>,
}
