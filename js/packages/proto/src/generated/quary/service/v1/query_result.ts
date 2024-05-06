/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "quary.service.v1";

/** QueryResult is the result of a ran query. */
export interface QueryResult {
  columns: QueryResultColumn[];
}

export interface QueryResultColumn {
  name: string;
  type?: string | undefined;
  values: string[];
}

function createBaseQueryResult(): QueryResult {
  return { columns: [] };
}

export const QueryResult = {
  encode(message: QueryResult, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.columns) {
      QueryResultColumn.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryResult {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryResult();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.columns.push(QueryResultColumn.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): QueryResult {
    return {
      columns: gt.Array.isArray(object?.columns) ? object.columns.map((e: any) => QueryResultColumn.fromJSON(e)) : [],
    };
  },

  toJSON(message: QueryResult): unknown {
    const obj: any = {};
    if (message.columns?.length) {
      obj.columns = message.columns.map((e) => QueryResultColumn.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryResult>, I>>(base?: I): QueryResult {
    return QueryResult.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryResult>, I>>(object: I): QueryResult {
    const message = createBaseQueryResult();
    message.columns = object.columns?.map((e) => QueryResultColumn.fromPartial(e)) || [];
    return message;
  },
};

function createBaseQueryResultColumn(): QueryResultColumn {
  return { name: "", type: undefined, values: [] };
}

export const QueryResultColumn = {
  encode(message: QueryResultColumn, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.type !== undefined) {
      writer.uint32(26).string(message.type);
    }
    for (const v of message.values) {
      writer.uint32(18).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryResultColumn {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryResultColumn();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.type = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.values.push(reader.string());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): QueryResultColumn {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      type: isSet(object.type) ? gt.String(object.type) : undefined,
      values: gt.Array.isArray(object?.values) ? object.values.map((e: any) => gt.String(e)) : [],
    };
  },

  toJSON(message: QueryResultColumn): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.type !== undefined) {
      obj.type = message.type;
    }
    if (message.values?.length) {
      obj.values = message.values;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryResultColumn>, I>>(base?: I): QueryResultColumn {
    return QueryResultColumn.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryResultColumn>, I>>(object: I): QueryResultColumn {
    const message = createBaseQueryResultColumn();
    message.name = object.name ?? "";
    message.type = object.type ?? undefined;
    message.values = object.values?.map((e) => e) || [];
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
