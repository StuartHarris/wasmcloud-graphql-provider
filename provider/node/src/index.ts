import express from "express";
import { ParamsDictionary, RequestHandler } from "express-serve-static-core";
import { postgraphile } from "postgraphile";
import { ParsedQs } from "qs";
import request from "supertest";
import { options, schemas } from "./config";

interface ICallback {
  (id: string, error: Error | null, result?: string): void;
}

let middleware: RequestHandler<
  ParamsDictionary,
  { text: string },
  any,
  ParsedQs,
  Record<string, any>
> & { graphqlRoute: string; graphiqlRoute: string };
let agent: request.SuperAgentTest;
const app = express();

export const init = (database: string) => {
  console.log(
    `postgraphile initializing at ${database.replace(/:.*@/, ":****@")}`
  );
  middleware = postgraphile(database, schemas, options);
  app.use(middleware);
  agent = request.agent(app);
  console.log("postgraphile initialized");
};

export const query = (
  id: string,
  query: string,
  headers: Record<string, any>,
  cb: ICallback
) => {
  agent
    .post(middleware.graphqlRoute)
    .set(headers)
    .set("Content-Type", "application/json")
    .send(query)
    .then((res) => {
      cb(id, null, res.text);
    })
    .catch((err: Error) => {
      cb(id, err);
    });
};

export const graphiql = (id: string, cb: ICallback) => {
  agent
    .get(middleware.graphiqlRoute)
    .set("Content-Type", "text/html")
    .then((res) => {
      cb(id, null, res.text);
    })
    .catch((err: Error) => {
      cb(id, err);
    });
};
