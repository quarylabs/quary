/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "quary.service.v1";

export interface Var {
  name: string;
  value: string;
}

/** Configuration describes the configuration of the project. */
export interface ConnectionConfig {
  config?:
    | { $case: "duckdb"; duckdb: ConnectionConfig_ConnectionConfigDuckDB }
    | { $case: "duckdbInMemory"; duckdbInMemory: ConnectionConfig_ConnectionConfigDuckDBInMemory }
    | { $case: "sqlite"; sqlite: ConnectionConfig_ConnectionConfigSqLite }
    | { $case: "sqliteInMemory"; sqliteInMemory: ConnectionConfig_ConnectionConfigSqLiteInMemory }
    | { $case: "bigQuery"; bigQuery: ConnectionConfig_ConnectionConfigBigQuery }
    | { $case: "snowflake"; snowflake: ConnectionConfig_ConnectionConfigSnowflake }
    | { $case: "postgres"; postgres: ConnectionConfig_ConnectionConfigPostgres }
    | { $case: "redshift"; redshift: ConnectionConfig_ConnectionConfigRedshift }
    | undefined;
  vars: Var[];
}

export interface ConnectionConfig_ConnectionConfigSqLite {
  path: string;
}

export interface ConnectionConfig_ConnectionConfigSqLiteInMemory {
}

export interface ConnectionConfig_ConnectionConfigDuckDB {
  path: string;
  schema?: string | undefined;
}

export interface ConnectionConfig_ConnectionConfigDuckDBInMemory {
  schema?: string | undefined;
}

export interface ConnectionConfig_ConnectionConfigPostgres {
  schema: string;
}

export interface ConnectionConfig_ConnectionConfigRedshift {
  schema: string;
}

export interface ConnectionConfig_ConnectionConfigBigQuery {
  projectId: string;
  datasetId: string;
}

export interface ConnectionConfig_ConnectionConfigSnowflake {
  accountUrl: string;
  clientId: string;
  clientSecret: string;
  /** TODO: think about making optional */
  role: string;
  database: string;
  schema: string;
  warehouse: string;
}

function createBaseVar(): Var {
  return { name: "", value: "" };
}

