/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { ConnectionConfig } from "./connection_config";
import { ProjectFile } from "./project_file";
import { Model, Seed, Snapshot, Source, Test } from "./types";

export const protobufPackage = "quary.service.v1";

export interface Project {
  seeds: { [key: string]: Seed };
  models: { [key: string]: Model };
  tests: { [key: string]: Test };
  sources: { [key: string]: Source };
  snapshots: { [key: string]: Snapshot };
  projectFiles: { [key: string]: ProjectFile };
  connectionConfig: ConnectionConfig | undefined;
}

export interface Project_SeedsEntry {
  key: string;
  value: Seed | undefined;
}

export interface Project_ModelsEntry {
  key: string;
  value: Model | undefined;
}

export interface Project_TestsEntry {
  key: string;
  value: Test | undefined;
}

export interface Project_SourcesEntry {
  key: string;
  value: Source | undefined;
}

export interface Project_SnapshotsEntry {
  key: string;
  value: Snapshot | undefined;
}

export interface Project_ProjectFilesEntry {
  key: string;
  value: ProjectFile | undefined;
}

function createBaseProject(): Project {
  return {
    seeds: {},
    models: {},
    tests: {},
    sources: {},
    snapshots: {},
    projectFiles: {},
    connectionConfig: undefined,
  };
}

