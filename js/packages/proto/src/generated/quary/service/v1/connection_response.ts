/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "quary.service.v1";

export interface BigQueryProjectReference {
  projectId: string;
}

/** Project API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/projects/list */
export interface BigQueryProject {
  /** The resource type. */
  kind: string;
  /** An opaque ID of this project. */
  id: string;
  /** The numeric ID of this project. */
  numericId: string;
  /** A unique reference to a project. */
  projectReference:
    | BigQueryProjectReference
    | undefined;
  /** A descriptive name for this project. */
  friendlyName: string;
}

/** Dataset Reference API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/datasets#DatasetReference */
export interface BigQueryDatasetReference {
  projectId: string;
  datasetId: string;
}

/** Dataset API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/datasets/list */
export interface BigQueryDataset {
  /** The resource type. This property always returns the value "bigquery#dataset" */
  kind: string;
  /** The fully-qualified, unique, opaque ID of the dataset. */
  id: string;
  /** The dataset reference. Use this property to access specific parts of the dataset's ID, such as project ID or dataset ID. */
  datasetReference:
    | BigQueryDatasetReference
    | undefined;
  /** An alternate name for the dataset. The friendly name is purely decorative in nature. */
  friendlyName: string;
  /** The labels associated with this dataset. You can use these to organize and group your datasets. */
  labels: { [key: string]: string };
  /** The geographic location where the dataset resides. */
  location: string;
}

export interface BigQueryDataset_LabelsEntry {
  key: string;
  value: string;
}

/** Table Reference API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#TableReference */
export interface BigQueryTableReference {
  projectId: string;
  datasetId: string;
  tableId: string;
}

/** Table API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/tables/list */
export interface BigQueryTable {
  /** The resource type. */
  kind: string;
  /** An opaque ID uniquely identifying the table. */
  id: string;
  /** A reference uniquely identifying the table. */
  tableReference:
    | BigQueryTableReference
    | undefined;
  /** The type of table. Possible values are: TABLE, VIEW. */
  type: string;
  /** The time when this table was created, in milliseconds since the epoch. */
  creationTime: string;
}

/** Table Field API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#TableFieldSchema */
export interface BigQueryTableField {
  /** The field name. */
  name: string;
  /** The field data type. Possible values are: STRING, INTEGER, FLOAT, BOOLEAN, TIMESTAMP. */
  type: string;
  /** The field mode. Possible values are: NULLABLE, REQUIRED, REPEATED. */
  mode: string;
  /** Describes the nested schema fields if the type property is set to RECORD. */
  fields: BigQueryTableField[];
  /** The field description. */
  description: string;
}

/** Table Schema API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#TableSchema */
export interface BigQueryTableSchema {
  name: string;
  type: string;
  /** Describes the fields in a table. */
  fields: BigQueryTableField[];
  /** The field description. */
  description: string;
}

/** JobReference API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/JobReference */
export interface BigQueryJobReference {
  /** The ID of the project containing this job. */
  projectId: string;
  /** The ID of the job. */
  jobId: string;
  /** The geographic location of the job. */
  location: string;
}

/** BigQueryError API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/ErrorProto */
export interface BigQueryError {
  /** A short error code that summarizes the error. */
  reason: string;
  /** The location of the error, if applicable. */
  location: string;
  /** A human-readable description of the error. */
  debugInfo: string;
  /** A human-readable description of the error. */
  message: string;
}

/** JobStatus API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/JobStatus */
export interface BigQueryJobStatus {
  /** The state of the job. Possible values include: "PENDING", "RUNNING", "DONE". */
  state: string;
  /** Final error result of the job. If present, indicates that the job has completed and was unsuccessful. */
  errorResult: string;
  /** The final error result of the job as a human-readable string. */
  errorMessage: string;
}

/** Job API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/Job */
export interface BigQueryJob {
  /** The resource type. */
  kind: string;
  /** An opaque ID uniquely identifying the job. */
  id: string;
  /** A URL that can be used to access this resource again. */
  selfLink: string;
  /** The email address of the user who ran the job. */
  userEmail: string;
  /** A reference uniquely identifying the job. */
  jobReference:
    | BigQueryJobReference
    | undefined;
  /** Information about the job, including starting time and ending time of the job. */
  status: BigQueryJobStatus | undefined;
}

export interface BigQueryFieldValue {
  /** Represents the 'f' in a JSON object */
  f: string;
  /** Represents the 'v' in a JSON object */
  v: string;
}

export interface BigQueryTableRow {
  /** Represents a single row as a series of field-value pairs */
  f: BigQueryFieldValue[];
}

/** BigQueryJobResults API Documentation: https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/getQueryResults */
export interface BigQueryJobResults {
  /** The resource type. */
  kind: string;
  /** The schema of the results. Present only when the query completes successfully. */
  schema:
    | BigQueryTableSchema
    | undefined;
  /** A reference uniquely identifying the job. */
  jobReference:
    | BigQueryJobReference
    | undefined;
  /** The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results. */
  totalRows: string;
  /** A token used for paging results. */
  pageToken: string;
  /** An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above. */
  rows: BigQueryTableRow[];
  /** Whether the query result was fetched from the query cache. */
  jobComplete: string;
  /** The first errors encountered during the running of the job. The final message includes the number of errors encountered. */
  errors: BigQueryError[];
  /** Whether the query result was fetched from the query cache. */
  cacheHit: string;
  /** The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE. */
  numDmlAffectedRows: string;
}

/** Google Oauth 2.0 token API Documentation: https://developers.google.com/identity/protocols/oauth2/web-server#httprest */
export interface BigQueryOauthToken {
  /** short-lived token */
  accessToken: string;
  refreshToken: string;
  /** absolute time the token expires */
  expiryTime: string;
}

