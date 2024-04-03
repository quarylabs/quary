/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "quary.service.v1";

/** TableAddress is a struct that represents a table in a database. It contains the name of the table and the full path. */
export interface TableAddress {
  name: string;
  fullPath: string;
}

function createBaseTableAddress(): TableAddress {
  return { name: "", fullPath: "" };
}

export const TableAddress = {
  encode(message: TableAddress, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.fullPath !== "") {
      writer.uint32(18).string(message.fullPath);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TableAddress {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTableAddress();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.fullPath = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): TableAddress {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      fullPath: isSet(object.fullPath) ? gt.String(object.fullPath) : "",
    };
  },

  toJSON(message: TableAddress): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.fullPath !== "") {
      obj.fullPath = message.fullPath;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<TableAddress>, I>>(base?: I): TableAddress {
    return TableAddress.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<TableAddress>, I>>(object: I): TableAddress {
    const message = createBaseTableAddress();
    message.name = object.name ?? "";
    message.fullPath = object.fullPath ?? "";
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

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
