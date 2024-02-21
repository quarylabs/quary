/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { TestResult } from "./test_result";

export const protobufPackage = "quary.service.v1";

/**
 * WasmRunTestResponse is a temporary message type to work on inferring in Rust rather than in Typescript.
 * The goal is to make better interfaces over time.
 */
export interface TestResults {
  results: TestResult[];
}

function createBaseTestResults(): TestResults {
  return { results: [] };
}

export const TestResults = {
  encode(message: TestResults, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.results) {
      TestResult.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TestResults {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTestResults();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.results.push(TestResult.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): TestResults {
    return { results: gt.Array.isArray(object?.results) ? object.results.map((e: any) => TestResult.fromJSON(e)) : [] };
  },

  toJSON(message: TestResults): unknown {
    const obj: any = {};
    if (message.results?.length) {
      obj.results = message.results.map((e) => TestResult.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<TestResults>, I>>(base?: I): TestResults {
    return TestResults.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<TestResults>, I>>(object: I): TestResults {
    const message = createBaseTestResults();
    message.results = object.results?.map((e) => TestResult.fromPartial(e)) || [];
    return message;
  },
};

declare const self: any | undefined;
declare const window: any | undefined;
declare const global: any | undefined;
const gt: any = (() => {
  if (typeof globalThis !== "undefined") {
    return globalThis;
  }
  if (typeof self !== "undefined") {
    return self;
  }
  if (typeof window !== "undefined") {
    return window;
  }
  if (typeof global !== "undefined") {
    return global;
  }
  throw "Unable to locate global object";
})();

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends { $case: string } ? { [K in keyof Omit<T, "$case">]?: DeepPartial<T[K]> } & { $case: T["$case"] }
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };
