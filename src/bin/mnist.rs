extern crate futures;
extern crate grpcio;
extern crate tf_serving;
extern crate futures_cpupool;
extern crate tokio_core;

use futures::Future;
use futures::future::join_all;
use futures_cpupool::CpuPool;
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
    let pool = CpuPool::new_num_cpus();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("128.32.171.191:9000");
    let client = PredictionServiceClient::new(ch);

    let test_data = mnist::DataSet::test("/tmp")?;

    let err_counter = Arc::new(AtomicUsize::new(0));
    let total = 100;

    let mut tasks = Vec::new();
    for i in 0..total {
        let (ref image, label) = test_data[i as usize];
        let request = mnist::predict_request(&image);
        let error = err_counter.clone();
        let predict = client.predict_async(request).and_then(move |response| {
            let output = response.get_outputs();
            let output = output.get("scores").unwrap();
            let scores = &output.float_val;
            let max_idx = scores
                .iter()
                .enumerate()
                .max_by(|&(_, x), &(_, y)| x.partial_cmp(y).unwrap())
                .unwrap();

            if label != max_idx.0 as u8 {
                error.fetch_add(1, Ordering::Relaxed);
            }
            Ok(())
        });

        tasks.push(pool.spawn(predict));
    }

    join_all(tasks).wait().unwrap();
    println!(
        "Error rate: {}%",
        err_counter.load(Ordering::SeqCst) as f32 / total as f32 * 100.0
    );

    Ok(())
}
