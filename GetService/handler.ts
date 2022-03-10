import * as express from "express";
import * as TE from "fp-ts/TaskEither";

import { Context } from "@azure/functions";
import { ServiceModel } from "@pagopa/io-functions-commons/dist/src/models/service";
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
  ResponseErrorNotFound,
  ResponseSuccessJson
} from "@pagopa/ts-commons/lib/responses";
import { NonEmptyString } from "@pagopa/ts-commons/lib/strings";
import { pipe } from "fp-ts/lib/function";
import { ServicePublic } from "../generated/definitions/ServicePublic";
import { retrievedServiceToPublic } from "../utils/services";

type IHttpHandler = (
  context: Context,
  serviceId: NonEmptyString
) => Promise<
  | IResponseSuccessJson<ServicePublic>
  | IResponseErrorNotFound
  | IResponseErrorInternal
>;

export const HttpHandler = (serviceModel: ServiceModel): IHttpHandler => async (
  _,
  serviceId
): Promise<
  | IResponseSuccessJson<ServicePublic>
  | IResponseErrorNotFound
  | IResponseErrorInternal
> =>
  pipe(
    serviceModel.findLastVersionByModelId([serviceId]),
    TE.mapLeft(() => ResponseErrorInternal("Internal Error")),
    TE.chainW(
      TE.fromOption(() =>
        ResponseErrorNotFound("Not Found", "Missing document")
      )
    ),
    TE.map(retrievedServiceToPublic),
    TE.map(ResponseSuccessJson),
    TE.toUnion
  )();

export const HttpCtrl = (
  serviceModel: ServiceModel
): express.RequestHandler => {
  const handler = HttpHandler(serviceModel);

  const middlewaresWrap = withRequestMiddlewares(
    ContextMiddleware(),
    RequiredParamMiddleware("service_id", NonEmptyString)
  );

  return wrapRequestHandler(middlewaresWrap(handler));
};
