/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "quary.service.v1";

export interface ProjectFile {
  sources: ProjectFileSource[];
  models: ProjectFile_Model[];
  snapshots: ProjectFile_Snapshot[];
}

export interface ProjectFile_Model {
  name: string;
  tags: string[];
  description?:
    | string
    | undefined;
  /** The materialization of the model, available types are specified by each database. */
  materialization?: string | undefined;
  tests: ModelTest[];
  columns: ProjectFileColumn[];
}

export interface ProjectFile_Snapshot {
  name: string;
  uniqueKey: string;
  strategy: ProjectFile_SnapshotStrategy | undefined;
}

export interface ProjectFile_SnapshotStrategy {
  strategyType?: { $case: "timestamp"; timestamp: ProjectFile_TimestampStrategy } | undefined;
}

export interface ProjectFile_TimestampStrategy {
  updatedAt: string;
}

export interface ProjectFileSource {
  name: string;
  tags: string[];
  description?:
    | string
    | undefined;
  /**
   * The full path of the source table in the database. This is used to reference the table itself. For example:
   * - 'public.users' for where the schema is 'public' and the table is 'users'
   * - 'project_id_123.dataset_id_123.table_id_123' for a BigQuery table
   */
  path: string;
  tests: ModelTest[];
  columns: ProjectFileColumn[];
}

/**
 * Standard types are:
 * - not_null
 * - unique
 * - 'relationship' which takes into data (model and field)
 */
export interface ProjectFileColumn {
  name: string;
  description?: string | undefined;
  tests: ColumnTest[];
}

export interface ColumnTest {
  type: string;
  info: { [key: string]: string };
}

export interface ColumnTest_InfoEntry {
  key: string;
  value: string;
}

export interface ModelTest {
  type: string;
  info: { [key: string]: string };
}

export interface ModelTest_InfoEntry {
  key: string;
  value: string;
}

function createBaseProjectFile(): ProjectFile {
  return { sources: [], models: [], snapshots: [] };
}

