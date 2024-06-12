// @generated by protoc-gen-es v1.10.0 with parameter "target=ts"
// @generated from file quary/service/v1/project.proto (package quary.service.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3 } from "@bufbuild/protobuf";
import { Model, Seed, Snapshot, Source, Test } from "./types_pb.js";
import { Chart } from "./chart_pb.js";
import { DashboardFile } from "./dashboard_file_pb.js";
import { ProjectFile } from "./project_file_pb.js";
import { ConnectionConfig } from "./connection_config_pb.js";

/**
 * @generated from message quary.service.v1.Project
 */
export class Project extends Message<Project> {
  /**
   * @generated from field: map<string, quary.service.v1.Seed> seeds = 3;
   */
  seeds: { [key: string]: Seed } = {};

  /**
   * @generated from field: map<string, quary.service.v1.Model> models = 4;
   */
  models: { [key: string]: Model } = {};

  /**
   * @generated from field: map<string, quary.service.v1.Test> tests = 5;
   */
  tests: { [key: string]: Test } = {};

  /**
   * @generated from field: map<string, quary.service.v1.Source> sources = 6;
   */
  sources: { [key: string]: Source } = {};

  /**
   * @generated from field: map<string, quary.service.v1.Snapshot> snapshots = 9;
   */
  snapshots: { [key: string]: Snapshot } = {};

  /**
   * @generated from field: map<string, quary.service.v1.Chart> charts = 10;
   */
  charts: { [key: string]: Chart } = {};

  /**
   * @generated from field: map<string, quary.service.v1.DashboardFile> dashboards = 11;
   */
  dashboards: { [key: string]: DashboardFile } = {};

  /**
   * @generated from field: map<string, quary.service.v1.ProjectFile> project_files = 7;
   */
  projectFiles: { [key: string]: ProjectFile } = {};

  /**
   * @generated from field: quary.service.v1.ConnectionConfig connection_config = 8;
   */
  connectionConfig?: ConnectionConfig;

  constructor(data?: PartialMessage<Project>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "quary.service.v1.Project";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 3, name: "seeds", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: Seed} },
    { no: 4, name: "models", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: Model} },
    { no: 5, name: "tests", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: Test} },
    { no: 6, name: "sources", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: Source} },
    { no: 9, name: "snapshots", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: Snapshot} },
    { no: 10, name: "charts", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: Chart} },
    { no: 11, name: "dashboards", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: DashboardFile} },
    { no: 7, name: "project_files", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: ProjectFile} },
    { no: 8, name: "connection_config", kind: "message", T: ConnectionConfig },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Project {
    return new Project().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Project {
    return new Project().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Project {
    return new Project().fromJsonString(jsonString, options);
  }

  static equals(a: Project | PlainMessage<Project> | undefined, b: Project | PlainMessage<Project> | undefined): boolean {
    return proto3.util.equals(Project, a, b);
  }
}

