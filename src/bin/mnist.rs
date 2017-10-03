extern crate futures;
extern crate grpcio;
extern crate tf_serving;

use futures::Future;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tf_serving::errors::*;
use tf_serving::examples::mnist;
use tf_serving::prediction_service_grpc::PredictionServiceClient;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("128.32.171.191:9000");
    let client = PredictionServiceClient::new(ch);

    let test_data = mnist::DataSet::test("/tmp")?;

    let err_counter = AtomicUsize::new(0);
    let total = 10;
    for (image, label) in test_data.iter().take(total) {
        let request = mnist::predict_request(image);
        let predict = client.predict_async(request).and_then(|response| {
            let output = response.get_outputs();
            let output = output.get("scores").unwrap();
            let scores = &output.float_val;
            let max_idx = scores
                .iter()
                .enumerate()
                .max_by(|&(_, x), &(_, y)| x.partial_cmp(y).unwrap())
                .unwrap();

            if label[0] != max_idx.0 as u8 {
                err_counter.fetch_add(1, Ordering::SeqCst);
            }
            Ok(())
        });

        predict.wait().unwrap();
    }

    println!(
        "Error rate: {}%",
        err_counter.load(Ordering::SeqCst) as f32 / total as f32 * 100.0
    );

    Ok(())
}