export const ProjectFile = {
  encode(message: ProjectFile, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.sources) {
      ProjectFileSource.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.models) {
      ProjectFile_Model.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    for (const v of message.snapshots) {
      ProjectFile_Snapshot.encode(v!, writer.uint32(26).fork()).ldelim();
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

          message.sources.push(ProjectFileSource.decode(reader, reader.uint32()));
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.models.push(ProjectFile_Model.decode(reader, reader.uint32()));
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.snapshots.push(ProjectFile_Snapshot.decode(reader, reader.uint32()));
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
      sources: gt.Array.isArray(object?.sources) ? object.sources.map((e: any) => ProjectFileSource.fromJSON(e)) : [],
      models: gt.Array.isArray(object?.models) ? object.models.map((e: any) => ProjectFile_Model.fromJSON(e)) : [],
      snapshots: gt.Array.isArray(object?.snapshots)
        ? object.snapshots.map((e: any) => ProjectFile_Snapshot.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ProjectFile): unknown {
    const obj: any = {};
    if (message.sources?.length) {
      obj.sources = message.sources.map((e) => ProjectFileSource.toJSON(e));
    }
    if (message.models?.length) {
      obj.models = message.models.map((e) => ProjectFile_Model.toJSON(e));
    }
    if (message.snapshots?.length) {
      obj.snapshots = message.snapshots.map((e) => ProjectFile_Snapshot.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile>, I>>(base?: I): ProjectFile {
    return ProjectFile.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile>, I>>(object: I): ProjectFile {
    const message = createBaseProjectFile();
    message.sources = object.sources?.map((e) => ProjectFileSource.fromPartial(e)) || [];
    message.models = object.models?.map((e) => ProjectFile_Model.fromPartial(e)) || [];
    message.snapshots = object.snapshots?.map((e) => ProjectFile_Snapshot.fromPartial(e)) || [];
    return message;
  },
};

function createBaseProjectFile_Model(): ProjectFile_Model {
  return { name: "", tags: [], description: undefined, materialization: undefined, tests: [], columns: [] };
}

export const ProjectFile_Model = {
  encode(message: ProjectFile_Model, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    for (const v of message.tags) {
      writer.uint32(50).string(v!);
    }
    if (message.description !== undefined) {
      writer.uint32(18).string(message.description);
    }
    if (message.materialization !== undefined) {
      writer.uint32(34).string(message.materialization);
    }
    for (const v of message.tests) {
      ModelTest.encode(v!, writer.uint32(42).fork()).ldelim();
    }
    for (const v of message.columns) {
      ProjectFileColumn.encode(v!, writer.uint32(26).fork()).ldelim();
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
        case 6:
          if (tag !== 50) {
            break;
          }

          message.tags.push(reader.string());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.description = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.materialization = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.tests.push(ModelTest.decode(reader, reader.uint32()));
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.columns.push(ProjectFileColumn.decode(reader, reader.uint32()));
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
      tags: gt.Array.isArray(object?.tags) ? object.tags.map((e: any) => gt.String(e)) : [],
      description: isSet(object.description) ? gt.String(object.description) : undefined,
      materialization: isSet(object.materialization) ? gt.String(object.materialization) : undefined,
      tests: gt.Array.isArray(object?.tests) ? object.tests.map((e: any) => ModelTest.fromJSON(e)) : [],
      columns: gt.Array.isArray(object?.columns) ? object.columns.map((e: any) => ProjectFileColumn.fromJSON(e)) : [],
    };
  },

  toJSON(message: ProjectFile_Model): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.tags?.length) {
      obj.tags = message.tags;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    if (message.materialization !== undefined) {
      obj.materialization = message.materialization;
    }
    if (message.tests?.length) {
      obj.tests = message.tests.map((e) => ModelTest.toJSON(e));
    }
    if (message.columns?.length) {
      obj.columns = message.columns.map((e) => ProjectFileColumn.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile_Model>, I>>(base?: I): ProjectFile_Model {
    return ProjectFile_Model.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile_Model>, I>>(object: I): ProjectFile_Model {
    const message = createBaseProjectFile_Model();
    message.name = object.name ?? "";
    message.tags = object.tags?.map((e) => e) || [];
    message.description = object.description ?? undefined;
    message.materialization = object.materialization ?? undefined;
    message.tests = object.tests?.map((e) => ModelTest.fromPartial(e)) || [];
    message.columns = object.columns?.map((e) => ProjectFileColumn.fromPartial(e)) || [];
    return message;
  },
};

function createBaseProjectFile_Snapshot(): ProjectFile_Snapshot {
  return { name: "", uniqueKey: "", strategy: undefined };
}

export const ProjectFile_Snapshot = {
  encode(message: ProjectFile_Snapshot, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.uniqueKey !== "") {
      writer.uint32(18).string(message.uniqueKey);
    }
    if (message.strategy !== undefined) {
      ProjectFile_SnapshotStrategy.encode(message.strategy, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFile_Snapshot {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFile_Snapshot();
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

          message.uniqueKey = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.strategy = ProjectFile_SnapshotStrategy.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFile_Snapshot {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      uniqueKey: isSet(object.uniqueKey) ? gt.String(object.uniqueKey) : "",
      strategy: isSet(object.strategy) ? ProjectFile_SnapshotStrategy.fromJSON(object.strategy) : undefined,
    };
  },

  toJSON(message: ProjectFile_Snapshot): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.uniqueKey !== "") {
      obj.uniqueKey = message.uniqueKey;
    }
    if (message.strategy !== undefined) {
      obj.strategy = ProjectFile_SnapshotStrategy.toJSON(message.strategy);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile_Snapshot>, I>>(base?: I): ProjectFile_Snapshot {
    return ProjectFile_Snapshot.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile_Snapshot>, I>>(object: I): ProjectFile_Snapshot {
    const message = createBaseProjectFile_Snapshot();
    message.name = object.name ?? "";
    message.uniqueKey = object.uniqueKey ?? "";
    message.strategy = (object.strategy !== undefined && object.strategy !== null)
      ? ProjectFile_SnapshotStrategy.fromPartial(object.strategy)
      : undefined;
    return message;
  },
};

function createBaseProjectFile_SnapshotStrategy(): ProjectFile_SnapshotStrategy {
  return { strategyType: undefined };
}

export const ProjectFile_SnapshotStrategy = {
  encode(message: ProjectFile_SnapshotStrategy, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.strategyType?.$case) {
      case "timestamp":
        ProjectFile_TimestampStrategy.encode(message.strategyType.timestamp, writer.uint32(10).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFile_SnapshotStrategy {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFile_SnapshotStrategy();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.strategyType = {
            $case: "timestamp",
            timestamp: ProjectFile_TimestampStrategy.decode(reader, reader.uint32()),
          };
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFile_SnapshotStrategy {
    return {
      strategyType: isSet(object.timestamp)
        ? { $case: "timestamp", timestamp: ProjectFile_TimestampStrategy.fromJSON(object.timestamp) }
        : undefined,
    };
  },

  toJSON(message: ProjectFile_SnapshotStrategy): unknown {
    const obj: any = {};
    if (message.strategyType?.$case === "timestamp") {
      obj.timestamp = ProjectFile_TimestampStrategy.toJSON(message.strategyType.timestamp);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile_SnapshotStrategy>, I>>(base?: I): ProjectFile_SnapshotStrategy {
    return ProjectFile_SnapshotStrategy.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile_SnapshotStrategy>, I>>(object: I): ProjectFile_SnapshotStrategy {
    const message = createBaseProjectFile_SnapshotStrategy();
    if (
      object.strategyType?.$case === "timestamp" &&
      object.strategyType?.timestamp !== undefined &&
      object.strategyType?.timestamp !== null
    ) {
      message.strategyType = {
        $case: "timestamp",
        timestamp: ProjectFile_TimestampStrategy.fromPartial(object.strategyType.timestamp),
      };
    }
    return message;
  },
};

function createBaseProjectFile_TimestampStrategy(): ProjectFile_TimestampStrategy {
  return { updatedAt: "" };
}

export const ProjectFile_TimestampStrategy = {
  encode(message: ProjectFile_TimestampStrategy, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.updatedAt !== "") {
      writer.uint32(10).string(message.updatedAt);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFile_TimestampStrategy {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFile_TimestampStrategy();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.updatedAt = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFile_TimestampStrategy {
    return { updatedAt: isSet(object.updatedAt) ? gt.String(object.updatedAt) : "" };
  },

  toJSON(message: ProjectFile_TimestampStrategy): unknown {
    const obj: any = {};
    if (message.updatedAt !== "") {
      obj.updatedAt = message.updatedAt;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFile_TimestampStrategy>, I>>(base?: I): ProjectFile_TimestampStrategy {
    return ProjectFile_TimestampStrategy.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFile_TimestampStrategy>, I>>(
    object: I,
  ): ProjectFile_TimestampStrategy {
    const message = createBaseProjectFile_TimestampStrategy();
    message.updatedAt = object.updatedAt ?? "";
    return message;
  },
};

function createBaseProjectFileSource(): ProjectFileSource {
  return { name: "", tags: [], description: undefined, path: "", tests: [], columns: [] };
}

export const ProjectFileSource = {
  encode(message: ProjectFileSource, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    for (const v of message.tags) {
      writer.uint32(50).string(v!);
    }
    if (message.description !== undefined) {
      writer.uint32(18).string(message.description);
    }
    if (message.path !== "") {
      writer.uint32(26).string(message.path);
    }
    for (const v of message.tests) {
      ModelTest.encode(v!, writer.uint32(42).fork()).ldelim();
    }
    for (const v of message.columns) {
      ProjectFileColumn.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFileSource {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFileSource();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.tags.push(reader.string());
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
        case 5:
          if (tag !== 42) {
            break;
          }

          message.tests.push(ModelTest.decode(reader, reader.uint32()));
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.columns.push(ProjectFileColumn.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ProjectFileSource {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      tags: gt.Array.isArray(object?.tags) ? object.tags.map((e: any) => gt.String(e)) : [],
      description: isSet(object.description) ? gt.String(object.description) : undefined,
      path: isSet(object.path) ? gt.String(object.path) : "",
      tests: gt.Array.isArray(object?.tests) ? object.tests.map((e: any) => ModelTest.fromJSON(e)) : [],
      columns: gt.Array.isArray(object?.columns) ? object.columns.map((e: any) => ProjectFileColumn.fromJSON(e)) : [],
    };
  },

  toJSON(message: ProjectFileSource): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.tags?.length) {
      obj.tags = message.tags;
    }
    if (message.description !== undefined) {
      obj.description = message.description;
    }
    if (message.path !== "") {
      obj.path = message.path;
    }
    if (message.tests?.length) {
      obj.tests = message.tests.map((e) => ModelTest.toJSON(e));
    }
    if (message.columns?.length) {
      obj.columns = message.columns.map((e) => ProjectFileColumn.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ProjectFileSource>, I>>(base?: I): ProjectFileSource {
    return ProjectFileSource.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFileSource>, I>>(object: I): ProjectFileSource {
    const message = createBaseProjectFileSource();
    message.name = object.name ?? "";
    message.tags = object.tags?.map((e) => e) || [];
    message.description = object.description ?? undefined;
    message.path = object.path ?? "";
    message.tests = object.tests?.map((e) => ModelTest.fromPartial(e)) || [];
    message.columns = object.columns?.map((e) => ProjectFileColumn.fromPartial(e)) || [];
    return message;
  },
};

function createBaseProjectFileColumn(): ProjectFileColumn {
  return { name: "", description: undefined, tests: [] };
}

export const ProjectFileColumn = {
  encode(message: ProjectFileColumn, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
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

  decode(input: _m0.Reader | Uint8Array, length?: number): ProjectFileColumn {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseProjectFileColumn();
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

  fromJSON(object: any): ProjectFileColumn {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      description: isSet(object.description) ? gt.String(object.description) : undefined,
      tests: gt.Array.isArray(object?.tests) ? object.tests.map((e: any) => ColumnTest.fromJSON(e)) : [],
    };
  },

  toJSON(message: ProjectFileColumn): unknown {
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

  create<I extends Exact<DeepPartial<ProjectFileColumn>, I>>(base?: I): ProjectFileColumn {
    return ProjectFileColumn.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ProjectFileColumn>, I>>(object: I): ProjectFileColumn {
    const message = createBaseProjectFileColumn();
    message.name = object.name ?? "";
    message.description = object.description ?? undefined;
    message.tests = object.tests?.map((e) => ColumnTest.fromPartial(e)) || [];
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

function createBaseModelTest(): ModelTest {
  return { type: "", info: {} };
}

export const ModelTest = {
  encode(message: ModelTest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.type !== "") {
      writer.uint32(10).string(message.type);
    }
    Object.entries(message.info).forEach(([key, value]) => {
      ModelTest_InfoEntry.encode({ key: key as any, value }, writer.uint32(18).fork()).ldelim();
    });
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ModelTest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelTest();
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

          const entry2 = ModelTest_InfoEntry.decode(reader, reader.uint32());
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

  fromJSON(object: any): ModelTest {
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

  toJSON(message: ModelTest): unknown {
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

  create<I extends Exact<DeepPartial<ModelTest>, I>>(base?: I): ModelTest {
    return ModelTest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ModelTest>, I>>(object: I): ModelTest {
    const message = createBaseModelTest();
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

function createBaseModelTest_InfoEntry(): ModelTest_InfoEntry {
  return { key: "", value: "" };
}

export const ModelTest_InfoEntry = {
  encode(message: ModelTest_InfoEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ModelTest_InfoEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelTest_InfoEntry();
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

  fromJSON(object: any): ModelTest_InfoEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? gt.String(object.value) : "",
    };
  },

  toJSON(message: ModelTest_InfoEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ModelTest_InfoEntry>, I>>(base?: I): ModelTest_InfoEntry {
    return ModelTest_InfoEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ModelTest_InfoEntry>, I>>(object: I): ModelTest_InfoEntry {
    const message = createBaseModelTest_InfoEntry();
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
