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

operation Query { 
  input: Request
}

structure Request {
  @required
  query: String,

  variables: String,

  headers: String
}