export const Project = {
  encode(message: Project, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    Object.entries(message.seeds).forEach(([key, value]) => {
      Project_SeedsEntry.encode({ key: key as any, value }, writer.uint32(26).fork()).ldelim();
    });
    Object.entries(message.models).forEach(([key, value]) => {
      Project_ModelsEntry.encode({ key: key as any, value }, writer.uint32(34).fork()).ldelim();
    });
    Object.entries(message.tests).forEach(([key, value]) => {
      Project_TestsEntry.encode({ key: key as any, value }, writer.uint32(42).fork()).ldelim();
    });
    Object.entries(message.sources).forEach(([key, value]) => {
      Project_SourcesEntry.encode({ key: key as any, value }, writer.uint32(50).fork()).ldelim();
    });
    Object.entries(message.snapshots).forEach(([key, value]) => {
      Project_SnapshotsEntry.encode({ key: key as any, value }, writer.uint32(74).fork()).ldelim();
    });
    Object.entries(message.projectFiles).forEach(([key, value]) => {
      Project_ProjectFilesEntry.encode({ key: key as any, value }, writer.uint32(58).fork()).ldelim();
    });
    if (message.connectionConfig !== undefined) {
      ConnectionConfig.encode(message.connectionConfig, writer.uint32(66).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Project {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProject();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 3:
          if (tag !== 26) {
            break;
          }

          const entry3 = Project_SeedsEntry.decode(reader, reader.uint32());
          if (entry3.value !== undefined) {
            message.seeds[entry3.key] = entry3.value;
          }
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          const entry4 = Project_ModelsEntry.decode(reader, reader.uint32());
          if (entry4.value !== undefined) {
            message.models[entry4.key] = entry4.value;
          }
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          const entry5 = Project_TestsEntry.decode(reader, reader.uint32());
          if (entry5.value !== undefined) {
            message.tests[entry5.key] = entry5.value;
          }
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          const entry6 = Project_SourcesEntry.decode(reader, reader.uint32());
          if (entry6.value !== undefined) {
            message.sources[entry6.key] = entry6.value;
          }
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          const entry9 = Project_SnapshotsEntry.decode(reader, reader.uint32());
          if (entry9.value !== undefined) {
            message.snapshots[entry9.key] = entry9.value;
          }
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          const entry7 = Project_ProjectFilesEntry.decode(reader, reader.uint32());
          if (entry7.value !== undefined) {
            message.projectFiles[entry7.key] = entry7.value;
          }
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          message.connectionConfig = ConnectionConfig.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Project {
    return {
      seeds: isObject(object.seeds)
        ? Object.entries(object.seeds).reduce<{ [key: string]: Seed }>((acc, [key, value]) => {
          acc[key] = Seed.fromJSON(value);
          return acc;
        }, {})
        : {},
      models: isObject(object.models)
        ? Object.entries(object.models).reduce<{ [key: string]: Model }>((acc, [key, value]) => {
          acc[key] = Model.fromJSON(value);
          return acc;
        }, {})
        : {},
      tests: isObject(object.tests)
        ? Object.entries(object.tests).reduce<{ [key: string]: Test }>((acc, [key, value]) => {
          acc[key] = Test.fromJSON(value);
          return acc;
        }, {})
        : {},
      sources: isObject(object.sources)
        ? Object.entries(object.sources).reduce<{ [key: string]: Source }>((acc, [key, value]) => {
          acc[key] = Source.fromJSON(value);
          return acc;
        }, {})
        : {},
      snapshots: isObject(object.snapshots)
        ? Object.entries(object.snapshots).reduce<{ [key: string]: Snapshot }>((acc, [key, value]) => {
          acc[key] = Snapshot.fromJSON(value);
          return acc;
        }, {})
        : {},
      projectFiles: isObject(object.projectFiles)
        ? Object.entries(object.projectFiles).reduce<{ [key: string]: ProjectFile }>((acc, [key, value]) => {
          acc[key] = ProjectFile.fromJSON(value);
          return acc;
        }, {})
        : {},
      connectionConfig: isSet(object.connectionConfig) ? ConnectionConfig.fromJSON(object.connectionConfig) : undefined,
    };
  },

  toJSON(message: Project): unknown {
    const obj: any = {};
    if (message.seeds) {
      const entries = Object.entries(message.seeds);
      if (entries.length > 0) {
        obj.seeds = {};
        entries.forEach(([k, v]) => {
          obj.seeds[k] = Seed.toJSON(v);
        });
      }
    }
    if (message.models) {
      const entries = Object.entries(message.models);
      if (entries.length > 0) {
        obj.models = {};
        entries.forEach(([k, v]) => {
          obj.models[k] = Model.toJSON(v);
        });
      }
    }
    if (message.tests) {
      const entries = Object.entries(message.tests);
      if (entries.length > 0) {
        obj.tests = {};
        entries.forEach(([k, v]) => {
          obj.tests[k] = Test.toJSON(v);
        });
      }
    }
    if (message.sources) {
      const entries = Object.entries(message.sources);
      if (entries.length > 0) {
        obj.sources = {};
        entries.forEach(([k, v]) => {
          obj.sources[k] = Source.toJSON(v);
        });
      }
    }
    if (message.snapshots) {
      const entries = Object.entries(message.snapshots);
      if (entries.length > 0) {
        obj.snapshots = {};
        entries.forEach(([k, v]) => {
          obj.snapshots[k] = Snapshot.toJSON(v);
        });
      }
    }
    if (message.projectFiles) {
      const entries = Object.entries(message.projectFiles);
      if (entries.length > 0) {
        obj.projectFiles = {};
        entries.forEach(([k, v]) => {
          obj.projectFiles[k] = ProjectFile.toJSON(v);
        });
      }
    }
    if (message.connectionConfig !== undefined) {
      obj.connectionConfig = ConnectionConfig.toJSON(message.connectionConfig);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Project>, I>>(base?: I): Project {
    return Project.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Project>, I>>(object: I): Project {
    const message = createBaseProject();
    message.seeds = Object.entries(object.seeds ?? {}).reduce<{ [key: string]: Seed }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = Seed.fromPartial(value);
      }
      return acc;
    }, {});
    message.models = Object.entries(object.models ?? {}).reduce<{ [key: string]: Model }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = Model.fromPartial(value);
      }
      return acc;
    }, {});
    message.tests = Object.entries(object.tests ?? {}).reduce<{ [key: string]: Test }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = Test.fromPartial(value);
      }
      return acc;
    }, {});
    message.sources = Object.entries(object.sources ?? {}).reduce<{ [key: string]: Source }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = Source.fromPartial(value);
      }
      return acc;
    }, {});
    message.snapshots = Object.entries(object.snapshots ?? {}).reduce<{ [key: string]: Snapshot }>(
      (acc, [key, value]) => {
        if (value !== undefined) {
          acc[key] = Snapshot.fromPartial(value);
        }
        return acc;
      },
      {},
    );
    message.projectFiles = Object.entries(object.projectFiles ?? {}).reduce<{ [key: string]: ProjectFile }>(
      (acc, [key, value]) => {
        if (value !== undefined) {
          acc[key] = ProjectFile.fromPartial(value);
        }
        return acc;
      },
      {},
    );
    message.connectionConfig = (object.connectionConfig !== undefined && object.connectionConfig !== null)
      ? ConnectionConfig.fromPartial(object.connectionConfig)
      : undefined;
    return message;
  },
};

