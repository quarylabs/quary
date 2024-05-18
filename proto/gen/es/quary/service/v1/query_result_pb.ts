// @generated by protoc-gen-es v1.9.0 with parameter "target=ts"
// @generated from file quary/service/v1/query_result.proto (package quary.service.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3 } from "@bufbuild/protobuf";

/**
 * QueryResult is the result of a ran query.
 *
 * @generated from message quary.service.v1.QueryResult
 */
export class QueryResult extends Message<QueryResult> {
  /**
   * @generated from field: repeated quary.service.v1.QueryResultColumn columns = 1;
   */
  columns: QueryResultColumn[] = [];

  constructor(data?: PartialMessage<QueryResult>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "quary.service.v1.QueryResult";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "columns", kind: "message", T: QueryResultColumn, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): QueryResult {
    return new QueryResult().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): QueryResult {
    return new QueryResult().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): QueryResult {
    return new QueryResult().fromJsonString(jsonString, options);
  }

  static equals(a: QueryResult | PlainMessage<QueryResult> | undefined, b: QueryResult | PlainMessage<QueryResult> | undefined): boolean {
    return proto3.util.equals(QueryResult, a, b);
  }
}

/**
 * @generated from message quary.service.v1.QueryResultColumn
 */
export class QueryResultColumn extends Message<QueryResultColumn> {
  /**
   * @generated from field: string name = 1;
   */
  name = "";

  /**
   * @generated from field: optional string type = 3;
   */
  type?: string;

  /**
   * @generated from field: repeated string values = 2;
   */
  values: string[] = [];

  constructor(data?: PartialMessage<QueryResultColumn>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "quary.service.v1.QueryResultColumn";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "type", kind: "scalar", T: 9 /* ScalarType.STRING */, opt: true },
    { no: 2, name: "values", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): QueryResultColumn {
    return new QueryResultColumn().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): QueryResultColumn {
    return new QueryResultColumn().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): QueryResultColumn {
    return new QueryResultColumn().fromJsonString(jsonString, options);
  }

  static equals(a: QueryResultColumn | PlainMessage<QueryResultColumn> | undefined, b: QueryResultColumn | PlainMessage<QueryResultColumn> | undefined): boolean {
    return proto3.util.equals(QueryResultColumn, a, b);
  }
}

