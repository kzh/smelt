use node_server::{Node, NodeServer};
use tonic::{transport::server::Router, transport::Server, Request, Response, Status};

tonic::include_proto!("node");

#[derive(Debug, Default)]
pub struct RaftNode {}

#[tonic::async_trait]
impl Node for RaftNode {
    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = AppendEntriesResponse {};
        Ok(Response::new(resp))
    }

    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        println!("Request: {:?}", request);

        let resp = RequestVoteResponse {};
        Ok(Response::new(resp))
    }
}

pub fn get_router() -> Router {
    let node = RaftNode::default();
    Server::builder().add_service(NodeServer::new(node))
}
