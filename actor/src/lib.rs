use std::str;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_graphql_interface::{GraphQL, GraphQLSender, QueryRequest};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::debug;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct PassThroughActor {}

#[async_trait]
impl HttpServer for PassThroughActor {
    /// passes the body of a POST request to the wasmcloud-graphql-interface
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let query = str::from_utf8(&req.body)
            .map_err(|e| RpcError::Deser(format!("{}", e)))?
            .to_string();
        debug!("Received query: {:?}", query);

        let response = GraphQLSender::new()
            .query(
                ctx,
                &QueryRequest {
                    query,
                    ..Default::default()
                },
            )
            .await?;
        Ok(HttpResponse {
            body: response.data.as_bytes().to_vec(),
            ..Default::default()
        })
    }
}
