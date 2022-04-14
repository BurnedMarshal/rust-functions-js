import { isObject } from "@pagopa/ts-commons/lib/types";

/**
 * Return an object filtering out keys that point to undefined values.
 */
export const withoutNullValues = <T, K extends keyof T>(obj: T): T => {
  // note that T has been already validated by the type system and we can
  // be sure now that only attributes that may be undefined can be actually
  // filtered out by the following code, so the output type T is always
  // a valid T
  const keys = Object.keys(obj);
  return keys.reduce((acc, key) => {
    const value = obj[key as K];
    return value !== null
      ? {
          // see https://github.com/Microsoft/TypeScript/pull/13288
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          ...(acc as any),
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          [key]: isObject(value as any) ? withoutNullValues(value) : value
        }
      : acc;
  }, {} as T);
};
