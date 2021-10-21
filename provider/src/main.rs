use log::debug;
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_graphql_interface::{GraphQL, GraphQLReceiver, QueryRequest, QueryResponse};

mod upstream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(GraphQLProvider::default())?;

    eprintln!("GraphQL provider exiting");
    Ok(())
}

#[derive(Default, Clone, Provider)]
#[services(GraphQL)]
struct GraphQLProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for GraphQLProvider {}
impl ProviderHandler for GraphQLProvider {}

/// Handle GraphQL methods
#[async_trait]
impl GraphQL for GraphQLProvider {
    /// updates the text on the GraphQL display
    async fn query(&self, _ctx: &Context, req: &QueryRequest) -> RpcResult<QueryResponse> {
        debug!("processing request update({})", req.query);
        Ok(QueryResponse::default())
    }
}
