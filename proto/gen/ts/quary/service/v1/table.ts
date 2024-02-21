/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Empty } from "../../../google/protobuf/empty";
import { ColumnTest } from "./project_file";

export const protobufPackage = "quary.service.v1";

/** Table is a table in the project that is used to visualise columns, descriptions, and column tests. */
export interface Table {
  tableType?: { $case: "present"; present: Table_PresentInSchema } | {
    $case: "notPresent";
    notPresent: Table_NotPresentInSchema;
  } | undefined;
}

/**
 * PresentInSchema is a table that is present in a SQL document and in the YAML definitions.
 * Values are inferred and read from YAML.
 */
export interface Table_PresentInSchema {
  rows: Table_PresentInSchema_PresentRow[];
}

export interface Table_PresentInSchema_PresentRow {
  row?:
    | { $case: "presentInSqlAndDefinitions"; presentInSqlAndDefinitions: Row }
    | { $case: "missingInDefinitions"; missingInDefinitions: Row }
    | { $case: "presentInDefinitionsButNotRecognisableInSql"; presentInDefinitionsButNotRecognisableInSql: Row }
    | undefined;
}

/**
 * NotPresentInSchema is a table that is present as SQL but not in the YAML definitions. And so all the values
 * are inferred.
 */
export interface Table_NotPresentInSchema {
  rows: Row[];
}

/** Row is a row in the table. */
export interface Row {
  title: string;
  tests: RowTest[];
  description: RowDescription | undefined;
}

/**
 * TableTest is a test that is run against a row in a table. It is a oneof because the test can be inferred from the
 * YAML definitions, or it can be present in the SQL document.
 */
export interface RowTest {
  test?:
    | { $case: "presentAndNotInferred"; presentAndNotInferred: RowTestDetails }
    | { $case: "presentAndInferred"; presentAndInferred: RowTestDetails }
    | { $case: "notPresentButInferred"; notPresentButInferred: RowTestDetails }
    | undefined;
}

/**
 * RowTestDetails encapsulates the details of tests associated with row and column data.
 * The 'column_test' field within this structure is specifically used to manage test operations
 * such as deletion and addition. These operations are typically invoked through callbacks.
 */
export interface RowTestDetails {
  text: string;
  columnTest: ColumnTest | undefined;
}

/**
 * RowDescription is a description of a row in a table. It is a oneof because the description can be inferred from the
 * YAML definitions and/or it can be present in the SQL document.
 */
export interface RowDescription {
  description?:
    | { $case: "present"; present: string }
    | { $case: "presentAndInferredIdentical"; presentAndInferredIdentical: string }
    | { $case: "presentWithDifferentInference"; presentWithDifferentInference: RowDescription_PresentWithInference }
    | { $case: "inferred"; inferred: string }
    | { $case: "notPresent"; notPresent: Empty }
    | undefined;
}

export interface RowDescription_PresentWithInference {
  present: string;
  inferred: string;
}

function createBaseTable(): Table {
  return { tableType: undefined };
}

