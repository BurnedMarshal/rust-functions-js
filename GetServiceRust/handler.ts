/* eslint-disable extra-rules/no-commented-out-code */
import * as express from "express";
import * as E from "fp-ts/Either";
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
  ResponseErrorInternal,
  ResponseSuccessJson
} from "@pagopa/ts-commons/lib/responses";
import { NonEmptyString } from "@pagopa/ts-commons/lib/strings";
import { Config, getService } from "rust-module";
import { pipe } from "fp-ts/lib/function";
import { IConfig } from "../utils/config";
import { ServicePublic } from "../generated/definitions/ServicePublic";
import { withoutNullValues } from "./utils";
// eslint-disable-next-line @typescript-eslint/no-var-requires

type IHttpHandler = (
  context: Context,
  serviceId: NonEmptyString
) => Promise<
  | IResponseSuccessJson<ServicePublic>
  | IResponseErrorNotFound
  | IResponseErrorInternal
>;

export const HttpHandler = (_: IConfig): IHttpHandler => async (
  __,
  serviceId
): Promise<
  | IResponseSuccessJson<ServicePublic>
  | IResponseErrorNotFound
  | IResponseErrorInternal
> =>
  pipe(
    await getService(
      new Config(_.COSMOSDB_KEY, _.COSMOSDB_NAME, _.COSMOSDB_URI),
      serviceId
    ),
    withoutNullValues,
    ServicePublic.decode,
    E.map(ResponseSuccessJson),
    E.mapLeft(err => ResponseErrorInternal(`error decoding: ${err}`)),
    E.toUnion
  );

export const HttpCtrl = (config: IConfig): express.RequestHandler => {
  const handler = HttpHandler(config);

  const middlewaresWrap = withRequestMiddlewares(
    ContextMiddleware(),
    RequiredParamMiddleware("service_id", NonEmptyString)
  );

  return wrapRequestHandler(middlewaresWrap(handler));
};
