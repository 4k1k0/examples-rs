use tonic::{Request, Response, Status, transport::Server};

use identiconpro::identiconer_server::{Identiconer, IdenticonerServer};
use identiconpro::{IdenticonReply, IdenticonRequest};

pub mod identiconpro {
    tonic::include_proto!("identicon");
}

mod identicon;

#[derive(Debug, Default)]
pub struct MyIdenticoner {}

#[tonic::async_trait]
impl Identiconer for MyIdenticoner {
    async fn create(
        &self,
        request: Request<IdenticonRequest>,
    ) -> Result<Response<IdenticonReply>, Status> {
        println!("request: {:?}", request);

        let path = match identicon::run(&request.get_ref().username) {
            Ok(_) => "good".to_string(),
            Err(e) => e.to_string(),
        };

        let res = IdenticonReply{ path };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let identi = MyIdenticoner::default();

    Server::builder()
        .add_service(IdenticonerServer::new(identi))
        .serve(addr)
        .await?;

    Ok(())
}
