// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

pub type HeaderMap = std::collections::HashMap<String, HeaderValues>;

pub type HeaderValues = Vec<String>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct QueryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<HeaderMap>,
    #[serde(default)]
    pub query: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct QueryResponse {
    #[serde(default)]
    pub data: String,
}

/// wasmbus.contractId: stuart-harris:graphql-provider
/// wasmbus.providerReceive
#[async_trait]
pub trait GraphQL {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "stuart-harris:graphql-provider"
    }
    async fn query(&self, ctx: &Context, arg: &QueryRequest) -> RpcResult<QueryResponse>;
    async fn graphiql(&self, ctx: &Context) -> RpcResult<QueryResponse>;
}

/// GraphQLReceiver receives messages defined in the GraphQL service trait
#[doc(hidden)]
#[async_trait]
pub trait GraphQLReceiver: MessageDispatch + GraphQL {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Query" => {
                let value: QueryRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = GraphQL::query(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "GraphQL.Query",
                    arg: Cow::Owned(buf),
                })
            }
            "Graphiql" => {
                let resp = GraphQL::graphiql(self, ctx).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "GraphQL.Graphiql",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "GraphQL::{}",
                message.method
            ))),
        }
    }
}

/// GraphQLSender sends messages to a GraphQL service
/// client for sending GraphQL messages
#[derive(Debug)]
pub struct GraphQLSender<T: Transport> {
    transport: T,
}

impl<T: Transport> GraphQLSender<T> {
    /// Constructs a GraphQLSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl GraphQLSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a GraphQL provider
    /// implementing the 'stuart-harris:graphql-provider' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "stuart-harris:graphql-provider",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a GraphQL provider
    /// implementing the 'stuart-harris:graphql-provider' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "stuart-harris:graphql-provider",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> GraphQL for GraphQLSender<T> {
    #[allow(unused)]
    async fn query(&self, ctx: &Context, arg: &QueryRequest) -> RpcResult<QueryResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "GraphQL.Query",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Query", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn graphiql(&self, ctx: &Context) -> RpcResult<QueryResponse> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "GraphQL.Graphiql",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Graphiql", e)))?;
        Ok(value)
    }
}
