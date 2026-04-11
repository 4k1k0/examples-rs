use tonic::Code;
use tonic::{Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

use identiconpro::identiconer_server::Identiconer;
use identiconpro::{IdenticonReply, IdenticonRequest};

pub mod identiconpro {
    tonic::include_proto!("server");
}

use crate::identicon;

#[derive(Debug, Default)]
pub struct MyIdenticoner {}

#[tonic::async_trait]
impl Identiconer for MyIdenticoner {
    async fn create(
        &self,
        request: Request<IdenticonRequest>,
    ) -> Result<Response<IdenticonReply>, Status> {
        println!("request: {:?}", request);

        return match identicon::run(&request.get_ref().username) {
            Ok(_) => res_success(),
            Err(_) => res_error(),
        };
    }
}

fn res_success() -> Result<Response<IdenticonReply>, Status> {
    let res = IdenticonReply {
        path: "good".to_string(),
    };

    Ok(Response::new(res))
}

fn res_error() -> Result<Response<IdenticonReply>, Status> {
    let mut err_details = ErrorDetails::new();
    err_details.add_bad_request_violation("image", "cannot be created");
    err_details.add_bad_request_violation("foo", "foo value");
    err_details.set_localized_message("en-US", "message for the user");

    let status = Status::with_error_details(Code::InvalidArgument, "error grpc", err_details);

    Err(status)
}
