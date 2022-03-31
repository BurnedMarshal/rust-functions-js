import * as express from "express";

import { Context } from "@azure/functions";
import { ContextMiddleware } from "@pagopa/io-functions-commons/dist/src/utils/middlewares/context_middleware";
import { RequiredParamMiddleware } from "@pagopa/io-functions-commons/dist/src/utils/middlewares/required_param";
import {
  withRequestMiddlewares,
  wrapRequestHandler
} from "@pagopa/io-functions-commons/dist/src/utils/request_middleware";
import {
  IResponseErrorInternal,
  IResponseErrorNotFound,
  IResponseSuccessJson,
  ResponseSuccessJson
} from "@pagopa/ts-commons/lib/responses";
import { NonEmptyString } from "@pagopa/ts-commons/lib/strings";
import { getService, Config } from "rust-module";
import { IConfig } from "../utils/config";
// eslint-disable-next-line @typescript-eslint/no-var-requires

type IHttpHandler = (
  context: Context,
  serviceId: NonEmptyString
) => Promise<
  | IResponseSuccessJson<{ readonly value: string }>
  | IResponseErrorNotFound
  | IResponseErrorInternal
>;

export const HttpHandler = (_: IConfig): IHttpHandler => async (
  __,
  serviceId
): Promise<
  | IResponseSuccessJson<{ readonly value: string }>
  | IResponseErrorNotFound
  | IResponseErrorInternal
> => {
  const r = await getService(
    new Config(_.COSMOSDB_KEY, _.COSMOSDB_NAME, _.COSMOSDB_URI),
    serviceId
  );
  return Promise.resolve(ResponseSuccessJson({ value: r }));
};

export const HttpCtrl = (config: IConfig): express.RequestHandler => {
  const handler = HttpHandler(config);

  const middlewaresWrap = withRequestMiddlewares(
    ContextMiddleware(),
    RequiredParamMiddleware("service_id", NonEmptyString)
  );

  return wrapRequestHandler(middlewaresWrap(handler));
};
