import express from "express";
import { ParamsDictionary, RequestHandler } from "express-serve-static-core";
import { postgraphile } from "postgraphile";
import { ParsedQs } from "qs";
import request from "supertest";
import { options, schemas } from "./config";

interface ICallback {
  (error: Error | null, result?: string): void;
}

let middleware: RequestHandler<
  ParamsDictionary,
  { text: string },
  any,
  ParsedQs,
  Record<string, any>
> & { graphqlRoute: string };
let agent: request.SuperAgentTest;

export const init = (database: string) => {
  middleware = postgraphile(database, schemas, options);

  const app = express();
  app.use(middleware);
  agent = request.agent(app);
};

export const query = (query: string, cb: ICallback) => {
  agent
    .post(middleware.graphqlRoute)
    .set("Content-Type", "application/json")
    .send({ query })
    .expect(200)
    .expect("Content-Type", /json/)
    .then((res) => cb(null, res.text))
    .catch(cb);
};
