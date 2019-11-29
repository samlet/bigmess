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

pub trait NlpProcs {
    fn ping(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::common_types::PingRequest>, resp: ::grpc::ServerResponseUnarySink<super::common_types::PingReply>) -> ::grpc::Result<()>;

    fn parse_dependency(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlParseRequest>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlSentence>) -> ::grpc::Result<()>;

    fn get_pinyin(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlPinyinRequest>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlText>) -> ::grpc::Result<()>;

    fn add_documents(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlDocumentSet>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlResult>) -> ::grpc::Result<()>;

    fn get_nearest_documents(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlText>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlDocumentSimilaritySet>) -> ::grpc::Result<()>;

    fn tokenizer(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlTokenizerRequest>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlTokens>) -> ::grpc::Result<()>;

    fn entity_extractor(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlTokenizerRequest>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlEntities>) -> ::grpc::Result<()>;

    fn parse_amount_terms(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlText>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlAmountList>) -> ::grpc::Result<()>;
}

// client

pub struct NlpProcsClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for NlpProcsClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        NlpProcsClient {
            grpc_client: grpc_client,
        }
    }
}

impl NlpProcsClient {
    pub fn ping(&self, o: ::grpc::RequestOptions, req: super::common_types::PingRequest) -> ::grpc::SingleResponse<super::common_types::PingReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/Ping"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn parse_dependency(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlParseRequest) -> ::grpc::SingleResponse<super::nlpserv::NlSentence> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/ParseDependency"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_pinyin(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlPinyinRequest) -> ::grpc::SingleResponse<super::nlpserv::NlText> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/GetPinyin"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn add_documents(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlDocumentSet) -> ::grpc::SingleResponse<super::nlpserv::NlResult> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/AddDocuments"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_nearest_documents(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlText) -> ::grpc::SingleResponse<super::nlpserv::NlDocumentSimilaritySet> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/GetNearestDocuments"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn tokenizer(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlTokenizerRequest) -> ::grpc::SingleResponse<super::nlpserv::NlTokens> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/Tokenizer"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn entity_extractor(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlTokenizerRequest) -> ::grpc::SingleResponse<super::nlpserv::NlEntities> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/EntityExtractor"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn parse_amount_terms(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlText) -> ::grpc::SingleResponse<super::nlpserv::NlAmountList> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/ParseAmountTerms"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct NlpProcsServer;


impl NlpProcsServer {
    pub fn new_service_def<H : NlpProcs + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/nlpserv.NlpProcs",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/Ping"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).ping(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/ParseDependency"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).parse_dependency(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/GetPinyin"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_pinyin(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/AddDocuments"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).add_documents(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/GetNearestDocuments"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_nearest_documents(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/Tokenizer"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).tokenizer(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/EntityExtractor"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).entity_extractor(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.NlpProcs/ParseAmountTerms"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).parse_amount_terms(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait CabochaNlpProcs {
    fn tokenizer(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::nlpserv::NlText>, resp: ::grpc::ServerResponseUnarySink<super::nlpserv::NlCabochaChunks>) -> ::grpc::Result<()>;
}

// client

pub struct CabochaNlpProcsClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for CabochaNlpProcsClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        CabochaNlpProcsClient {
            grpc_client: grpc_client,
        }
    }
}

impl CabochaNlpProcsClient {
    pub fn tokenizer(&self, o: ::grpc::RequestOptions, req: super::nlpserv::NlText) -> ::grpc::SingleResponse<super::nlpserv::NlCabochaChunks> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/nlpserv.CabochaNlpProcs/Tokenizer"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct CabochaNlpProcsServer;


impl CabochaNlpProcsServer {
    pub fn new_service_def<H : CabochaNlpProcs + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/nlpserv.CabochaNlpProcs",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/nlpserv.CabochaNlpProcs/Tokenizer"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).tokenizer(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
