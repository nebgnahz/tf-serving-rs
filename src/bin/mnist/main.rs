// extern crate tf_serving;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate bytes;
#[macro_use]
extern crate ndarray;

mod errors {
    use reqwest;
    error_chain! {
        foreign_links {
            Reqwest(reqwest::Error);
            UrlParse(reqwest::UrlError);
            Io(::std::io::Error);
            ArrayShape(::ndarray::ShapeError);
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
    for (_image, label) in test.iter() {
        println!("{:?}", label.into_slice());
    }
    Ok(())
}