function createBaseProject_SeedsEntry(): Project_SeedsEntry {
  return { key: "", value: undefined };
}

export const Project_SeedsEntry = {
  encode(message: Project_SeedsEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      Seed.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Project_SeedsEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProject_SeedsEntry();
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

          message.value = Seed.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Project_SeedsEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? Seed.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: Project_SeedsEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = Seed.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Project_SeedsEntry>, I>>(base?: I): Project_SeedsEntry {
    return Project_SeedsEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Project_SeedsEntry>, I>>(object: I): Project_SeedsEntry {
    const message = createBaseProject_SeedsEntry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null) ? Seed.fromPartial(object.value) : undefined;
    return message;
  },
};

function createBaseProject_ModelsEntry(): Project_ModelsEntry {
  return { key: "", value: undefined };
}

export const Project_ModelsEntry = {
  encode(message: Project_ModelsEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      Model.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Project_ModelsEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProject_ModelsEntry();
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

          message.value = Model.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Project_ModelsEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? Model.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: Project_ModelsEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = Model.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Project_ModelsEntry>, I>>(base?: I): Project_ModelsEntry {
    return Project_ModelsEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Project_ModelsEntry>, I>>(object: I): Project_ModelsEntry {
    const message = createBaseProject_ModelsEntry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null) ? Model.fromPartial(object.value) : undefined;
    return message;
  },
};

function createBaseProject_TestsEntry(): Project_TestsEntry {
  return { key: "", value: undefined };
}

export const Project_TestsEntry = {
  encode(message: Project_TestsEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      Test.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Project_TestsEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProject_TestsEntry();
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

          message.value = Test.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Project_TestsEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? Test.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: Project_TestsEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = Test.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Project_TestsEntry>, I>>(base?: I): Project_TestsEntry {
    return Project_TestsEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Project_TestsEntry>, I>>(object: I): Project_TestsEntry {
    const message = createBaseProject_TestsEntry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null) ? Test.fromPartial(object.value) : undefined;
    return message;
  },
};

function createBaseProject_SourcesEntry(): Project_SourcesEntry {
  return { key: "", value: undefined };
}

export const Project_SourcesEntry = {
  encode(message: Project_SourcesEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      Source.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Project_SourcesEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProject_SourcesEntry();
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

          message.value = Source.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Project_SourcesEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? Source.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: Project_SourcesEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = Source.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Project_SourcesEntry>, I>>(base?: I): Project_SourcesEntry {
    return Project_SourcesEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Project_SourcesEntry>, I>>(object: I): Project_SourcesEntry {
    const message = createBaseProject_SourcesEntry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? Source.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseProject_SnapshotsEntry(): Project_SnapshotsEntry {
  return { key: "", value: undefined };
}

export const Project_SnapshotsEntry = {
  encode(message: Project_SnapshotsEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      Snapshot.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Project_SnapshotsEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProject_SnapshotsEntry();
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

          message.value = Snapshot.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Project_SnapshotsEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? Snapshot.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: Project_SnapshotsEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = Snapshot.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Project_SnapshotsEntry>, I>>(base?: I): Project_SnapshotsEntry {
    return Project_SnapshotsEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Project_SnapshotsEntry>, I>>(object: I): Project_SnapshotsEntry {
    const message = createBaseProject_SnapshotsEntry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? Snapshot.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseProject_ProjectFilesEntry(): Project_ProjectFilesEntry {
  return { key: "", value: undefined };
}

export const Project_ProjectFilesEntry = {
  encode(message: Project_ProjectFilesEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      ProjectFile.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Project_ProjectFilesEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProject_ProjectFilesEntry();
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

          message.value = ProjectFile.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Project_ProjectFilesEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? ProjectFile.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: Project_ProjectFilesEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = ProjectFile.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Project_ProjectFilesEntry>, I>>(base?: I): Project_ProjectFilesEntry {
    return Project_ProjectFilesEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Project_ProjectFilesEntry>, I>>(object: I): Project_ProjectFilesEntry {
    const message = createBaseProject_ProjectFilesEntry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? ProjectFile.fromPartial(object.value)
      : undefined;
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
