// extern crate tf_serving;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate bytes;

mod errors {
    use reqwest;
    error_chain! {
        foreign_links {
            Reqwest(reqwest::Error);
            UrlParse(reqwest::UrlError);
            Io(::std::io::Error);
        }
        errors {
            Server {
                description("failed to download from the server")
                display("failed to download from the server")
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
    let test_images = input::maybe_download(input::TEST_IMAGES, "/tmp")?;
    let _test_labels = input::maybe_download(input::TEST_LABELS, "/tmp")?;
    let _train_images = input::maybe_download(input::TRAIN_IMAGES, "/tmp")?;
    let _train_labels = input::maybe_download(input::TRAIN_LABELS, "/tmp")?;

    let _images = input::extract_images(test_images)?;

    Ok(())
}
