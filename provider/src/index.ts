import express from "express";
import { postgraphile } from "postgraphile";
import request from "supertest";
import { options, schemas } from "./config";

interface ICallback {
  (id: string, error: Error | null, result?: string): void;
}

let agent: request.SuperAgentTest;
let graphqlRoute: string;
let graphiqlRoute: string;

export const init = (database: string) => {
  const app = express();
  const middleware = postgraphile(database, schemas, options);
  app.use(middleware);

  agent = request.agent(app);
  graphqlRoute = middleware.graphqlRoute;
  graphiqlRoute = middleware.graphiqlRoute;
};

export const query = (
  id: string,
  query: string,
  headers: Record<string, any>,
  cb: ICallback
) => {
  agent
    .post(graphqlRoute)
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
    .get(graphiqlRoute)
    .set("Content-Type", "text/html")
    .then((res) => {
      cb(id, null, res.text);
    })
    .catch((err: Error) => {
      cb(id, err);
    });
};
