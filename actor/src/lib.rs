use std::str;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_graphql_interface::{GraphQL, GraphQLSender, QueryRequest};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::{debug, warn};

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
        match req.method.as_ref() {
            "POST" => {
                let query = str::from_utf8(&req.body)
                    .map_err(|e| RpcError::Deser(format!("{}", e)))?
                    .to_string();
                debug!("Received query: {:?}", query);

                let headers = if !req.header.is_empty() {
                    Some(req.header.clone())
                } else {
                    None
                };
                let response = GraphQLSender::new()
                    .query(ctx, &QueryRequest { query, headers })
                    .await?;
                Ok(HttpResponse {
                    body: response.data.as_bytes().to_vec(),
                    ..Default::default()
                })
            }
            "GET" => {
                let response = GraphQLSender::new().graphiql(ctx).await?;
                Ok(HttpResponse {
                    body: response.data.as_bytes().to_vec(),
                    ..Default::default()
                })
            }
            _ => {
                warn!("no route for this request: {:?}", req);
                Ok(HttpResponse::not_found())
            }
        }
    }
}
