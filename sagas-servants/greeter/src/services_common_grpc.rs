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


// server interface

pub trait SysInfo {
    fn get_sys_info(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::services_common::InfoQuery>, resp: ::grpc::ServerResponseUnarySink<super::services_common::InfoMap>) -> ::grpc::Result<()>;

    fn query_meta(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::metainfo::MetaQuery>, resp: ::grpc::ServerResponseUnarySink<super::metainfo::MetaPayload>) -> ::grpc::Result<()>;
}

// client

pub struct SysInfoClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for SysInfoClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        SysInfoClient {
            grpc_client: grpc_client,
        }
    }
}

impl SysInfoClient {
    pub fn get_sys_info(&self, o: ::grpc::RequestOptions, req: super::services_common::InfoQuery) -> ::grpc::SingleResponse<super::services_common::InfoMap> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/model.SysInfo/GetSysInfo"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn query_meta(&self, o: ::grpc::RequestOptions, req: super::metainfo::MetaQuery) -> ::grpc::SingleResponse<super::metainfo::MetaPayload> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/model.SysInfo/QueryMeta"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct SysInfoServer;


impl SysInfoServer {
    pub fn new_service_def<H : SysInfo + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/model.SysInfo",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/model.SysInfo/GetSysInfo"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_sys_info(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/model.SysInfo/QueryMeta"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).query_meta(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait EntityServant {
    fn get_entity_names(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::services_common::InfoQuery>, resp: ::grpc::ServerResponseUnarySink<super::services_common::Names>) -> ::grpc::Result<()>;
}

// client

pub struct EntityServantClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for EntityServantClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        EntityServantClient {
            grpc_client: grpc_client,
        }
    }
}

impl EntityServantClient {
    pub fn get_entity_names(&self, o: ::grpc::RequestOptions, req: super::services_common::InfoQuery) -> ::grpc::SingleResponse<super::services_common::Names> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/model.EntityServant/GetEntityNames"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct EntityServantServer;


impl EntityServantServer {
    pub fn new_service_def<H : EntityServant + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/model.EntityServant",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/model.EntityServant/GetEntityNames"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_entity_names(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