export const Table = {
  encode(message: Table, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.tableType?.$case) {
      case "present":
        Table_PresentInSchema.encode(message.tableType.present, writer.uint32(10).fork()).ldelim();
        break;
      case "notPresent":
        Table_NotPresentInSchema.encode(message.tableType.notPresent, writer.uint32(18).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Table {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTable();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.tableType = { $case: "present", present: Table_PresentInSchema.decode(reader, reader.uint32()) };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.tableType = {
            $case: "notPresent",
            notPresent: Table_NotPresentInSchema.decode(reader, reader.uint32()),
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

  fromJSON(object: any): Table {
    return {
      tableType: isSet(object.present)
        ? { $case: "present", present: Table_PresentInSchema.fromJSON(object.present) }
        : isSet(object.notPresent)
        ? { $case: "notPresent", notPresent: Table_NotPresentInSchema.fromJSON(object.notPresent) }
        : undefined,
    };
  },

  toJSON(message: Table): unknown {
    const obj: any = {};
    if (message.tableType?.$case === "present") {
      obj.present = Table_PresentInSchema.toJSON(message.tableType.present);
    }
    if (message.tableType?.$case === "notPresent") {
      obj.notPresent = Table_NotPresentInSchema.toJSON(message.tableType.notPresent);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Table>, I>>(base?: I): Table {
    return Table.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Table>, I>>(object: I): Table {
    const message = createBaseTable();
    if (
      object.tableType?.$case === "present" &&
      object.tableType?.present !== undefined &&
      object.tableType?.present !== null
    ) {
      message.tableType = { $case: "present", present: Table_PresentInSchema.fromPartial(object.tableType.present) };
    }
    if (
      object.tableType?.$case === "notPresent" &&
      object.tableType?.notPresent !== undefined &&
      object.tableType?.notPresent !== null
    ) {
      message.tableType = {
        $case: "notPresent",
        notPresent: Table_NotPresentInSchema.fromPartial(object.tableType.notPresent),
      };
    }
    return message;
  },
};

function createBaseTable_PresentInSchema(): Table_PresentInSchema {
  return { rows: [] };
}

export const Table_PresentInSchema = {
  encode(message: Table_PresentInSchema, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.rows) {
      Table_PresentInSchema_PresentRow.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Table_PresentInSchema {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTable_PresentInSchema();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.rows.push(Table_PresentInSchema_PresentRow.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Table_PresentInSchema {
    return {
      rows: gt.Array.isArray(object?.rows)
        ? object.rows.map((e: any) => Table_PresentInSchema_PresentRow.fromJSON(e))
        : [],
    };
  },

  toJSON(message: Table_PresentInSchema): unknown {
    const obj: any = {};
    if (message.rows?.length) {
      obj.rows = message.rows.map((e) => Table_PresentInSchema_PresentRow.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Table_PresentInSchema>, I>>(base?: I): Table_PresentInSchema {
    return Table_PresentInSchema.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Table_PresentInSchema>, I>>(object: I): Table_PresentInSchema {
    const message = createBaseTable_PresentInSchema();
    message.rows = object.rows?.map((e) => Table_PresentInSchema_PresentRow.fromPartial(e)) || [];
    return message;
  },
};

function createBaseTable_PresentInSchema_PresentRow(): Table_PresentInSchema_PresentRow {
  return { row: undefined };
}

export const Table_PresentInSchema_PresentRow = {
  encode(message: Table_PresentInSchema_PresentRow, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.row?.$case) {
      case "presentInSqlAndDefinitions":
        Row.encode(message.row.presentInSqlAndDefinitions, writer.uint32(10).fork()).ldelim();
        break;
      case "missingInDefinitions":
        Row.encode(message.row.missingInDefinitions, writer.uint32(18).fork()).ldelim();
        break;
      case "presentInDefinitionsButNotRecognisableInSql":
        Row.encode(message.row.presentInDefinitionsButNotRecognisableInSql, writer.uint32(26).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Table_PresentInSchema_PresentRow {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTable_PresentInSchema_PresentRow();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.row = {
            $case: "presentInSqlAndDefinitions",
            presentInSqlAndDefinitions: Row.decode(reader, reader.uint32()),
          };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.row = { $case: "missingInDefinitions", missingInDefinitions: Row.decode(reader, reader.uint32()) };
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.row = {
            $case: "presentInDefinitionsButNotRecognisableInSql",
            presentInDefinitionsButNotRecognisableInSql: Row.decode(reader, reader.uint32()),
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

  fromJSON(object: any): Table_PresentInSchema_PresentRow {
    return {
      row: isSet(object.presentInSqlAndDefinitions)
        ? {
          $case: "presentInSqlAndDefinitions",
          presentInSqlAndDefinitions: Row.fromJSON(object.presentInSqlAndDefinitions),
        }
        : isSet(object.missingInDefinitions)
        ? { $case: "missingInDefinitions", missingInDefinitions: Row.fromJSON(object.missingInDefinitions) }
        : isSet(object.presentInDefinitionsButNotRecognisableInSql)
        ? {
          $case: "presentInDefinitionsButNotRecognisableInSql",
          presentInDefinitionsButNotRecognisableInSql: Row.fromJSON(object.presentInDefinitionsButNotRecognisableInSql),
        }
        : undefined,
    };
  },

  toJSON(message: Table_PresentInSchema_PresentRow): unknown {
    const obj: any = {};
    if (message.row?.$case === "presentInSqlAndDefinitions") {
      obj.presentInSqlAndDefinitions = Row.toJSON(message.row.presentInSqlAndDefinitions);
    }
    if (message.row?.$case === "missingInDefinitions") {
      obj.missingInDefinitions = Row.toJSON(message.row.missingInDefinitions);
    }
    if (message.row?.$case === "presentInDefinitionsButNotRecognisableInSql") {
      obj.presentInDefinitionsButNotRecognisableInSql = Row.toJSON(
        message.row.presentInDefinitionsButNotRecognisableInSql,
      );
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Table_PresentInSchema_PresentRow>, I>>(
    base?: I,
  ): Table_PresentInSchema_PresentRow {
    return Table_PresentInSchema_PresentRow.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Table_PresentInSchema_PresentRow>, I>>(
    object: I,
  ): Table_PresentInSchema_PresentRow {
    const message = createBaseTable_PresentInSchema_PresentRow();
    if (
      object.row?.$case === "presentInSqlAndDefinitions" &&
      object.row?.presentInSqlAndDefinitions !== undefined &&
      object.row?.presentInSqlAndDefinitions !== null
    ) {
      message.row = {
        $case: "presentInSqlAndDefinitions",
        presentInSqlAndDefinitions: Row.fromPartial(object.row.presentInSqlAndDefinitions),
      };
    }
    if (
      object.row?.$case === "missingInDefinitions" &&
      object.row?.missingInDefinitions !== undefined &&
      object.row?.missingInDefinitions !== null
    ) {
      message.row = {
        $case: "missingInDefinitions",
        missingInDefinitions: Row.fromPartial(object.row.missingInDefinitions),
      };
    }
    if (
      object.row?.$case === "presentInDefinitionsButNotRecognisableInSql" &&
      object.row?.presentInDefinitionsButNotRecognisableInSql !== undefined &&
      object.row?.presentInDefinitionsButNotRecognisableInSql !== null
    ) {
      message.row = {
        $case: "presentInDefinitionsButNotRecognisableInSql",
        presentInDefinitionsButNotRecognisableInSql: Row.fromPartial(
          object.row.presentInDefinitionsButNotRecognisableInSql,
        ),
      };
    }
    return message;
  },
};

function createBaseTable_NotPresentInSchema(): Table_NotPresentInSchema {
  return { rows: [] };
}

export const Table_NotPresentInSchema = {
  encode(message: Table_NotPresentInSchema, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.rows) {
      Row.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Table_NotPresentInSchema {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTable_NotPresentInSchema();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.rows.push(Row.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Table_NotPresentInSchema {
    return { rows: gt.Array.isArray(object?.rows) ? object.rows.map((e: any) => Row.fromJSON(e)) : [] };
  },

  toJSON(message: Table_NotPresentInSchema): unknown {
    const obj: any = {};
    if (message.rows?.length) {
      obj.rows = message.rows.map((e) => Row.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Table_NotPresentInSchema>, I>>(base?: I): Table_NotPresentInSchema {
    return Table_NotPresentInSchema.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Table_NotPresentInSchema>, I>>(object: I): Table_NotPresentInSchema {
    const message = createBaseTable_NotPresentInSchema();
    message.rows = object.rows?.map((e) => Row.fromPartial(e)) || [];
    return message;
  },
};

function createBaseRow(): Row {
  return { title: "", tests: [], description: undefined };
}

export const Row = {
  encode(message: Row, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.title !== "") {
      writer.uint32(10).string(message.title);
    }
    for (const v of message.tests) {
      RowTest.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.description !== undefined) {
      RowDescription.encode(message.description, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Row {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRow();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.title = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.tests.push(RowTest.decode(reader, reader.uint32()));
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.description = RowDescription.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Row {
    return {
      title: isSet(object.title) ? gt.String(object.title) : "",
      tests: gt.Array.isArray(object?.tests) ? object.tests.map((e: any) => RowTest.fromJSON(e)) : [],
      description: isSet(object.description) ? RowDescription.fromJSON(object.description) : undefined,
    };
  },

  toJSON(message: Row): unknown {
    const obj: any = {};
    if (message.title !== "") {
      obj.title = message.title;
    }
    if (message.tests?.length) {
      obj.tests = message.tests.map((e) => RowTest.toJSON(e));
    }
    if (message.description !== undefined) {
      obj.description = RowDescription.toJSON(message.description);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Row>, I>>(base?: I): Row {
    return Row.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Row>, I>>(object: I): Row {
    const message = createBaseRow();
    message.title = object.title ?? "";
    message.tests = object.tests?.map((e) => RowTest.fromPartial(e)) || [];
    message.description = (object.description !== undefined && object.description !== null)
      ? RowDescription.fromPartial(object.description)
      : undefined;
    return message;
  },
};

function createBaseRowTest(): RowTest {
  return { test: undefined };
}

export const RowTest = {
  encode(message: RowTest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.test?.$case) {
      case "presentAndNotInferred":
        RowTestDetails.encode(message.test.presentAndNotInferred, writer.uint32(10).fork()).ldelim();
        break;
      case "presentAndInferred":
        RowTestDetails.encode(message.test.presentAndInferred, writer.uint32(18).fork()).ldelim();
        break;
      case "notPresentButInferred":
        RowTestDetails.encode(message.test.notPresentButInferred, writer.uint32(26).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RowTest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRowTest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.test = {
            $case: "presentAndNotInferred",
            presentAndNotInferred: RowTestDetails.decode(reader, reader.uint32()),
          };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.test = {
            $case: "presentAndInferred",
            presentAndInferred: RowTestDetails.decode(reader, reader.uint32()),
          };
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.test = {
            $case: "notPresentButInferred",
            notPresentButInferred: RowTestDetails.decode(reader, reader.uint32()),
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

  fromJSON(object: any): RowTest {
    return {
      test: isSet(object.presentAndNotInferred)
        ? {
          $case: "presentAndNotInferred",
          presentAndNotInferred: RowTestDetails.fromJSON(object.presentAndNotInferred),
        }
        : isSet(object.presentAndInferred)
        ? { $case: "presentAndInferred", presentAndInferred: RowTestDetails.fromJSON(object.presentAndInferred) }
        : isSet(object.notPresentButInferred)
        ? {
          $case: "notPresentButInferred",
          notPresentButInferred: RowTestDetails.fromJSON(object.notPresentButInferred),
        }
        : undefined,
    };
  },

  toJSON(message: RowTest): unknown {
    const obj: any = {};
    if (message.test?.$case === "presentAndNotInferred") {
      obj.presentAndNotInferred = RowTestDetails.toJSON(message.test.presentAndNotInferred);
    }
    if (message.test?.$case === "presentAndInferred") {
      obj.presentAndInferred = RowTestDetails.toJSON(message.test.presentAndInferred);
    }
    if (message.test?.$case === "notPresentButInferred") {
      obj.notPresentButInferred = RowTestDetails.toJSON(message.test.notPresentButInferred);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RowTest>, I>>(base?: I): RowTest {
    return RowTest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RowTest>, I>>(object: I): RowTest {
    const message = createBaseRowTest();
    if (
      object.test?.$case === "presentAndNotInferred" &&
      object.test?.presentAndNotInferred !== undefined &&
      object.test?.presentAndNotInferred !== null
    ) {
      message.test = {
        $case: "presentAndNotInferred",
        presentAndNotInferred: RowTestDetails.fromPartial(object.test.presentAndNotInferred),
      };
    }
    if (
      object.test?.$case === "presentAndInferred" &&
      object.test?.presentAndInferred !== undefined &&
      object.test?.presentAndInferred !== null
    ) {
      message.test = {
        $case: "presentAndInferred",
        presentAndInferred: RowTestDetails.fromPartial(object.test.presentAndInferred),
      };
    }
    if (
      object.test?.$case === "notPresentButInferred" &&
      object.test?.notPresentButInferred !== undefined &&
      object.test?.notPresentButInferred !== null
    ) {
      message.test = {
        $case: "notPresentButInferred",
        notPresentButInferred: RowTestDetails.fromPartial(object.test.notPresentButInferred),
      };
    }
    return message;
  },
};

function createBaseRowTestDetails(): RowTestDetails {
  return { text: "", columnTest: undefined };
}

export const RowTestDetails = {
  encode(message: RowTestDetails, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.text !== "") {
      writer.uint32(10).string(message.text);
    }
    if (message.columnTest !== undefined) {
      ColumnTest.encode(message.columnTest, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RowTestDetails {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRowTestDetails();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.text = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.columnTest = ColumnTest.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RowTestDetails {
    return {
      text: isSet(object.text) ? gt.String(object.text) : "",
      columnTest: isSet(object.columnTest) ? ColumnTest.fromJSON(object.columnTest) : undefined,
    };
  },

  toJSON(message: RowTestDetails): unknown {
    const obj: any = {};
    if (message.text !== "") {
      obj.text = message.text;
    }
    if (message.columnTest !== undefined) {
      obj.columnTest = ColumnTest.toJSON(message.columnTest);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RowTestDetails>, I>>(base?: I): RowTestDetails {
    return RowTestDetails.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RowTestDetails>, I>>(object: I): RowTestDetails {
    const message = createBaseRowTestDetails();
    message.text = object.text ?? "";
    message.columnTest = (object.columnTest !== undefined && object.columnTest !== null)
      ? ColumnTest.fromPartial(object.columnTest)
      : undefined;
    return message;
  },
};

function createBaseRowDescription(): RowDescription {
  return { description: undefined };
}

export const RowDescription = {
  encode(message: RowDescription, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.description?.$case) {
      case "present":
        writer.uint32(10).string(message.description.present);
        break;
      case "presentAndInferredIdentical":
        writer.uint32(18).string(message.description.presentAndInferredIdentical);
        break;
      case "presentWithDifferentInference":
        RowDescription_PresentWithInference.encode(
          message.description.presentWithDifferentInference,
          writer.uint32(26).fork(),
        ).ldelim();
        break;
      case "inferred":
        writer.uint32(34).string(message.description.inferred);
        break;
      case "notPresent":
        Empty.encode(message.description.notPresent, writer.uint32(42).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RowDescription {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRowDescription();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.description = { $case: "present", present: reader.string() };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.description = { $case: "presentAndInferredIdentical", presentAndInferredIdentical: reader.string() };
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.description = {
            $case: "presentWithDifferentInference",
            presentWithDifferentInference: RowDescription_PresentWithInference.decode(reader, reader.uint32()),
          };
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.description = { $case: "inferred", inferred: reader.string() };
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.description = { $case: "notPresent", notPresent: Empty.decode(reader, reader.uint32()) };
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RowDescription {
    return {
      description: isSet(object.present)
        ? { $case: "present", present: gt.String(object.present) }
        : isSet(object.presentAndInferredIdentical)
        ? {
          $case: "presentAndInferredIdentical",
          presentAndInferredIdentical: gt.String(object.presentAndInferredIdentical),
        }
        : isSet(object.presentWithDifferentInference)
        ? {
          $case: "presentWithDifferentInference",
          presentWithDifferentInference: RowDescription_PresentWithInference.fromJSON(
            object.presentWithDifferentInference,
          ),
        }
        : isSet(object.inferred)
        ? { $case: "inferred", inferred: gt.String(object.inferred) }
        : isSet(object.notPresent)
        ? { $case: "notPresent", notPresent: Empty.fromJSON(object.notPresent) }
        : undefined,
    };
  },

  toJSON(message: RowDescription): unknown {
    const obj: any = {};
    if (message.description?.$case === "present") {
      obj.present = message.description.present;
    }
    if (message.description?.$case === "presentAndInferredIdentical") {
      obj.presentAndInferredIdentical = message.description.presentAndInferredIdentical;
    }
    if (message.description?.$case === "presentWithDifferentInference") {
      obj.presentWithDifferentInference = RowDescription_PresentWithInference.toJSON(
        message.description.presentWithDifferentInference,
      );
    }
    if (message.description?.$case === "inferred") {
      obj.inferred = message.description.inferred;
    }
    if (message.description?.$case === "notPresent") {
      obj.notPresent = Empty.toJSON(message.description.notPresent);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RowDescription>, I>>(base?: I): RowDescription {
    return RowDescription.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RowDescription>, I>>(object: I): RowDescription {
    const message = createBaseRowDescription();
    if (
      object.description?.$case === "present" &&
      object.description?.present !== undefined &&
      object.description?.present !== null
    ) {
      message.description = { $case: "present", present: object.description.present };
    }
    if (
      object.description?.$case === "presentAndInferredIdentical" &&
      object.description?.presentAndInferredIdentical !== undefined &&
      object.description?.presentAndInferredIdentical !== null
    ) {
      message.description = {
        $case: "presentAndInferredIdentical",
        presentAndInferredIdentical: object.description.presentAndInferredIdentical,
      };
    }
    if (
      object.description?.$case === "presentWithDifferentInference" &&
      object.description?.presentWithDifferentInference !== undefined &&
      object.description?.presentWithDifferentInference !== null
    ) {
      message.description = {
        $case: "presentWithDifferentInference",
        presentWithDifferentInference: RowDescription_PresentWithInference.fromPartial(
          object.description.presentWithDifferentInference,
        ),
      };
    }
    if (
      object.description?.$case === "inferred" &&
      object.description?.inferred !== undefined &&
      object.description?.inferred !== null
    ) {
      message.description = { $case: "inferred", inferred: object.description.inferred };
    }
    if (
      object.description?.$case === "notPresent" &&
      object.description?.notPresent !== undefined &&
      object.description?.notPresent !== null
    ) {
      message.description = { $case: "notPresent", notPresent: Empty.fromPartial(object.description.notPresent) };
    }
    return message;
  },
};

function createBaseRowDescription_PresentWithInference(): RowDescription_PresentWithInference {
  return { present: "", inferred: "" };
}

export const RowDescription_PresentWithInference = {
  encode(message: RowDescription_PresentWithInference, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.present !== "") {
      writer.uint32(10).string(message.present);
    }
    if (message.inferred !== "") {
      writer.uint32(18).string(message.inferred);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RowDescription_PresentWithInference {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRowDescription_PresentWithInference();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.present = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.inferred = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RowDescription_PresentWithInference {
    return {
      present: isSet(object.present) ? gt.String(object.present) : "",
      inferred: isSet(object.inferred) ? gt.String(object.inferred) : "",
    };
  },

  toJSON(message: RowDescription_PresentWithInference): unknown {
    const obj: any = {};
    if (message.present !== "") {
      obj.present = message.present;
    }
    if (message.inferred !== "") {
      obj.inferred = message.inferred;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RowDescription_PresentWithInference>, I>>(
    base?: I,
  ): RowDescription_PresentWithInference {
    return RowDescription_PresentWithInference.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RowDescription_PresentWithInference>, I>>(
    object: I,
  ): RowDescription_PresentWithInference {
    const message = createBaseRowDescription_PresentWithInference();
    message.present = object.present ?? "";
    message.inferred = object.inferred ?? "";
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