export const Var = {
  encode(message: Var, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Var {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVar();
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

  fromJSON(object: any): Var {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      value: isSet(object.value) ? gt.String(object.value) : "",
    };
  },

  toJSON(message: Var): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Var>, I>>(base?: I): Var {
    return Var.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Var>, I>>(object: I): Var {
    const message = createBaseVar();
    message.name = object.name ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseConnectionConfig(): ConnectionConfig {
  return { config: undefined, vars: [] };
}

export const ConnectionConfig = {
  encode(message: ConnectionConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.config?.$case) {
      case "duckdb":
        ConnectionConfig_ConnectionConfigDuckDB.encode(message.config.duckdb, writer.uint32(10).fork()).ldelim();
        break;
      case "duckdbInMemory":
        ConnectionConfig_ConnectionConfigDuckDBInMemory.encode(message.config.duckdbInMemory, writer.uint32(18).fork())
          .ldelim();
        break;
      case "sqlite":
        ConnectionConfig_ConnectionConfigSqLite.encode(message.config.sqlite, writer.uint32(26).fork()).ldelim();
        break;
      case "sqliteInMemory":
        ConnectionConfig_ConnectionConfigSqLiteInMemory.encode(message.config.sqliteInMemory, writer.uint32(34).fork())
          .ldelim();
        break;
      case "bigQuery":
        ConnectionConfig_ConnectionConfigBigQuery.encode(message.config.bigQuery, writer.uint32(42).fork()).ldelim();
        break;
      case "snowflake":
        ConnectionConfig_ConnectionConfigSnowflake.encode(message.config.snowflake, writer.uint32(50).fork()).ldelim();
        break;
      case "postgres":
        ConnectionConfig_ConnectionConfigPostgres.encode(message.config.postgres, writer.uint32(58).fork()).ldelim();
        break;
      case "redshift":
        ConnectionConfig_ConnectionConfigRedshift.encode(message.config.redshift, writer.uint32(74).fork()).ldelim();
        break;
    }
    for (const v of message.vars) {
      Var.encode(v!, writer.uint32(66).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.config = {
            $case: "duckdb",
            duckdb: ConnectionConfig_ConnectionConfigDuckDB.decode(reader, reader.uint32()),
          };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.config = {
            $case: "duckdbInMemory",
            duckdbInMemory: ConnectionConfig_ConnectionConfigDuckDBInMemory.decode(reader, reader.uint32()),
          };
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.config = {
            $case: "sqlite",
            sqlite: ConnectionConfig_ConnectionConfigSqLite.decode(reader, reader.uint32()),
          };
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.config = {
            $case: "sqliteInMemory",
            sqliteInMemory: ConnectionConfig_ConnectionConfigSqLiteInMemory.decode(reader, reader.uint32()),
          };
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.config = {
            $case: "bigQuery",
            bigQuery: ConnectionConfig_ConnectionConfigBigQuery.decode(reader, reader.uint32()),
          };
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.config = {
            $case: "snowflake",
            snowflake: ConnectionConfig_ConnectionConfigSnowflake.decode(reader, reader.uint32()),
          };
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          message.config = {
            $case: "postgres",
            postgres: ConnectionConfig_ConnectionConfigPostgres.decode(reader, reader.uint32()),
          };
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          message.config = {
            $case: "redshift",
            redshift: ConnectionConfig_ConnectionConfigRedshift.decode(reader, reader.uint32()),
          };
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          message.vars.push(Var.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig {
    return {
      config: isSet(object.duckdb)
        ? { $case: "duckdb", duckdb: ConnectionConfig_ConnectionConfigDuckDB.fromJSON(object.duckdb) }
        : isSet(object.duckdbInMemory)
        ? {
          $case: "duckdbInMemory",
          duckdbInMemory: ConnectionConfig_ConnectionConfigDuckDBInMemory.fromJSON(object.duckdbInMemory),
        }
        : isSet(object.sqlite)
        ? { $case: "sqlite", sqlite: ConnectionConfig_ConnectionConfigSqLite.fromJSON(object.sqlite) }
        : isSet(object.sqliteInMemory)
        ? {
          $case: "sqliteInMemory",
          sqliteInMemory: ConnectionConfig_ConnectionConfigSqLiteInMemory.fromJSON(object.sqliteInMemory),
        }
        : isSet(object.bigQuery)
        ? { $case: "bigQuery", bigQuery: ConnectionConfig_ConnectionConfigBigQuery.fromJSON(object.bigQuery) }
        : isSet(object.snowflake)
        ? { $case: "snowflake", snowflake: ConnectionConfig_ConnectionConfigSnowflake.fromJSON(object.snowflake) }
        : isSet(object.postgres)
        ? { $case: "postgres", postgres: ConnectionConfig_ConnectionConfigPostgres.fromJSON(object.postgres) }
        : isSet(object.redshift)
        ? { $case: "redshift", redshift: ConnectionConfig_ConnectionConfigRedshift.fromJSON(object.redshift) }
        : undefined,
      vars: gt.Array.isArray(object?.vars) ? object.vars.map((e: any) => Var.fromJSON(e)) : [],
    };
  },

  toJSON(message: ConnectionConfig): unknown {
    const obj: any = {};
    if (message.config?.$case === "duckdb") {
      obj.duckdb = ConnectionConfig_ConnectionConfigDuckDB.toJSON(message.config.duckdb);
    }
    if (message.config?.$case === "duckdbInMemory") {
      obj.duckdbInMemory = ConnectionConfig_ConnectionConfigDuckDBInMemory.toJSON(message.config.duckdbInMemory);
    }
    if (message.config?.$case === "sqlite") {
      obj.sqlite = ConnectionConfig_ConnectionConfigSqLite.toJSON(message.config.sqlite);
    }
    if (message.config?.$case === "sqliteInMemory") {
      obj.sqliteInMemory = ConnectionConfig_ConnectionConfigSqLiteInMemory.toJSON(message.config.sqliteInMemory);
    }
    if (message.config?.$case === "bigQuery") {
      obj.bigQuery = ConnectionConfig_ConnectionConfigBigQuery.toJSON(message.config.bigQuery);
    }
    if (message.config?.$case === "snowflake") {
      obj.snowflake = ConnectionConfig_ConnectionConfigSnowflake.toJSON(message.config.snowflake);
    }
    if (message.config?.$case === "postgres") {
      obj.postgres = ConnectionConfig_ConnectionConfigPostgres.toJSON(message.config.postgres);
    }
    if (message.config?.$case === "redshift") {
      obj.redshift = ConnectionConfig_ConnectionConfigRedshift.toJSON(message.config.redshift);
    }
    if (message.vars?.length) {
      obj.vars = message.vars.map((e) => Var.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig>, I>>(base?: I): ConnectionConfig {
    return ConnectionConfig.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig>, I>>(object: I): ConnectionConfig {
    const message = createBaseConnectionConfig();
    if (object.config?.$case === "duckdb" && object.config?.duckdb !== undefined && object.config?.duckdb !== null) {
      message.config = {
        $case: "duckdb",
        duckdb: ConnectionConfig_ConnectionConfigDuckDB.fromPartial(object.config.duckdb),
      };
    }
    if (
      object.config?.$case === "duckdbInMemory" &&
      object.config?.duckdbInMemory !== undefined &&
      object.config?.duckdbInMemory !== null
    ) {
      message.config = {
        $case: "duckdbInMemory",
        duckdbInMemory: ConnectionConfig_ConnectionConfigDuckDBInMemory.fromPartial(object.config.duckdbInMemory),
      };
    }
    if (object.config?.$case === "sqlite" && object.config?.sqlite !== undefined && object.config?.sqlite !== null) {
      message.config = {
        $case: "sqlite",
        sqlite: ConnectionConfig_ConnectionConfigSqLite.fromPartial(object.config.sqlite),
      };
    }
    if (
      object.config?.$case === "sqliteInMemory" &&
      object.config?.sqliteInMemory !== undefined &&
      object.config?.sqliteInMemory !== null
    ) {
      message.config = {
        $case: "sqliteInMemory",
        sqliteInMemory: ConnectionConfig_ConnectionConfigSqLiteInMemory.fromPartial(object.config.sqliteInMemory),
      };
    }
    if (
      object.config?.$case === "bigQuery" && object.config?.bigQuery !== undefined && object.config?.bigQuery !== null
    ) {
      message.config = {
        $case: "bigQuery",
        bigQuery: ConnectionConfig_ConnectionConfigBigQuery.fromPartial(object.config.bigQuery),
      };
    }
    if (
      object.config?.$case === "snowflake" &&
      object.config?.snowflake !== undefined &&
      object.config?.snowflake !== null
    ) {
      message.config = {
        $case: "snowflake",
        snowflake: ConnectionConfig_ConnectionConfigSnowflake.fromPartial(object.config.snowflake),
      };
    }
    if (
      object.config?.$case === "postgres" && object.config?.postgres !== undefined && object.config?.postgres !== null
    ) {
      message.config = {
        $case: "postgres",
        postgres: ConnectionConfig_ConnectionConfigPostgres.fromPartial(object.config.postgres),
      };
    }
    if (
      object.config?.$case === "redshift" && object.config?.redshift !== undefined && object.config?.redshift !== null
    ) {
      message.config = {
        $case: "redshift",
        redshift: ConnectionConfig_ConnectionConfigRedshift.fromPartial(object.config.redshift),
      };
    }
    message.vars = object.vars?.map((e) => Var.fromPartial(e)) || [];
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigSqLite(): ConnectionConfig_ConnectionConfigSqLite {
  return { path: "" };
}

export const ConnectionConfig_ConnectionConfigSqLite = {
  encode(message: ConnectionConfig_ConnectionConfigSqLite, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigSqLite {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigSqLite();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig_ConnectionConfigSqLite {
    return { path: isSet(object.path) ? gt.String(object.path) : "" };
  },

  toJSON(message: ConnectionConfig_ConnectionConfigSqLite): unknown {
    const obj: any = {};
    if (message.path !== "") {
      obj.path = message.path;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigSqLite>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigSqLite {
    return ConnectionConfig_ConnectionConfigSqLite.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigSqLite>, I>>(
    object: I,
  ): ConnectionConfig_ConnectionConfigSqLite {
    const message = createBaseConnectionConfig_ConnectionConfigSqLite();
    message.path = object.path ?? "";
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigSqLiteInMemory(): ConnectionConfig_ConnectionConfigSqLiteInMemory {
  return {};
}

export const ConnectionConfig_ConnectionConfigSqLiteInMemory = {
  encode(_: ConnectionConfig_ConnectionConfigSqLiteInMemory, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigSqLiteInMemory {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigSqLiteInMemory();
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

  fromJSON(_: any): ConnectionConfig_ConnectionConfigSqLiteInMemory {
    return {};
  },

  toJSON(_: ConnectionConfig_ConnectionConfigSqLiteInMemory): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigSqLiteInMemory>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigSqLiteInMemory {
    return ConnectionConfig_ConnectionConfigSqLiteInMemory.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigSqLiteInMemory>, I>>(
    _: I,
  ): ConnectionConfig_ConnectionConfigSqLiteInMemory {
    const message = createBaseConnectionConfig_ConnectionConfigSqLiteInMemory();
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigDuckDB(): ConnectionConfig_ConnectionConfigDuckDB {
  return { path: "", schema: undefined };
}

export const ConnectionConfig_ConnectionConfigDuckDB = {
  encode(message: ConnectionConfig_ConnectionConfigDuckDB, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    if (message.schema !== undefined) {
      writer.uint32(18).string(message.schema);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigDuckDB {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigDuckDB();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.path = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.schema = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig_ConnectionConfigDuckDB {
    return {
      path: isSet(object.path) ? gt.String(object.path) : "",
      schema: isSet(object.schema) ? gt.String(object.schema) : undefined,
    };
  },

  toJSON(message: ConnectionConfig_ConnectionConfigDuckDB): unknown {
    const obj: any = {};
    if (message.path !== "") {
      obj.path = message.path;
    }
    if (message.schema !== undefined) {
      obj.schema = message.schema;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigDuckDB>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigDuckDB {
    return ConnectionConfig_ConnectionConfigDuckDB.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigDuckDB>, I>>(
    object: I,
  ): ConnectionConfig_ConnectionConfigDuckDB {
    const message = createBaseConnectionConfig_ConnectionConfigDuckDB();
    message.path = object.path ?? "";
    message.schema = object.schema ?? undefined;
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigDuckDBInMemory(): ConnectionConfig_ConnectionConfigDuckDBInMemory {
  return { schema: undefined };
}

export const ConnectionConfig_ConnectionConfigDuckDBInMemory = {
  encode(
    message: ConnectionConfig_ConnectionConfigDuckDBInMemory,
    writer: _m0.Writer = _m0.Writer.create(),
  ): _m0.Writer {
    if (message.schema !== undefined) {
      writer.uint32(10).string(message.schema);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigDuckDBInMemory {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigDuckDBInMemory();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.schema = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig_ConnectionConfigDuckDBInMemory {
    return { schema: isSet(object.schema) ? gt.String(object.schema) : undefined };
  },

  toJSON(message: ConnectionConfig_ConnectionConfigDuckDBInMemory): unknown {
    const obj: any = {};
    if (message.schema !== undefined) {
      obj.schema = message.schema;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigDuckDBInMemory>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigDuckDBInMemory {
    return ConnectionConfig_ConnectionConfigDuckDBInMemory.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigDuckDBInMemory>, I>>(
    object: I,
  ): ConnectionConfig_ConnectionConfigDuckDBInMemory {
    const message = createBaseConnectionConfig_ConnectionConfigDuckDBInMemory();
    message.schema = object.schema ?? undefined;
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigPostgres(): ConnectionConfig_ConnectionConfigPostgres {
  return { schema: "" };
}

export const ConnectionConfig_ConnectionConfigPostgres = {
  encode(message: ConnectionConfig_ConnectionConfigPostgres, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.schema !== "") {
      writer.uint32(10).string(message.schema);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigPostgres {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigPostgres();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.schema = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig_ConnectionConfigPostgres {
    return { schema: isSet(object.schema) ? gt.String(object.schema) : "" };
  },

  toJSON(message: ConnectionConfig_ConnectionConfigPostgres): unknown {
    const obj: any = {};
    if (message.schema !== "") {
      obj.schema = message.schema;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigPostgres>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigPostgres {
    return ConnectionConfig_ConnectionConfigPostgres.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigPostgres>, I>>(
    object: I,
  ): ConnectionConfig_ConnectionConfigPostgres {
    const message = createBaseConnectionConfig_ConnectionConfigPostgres();
    message.schema = object.schema ?? "";
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigRedshift(): ConnectionConfig_ConnectionConfigRedshift {
  return { schema: "" };
}

export const ConnectionConfig_ConnectionConfigRedshift = {
  encode(message: ConnectionConfig_ConnectionConfigRedshift, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.schema !== "") {
      writer.uint32(10).string(message.schema);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigRedshift {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigRedshift();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.schema = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig_ConnectionConfigRedshift {
    return { schema: isSet(object.schema) ? gt.String(object.schema) : "" };
  },

  toJSON(message: ConnectionConfig_ConnectionConfigRedshift): unknown {
    const obj: any = {};
    if (message.schema !== "") {
      obj.schema = message.schema;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigRedshift>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigRedshift {
    return ConnectionConfig_ConnectionConfigRedshift.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigRedshift>, I>>(
    object: I,
  ): ConnectionConfig_ConnectionConfigRedshift {
    const message = createBaseConnectionConfig_ConnectionConfigRedshift();
    message.schema = object.schema ?? "";
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigBigQuery(): ConnectionConfig_ConnectionConfigBigQuery {
  return { projectId: "", datasetId: "" };
}

export const ConnectionConfig_ConnectionConfigBigQuery = {
  encode(message: ConnectionConfig_ConnectionConfigBigQuery, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.projectId !== "") {
      writer.uint32(10).string(message.projectId);
    }
    if (message.datasetId !== "") {
      writer.uint32(18).string(message.datasetId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigBigQuery {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigBigQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.projectId = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.datasetId = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig_ConnectionConfigBigQuery {
    return {
      projectId: isSet(object.projectId) ? gt.String(object.projectId) : "",
      datasetId: isSet(object.datasetId) ? gt.String(object.datasetId) : "",
    };
  },

  toJSON(message: ConnectionConfig_ConnectionConfigBigQuery): unknown {
    const obj: any = {};
    if (message.projectId !== "") {
      obj.projectId = message.projectId;
    }
    if (message.datasetId !== "") {
      obj.datasetId = message.datasetId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigBigQuery>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigBigQuery {
    return ConnectionConfig_ConnectionConfigBigQuery.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigBigQuery>, I>>(
    object: I,
  ): ConnectionConfig_ConnectionConfigBigQuery {
    const message = createBaseConnectionConfig_ConnectionConfigBigQuery();
    message.projectId = object.projectId ?? "";
    message.datasetId = object.datasetId ?? "";
    return message;
  },
};

function createBaseConnectionConfig_ConnectionConfigSnowflake(): ConnectionConfig_ConnectionConfigSnowflake {
  return { accountUrl: "", clientId: "", clientSecret: "", role: "", database: "", schema: "", warehouse: "" };
}

export const ConnectionConfig_ConnectionConfigSnowflake = {
  encode(message: ConnectionConfig_ConnectionConfigSnowflake, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.accountUrl !== "") {
      writer.uint32(10).string(message.accountUrl);
    }
    if (message.clientId !== "") {
      writer.uint32(18).string(message.clientId);
    }
    if (message.clientSecret !== "") {
      writer.uint32(26).string(message.clientSecret);
    }
    if (message.role !== "") {
      writer.uint32(34).string(message.role);
    }
    if (message.database !== "") {
      writer.uint32(42).string(message.database);
    }
    if (message.schema !== "") {
      writer.uint32(50).string(message.schema);
    }
    if (message.warehouse !== "") {
      writer.uint32(58).string(message.warehouse);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ConnectionConfig_ConnectionConfigSnowflake {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConnectionConfig_ConnectionConfigSnowflake();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.accountUrl = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.clientId = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.clientSecret = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.role = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.database = reader.string();
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.schema = reader.string();
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          message.warehouse = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ConnectionConfig_ConnectionConfigSnowflake {
    return {
      accountUrl: isSet(object.accountUrl) ? gt.String(object.accountUrl) : "",
      clientId: isSet(object.clientId) ? gt.String(object.clientId) : "",
      clientSecret: isSet(object.clientSecret) ? gt.String(object.clientSecret) : "",
      role: isSet(object.role) ? gt.String(object.role) : "",
      database: isSet(object.database) ? gt.String(object.database) : "",
      schema: isSet(object.schema) ? gt.String(object.schema) : "",
      warehouse: isSet(object.warehouse) ? gt.String(object.warehouse) : "",
    };
  },

  toJSON(message: ConnectionConfig_ConnectionConfigSnowflake): unknown {
    const obj: any = {};
    if (message.accountUrl !== "") {
      obj.accountUrl = message.accountUrl;
    }
    if (message.clientId !== "") {
      obj.clientId = message.clientId;
    }
    if (message.clientSecret !== "") {
      obj.clientSecret = message.clientSecret;
    }
    if (message.role !== "") {
      obj.role = message.role;
    }
    if (message.database !== "") {
      obj.database = message.database;
    }
    if (message.schema !== "") {
      obj.schema = message.schema;
    }
    if (message.warehouse !== "") {
      obj.warehouse = message.warehouse;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigSnowflake>, I>>(
    base?: I,
  ): ConnectionConfig_ConnectionConfigSnowflake {
    return ConnectionConfig_ConnectionConfigSnowflake.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ConnectionConfig_ConnectionConfigSnowflake>, I>>(
    object: I,
  ): ConnectionConfig_ConnectionConfigSnowflake {
    const message = createBaseConnectionConfig_ConnectionConfigSnowflake();
    message.accountUrl = object.accountUrl ?? "";
    message.clientId = object.clientId ?? "";
    message.clientSecret = object.clientSecret ?? "";
    message.role = object.role ?? "";
    message.database = object.database ?? "";
    message.schema = object.schema ?? "";
    message.warehouse = object.warehouse ?? "";
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
