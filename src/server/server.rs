use crate::server::raft;

use smelt_server::{Smelt, SmeltServer};
use tonic::{transport::server::Router, transport::Server, Request, Response, Status};

tonic::include_proto!("smelt");

#[derive(Debug, Default)]
pub struct API {}

#[tonic::async_trait]
impl Smelt for API {
    async fn begin(
        &self,
        request: Request<BeginRequest>,
    ) -> Result<Response<BeginResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = BeginResponse {};
        Ok(Response::new(resp))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = GetResponse {};
        Ok(Response::new(resp))
    }

    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = PutResponse {};
        Ok(Response::new(resp))
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = DeleteResponse {};
        Ok(Response::new(resp))
    }

    async fn commit(
        &self,
        request: Request<CommitRequest>,
    ) -> Result<Response<CommitResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = CommitResponse {};
        Ok(Response::new(resp))
    }

    async fn abort(
        &self,
        request: Request<AbortRequest>,
    ) -> Result<Response<AbortResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = AbortResponse {};
        Ok(Response::new(resp))
    }
}

pub struct ServerOptions<'a> {
    pub api_addr: &'a str,
    pub raft_addr: &'a str,
}

pub fn get_router() -> Router {
    let api = API::default();
    Server::builder().add_service(SmeltServer::new(api))
}

pub async fn start_server(opts: &ServerOptions<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let smelt = get_router().serve(opts.api_addr.parse()?);
    let raft = raft::get_router().serve(opts.raft_addr.parse()?);

    tokio::select! {
        res = raft => {
            if let Err(e) = res {
                eprintln!("Error running Raft service: {}", e);
            }
        }
        res = smelt => {
            if let Err(e) = res {
                eprintln!("Error running Smelt service: {}", e);
            }
        }
    }
    Ok(())
}
