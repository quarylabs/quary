/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { ProjectFileSource } from "./project_file";
import { QueryResult } from "./query_result";
import { TableAddress } from "./table_address";

export const protobufPackage = "quary.service.v1";

export interface ListTablesRequest {
}

export interface ListTablesResponse {
  tables: TableAddress[];
}

export interface ListViewsRequest {
}

export interface ListViewsResponse {
  views: TableAddress[];
}

export interface ExecRequest {
  query: string;
}

export interface ExecResponse {
}

export interface QueryRequest {
  query: string;
}

export interface QueryResponse {
  result: QueryResult | undefined;
}

export interface ListColumnsRequest {
  tableName: string;
}

export interface ListColumnsResponse {
  columns: string[];
}

export interface ListSourcesRequest {
}

export interface ListSourcesResponse {
  sources: ProjectFileSource[];
}

function createBaseListTablesRequest(): ListTablesRequest {
  return {};
}

export const ListTablesRequest = {
  encode(_: ListTablesRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListTablesRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListTablesRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): ListTablesRequest {
    return {};
  },

  toJSON(_: ListTablesRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<ListTablesRequest>, I>>(base?: I): ListTablesRequest {
    return ListTablesRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListTablesRequest>, I>>(_: I): ListTablesRequest {
    const message = createBaseListTablesRequest();
    return message;
  },
};

function createBaseListTablesResponse(): ListTablesResponse {
  return { tables: [] };
}

export const ListTablesResponse = {
  encode(message: ListTablesResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.tables) {
      TableAddress.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListTablesResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListTablesResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.tables.push(TableAddress.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ListTablesResponse {
    return { tables: gt.Array.isArray(object?.tables) ? object.tables.map((e: any) => TableAddress.fromJSON(e)) : [] };
  },

  toJSON(message: ListTablesResponse): unknown {
    const obj: any = {};
    if (message.tables?.length) {
      obj.tables = message.tables.map((e) => TableAddress.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ListTablesResponse>, I>>(base?: I): ListTablesResponse {
    return ListTablesResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListTablesResponse>, I>>(object: I): ListTablesResponse {
    const message = createBaseListTablesResponse();
    message.tables = object.tables?.map((e) => TableAddress.fromPartial(e)) || [];
    return message;
  },
};

function createBaseListViewsRequest(): ListViewsRequest {
  return {};
}

export const ListViewsRequest = {
  encode(_: ListViewsRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListViewsRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListViewsRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): ListViewsRequest {
    return {};
  },

  toJSON(_: ListViewsRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<ListViewsRequest>, I>>(base?: I): ListViewsRequest {
    return ListViewsRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListViewsRequest>, I>>(_: I): ListViewsRequest {
    const message = createBaseListViewsRequest();
    return message;
  },
};

function createBaseListViewsResponse(): ListViewsResponse {
  return { views: [] };
}

export const ListViewsResponse = {
  encode(message: ListViewsResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.views) {
      TableAddress.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListViewsResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListViewsResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.views.push(TableAddress.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ListViewsResponse {
    return { views: gt.Array.isArray(object?.views) ? object.views.map((e: any) => TableAddress.fromJSON(e)) : [] };
  },

  toJSON(message: ListViewsResponse): unknown {
    const obj: any = {};
    if (message.views?.length) {
      obj.views = message.views.map((e) => TableAddress.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ListViewsResponse>, I>>(base?: I): ListViewsResponse {
    return ListViewsResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListViewsResponse>, I>>(object: I): ListViewsResponse {
    const message = createBaseListViewsResponse();
    message.views = object.views?.map((e) => TableAddress.fromPartial(e)) || [];
    return message;
  },
};

function createBaseExecRequest(): ExecRequest {
  return { query: "" };
}

export const ExecRequest = {
  encode(message: ExecRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.query !== "") {
      writer.uint32(10).string(message.query);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ExecRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseExecRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.query = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ExecRequest {
    return { query: isSet(object.query) ? gt.String(object.query) : "" };
  },

  toJSON(message: ExecRequest): unknown {
    const obj: any = {};
    if (message.query !== "") {
      obj.query = message.query;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ExecRequest>, I>>(base?: I): ExecRequest {
    return ExecRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ExecRequest>, I>>(object: I): ExecRequest {
    const message = createBaseExecRequest();
    message.query = object.query ?? "";
    return message;
  },
};

function createBaseExecResponse(): ExecResponse {
  return {};
}

export const ExecResponse = {
  encode(_: ExecResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ExecResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseExecResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): ExecResponse {
    return {};
  },

  toJSON(_: ExecResponse): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<ExecResponse>, I>>(base?: I): ExecResponse {
    return ExecResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ExecResponse>, I>>(_: I): ExecResponse {
    const message = createBaseExecResponse();
    return message;
  },
};

function createBaseQueryRequest(): QueryRequest {
  return { query: "" };
}

export const QueryRequest = {
  encode(message: QueryRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.query !== "") {
      writer.uint32(10).string(message.query);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.query = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): QueryRequest {
    return { query: isSet(object.query) ? gt.String(object.query) : "" };
  },

  toJSON(message: QueryRequest): unknown {
    const obj: any = {};
    if (message.query !== "") {
      obj.query = message.query;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryRequest>, I>>(base?: I): QueryRequest {
    return QueryRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryRequest>, I>>(object: I): QueryRequest {
    const message = createBaseQueryRequest();
    message.query = object.query ?? "";
    return message;
  },
};

function createBaseQueryResponse(): QueryResponse {
  return { result: undefined };
}

export const QueryResponse = {
  encode(message: QueryResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.result !== undefined) {
      QueryResult.encode(message.result, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): QueryResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseQueryResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.result = QueryResult.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): QueryResponse {
    return { result: isSet(object.result) ? QueryResult.fromJSON(object.result) : undefined };
  },

  toJSON(message: QueryResponse): unknown {
    const obj: any = {};
    if (message.result !== undefined) {
      obj.result = QueryResult.toJSON(message.result);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<QueryResponse>, I>>(base?: I): QueryResponse {
    return QueryResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<QueryResponse>, I>>(object: I): QueryResponse {
    const message = createBaseQueryResponse();
    message.result = (object.result !== undefined && object.result !== null)
      ? QueryResult.fromPartial(object.result)
      : undefined;
    return message;
  },
};

function createBaseListColumnsRequest(): ListColumnsRequest {
  return { tableName: "" };
}

export const ListColumnsRequest = {
  encode(message: ListColumnsRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.tableName !== "") {
      writer.uint32(10).string(message.tableName);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListColumnsRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListColumnsRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.tableName = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ListColumnsRequest {
    return { tableName: isSet(object.tableName) ? gt.String(object.tableName) : "" };
  },

  toJSON(message: ListColumnsRequest): unknown {
    const obj: any = {};
    if (message.tableName !== "") {
      obj.tableName = message.tableName;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ListColumnsRequest>, I>>(base?: I): ListColumnsRequest {
    return ListColumnsRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListColumnsRequest>, I>>(object: I): ListColumnsRequest {
    const message = createBaseListColumnsRequest();
    message.tableName = object.tableName ?? "";
    return message;
  },
};

function createBaseListColumnsResponse(): ListColumnsResponse {
  return { columns: [] };
}

export const ListColumnsResponse = {
  encode(message: ListColumnsResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.columns) {
      writer.uint32(10).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListColumnsResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListColumnsResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.columns.push(reader.string());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ListColumnsResponse {
    return { columns: gt.Array.isArray(object?.columns) ? object.columns.map((e: any) => gt.String(e)) : [] };
  },

  toJSON(message: ListColumnsResponse): unknown {
    const obj: any = {};
    if (message.columns?.length) {
      obj.columns = message.columns;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ListColumnsResponse>, I>>(base?: I): ListColumnsResponse {
    return ListColumnsResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListColumnsResponse>, I>>(object: I): ListColumnsResponse {
    const message = createBaseListColumnsResponse();
    message.columns = object.columns?.map((e) => e) || [];
    return message;
  },
};

function createBaseListSourcesRequest(): ListSourcesRequest {
  return {};
}

export const ListSourcesRequest = {
  encode(_: ListSourcesRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListSourcesRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListSourcesRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): ListSourcesRequest {
    return {};
  },

  toJSON(_: ListSourcesRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<ListSourcesRequest>, I>>(base?: I): ListSourcesRequest {
    return ListSourcesRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListSourcesRequest>, I>>(_: I): ListSourcesRequest {
    const message = createBaseListSourcesRequest();
    return message;
  },
};

function createBaseListSourcesResponse(): ListSourcesResponse {
  return { sources: [] };
}

export const ListSourcesResponse = {
  encode(message: ListSourcesResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.sources) {
      ProjectFileSource.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListSourcesResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListSourcesResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.sources.push(ProjectFileSource.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ListSourcesResponse {
    return {
      sources: gt.Array.isArray(object?.sources) ? object.sources.map((e: any) => ProjectFileSource.fromJSON(e)) : [],
    };
  },

  toJSON(message: ListSourcesResponse): unknown {
    const obj: any = {};
    if (message.sources?.length) {
      obj.sources = message.sources.map((e) => ProjectFileSource.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ListSourcesResponse>, I>>(base?: I): ListSourcesResponse {
    return ListSourcesResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ListSourcesResponse>, I>>(object: I): ListSourcesResponse {
    const message = createBaseListSourcesResponse();
    message.sources = object.sources?.map((e) => ProjectFileSource.fromPartial(e)) || [];
    return message;
  },
};

export interface CLIRPCService {
  ListTables(request: ListTablesRequest): Promise<ListTablesResponse>;
  ListViews(request: ListViewsRequest): Promise<ListViewsResponse>;
  Exec(request: ExecRequest): Promise<ExecResponse>;
  Query(request: QueryRequest): Promise<QueryResponse>;
  ListColumns(request: ListColumnsRequest): Promise<ListColumnsResponse>;
  ListSources(request: ListSourcesRequest): Promise<ListSourcesResponse>;
}

export const CLIRPCServiceServiceName = "quary.service.v1.CLIRPCService";
export class CLIRPCServiceClientImpl implements CLIRPCService {
  private readonly rpc: Rpc;
  private readonly service: string;
  constructor(rpc: Rpc, opts?: { service?: string }) {
    this.service = opts?.service || CLIRPCServiceServiceName;
    this.rpc = rpc;
    this.ListTables = this.ListTables.bind(this);
    this.ListViews = this.ListViews.bind(this);
    this.Exec = this.Exec.bind(this);
    this.Query = this.Query.bind(this);
    this.ListColumns = this.ListColumns.bind(this);
    this.ListSources = this.ListSources.bind(this);
  }
  ListTables(request: ListTablesRequest): Promise<ListTablesResponse> {
    const data = ListTablesRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "ListTables", data);
    return promise.then((data) => ListTablesResponse.decode(_m0.Reader.create(data)));
  }

  ListViews(request: ListViewsRequest): Promise<ListViewsResponse> {
    const data = ListViewsRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "ListViews", data);
    return promise.then((data) => ListViewsResponse.decode(_m0.Reader.create(data)));
  }

  Exec(request: ExecRequest): Promise<ExecResponse> {
    const data = ExecRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "Exec", data);
    return promise.then((data) => ExecResponse.decode(_m0.Reader.create(data)));
  }

  Query(request: QueryRequest): Promise<QueryResponse> {
    const data = QueryRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "Query", data);
    return promise.then((data) => QueryResponse.decode(_m0.Reader.create(data)));
  }

  ListColumns(request: ListColumnsRequest): Promise<ListColumnsResponse> {
    const data = ListColumnsRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "ListColumns", data);
    return promise.then((data) => ListColumnsResponse.decode(_m0.Reader.create(data)));
  }

  ListSources(request: ListSourcesRequest): Promise<ListSourcesResponse> {
    const data = ListSourcesRequest.encode(request).finish();
    const promise = this.rpc.request(this.service, "ListSources", data);
    return promise.then((data) => ListSourcesResponse.decode(_m0.Reader.create(data)));
  }
}

interface Rpc {
  request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}

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
