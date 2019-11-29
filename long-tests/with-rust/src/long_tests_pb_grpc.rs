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

pub trait LongTests {
    fn echo(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::long_tests_pb::EchoRequest>, resp: ::grpc::ServerResponseUnarySink<super::long_tests_pb::EchoResponse>) -> ::grpc::Result<()>;

    fn char_count(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequest<super::long_tests_pb::CharCountRequest>, resp: ::grpc::ServerResponseUnarySink<super::long_tests_pb::CharCountResponse>) -> ::grpc::Result<()>;

    fn random_strings(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::long_tests_pb::RandomStringsRequest>, resp: ::grpc::ServerResponseSink<super::long_tests_pb::RandomStringsResponse>) -> ::grpc::Result<()>;
}

// client

pub struct LongTestsClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for LongTestsClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        LongTestsClient {
            grpc_client: grpc_client,
        }
    }
}

impl LongTestsClient {
    pub fn echo(&self, o: ::grpc::RequestOptions, req: super::long_tests_pb::EchoRequest) -> ::grpc::SingleResponse<super::long_tests_pb::EchoResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/LongTests/echo"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn char_count(&self, o: ::grpc::RequestOptions) -> impl ::futures::future::Future<Item=(::grpc::ClientRequestSink<super::long_tests_pb::CharCountRequest>, ::grpc::SingleResponse<super::long_tests_pb::CharCountResponse>), Error=::grpc::Error> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/LongTests/char_count"),
            streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_client_streaming(o, descriptor)
    }

    pub fn random_strings(&self, o: ::grpc::RequestOptions, req: super::long_tests_pb::RandomStringsRequest) -> ::grpc::StreamingResponse<super::long_tests_pb::RandomStringsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/LongTests/random_strings"),
            streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_server_streaming(o, req, descriptor)
    }
}

// server

pub struct LongTestsServer;


impl LongTestsServer {
    pub fn new_service_def<H : LongTests + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/LongTests",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/LongTests/echo"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).echo(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/LongTests/char_count"),
                        streaming: ::grpc::rt::GrpcStreaming::ClientStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerClientStreaming::new(move |ctx, req, resp| (*handler_copy).char_count(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/LongTests/random_strings"),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |ctx, req, resp| (*handler_copy).random_strings(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
