/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Struct } from "../../../google/protobuf/struct";

export const protobufPackage = "quary.service.v1";

/** Parsed chart that is in a project */
export interface Chart {
  /** Name of the chart */
  name: string;
  /** Description of the cart */
  description?:
    | string
    | undefined;
  /**
   * Tags are used to group different parts of the project together. For example, you could tag all models that are
   * related to a specific department with the same tag.
   */
  tags: string[];
  /** Path of the file */
  path: string;
  source?:
    | { $case: "rawSql"; rawSql: string }
    | { $case: "preTemplatedSql"; preTemplatedSql: string }
    | { $case: "reference"; reference: Chart_AssetReference }
    | undefined;
  /** Configuration for the chart that is passed to perspective */
  config:
    | { [key: string]: any }
    | undefined;
  /** References that this chart has, these can be models/sources/seeds/snapshots */
  references: string[];
}

export interface Chart_AssetReference {
  name: string;
}

function createBaseChart(): Chart {
  return { name: "", description: undefined, tags: [], path: "", source: undefined, config: undefined, references: [] };
}

export const Chart = {
  encode(message: Chart, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.description !== undefined) {
      writer.uint32(18).string(message.description);
    }
    for (const v of message.tags) {
      writer.uint32(26).string(v!);
    }
    if (message.path !== "") {
      writer.uint32(74).string(message.path);
    }
    switch (message.source?.$case) {
      case "rawSql":
        writer.uint32(34).string(message.source.rawSql);
        break;
      case "preTemplatedSql":
        writer.uint32(42).string(message.source.preTemplatedSql);
        break;
      case "reference":
        Chart_AssetReference.encode(message.source.reference, writer.uint32(50).fork()).ldelim();
        break;
    }
    if (message.config !== undefined) {
      Struct.encode(Struct.wrap(message.config), writer.uint32(58).fork()).ldelim();
    }
    for (const v of message.references) {
      writer.uint32(66).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Chart {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseChart();
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

          message.description = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.tags.push(reader.string());
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          message.path = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.source = { $case: "rawSql", rawSql: reader.string() };
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.source = { $case: "preTemplatedSql", preTemplatedSql: reader.string() };
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.source = { $case: "reference", reference: Chart_AssetReference.decode(reader, reader.uint32()) };
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          message.config = Struct.unwrap(Struct.decode(reader, reader.uint32()));
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          message.references.push(reader.string());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Chart {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      description: isSet(object.description) ? gt.String(object.description) : undefined,
      tags: gt.Array.isArray(object?.tags) ? object.tags.map((e: any) => gt.String(e)) : [],
      path: isSet(object.path) ? gt.String(object.path) : "",
      source: isSet(object.rawSql)
        ? { $case: "rawSql", rawSql: gt.String(object.rawSql) }
        : isSet(object.preTemplatedSql)
        ? { $case: "preTemplatedSql", preTemplatedSql: gt.String(object.preTemplatedSql) }
        : isSet(object.reference)
        ? { $case: "reference", reference: Chart_AssetReference.fromJSON(object.reference) }
        : undefined,
      config: isObject(object.config) ? object.config : undefined,
      references: gt.Array.isArray(object?.references) ? object.references.map((e: any) => gt.String(e)) : [],
    };
  },

  toJSON(message: Chart): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    if (message.tags?.length) {
      obj.tags = message.tags;
    }
    if (message.path !== "") {
      obj.path = message.path;
    }
    if (message.source?.$case === "rawSql") {
      obj.rawSql = message.source.rawSql;
    }
    if (message.source?.$case === "preTemplatedSql") {
      obj.preTemplatedSql = message.source.preTemplatedSql;
    }
    if (message.source?.$case === "reference") {
      obj.reference = Chart_AssetReference.toJSON(message.source.reference);
    }
    if (message.config !== undefined) {
      obj.config = message.config;
    }
    if (message.references?.length) {
      obj.references = message.references;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Chart>, I>>(base?: I): Chart {
    return Chart.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Chart>, I>>(object: I): Chart {
    const message = createBaseChart();
    message.name = object.name ?? "";
    message.description = object.description ?? undefined;
    message.tags = object.tags?.map((e) => e) || [];
    message.path = object.path ?? "";
    if (object.source?.$case === "rawSql" && object.source?.rawSql !== undefined && object.source?.rawSql !== null) {
      message.source = { $case: "rawSql", rawSql: object.source.rawSql };
    }
    if (
      object.source?.$case === "preTemplatedSql" &&
      object.source?.preTemplatedSql !== undefined &&
      object.source?.preTemplatedSql !== null
    ) {
      message.source = { $case: "preTemplatedSql", preTemplatedSql: object.source.preTemplatedSql };
    }
    if (
      object.source?.$case === "reference" &&
      object.source?.reference !== undefined &&
      object.source?.reference !== null
    ) {
      message.source = { $case: "reference", reference: Chart_AssetReference.fromPartial(object.source.reference) };
    }
    message.config = object.config ?? undefined;
    message.references = object.references?.map((e) => e) || [];
    return message;
  },
};

function createBaseChart_AssetReference(): Chart_AssetReference {
  return { name: "" };
}

export const Chart_AssetReference = {
  encode(message: Chart_AssetReference, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Chart_AssetReference {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseChart_AssetReference();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Chart_AssetReference {
    return { name: isSet(object.name) ? gt.String(object.name) : "" };
  },

  toJSON(message: Chart_AssetReference): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Chart_AssetReference>, I>>(base?: I): Chart_AssetReference {
    return Chart_AssetReference.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Chart_AssetReference>, I>>(object: I): Chart_AssetReference {
    const message = createBaseChart_AssetReference();
    message.name = object.name ?? "";
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

function isObject(value: any): boolean {
  return typeof value === "object" && value !== null;
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
