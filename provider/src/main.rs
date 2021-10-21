use log::debug;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::RwLock;
use upstream::QueryResult;
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_graphql_interface::{GraphQL, GraphQLReceiver, QueryRequest, QueryResponse};

mod upstream;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(GraphQLProvider::default())?;

    eprintln!("GraphQL provider exiting");
    Ok(())
}

#[derive(Default, Clone, Provider)]
#[services(GraphQL)]
struct GraphQLProvider {
    instance: Arc<RwLock<Option<String>>>,
}

/// use default implementations of provider message handlers
impl ProviderDispatch for GraphQLProvider {}
#[async_trait]
impl ProviderHandler for GraphQLProvider {
    /// Provider should perform any operations needed for a new link,
    /// including setting up per-actor resources, and checking authorization.
    /// If the link is allowed, return true, otherwise return false to deny the link.
    async fn put_link(&self, ld: &LinkDefinition) -> RpcResult<bool> {
        let database_url = match ld.values.get(DATABASE_URL_KEY) {
            Some(v) => Some(v.to_string()),
            None => {
                return Err(RpcError::InvalidParameter(format!(
                    "{} must be set",
                    DATABASE_URL_KEY
                )))
            }
        };
        let mut instance = self.instance.write().await;
        if *instance != database_url {
            return Err(RpcError::InvalidParameter(format!(
				"instance already initialised with a different {}, and we currently only support one connection",
				DATABASE_URL_KEY
			)));
        }
        upstream::init("1");
        *instance = database_url;
        Ok(true)
    }

    /// Handle notification that a link is dropped - release the upstream
    async fn delete_link(&self, _actor_id: &str) {
        let mut instance = self.instance.write().await;
        upstream::remove("1");
        *instance = None;
    }

    /// Handle shutdown request by releasing upstream
    async fn shutdown(&self) -> Result<(), Infallible> {
        let mut instance = self.instance.write().await;
        upstream::remove("1");
        *instance = None;
        Ok(())
    }
}

/// Handle GraphQL methods
#[async_trait]
impl GraphQL for GraphQLProvider {
    /// updates the text on the GraphQL display
    async fn query(&self, _ctx: &Context, req: &QueryRequest) -> RpcResult<QueryResponse> {
        debug!("processing request update({})", req.query);
        match upstream::query(
            "1",
            r#"
		query MyQuery {
			productLists {
				nodes {
					nodeId
					userId
					id
					title
				}
			}
		}"#,
        ) {
            QueryResult::Ok(result) => Ok(QueryResponse { data: result }),
            QueryResult::Err(err) => Err(RpcError::MethodNotHandled(err)),
        }
    }
}
