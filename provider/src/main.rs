use std::{convert::Infallible, sync::Arc};
use temp_dir::TempDir;
use tokio::sync::RwLock;
use upstream::QueryResult;
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_graphql_interface::{GraphQL, GraphQLReceiver, QueryRequest, QueryResponse};

mod unpack_files;
mod upstream;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    unpack_files::unpack(&temp_dir)?;
    provider_main(GraphQLProvider::new(temp_dir))?;

    eprintln!("GraphQL provider exiting");
    Ok(())
}

#[derive(Clone, Provider)]
#[services(GraphQL)]
struct GraphQLProvider {
    node_files: Arc<RwLock<Option<TempDir>>>,
    instance: Arc<RwLock<Option<String>>>,
}

impl GraphQLProvider {
    fn new(node_files: TempDir) -> Self {
        Self {
            node_files: Arc::new(RwLock::new(Some(node_files))),
            instance: Arc::new(RwLock::new(None)),
        }
    }
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
        let node_files = self.node_files.read().await;
        if let Some(node_files) = &*node_files {
            upstream::init("1", &node_files.path().to_string_lossy());
        }
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
        let mut node_files = self.node_files.write().await;
        *node_files = None; // will remove temporary node files directory because TempDir is dropped
        Ok(())
    }
}

/// Handle GraphQL methods
#[async_trait]
impl GraphQL for GraphQLProvider {
    /// Execute the GraphQL query
    async fn query(&self, _ctx: &Context, req: &QueryRequest) -> RpcResult<QueryResponse> {
        match upstream::query("1", &req.query) {
            QueryResult::Ok(result) => Ok(QueryResponse { data: result }),
            QueryResult::Err(err) => Err(RpcError::MethodNotHandled(err)),
        }
    }
}
