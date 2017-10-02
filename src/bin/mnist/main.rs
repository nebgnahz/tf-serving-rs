extern crate grpcio;
extern crate tf_serving;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate bytes;
#[macro_use]
extern crate ndarray;

use bytes::ByteOrder;
use tf_serving as serving;
use grpcio::{ChannelBuilder, EnvBuilder};
use tf_serving::model::ModelSpec;
use tf_serving::predict::PredictRequest;
use tf_serving::prediction_service_grpc::PredictionServiceClient;
use tf_serving::tensor::TensorProto;
use tf_serving::tensor_shape::{TensorShapeProto, TensorShapeProto_Dim};
use std::sync::Arc;

mod errors {
    use reqwest;
    error_chain! {
        foreign_links {
            Reqwest(reqwest::Error);
            UrlParse(reqwest::UrlError);
            Io(::std::io::Error);
            ArrayShape(::ndarray::ShapeError);
            GRPC(::grpcio::Error);
        }
        errors {
            Server {
                description("failed to download from the server")
                display("failed to download from the server")
            }
            DataCorruption {
                description("failed parse downloaded data")
                display("failed parse downloaded data")
            }
        }
    }
}
mod input;
use errors::*;

fn main() {
    run().unwrap()
}

fn run() -> Result<()> {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("128.32.171.191:9000");
    let client = PredictionServiceClient::new(ch);

    let test_data = input::DataSet::test("/tmp")?;
    for (image, _label) in test_data.iter().take(1) {
        let mut model_spec = ModelSpec::new();
        model_spec.set_name("mnist".into());
        model_spec.set_signature_name("predict_images".into());

        let mut request = PredictRequest::new();
        request.set_model_spec(model_spec);

        let mut image_proto = TensorProto::new();
        image_proto.set_dtype(serving::types::DataType::DT_FLOAT);

        // Set shape
        let mut shape = TensorShapeProto::new();
        let mut dim = TensorShapeProto_Dim::new();
        dim.set_size(1);
        shape.dim.insert(0, dim);
        let mut dim = TensorShapeProto_Dim::new();
        dim.set_size(784);
        shape.dim.insert(1, dim);
        image_proto.set_tensor_shape(shape);

        // Set content
        let tensor_content = image.into_slice().unwrap();
        image_proto.set_tensor_content(encode(tensor_content));

        request.inputs.insert("images".into(), image_proto);
        let response = client.predict(request);
        println!("{:?}", response);
    }
    Ok(())
}

fn encode<'a>(input: &'a [f32]) -> Vec<u8> {
    let mut buf = vec![0; input.len() * 4];
    for i in input {
        bytes::BigEndian::write_f32(&mut buf, *i);
    }
    buf
}
