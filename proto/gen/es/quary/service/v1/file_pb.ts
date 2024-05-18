// @generated by protoc-gen-es v1.9.0 with parameter "target=ts"
// @generated from file quary/service/v1/file.proto (package quary.service.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3 } from "@bufbuild/protobuf";

/**
 * @generated from message quary.service.v1.File
 */
export class File extends Message<File> {
  /**
   * The name of the file. Names are relative to the root of the file system.
   *
   * @generated from field: string name = 1;
   */
  name = "";

  /**
   * @generated from field: bytes contents = 2;
   */
  contents = new Uint8Array(0);

  constructor(data?: PartialMessage<File>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "quary.service.v1.File";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "contents", kind: "scalar", T: 12 /* ScalarType.BYTES */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): File {
    return new File().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): File {
    return new File().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): File {
    return new File().fromJsonString(jsonString, options);
  }

  static equals(a: File | PlainMessage<File> | undefined, b: File | PlainMessage<File> | undefined): boolean {
    return proto3.util.equals(File, a, b);
  }
}

/**
 * @generated from message quary.service.v1.FileSystem
 */
export class FileSystem extends Message<FileSystem> {
  /**
   * @generated from field: map<string, quary.service.v1.File> files = 1;
   */
  files: { [key: string]: File } = {};

  constructor(data?: PartialMessage<FileSystem>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "quary.service.v1.FileSystem";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "files", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "message", T: File} },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): FileSystem {
    return new FileSystem().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): FileSystem {
    return new FileSystem().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): FileSystem {
    return new FileSystem().fromJsonString(jsonString, options);
  }

  static equals(a: FileSystem | PlainMessage<FileSystem> | undefined, b: FileSystem | PlainMessage<FileSystem> | undefined): boolean {
    return proto3.util.equals(FileSystem, a, b);
  }
}

