/// "VIRUS" + meta_length + message_length + meta + message
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
    #[prost(map="string, string", tag="11")]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Demo {
    #[prost(string, tag="1")]
    pub field1: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub field2: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag="1")]
    pub detail: ::prost::alloc::string::String,
}
