extern crate tf_serving;
extern crate grpc;
extern crate protobuf;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate bytes;
#[macro_use]
extern crate ndarray;

use protobuf::singular::SingularPtrField;
use tf_serving::model::ModelSpec;
use tf_serving::predict::PredictRequest;
use tf_serving::prediction_service_grpc::PredictionService;
use tf_serving::prediction_service_grpc::PredictionServiceClient;

mod errors {
    use reqwest;
    error_chain! {
        foreign_links {
            Reqwest(reqwest::Error);
            UrlParse(reqwest::UrlError);
            Io(::std::io::Error);
            ArrayShape(::ndarray::ShapeError);
            GRPC(::grpc::Error);
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
    let conf = grpc::ClientConf::default();
    let client = PredictionServiceClient::new_plain("128.32.171.191", 9000, conf)?;

    for (_image, label) in test.iter().take(1) {
        let mut model_spec = ModelSpec::new();
        model_spec.set_name("mnist".into());
        model_spec.set_signature_name("predict_images".into());

        let req_opt = grpc::RequestOptions::new();
        let mut request = PredictRequest::new();
        request.model_spec = SingularPtrField::some(model_spec);

        // let image_proto = XXX;
        // request.inputs.insert("images", image_proto);
        // request.inputs['images'].CopyFrom(
        // tf.contrib.util.make_tensor_proto(image[0], shape=[1, image[0].size]))

        println!("{:?}", label.into_slice());
        let response = client.predict(req_opt, request);
        let response = response.wait();
        println!("{:?}", response);
    }
    Ok(())
}
