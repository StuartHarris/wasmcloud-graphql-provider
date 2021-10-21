import express from "express";
import { postgraphile } from "postgraphile";
import request from "supertest";
import { database, options, schemas } from "./config";

interface ICallback {
  (error: Error | null, result?: string): void;
}

export const middleware = postgraphile(database, schemas, options);

const app = express();
app.use(middleware);
const agent = request.agent(app);

export const run = (query: string, cb: ICallback) => {
  agent
    .post(middleware.graphqlRoute)
    .set("Content-Type", "application/json")
    .send({ query })
    .expect(200)
    .expect("Content-Type", /json/)
    .then((res) => cb(null, res.text))
    .catch(cb);
};