export interface BigQueryOauthTokenRefresh {
  /** short-lived token */
  accessToken: string;
  /** absolute time the token expires */
  expiryTime: string;
}

/** Snowflake SSO */
export interface SnowflakeOauthProxyRequest {
  accountUrl: string;
  clientId: string;
  clientSecret: string;
  role: string;
}

export interface SnowflakeOauthToken {
  accessToken: string;
  refreshToken: string;
  expiryTime: string;
}

export interface SnowflakeOauthRefreshToken {
  accessToken: string;
  expiryTime: string;
}

function createBaseBigQueryProjectReference(): BigQueryProjectReference {
  return { projectId: "" };
}

export const BigQueryProjectReference = {
  encode(message: BigQueryProjectReference, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.projectId !== "") {
      writer.uint32(10).string(message.projectId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryProjectReference {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryProjectReference();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.projectId = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryProjectReference {
    return { projectId: isSet(object.projectId) ? gt.String(object.projectId) : "" };
  },

  toJSON(message: BigQueryProjectReference): unknown {
    const obj: any = {};
    if (message.projectId !== "") {
      obj.projectId = message.projectId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryProjectReference>, I>>(base?: I): BigQueryProjectReference {
    return BigQueryProjectReference.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryProjectReference>, I>>(object: I): BigQueryProjectReference {
    const message = createBaseBigQueryProjectReference();
    message.projectId = object.projectId ?? "";
    return message;
  },
};

function createBaseBigQueryProject(): BigQueryProject {
  return { kind: "", id: "", numericId: "", projectReference: undefined, friendlyName: "" };
}

export const BigQueryProject = {
  encode(message: BigQueryProject, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.kind !== "") {
      writer.uint32(10).string(message.kind);
    }
    if (message.id !== "") {
      writer.uint32(18).string(message.id);
    }
    if (message.numericId !== "") {
      writer.uint32(26).string(message.numericId);
    }
    if (message.projectReference !== undefined) {
      BigQueryProjectReference.encode(message.projectReference, writer.uint32(34).fork()).ldelim();
    }
    if (message.friendlyName !== "") {
      writer.uint32(42).string(message.friendlyName);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryProject {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryProject();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.kind = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.id = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.numericId = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.projectReference = BigQueryProjectReference.decode(reader, reader.uint32());
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.friendlyName = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryProject {
    return {
      kind: isSet(object.kind) ? gt.String(object.kind) : "",
      id: isSet(object.id) ? gt.String(object.id) : "",
      numericId: isSet(object.numericId) ? gt.String(object.numericId) : "",
      projectReference: isSet(object.projectReference)
        ? BigQueryProjectReference.fromJSON(object.projectReference)
        : undefined,
      friendlyName: isSet(object.friendlyName) ? gt.String(object.friendlyName) : "",
    };
  },

  toJSON(message: BigQueryProject): unknown {
    const obj: any = {};
    if (message.kind !== "") {
      obj.kind = message.kind;
    }
    if (message.id !== "") {
      obj.id = message.id;
    }
    if (message.numericId !== "") {
      obj.numericId = message.numericId;
    }
    if (message.projectReference !== undefined) {
      obj.projectReference = BigQueryProjectReference.toJSON(message.projectReference);
    }
    if (message.friendlyName !== "") {
      obj.friendlyName = message.friendlyName;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryProject>, I>>(base?: I): BigQueryProject {
    return BigQueryProject.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryProject>, I>>(object: I): BigQueryProject {
    const message = createBaseBigQueryProject();
    message.kind = object.kind ?? "";
    message.id = object.id ?? "";
    message.numericId = object.numericId ?? "";
    message.projectReference = (object.projectReference !== undefined && object.projectReference !== null)
      ? BigQueryProjectReference.fromPartial(object.projectReference)
      : undefined;
    message.friendlyName = object.friendlyName ?? "";
    return message;
  },
};

function createBaseBigQueryDatasetReference(): BigQueryDatasetReference {
  return { projectId: "", datasetId: "" };
}

export const BigQueryDatasetReference = {
  encode(message: BigQueryDatasetReference, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.projectId !== "") {
      writer.uint32(10).string(message.projectId);
    }
    if (message.datasetId !== "") {
      writer.uint32(18).string(message.datasetId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryDatasetReference {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryDatasetReference();
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

  fromJSON(object: any): BigQueryDatasetReference {
    return {
      projectId: isSet(object.projectId) ? gt.String(object.projectId) : "",
      datasetId: isSet(object.datasetId) ? gt.String(object.datasetId) : "",
    };
  },

  toJSON(message: BigQueryDatasetReference): unknown {
    const obj: any = {};
    if (message.projectId !== "") {
      obj.projectId = message.projectId;
    }
    if (message.datasetId !== "") {
      obj.datasetId = message.datasetId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryDatasetReference>, I>>(base?: I): BigQueryDatasetReference {
    return BigQueryDatasetReference.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryDatasetReference>, I>>(object: I): BigQueryDatasetReference {
    const message = createBaseBigQueryDatasetReference();
    message.projectId = object.projectId ?? "";
    message.datasetId = object.datasetId ?? "";
    return message;
  },
};

function createBaseBigQueryDataset(): BigQueryDataset {
  return { kind: "", id: "", datasetReference: undefined, friendlyName: "", labels: {}, location: "" };
}

export const BigQueryDataset = {
  encode(message: BigQueryDataset, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.kind !== "") {
      writer.uint32(10).string(message.kind);
    }
    if (message.id !== "") {
      writer.uint32(18).string(message.id);
    }
    if (message.datasetReference !== undefined) {
      BigQueryDatasetReference.encode(message.datasetReference, writer.uint32(26).fork()).ldelim();
    }
    if (message.friendlyName !== "") {
      writer.uint32(34).string(message.friendlyName);
    }
    Object.entries(message.labels).forEach(([key, value]) => {
      BigQueryDataset_LabelsEntry.encode({ key: key as any, value }, writer.uint32(42).fork()).ldelim();
    });
    if (message.location !== "") {
      writer.uint32(50).string(message.location);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryDataset {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryDataset();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.kind = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.id = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.datasetReference = BigQueryDatasetReference.decode(reader, reader.uint32());
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.friendlyName = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          const entry5 = BigQueryDataset_LabelsEntry.decode(reader, reader.uint32());
          if (entry5.value !== undefined) {
            message.labels[entry5.key] = entry5.value;
          }
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.location = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryDataset {
    return {
      kind: isSet(object.kind) ? gt.String(object.kind) : "",
      id: isSet(object.id) ? gt.String(object.id) : "",
      datasetReference: isSet(object.datasetReference)
        ? BigQueryDatasetReference.fromJSON(object.datasetReference)
        : undefined,
      friendlyName: isSet(object.friendlyName) ? gt.String(object.friendlyName) : "",
      labels: isObject(object.labels)
        ? Object.entries(object.labels).reduce<{ [key: string]: string }>((acc, [key, value]) => {
          acc[key] = String(value);
          return acc;
        }, {})
        : {},
      location: isSet(object.location) ? gt.String(object.location) : "",
    };
  },

  toJSON(message: BigQueryDataset): unknown {
    const obj: any = {};
    if (message.kind !== "") {
      obj.kind = message.kind;
    }
    if (message.id !== "") {
      obj.id = message.id;
    }
    if (message.datasetReference !== undefined) {
      obj.datasetReference = BigQueryDatasetReference.toJSON(message.datasetReference);
    }
    if (message.friendlyName !== "") {
      obj.friendlyName = message.friendlyName;
    }
    if (message.labels) {
      const entries = Object.entries(message.labels);
      if (entries.length > 0) {
        obj.labels = {};
        entries.forEach(([k, v]) => {
          obj.labels[k] = v;
        });
      }
    }
    if (message.location !== "") {
      obj.location = message.location;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryDataset>, I>>(base?: I): BigQueryDataset {
    return BigQueryDataset.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryDataset>, I>>(object: I): BigQueryDataset {
    const message = createBaseBigQueryDataset();
    message.kind = object.kind ?? "";
    message.id = object.id ?? "";
    message.datasetReference = (object.datasetReference !== undefined && object.datasetReference !== null)
      ? BigQueryDatasetReference.fromPartial(object.datasetReference)
      : undefined;
    message.friendlyName = object.friendlyName ?? "";
    message.labels = Object.entries(object.labels ?? {}).reduce<{ [key: string]: string }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = gt.String(value);
      }
      return acc;
    }, {});
    message.location = object.location ?? "";
    return message;
  },
};

function createBaseBigQueryDataset_LabelsEntry(): BigQueryDataset_LabelsEntry {
  return { key: "", value: "" };
}

export const BigQueryDataset_LabelsEntry = {
  encode(message: BigQueryDataset_LabelsEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryDataset_LabelsEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryDataset_LabelsEntry();
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

  fromJSON(object: any): BigQueryDataset_LabelsEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? gt.String(object.value) : "",
    };
  },

  toJSON(message: BigQueryDataset_LabelsEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryDataset_LabelsEntry>, I>>(base?: I): BigQueryDataset_LabelsEntry {
    return BigQueryDataset_LabelsEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryDataset_LabelsEntry>, I>>(object: I): BigQueryDataset_LabelsEntry {
    const message = createBaseBigQueryDataset_LabelsEntry();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseBigQueryTableReference(): BigQueryTableReference {
  return { projectId: "", datasetId: "", tableId: "" };
}

export const BigQueryTableReference = {
  encode(message: BigQueryTableReference, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.projectId !== "") {
      writer.uint32(10).string(message.projectId);
    }
    if (message.datasetId !== "") {
      writer.uint32(18).string(message.datasetId);
    }
    if (message.tableId !== "") {
      writer.uint32(26).string(message.tableId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryTableReference {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryTableReference();
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
        case 3:
          if (tag !== 26) {
            break;
          }

          message.tableId = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryTableReference {
    return {
      projectId: isSet(object.projectId) ? gt.String(object.projectId) : "",
      datasetId: isSet(object.datasetId) ? gt.String(object.datasetId) : "",
      tableId: isSet(object.tableId) ? gt.String(object.tableId) : "",
    };
  },

  toJSON(message: BigQueryTableReference): unknown {
    const obj: any = {};
    if (message.projectId !== "") {
      obj.projectId = message.projectId;
    }
    if (message.datasetId !== "") {
      obj.datasetId = message.datasetId;
    }
    if (message.tableId !== "") {
      obj.tableId = message.tableId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryTableReference>, I>>(base?: I): BigQueryTableReference {
    return BigQueryTableReference.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryTableReference>, I>>(object: I): BigQueryTableReference {
    const message = createBaseBigQueryTableReference();
    message.projectId = object.projectId ?? "";
    message.datasetId = object.datasetId ?? "";
    message.tableId = object.tableId ?? "";
    return message;
  },
};

function createBaseBigQueryTable(): BigQueryTable {
  return { kind: "", id: "", tableReference: undefined, type: "", creationTime: "" };
}

export const BigQueryTable = {
  encode(message: BigQueryTable, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.kind !== "") {
      writer.uint32(10).string(message.kind);
    }
    if (message.id !== "") {
      writer.uint32(18).string(message.id);
    }
    if (message.tableReference !== undefined) {
      BigQueryTableReference.encode(message.tableReference, writer.uint32(26).fork()).ldelim();
    }
    if (message.type !== "") {
      writer.uint32(34).string(message.type);
    }
    if (message.creationTime !== "") {
      writer.uint32(42).string(message.creationTime);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryTable {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryTable();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.kind = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.id = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.tableReference = BigQueryTableReference.decode(reader, reader.uint32());
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.type = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.creationTime = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryTable {
    return {
      kind: isSet(object.kind) ? gt.String(object.kind) : "",
      id: isSet(object.id) ? gt.String(object.id) : "",
      tableReference: isSet(object.tableReference) ? BigQueryTableReference.fromJSON(object.tableReference) : undefined,
      type: isSet(object.type) ? gt.String(object.type) : "",
      creationTime: isSet(object.creationTime) ? gt.String(object.creationTime) : "",
    };
  },

  toJSON(message: BigQueryTable): unknown {
    const obj: any = {};
    if (message.kind !== "") {
      obj.kind = message.kind;
    }
    if (message.id !== "") {
      obj.id = message.id;
    }
    if (message.tableReference !== undefined) {
      obj.tableReference = BigQueryTableReference.toJSON(message.tableReference);
    }
    if (message.type !== "") {
      obj.type = message.type;
    }
    if (message.creationTime !== "") {
      obj.creationTime = message.creationTime;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryTable>, I>>(base?: I): BigQueryTable {
    return BigQueryTable.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryTable>, I>>(object: I): BigQueryTable {
    const message = createBaseBigQueryTable();
    message.kind = object.kind ?? "";
    message.id = object.id ?? "";
    message.tableReference = (object.tableReference !== undefined && object.tableReference !== null)
      ? BigQueryTableReference.fromPartial(object.tableReference)
      : undefined;
    message.type = object.type ?? "";
    message.creationTime = object.creationTime ?? "";
    return message;
  },
};

function createBaseBigQueryTableField(): BigQueryTableField {
  return { name: "", type: "", mode: "", fields: [], description: "" };
}

export const BigQueryTableField = {
  encode(message: BigQueryTableField, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.type !== "") {
      writer.uint32(18).string(message.type);
    }
    if (message.mode !== "") {
      writer.uint32(26).string(message.mode);
    }
    for (const v of message.fields) {
      BigQueryTableField.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    if (message.description !== "") {
      writer.uint32(42).string(message.description);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryTableField {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryTableField();
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

          message.type = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.mode = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.fields.push(BigQueryTableField.decode(reader, reader.uint32()));
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.description = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryTableField {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      type: isSet(object.type) ? gt.String(object.type) : "",
      mode: isSet(object.mode) ? gt.String(object.mode) : "",
      fields: gt.Array.isArray(object?.fields) ? object.fields.map((e: any) => BigQueryTableField.fromJSON(e)) : [],
      description: isSet(object.description) ? gt.String(object.description) : "",
    };
  },

  toJSON(message: BigQueryTableField): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.type !== "") {
      obj.type = message.type;
    }
    if (message.mode !== "") {
      obj.mode = message.mode;
    }
    if (message.fields?.length) {
      obj.fields = message.fields.map((e) => BigQueryTableField.toJSON(e));
    }
    if (message.description !== "") {
      obj.description = message.description;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryTableField>, I>>(base?: I): BigQueryTableField {
    return BigQueryTableField.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryTableField>, I>>(object: I): BigQueryTableField {
    const message = createBaseBigQueryTableField();
    message.name = object.name ?? "";
    message.type = object.type ?? "";
    message.mode = object.mode ?? "";
    message.fields = object.fields?.map((e) => BigQueryTableField.fromPartial(e)) || [];
    message.description = object.description ?? "";
    return message;
  },
};

function createBaseBigQueryTableSchema(): BigQueryTableSchema {
  return { name: "", type: "", fields: [], description: "" };
}

export const BigQueryTableSchema = {
  encode(message: BigQueryTableSchema, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.type !== "") {
      writer.uint32(18).string(message.type);
    }
    for (const v of message.fields) {
      BigQueryTableField.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    if (message.description !== "") {
      writer.uint32(34).string(message.description);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryTableSchema {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryTableSchema();
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

          message.type = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.fields.push(BigQueryTableField.decode(reader, reader.uint32()));
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.description = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryTableSchema {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      type: isSet(object.type) ? gt.String(object.type) : "",
      fields: gt.Array.isArray(object?.fields) ? object.fields.map((e: any) => BigQueryTableField.fromJSON(e)) : [],
      description: isSet(object.description) ? gt.String(object.description) : "",
    };
  },

  toJSON(message: BigQueryTableSchema): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.type !== "") {
      obj.type = message.type;
    }
    if (message.fields?.length) {
      obj.fields = message.fields.map((e) => BigQueryTableField.toJSON(e));
    }
    if (message.description !== "") {
      obj.description = message.description;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryTableSchema>, I>>(base?: I): BigQueryTableSchema {
    return BigQueryTableSchema.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryTableSchema>, I>>(object: I): BigQueryTableSchema {
    const message = createBaseBigQueryTableSchema();
    message.name = object.name ?? "";
    message.type = object.type ?? "";
    message.fields = object.fields?.map((e) => BigQueryTableField.fromPartial(e)) || [];
    message.description = object.description ?? "";
    return message;
  },
};

function createBaseBigQueryJobReference(): BigQueryJobReference {
  return { projectId: "", jobId: "", location: "" };
}

export const BigQueryJobReference = {
  encode(message: BigQueryJobReference, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.projectId !== "") {
      writer.uint32(10).string(message.projectId);
    }
    if (message.jobId !== "") {
      writer.uint32(18).string(message.jobId);
    }
    if (message.location !== "") {
      writer.uint32(26).string(message.location);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryJobReference {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryJobReference();
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

          message.jobId = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.location = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryJobReference {
    return {
      projectId: isSet(object.projectId) ? gt.String(object.projectId) : "",
      jobId: isSet(object.jobId) ? gt.String(object.jobId) : "",
      location: isSet(object.location) ? gt.String(object.location) : "",
    };
  },

  toJSON(message: BigQueryJobReference): unknown {
    const obj: any = {};
    if (message.projectId !== "") {
      obj.projectId = message.projectId;
    }
    if (message.jobId !== "") {
      obj.jobId = message.jobId;
    }
    if (message.location !== "") {
      obj.location = message.location;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryJobReference>, I>>(base?: I): BigQueryJobReference {
    return BigQueryJobReference.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryJobReference>, I>>(object: I): BigQueryJobReference {
    const message = createBaseBigQueryJobReference();
    message.projectId = object.projectId ?? "";
    message.jobId = object.jobId ?? "";
    message.location = object.location ?? "";
    return message;
  },
};

function createBaseBigQueryError(): BigQueryError {
  return { reason: "", location: "", debugInfo: "", message: "" };
}

export const BigQueryError = {
  encode(message: BigQueryError, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.reason !== "") {
      writer.uint32(10).string(message.reason);
    }
    if (message.location !== "") {
      writer.uint32(18).string(message.location);
    }
    if (message.debugInfo !== "") {
      writer.uint32(26).string(message.debugInfo);
    }
    if (message.message !== "") {
      writer.uint32(34).string(message.message);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryError {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryError();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.reason = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.location = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.debugInfo = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.message = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryError {
    return {
      reason: isSet(object.reason) ? gt.String(object.reason) : "",
      location: isSet(object.location) ? gt.String(object.location) : "",
      debugInfo: isSet(object.debugInfo) ? gt.String(object.debugInfo) : "",
      message: isSet(object.message) ? gt.String(object.message) : "",
    };
  },

  toJSON(message: BigQueryError): unknown {
    const obj: any = {};
    if (message.reason !== "") {
      obj.reason = message.reason;
    }
    if (message.location !== "") {
      obj.location = message.location;
    }
    if (message.debugInfo !== "") {
      obj.debugInfo = message.debugInfo;
    }
    if (message.message !== "") {
      obj.message = message.message;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryError>, I>>(base?: I): BigQueryError {
    return BigQueryError.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryError>, I>>(object: I): BigQueryError {
    const message = createBaseBigQueryError();
    message.reason = object.reason ?? "";
    message.location = object.location ?? "";
    message.debugInfo = object.debugInfo ?? "";
    message.message = object.message ?? "";
    return message;
  },
};

function createBaseBigQueryJobStatus(): BigQueryJobStatus {
  return { state: "", errorResult: "", errorMessage: "" };
}

export const BigQueryJobStatus = {
  encode(message: BigQueryJobStatus, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.state !== "") {
      writer.uint32(10).string(message.state);
    }
    if (message.errorResult !== "") {
      writer.uint32(18).string(message.errorResult);
    }
    if (message.errorMessage !== "") {
      writer.uint32(26).string(message.errorMessage);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryJobStatus {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryJobStatus();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.state = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.errorResult = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.errorMessage = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryJobStatus {
    return {
      state: isSet(object.state) ? gt.String(object.state) : "",
      errorResult: isSet(object.errorResult) ? gt.String(object.errorResult) : "",
      errorMessage: isSet(object.errorMessage) ? gt.String(object.errorMessage) : "",
    };
  },

  toJSON(message: BigQueryJobStatus): unknown {
    const obj: any = {};
    if (message.state !== "") {
      obj.state = message.state;
    }
    if (message.errorResult !== "") {
      obj.errorResult = message.errorResult;
    }
    if (message.errorMessage !== "") {
      obj.errorMessage = message.errorMessage;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryJobStatus>, I>>(base?: I): BigQueryJobStatus {
    return BigQueryJobStatus.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryJobStatus>, I>>(object: I): BigQueryJobStatus {
    const message = createBaseBigQueryJobStatus();
    message.state = object.state ?? "";
    message.errorResult = object.errorResult ?? "";
    message.errorMessage = object.errorMessage ?? "";
    return message;
  },
};

function createBaseBigQueryJob(): BigQueryJob {
  return { kind: "", id: "", selfLink: "", userEmail: "", jobReference: undefined, status: undefined };
}

export const BigQueryJob = {
  encode(message: BigQueryJob, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.kind !== "") {
      writer.uint32(10).string(message.kind);
    }
    if (message.id !== "") {
      writer.uint32(18).string(message.id);
    }
    if (message.selfLink !== "") {
      writer.uint32(26).string(message.selfLink);
    }
    if (message.userEmail !== "") {
      writer.uint32(34).string(message.userEmail);
    }
    if (message.jobReference !== undefined) {
      BigQueryJobReference.encode(message.jobReference, writer.uint32(42).fork()).ldelim();
    }
    if (message.status !== undefined) {
      BigQueryJobStatus.encode(message.status, writer.uint32(50).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryJob {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryJob();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.kind = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.id = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.selfLink = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.userEmail = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.jobReference = BigQueryJobReference.decode(reader, reader.uint32());
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.status = BigQueryJobStatus.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryJob {
    return {
      kind: isSet(object.kind) ? gt.String(object.kind) : "",
      id: isSet(object.id) ? gt.String(object.id) : "",
      selfLink: isSet(object.selfLink) ? gt.String(object.selfLink) : "",
      userEmail: isSet(object.userEmail) ? gt.String(object.userEmail) : "",
      jobReference: isSet(object.jobReference) ? BigQueryJobReference.fromJSON(object.jobReference) : undefined,
      status: isSet(object.status) ? BigQueryJobStatus.fromJSON(object.status) : undefined,
    };
  },

  toJSON(message: BigQueryJob): unknown {
    const obj: any = {};
    if (message.kind !== "") {
      obj.kind = message.kind;
    }
    if (message.id !== "") {
      obj.id = message.id;
    }
    if (message.selfLink !== "") {
      obj.selfLink = message.selfLink;
    }
    if (message.userEmail !== "") {
      obj.userEmail = message.userEmail;
    }
    if (message.jobReference !== undefined) {
      obj.jobReference = BigQueryJobReference.toJSON(message.jobReference);
    }
    if (message.status !== undefined) {
      obj.status = BigQueryJobStatus.toJSON(message.status);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryJob>, I>>(base?: I): BigQueryJob {
    return BigQueryJob.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryJob>, I>>(object: I): BigQueryJob {
    const message = createBaseBigQueryJob();
    message.kind = object.kind ?? "";
    message.id = object.id ?? "";
    message.selfLink = object.selfLink ?? "";
    message.userEmail = object.userEmail ?? "";
    message.jobReference = (object.jobReference !== undefined && object.jobReference !== null)
      ? BigQueryJobReference.fromPartial(object.jobReference)
      : undefined;
    message.status = (object.status !== undefined && object.status !== null)
      ? BigQueryJobStatus.fromPartial(object.status)
      : undefined;
    return message;
  },
};

function createBaseBigQueryFieldValue(): BigQueryFieldValue {
  return { f: "", v: "" };
}

export const BigQueryFieldValue = {
  encode(message: BigQueryFieldValue, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.f !== "") {
      writer.uint32(10).string(message.f);
    }
    if (message.v !== "") {
      writer.uint32(18).string(message.v);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryFieldValue {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryFieldValue();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.f = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.v = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryFieldValue {
    return { f: isSet(object.f) ? gt.String(object.f) : "", v: isSet(object.v) ? gt.String(object.v) : "" };
  },

  toJSON(message: BigQueryFieldValue): unknown {
    const obj: any = {};
    if (message.f !== "") {
      obj.f = message.f;
    }
    if (message.v !== "") {
      obj.v = message.v;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryFieldValue>, I>>(base?: I): BigQueryFieldValue {
    return BigQueryFieldValue.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryFieldValue>, I>>(object: I): BigQueryFieldValue {
    const message = createBaseBigQueryFieldValue();
    message.f = object.f ?? "";
    message.v = object.v ?? "";
    return message;
  },
};

function createBaseBigQueryTableRow(): BigQueryTableRow {
  return { f: [] };
}

export const BigQueryTableRow = {
  encode(message: BigQueryTableRow, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.f) {
      BigQueryFieldValue.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryTableRow {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryTableRow();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.f.push(BigQueryFieldValue.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryTableRow {
    return { f: gt.Array.isArray(object?.f) ? object.f.map((e: any) => BigQueryFieldValue.fromJSON(e)) : [] };
  },

  toJSON(message: BigQueryTableRow): unknown {
    const obj: any = {};
    if (message.f?.length) {
      obj.f = message.f.map((e) => BigQueryFieldValue.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryTableRow>, I>>(base?: I): BigQueryTableRow {
    return BigQueryTableRow.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryTableRow>, I>>(object: I): BigQueryTableRow {
    const message = createBaseBigQueryTableRow();
    message.f = object.f?.map((e) => BigQueryFieldValue.fromPartial(e)) || [];
    return message;
  },
};

function createBaseBigQueryJobResults(): BigQueryJobResults {
  return {
    kind: "",
    schema: undefined,
    jobReference: undefined,
    totalRows: "",
    pageToken: "",
    rows: [],
    jobComplete: "",
    errors: [],
    cacheHit: "",
    numDmlAffectedRows: "",
  };
}

export const BigQueryJobResults = {
  encode(message: BigQueryJobResults, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.kind !== "") {
      writer.uint32(10).string(message.kind);
    }
    if (message.schema !== undefined) {
      BigQueryTableSchema.encode(message.schema, writer.uint32(18).fork()).ldelim();
    }
    if (message.jobReference !== undefined) {
      BigQueryJobReference.encode(message.jobReference, writer.uint32(26).fork()).ldelim();
    }
    if (message.totalRows !== "") {
      writer.uint32(34).string(message.totalRows);
    }
    if (message.pageToken !== "") {
      writer.uint32(42).string(message.pageToken);
    }
    for (const v of message.rows) {
      BigQueryTableRow.encode(v!, writer.uint32(50).fork()).ldelim();
    }
    if (message.jobComplete !== "") {
      writer.uint32(58).string(message.jobComplete);
    }
    for (const v of message.errors) {
      BigQueryError.encode(v!, writer.uint32(66).fork()).ldelim();
    }
    if (message.cacheHit !== "") {
      writer.uint32(74).string(message.cacheHit);
    }
    if (message.numDmlAffectedRows !== "") {
      writer.uint32(82).string(message.numDmlAffectedRows);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryJobResults {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryJobResults();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.kind = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.schema = BigQueryTableSchema.decode(reader, reader.uint32());
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.jobReference = BigQueryJobReference.decode(reader, reader.uint32());
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.totalRows = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.pageToken = reader.string();
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.rows.push(BigQueryTableRow.decode(reader, reader.uint32()));
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          message.jobComplete = reader.string();
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          message.errors.push(BigQueryError.decode(reader, reader.uint32()));
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          message.cacheHit = reader.string();
          continue;
        case 10:
          if (tag !== 82) {
            break;
          }

          message.numDmlAffectedRows = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryJobResults {
    return {
      kind: isSet(object.kind) ? gt.String(object.kind) : "",
      schema: isSet(object.schema) ? BigQueryTableSchema.fromJSON(object.schema) : undefined,
      jobReference: isSet(object.jobReference) ? BigQueryJobReference.fromJSON(object.jobReference) : undefined,
      totalRows: isSet(object.totalRows) ? gt.String(object.totalRows) : "",
      pageToken: isSet(object.pageToken) ? gt.String(object.pageToken) : "",
      rows: gt.Array.isArray(object?.rows) ? object.rows.map((e: any) => BigQueryTableRow.fromJSON(e)) : [],
      jobComplete: isSet(object.jobComplete) ? gt.String(object.jobComplete) : "",
      errors: gt.Array.isArray(object?.errors) ? object.errors.map((e: any) => BigQueryError.fromJSON(e)) : [],
      cacheHit: isSet(object.cacheHit) ? gt.String(object.cacheHit) : "",
      numDmlAffectedRows: isSet(object.numDmlAffectedRows) ? gt.String(object.numDmlAffectedRows) : "",
    };
  },

  toJSON(message: BigQueryJobResults): unknown {
    const obj: any = {};
    if (message.kind !== "") {
      obj.kind = message.kind;
    }
    if (message.schema !== undefined) {
      obj.schema = BigQueryTableSchema.toJSON(message.schema);
    }
    if (message.jobReference !== undefined) {
      obj.jobReference = BigQueryJobReference.toJSON(message.jobReference);
    }
    if (message.totalRows !== "") {
      obj.totalRows = message.totalRows;
    }
    if (message.pageToken !== "") {
      obj.pageToken = message.pageToken;
    }
    if (message.rows?.length) {
      obj.rows = message.rows.map((e) => BigQueryTableRow.toJSON(e));
    }
    if (message.jobComplete !== "") {
      obj.jobComplete = message.jobComplete;
    }
    if (message.errors?.length) {
      obj.errors = message.errors.map((e) => BigQueryError.toJSON(e));
    }
    if (message.cacheHit !== "") {
      obj.cacheHit = message.cacheHit;
    }
    if (message.numDmlAffectedRows !== "") {
      obj.numDmlAffectedRows = message.numDmlAffectedRows;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryJobResults>, I>>(base?: I): BigQueryJobResults {
    return BigQueryJobResults.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryJobResults>, I>>(object: I): BigQueryJobResults {
    const message = createBaseBigQueryJobResults();
    message.kind = object.kind ?? "";
    message.schema = (object.schema !== undefined && object.schema !== null)
      ? BigQueryTableSchema.fromPartial(object.schema)
      : undefined;
    message.jobReference = (object.jobReference !== undefined && object.jobReference !== null)
      ? BigQueryJobReference.fromPartial(object.jobReference)
      : undefined;
    message.totalRows = object.totalRows ?? "";
    message.pageToken = object.pageToken ?? "";
    message.rows = object.rows?.map((e) => BigQueryTableRow.fromPartial(e)) || [];
    message.jobComplete = object.jobComplete ?? "";
    message.errors = object.errors?.map((e) => BigQueryError.fromPartial(e)) || [];
    message.cacheHit = object.cacheHit ?? "";
    message.numDmlAffectedRows = object.numDmlAffectedRows ?? "";
    return message;
  },
};

function createBaseBigQueryOauthToken(): BigQueryOauthToken {
  return { accessToken: "", refreshToken: "", expiryTime: "" };
}

export const BigQueryOauthToken = {
  encode(message: BigQueryOauthToken, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.accessToken !== "") {
      writer.uint32(10).string(message.accessToken);
    }
    if (message.refreshToken !== "") {
      writer.uint32(18).string(message.refreshToken);
    }
    if (message.expiryTime !== "") {
      writer.uint32(26).string(message.expiryTime);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryOauthToken {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryOauthToken();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.accessToken = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.refreshToken = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.expiryTime = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryOauthToken {
    return {
      accessToken: isSet(object.accessToken) ? gt.String(object.accessToken) : "",
      refreshToken: isSet(object.refreshToken) ? gt.String(object.refreshToken) : "",
      expiryTime: isSet(object.expiryTime) ? gt.String(object.expiryTime) : "",
    };
  },

  toJSON(message: BigQueryOauthToken): unknown {
    const obj: any = {};
    if (message.accessToken !== "") {
      obj.accessToken = message.accessToken;
    }
    if (message.refreshToken !== "") {
      obj.refreshToken = message.refreshToken;
    }
    if (message.expiryTime !== "") {
      obj.expiryTime = message.expiryTime;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryOauthToken>, I>>(base?: I): BigQueryOauthToken {
    return BigQueryOauthToken.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryOauthToken>, I>>(object: I): BigQueryOauthToken {
    const message = createBaseBigQueryOauthToken();
    message.accessToken = object.accessToken ?? "";
    message.refreshToken = object.refreshToken ?? "";
    message.expiryTime = object.expiryTime ?? "";
    return message;
  },
};

function createBaseBigQueryOauthTokenRefresh(): BigQueryOauthTokenRefresh {
  return { accessToken: "", expiryTime: "" };
}

export const BigQueryOauthTokenRefresh = {
  encode(message: BigQueryOauthTokenRefresh, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.accessToken !== "") {
      writer.uint32(10).string(message.accessToken);
    }
    if (message.expiryTime !== "") {
      writer.uint32(18).string(message.expiryTime);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BigQueryOauthTokenRefresh {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBigQueryOauthTokenRefresh();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.accessToken = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.expiryTime = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): BigQueryOauthTokenRefresh {
    return {
      accessToken: isSet(object.accessToken) ? gt.String(object.accessToken) : "",
      expiryTime: isSet(object.expiryTime) ? gt.String(object.expiryTime) : "",
    };
  },

  toJSON(message: BigQueryOauthTokenRefresh): unknown {
    const obj: any = {};
    if (message.accessToken !== "") {
      obj.accessToken = message.accessToken;
    }
    if (message.expiryTime !== "") {
      obj.expiryTime = message.expiryTime;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<BigQueryOauthTokenRefresh>, I>>(base?: I): BigQueryOauthTokenRefresh {
    return BigQueryOauthTokenRefresh.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<BigQueryOauthTokenRefresh>, I>>(object: I): BigQueryOauthTokenRefresh {
    const message = createBaseBigQueryOauthTokenRefresh();
    message.accessToken = object.accessToken ?? "";
    message.expiryTime = object.expiryTime ?? "";
    return message;
  },
};

function createBaseSnowflakeOauthProxyRequest(): SnowflakeOauthProxyRequest {
  return { accountUrl: "", clientId: "", clientSecret: "", role: "" };
}

export const SnowflakeOauthProxyRequest = {
  encode(message: SnowflakeOauthProxyRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
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
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SnowflakeOauthProxyRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSnowflakeOauthProxyRequest();
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
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SnowflakeOauthProxyRequest {
    return {
      accountUrl: isSet(object.accountUrl) ? gt.String(object.accountUrl) : "",
      clientId: isSet(object.clientId) ? gt.String(object.clientId) : "",
      clientSecret: isSet(object.clientSecret) ? gt.String(object.clientSecret) : "",
      role: isSet(object.role) ? gt.String(object.role) : "",
    };
  },

  toJSON(message: SnowflakeOauthProxyRequest): unknown {
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
    return obj;
  },

  create<I extends Exact<DeepPartial<SnowflakeOauthProxyRequest>, I>>(base?: I): SnowflakeOauthProxyRequest {
    return SnowflakeOauthProxyRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SnowflakeOauthProxyRequest>, I>>(object: I): SnowflakeOauthProxyRequest {
    const message = createBaseSnowflakeOauthProxyRequest();
    message.accountUrl = object.accountUrl ?? "";
    message.clientId = object.clientId ?? "";
    message.clientSecret = object.clientSecret ?? "";
    message.role = object.role ?? "";
    return message;
  },
};

function createBaseSnowflakeOauthToken(): SnowflakeOauthToken {
  return { accessToken: "", refreshToken: "", expiryTime: "" };
}

export const SnowflakeOauthToken = {
  encode(message: SnowflakeOauthToken, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.accessToken !== "") {
      writer.uint32(10).string(message.accessToken);
    }
    if (message.refreshToken !== "") {
      writer.uint32(18).string(message.refreshToken);
    }
    if (message.expiryTime !== "") {
      writer.uint32(26).string(message.expiryTime);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SnowflakeOauthToken {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSnowflakeOauthToken();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.accessToken = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.refreshToken = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.expiryTime = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SnowflakeOauthToken {
    return {
      accessToken: isSet(object.accessToken) ? gt.String(object.accessToken) : "",
      refreshToken: isSet(object.refreshToken) ? gt.String(object.refreshToken) : "",
      expiryTime: isSet(object.expiryTime) ? gt.String(object.expiryTime) : "",
    };
  },

  toJSON(message: SnowflakeOauthToken): unknown {
    const obj: any = {};
    if (message.accessToken !== "") {
      obj.accessToken = message.accessToken;
    }
    if (message.refreshToken !== "") {
      obj.refreshToken = message.refreshToken;
    }
    if (message.expiryTime !== "") {
      obj.expiryTime = message.expiryTime;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SnowflakeOauthToken>, I>>(base?: I): SnowflakeOauthToken {
    return SnowflakeOauthToken.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SnowflakeOauthToken>, I>>(object: I): SnowflakeOauthToken {
    const message = createBaseSnowflakeOauthToken();
    message.accessToken = object.accessToken ?? "";
    message.refreshToken = object.refreshToken ?? "";
    message.expiryTime = object.expiryTime ?? "";
    return message;
  },
};

function createBaseSnowflakeOauthRefreshToken(): SnowflakeOauthRefreshToken {
  return { accessToken: "", expiryTime: "" };
}

export const SnowflakeOauthRefreshToken = {
  encode(message: SnowflakeOauthRefreshToken, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.accessToken !== "") {
      writer.uint32(10).string(message.accessToken);
    }
    if (message.expiryTime !== "") {
      writer.uint32(26).string(message.expiryTime);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SnowflakeOauthRefreshToken {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSnowflakeOauthRefreshToken();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.accessToken = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.expiryTime = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SnowflakeOauthRefreshToken {
    return {
      accessToken: isSet(object.accessToken) ? gt.String(object.accessToken) : "",
      expiryTime: isSet(object.expiryTime) ? gt.String(object.expiryTime) : "",
    };
  },

  toJSON(message: SnowflakeOauthRefreshToken): unknown {
    const obj: any = {};
    if (message.accessToken !== "") {
      obj.accessToken = message.accessToken;
    }
    if (message.expiryTime !== "") {
      obj.expiryTime = message.expiryTime;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SnowflakeOauthRefreshToken>, I>>(base?: I): SnowflakeOauthRefreshToken {
    return SnowflakeOauthRefreshToken.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SnowflakeOauthRefreshToken>, I>>(object: I): SnowflakeOauthRefreshToken {
    const message = createBaseSnowflakeOauthRefreshToken();
    message.accessToken = object.accessToken ?? "";
    message.expiryTime = object.expiryTime ?? "";
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
