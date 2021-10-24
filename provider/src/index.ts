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
const app = express();

export const init = (database: string) => {
  middleware = postgraphile(database, schemas, options);
  app.use(middleware);
  agent = request.agent(app);
};

export const query = (query: string, cb: ICallback) => {
  agent
    .post(middleware.graphqlRoute)
    .set("Content-Type", "application/json")
    .send(query)
    .then((res) => {
      let text = res.text;
      console.log({ text });
      cb(null, text);
    })
    .catch((err: Error) => {
      console.log({ err });
      cb(err);
    });
};
