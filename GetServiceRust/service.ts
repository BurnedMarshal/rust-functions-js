import * as t from "io-ts";
import {NonEmptyString} from "@pagopa/ts-commons/lib/strings";

export const Service = t.interface({
    serviceName: NonEmptyString,
    organizationName:NonEmptyString
});

export type Service = t.TypeOf<typeof Service>;