import manyToMany from "@graphile-contrib/pg-many-to-many";
import inflector from "@graphile-contrib/pg-simplify-inflector";
import { PostGraphileOptions } from "postgraphile";
import filter from "postgraphile-plugin-connection-filter";

// Database schemas to use
export const schemas: string | string[] = ["public"];

// PostGraphile options; see https://www.graphile.org/postgraphile/usage-library/#api-postgraphilepgconfig-schemaname-options
export const options: PostGraphileOptions = {
  allowExplain: false,
  appendPlugins: [inflector, manyToMany, filter],
  dynamicJson: true,
  enableQueryBatching: true,
  enhanceGraphiql: true,
  extendedErrors: ["hint", "detail", "errcode"],
  graphiql: true,
  ignoreIndexes: false,
  ignoreRBAC: false,
  legacyRelations: "omit",
  setofFunctionsContainNulls: false,
  showErrorStack: "json",
  sortExport: true,
  subscriptions: false,
  watchPg: true,
  pgSettings(req) {
    // Adding this to ensure that all servers pass through the request in a
    // good enough way that we can extract headers.
    // CREATE FUNCTION current_user_id() RETURNS text AS $$ SELECT current_setting('graphile.test.x-user-id', TRUE); $$ LANGUAGE sql STABLE;
    return {
      role:
        req.headers["role"] ||
        // `normalizedConnectionParams` comes from websocket connections, where
        // the headers often cannot be customized by the client.
        (req as any).normalizedConnectionParams?.["role"],
      "graphile.test.x-user-id":
        req.headers["x-user-id"] ||
        (req as any).normalizedConnectionParams?.["x-user-id"],
    };
  },
};
