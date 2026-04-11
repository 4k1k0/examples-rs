pub mod identicon;
mod protocol;

use protocol::grpc::identiconpro::identiconer_server::IdenticonerServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let identi = protocol::grpc::MyIdenticoner::default();

    protocol::rest::foo();

    Server::builder()
        .add_service(IdenticonerServer::new(identi))
        .serve(addr)
        .await?;

    Ok(())
}
