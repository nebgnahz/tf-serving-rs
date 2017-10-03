extern crate protobuf;
extern crate futures;
extern crate grpcio;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

#[cfg(feature = "examples")]
extern crate reqwest;
#[cfg(feature = "examples")]
extern crate flate2;
#[cfg(feature = "examples")]
extern crate bytes;
#[cfg(feature = "examples")]
extern crate itertools;

pub mod errors {
    #[cfg(feature = "examples")]
    use reqwest;
    error_chain! {
        foreign_links {
            Reqwest(reqwest::Error) #[cfg(feature = "examples")];
            UrlParse(reqwest::UrlError) #[cfg(feature = "examples")];
            Io(::std::io::Error);
            GRPC(::grpcio::Error) #[cfg(feature = "examples")];
        }
        errors {
            #[cfg(feature = "examples")]
            Server {
                description("failed to download from the server")
                display("failed to download from the server")
            }
            #[cfg(feature = "examples")]
            DataCorruption {
                description("failed parse downloaded data")
                display("failed parse downloaded data")
            }
        }
    }
}

// The export list is currently somewhat arbitrary. We will export (hide) more
// as examples are built.
mod attr_value;
pub mod classification;
mod example;
pub mod feature;
pub mod function;
mod get_model_metadata;
pub mod graph;
pub mod inference;
pub mod input;
pub mod meta_graph;
mod model;
pub mod node_def;
pub mod op_def;
pub mod predict;
pub mod prediction_service_grpc;
pub mod regression;
pub mod resource_handle;
mod saver;
mod tensor;
mod tensor_shape;
mod types;
pub mod versions;

#[cfg(feature = "examples")]
pub mod examples;
