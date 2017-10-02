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


// interface

pub trait PredictionService {
    fn classify(&self, o: ::grpc::RequestOptions, p: super::classification::ClassificationRequest) -> ::grpc::SingleResponse<super::classification::ClassificationResponse>;

    fn regress(&self, o: ::grpc::RequestOptions, p: super::regression::RegressionRequest) -> ::grpc::SingleResponse<super::regression::RegressionResponse>;

    fn predict(&self, o: ::grpc::RequestOptions, p: super::predict::PredictRequest) -> ::grpc::SingleResponse<super::predict::PredictResponse>;

    fn multi_inference(&self, o: ::grpc::RequestOptions, p: super::inference::MultiInferenceRequest) -> ::grpc::SingleResponse<super::inference::MultiInferenceResponse>;

    fn get_model_metadata(&self, o: ::grpc::RequestOptions, p: super::get_model_metadata::GetModelMetadataRequest) -> ::grpc::SingleResponse<super::get_model_metadata::GetModelMetadataResponse>;
}

// client

pub struct PredictionServiceClient {
    grpc_client: ::grpc::Client,
    method_Classify: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::classification::ClassificationRequest, super::classification::ClassificationResponse>>,
    method_Regress: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::regression::RegressionRequest, super::regression::RegressionResponse>>,
    method_Predict: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::predict::PredictRequest, super::predict::PredictResponse>>,
    method_MultiInference: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::inference::MultiInferenceRequest, super::inference::MultiInferenceResponse>>,
    method_GetModelMetadata: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::get_model_metadata::GetModelMetadataRequest, super::get_model_metadata::GetModelMetadataResponse>>,
}

impl PredictionServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        PredictionServiceClient {
            grpc_client: grpc_client,
            method_Classify: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/tensorflow.serving.PredictionService/Classify".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Regress: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/tensorflow.serving.PredictionService/Regress".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Predict: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/tensorflow.serving.PredictionService/Predict".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_MultiInference: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/tensorflow.serving.PredictionService/MultiInference".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetModelMetadata: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/tensorflow.serving.PredictionService/GetModelMetadata".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            PredictionServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            PredictionServiceClient::with_client(c)
        })
    }
}

impl PredictionService for PredictionServiceClient {
    fn classify(&self, o: ::grpc::RequestOptions, p: super::classification::ClassificationRequest) -> ::grpc::SingleResponse<super::classification::ClassificationResponse> {
        self.grpc_client.call_unary(o, p, self.method_Classify.clone())
    }

    fn regress(&self, o: ::grpc::RequestOptions, p: super::regression::RegressionRequest) -> ::grpc::SingleResponse<super::regression::RegressionResponse> {
        self.grpc_client.call_unary(o, p, self.method_Regress.clone())
    }

    fn predict(&self, o: ::grpc::RequestOptions, p: super::predict::PredictRequest) -> ::grpc::SingleResponse<super::predict::PredictResponse> {
        self.grpc_client.call_unary(o, p, self.method_Predict.clone())
    }

    fn multi_inference(&self, o: ::grpc::RequestOptions, p: super::inference::MultiInferenceRequest) -> ::grpc::SingleResponse<super::inference::MultiInferenceResponse> {
        self.grpc_client.call_unary(o, p, self.method_MultiInference.clone())
    }

    fn get_model_metadata(&self, o: ::grpc::RequestOptions, p: super::get_model_metadata::GetModelMetadataRequest) -> ::grpc::SingleResponse<super::get_model_metadata::GetModelMetadataResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetModelMetadata.clone())
    }
}

// server

pub struct PredictionServiceServer;


impl PredictionServiceServer {
    pub fn new_service_def<H : PredictionService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/tensorflow.serving.PredictionService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/tensorflow.serving.PredictionService/Classify".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.classify(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/tensorflow.serving.PredictionService/Regress".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.regress(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/tensorflow.serving.PredictionService/Predict".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.predict(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/tensorflow.serving.PredictionService/MultiInference".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.multi_inference(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/tensorflow.serving.PredictionService/GetModelMetadata".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_model_metadata(o, p))
                    },
                ),
            ],
        )
    }
}
