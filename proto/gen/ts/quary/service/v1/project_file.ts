/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "quary.service.v1";

export interface ProjectFile {
  sources: ProjectFile_Source[];
  models: ProjectFile_Model[];
}

/**
 * Standard types are:
 * - not_null
 * - unique
 * - 'relationship' which takes into data (model and field)
 */
export interface ProjectFile_Column {
  name: string;
  description?: string | undefined;
  tests: ColumnTest[];
}

export interface ProjectFile_Model {
  name: string;
  description?: string | undefined;
  columns: ProjectFile_Column[];
}

export interface ProjectFile_Source {
  name: string;
  description?:
    | string
    | undefined;
  /**
   * The full path of the source table in the database. This is used to reference the table itself. For example:
   * - 'public.users' for where the schema is 'public' and the table is 'users'
   * - 'project_id_123.dataset_id_123.table_id_123' for a BigQuery table
   */
  path: string;
  columns: ProjectFile_Column[];
}

export interface ColumnTest {
  type: string;
  info: { [key: string]: string };
}

export interface ColumnTest_InfoEntry {
  key: string;
  value: string;
}

function createBaseProjectFile(): ProjectFile {
  return { sources: [], models: [] };
}

export const ProjectFile = {
  encode(message: ProjectFile, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.sources) {
      ProjectFile_Source.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.models) {
      ProjectFile_Model.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFile {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFile();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.sources.push(ProjectFile_Source.decode(reader, reader.uint32()));
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.models.push(ProjectFile_Model.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFile {
    return {
      sources: gt.Array.isArray(object?.sources) ? object.sources.map((e: any) => ProjectFile_Source.fromJSON(e)) : [],
      models: gt.Array.isArray(object?.models) ? object.models.map((e: any) => ProjectFile_Model.fromJSON(e)) : [],
    };
  },

  toJSON(message: ProjectFile): unknown {
    const obj: any = {};
    if (message.sources?.length) {
      obj.sources = message.sources.map((e) => ProjectFile_Source.toJSON(e));
    }
    if (message.models?.length) {
      obj.models = message.models.map((e) => ProjectFile_Model.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile>, I>>(base?: I): ProjectFile {
    return ProjectFile.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile>, I>>(object: I): ProjectFile {
    const message = createBaseProjectFile();
    message.sources = object.sources?.map((e) => ProjectFile_Source.fromPartial(e)) || [];
    message.models = object.models?.map((e) => ProjectFile_Model.fromPartial(e)) || [];
    return message;
  },
};

function createBaseProjectFile_Column(): ProjectFile_Column {
  return { name: "", description: undefined, tests: [] };
}

export const ProjectFile_Column = {
  encode(message: ProjectFile_Column, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.description !== undefined) {
      writer.uint32(18).string(message.description);
    }
    for (const v of message.tests) {
      ColumnTest.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFile_Column {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFile_Column();
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

          message.tests.push(ColumnTest.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFile_Column {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      description: isSet(object.description) ? gt.String(object.description) : undefined,
      tests: gt.Array.isArray(object?.tests) ? object.tests.map((e: any) => ColumnTest.fromJSON(e)) : [],
    };
  },

  toJSON(message: ProjectFile_Column): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    if (message.tests?.length) {
      obj.tests = message.tests.map((e) => ColumnTest.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile_Column>, I>>(base?: I): ProjectFile_Column {
    return ProjectFile_Column.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile_Column>, I>>(object: I): ProjectFile_Column {
    const message = createBaseProjectFile_Column();
    message.name = object.name ?? "";
    message.description = object.description ?? undefined;
    message.tests = object.tests?.map((e) => ColumnTest.fromPartial(e)) || [];
    return message;
  },
};

function createBaseProjectFile_Model(): ProjectFile_Model {
  return { name: "", description: undefined, columns: [] };
}

export const ProjectFile_Model = {
  encode(message: ProjectFile_Model, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.description !== undefined) {
      writer.uint32(18).string(message.description);
    }
    for (const v of message.columns) {
      ProjectFile_Column.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFile_Model {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFile_Model();
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

          message.columns.push(ProjectFile_Column.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFile_Model {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      description: isSet(object.description) ? gt.String(object.description) : undefined,
      columns: gt.Array.isArray(object?.columns) ? object.columns.map((e: any) => ProjectFile_Column.fromJSON(e)) : [],
    };
  },

  toJSON(message: ProjectFile_Model): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    if (message.columns?.length) {
      obj.columns = message.columns.map((e) => ProjectFile_Column.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile_Model>, I>>(base?: I): ProjectFile_Model {
    return ProjectFile_Model.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile_Model>, I>>(object: I): ProjectFile_Model {
    const message = createBaseProjectFile_Model();
    message.name = object.name ?? "";
    message.description = object.description ?? undefined;
    message.columns = object.columns?.map((e) => ProjectFile_Column.fromPartial(e)) || [];
    return message;
  },
};

function createBaseProjectFile_Source(): ProjectFile_Source {
  return { name: "", description: undefined, path: "", columns: [] };
}

export const ProjectFile_Source = {
  encode(message: ProjectFile_Source, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.description !== undefined) {
      writer.uint32(18).string(message.description);
    }
    if (message.path !== "") {
      writer.uint32(26).string(message.path);
    }
    for (const v of message.columns) {
      ProjectFile_Column.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFile_Source {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFile_Source();
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

          message.path = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.columns.push(ProjectFile_Column.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFile_Source {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      description: isSet(object.description) ? gt.String(object.description) : undefined,
      path: isSet(object.path) ? gt.String(object.path) : "",
      columns: gt.Array.isArray(object?.columns) ? object.columns.map((e: any) => ProjectFile_Column.fromJSON(e)) : [],
    };
  },

  toJSON(message: ProjectFile_Source): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    if (message.path !== "") {
      obj.path = message.path;
    }
    if (message.columns?.length) {
      obj.columns = message.columns.map((e) => ProjectFile_Column.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile_Source>, I>>(base?: I): ProjectFile_Source {
    return ProjectFile_Source.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile_Source>, I>>(object: I): ProjectFile_Source {
    const message = createBaseProjectFile_Source();
    message.name = object.name ?? "";
    message.description = object.description ?? undefined;
    message.path = object.path ?? "";
    message.columns = object.columns?.map((e) => ProjectFile_Column.fromPartial(e)) || [];
    return message;
  },
};

function createBaseColumnTest(): ColumnTest {
  return { type: "", info: {} };
}

export const ColumnTest = {
  encode(message: ColumnTest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.type !== "") {
      writer.uint32(10).string(message.type);
    }
    Object.entries(message.info).forEach(([key, value]) => {
      ColumnTest_InfoEntry.encode({ key: key as any, value }, writer.uint32(18).fork()).ldelim();
    });
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ColumnTest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseColumnTest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.type = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          const entry2 = ColumnTest_InfoEntry.decode(reader, reader.uint32());
          if (entry2.value !== undefined) {
            message.info[entry2.key] = entry2.value;
          }
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ColumnTest {
    return {
      type: isSet(object.type) ? gt.String(object.type) : "",
      info: isObject(object.info)
        ? Object.entries(object.info).reduce<{ [key: string]: string }>((acc, [key, value]) => {
          acc[key] = String(value);
          return acc;
        }, {})
        : {},
    };
  },

  toJSON(message: ColumnTest): unknown {
    const obj: any = {};
    if (message.type !== "") {
      obj.type = message.type;
    }
    if (message.info) {
      const entries = Object.entries(message.info);
      if (entries.length > 0) {
        obj.info = {};
        entries.forEach(([k, v]) => {
          obj.info[k] = v;
        });
      }
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ColumnTest>, I>>(base?: I): ColumnTest {
    return ColumnTest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ColumnTest>, I>>(object: I): ColumnTest {
    const message = createBaseColumnTest();
    message.type = object.type ?? "";
    message.info = Object.entries(object.info ?? {}).reduce<{ [key: string]: string }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = gt.String(value);
      }
      return acc;
    }, {});
    return message;
  },
};

function createBaseColumnTest_InfoEntry(): ColumnTest_InfoEntry {
  return { key: "", value: "" };
}

export const ColumnTest_InfoEntry = {
  encode(message: ColumnTest_InfoEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ColumnTest_InfoEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseColumnTest_InfoEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.key = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.value = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ColumnTest_InfoEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? gt.String(object.value) : "",
    };
  },

  toJSON(message: ColumnTest_InfoEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ColumnTest_InfoEntry>, I>>(base?: I): ColumnTest_InfoEntry {
    return ColumnTest_InfoEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ColumnTest_InfoEntry>, I>>(object: I): ColumnTest_InfoEntry {
    const message = createBaseColumnTest_InfoEntry();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
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
