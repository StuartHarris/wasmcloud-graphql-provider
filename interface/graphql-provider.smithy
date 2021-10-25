metadata package = [
  {
    namespace: "org.stuartharris.graphqlprovider",
    crate: "wasmcloud-graphql-interface"
  }
]

namespace org.stuartharris.graphqlprovider

use org.wasmcloud.model#wasmbus

@wasmbus(
  contractId: "stuart-harris:graphql-provider",
  providerReceive: true
)
service GraphQL {
  version: "0.1",
  operations: [ Query ]
}

@readonly
operation Query { 
  input: QueryRequest,
  output: QueryResponse
}

structure QueryRequest {
  @required
  query: String,

  headers: HeaderMap
}

structure QueryResponse {
  @required
  data: String
}

/// map data structure for holding http headers
///
map HeaderMap {
    key: String,
    value: HeaderValues,
}

list HeaderValues {
    member: String
}
