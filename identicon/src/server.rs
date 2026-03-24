use tonic::{Request, Response, Status, transport::Server};

use identicon::identiconer_server::{Identiconer, IdenticonerServer};
use identicon::{IdenticonReply, IdenticonRequest};

pub mod identicon {
    tonic::include_proto!("identicon");
}

#[derive(Debug, Default)]
pub struct MyIdenticoner {}

#[tonic::async_trait]
impl Identiconer for MyIdenticoner {
    async fn create(
        &self,
        request: Request<IdenticonRequest>,
    ) -> Result<Response<IdenticonReply>, Status> {
        println!("request: {:?}", request);

        Ok(Response::new(IdenticonReply {
            path: "path fake".to_string(),
        }))
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
