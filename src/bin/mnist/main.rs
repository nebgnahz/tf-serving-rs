extern crate grpcio;
extern crate tf_serving;
extern crate protobuf;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate bytes;
#[macro_use]
extern crate ndarray;

use grpcio::{ChannelBuilder, EnvBuilder};
use tf_serving::model::ModelSpec;
use tf_serving::predict::PredictRequest;
use tf_serving::prediction_service_grpc::PredictionServiceClient;
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
    let test = input::DataSet::test("/tmp")?;

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("128.32.171.191:9000");
    let client = PredictionServiceClient::new(ch);

    for (_image, _label) in test.iter().take(1) {
        let mut model_spec = ModelSpec::new();
        model_spec.set_name("mnist".into());
        model_spec.set_signature_name("predict_images".into());

        let mut request = PredictRequest::new();
        request.set_model_spec(model_spec);

        let s = protobuf::text_format::print_to_string(&request);
        println!("{:?}", s);

        // let image_proto = XXX;
        // request.inputs.insert("images", image_proto);
        // request.inputs['images'].CopyFrom(
        // tf.contrib.util.make_tensor_proto(image[0], shape=[1, image[0].size]))

        let response = client.predict(request);
        println!("{:?}", response);
    }
    Ok(())
}
