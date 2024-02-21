/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "quary.service.v1";

export interface File {
  /** The name of the file. Names are relative to the root of the file system. */
  name: string;
  contents: Uint8Array;
}

export interface FileSystem {
  files: { [key: string]: File };
}

export interface FileSystem_FilesEntry {
  key: string;
  value: File | undefined;
}

function createBaseFile(): File {
  return { name: "", contents: new Uint8Array(0) };
}

export const File = {
  encode(message: File, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.contents.length !== 0) {
      writer.uint32(18).bytes(message.contents);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): File {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFile();
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

          message.contents = reader.bytes();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): File {
    return {
      name: isSet(object.name) ? gt.String(object.name) : "",
      contents: isSet(object.contents) ? bytesFromBase64(object.contents) : new Uint8Array(0),
    };
  },

  toJSON(message: File): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.contents.length !== 0) {
      obj.contents = base64FromBytes(message.contents);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<File>, I>>(base?: I): File {
    return File.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<File>, I>>(object: I): File {
    const message = createBaseFile();
    message.name = object.name ?? "";
    message.contents = object.contents ?? new Uint8Array(0);
    return message;
  },
};

function createBaseFileSystem(): FileSystem {
  return { files: {} };
}

export const FileSystem = {
  encode(message: FileSystem, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    Object.entries(message.files).forEach(([key, value]) => {
      FileSystem_FilesEntry.encode({ key: key as any, value }, writer.uint32(10).fork()).ldelim();
    });
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FileSystem {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFileSystem();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          const entry1 = FileSystem_FilesEntry.decode(reader, reader.uint32());
          if (entry1.value !== undefined) {
            message.files[entry1.key] = entry1.value;
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

  fromJSON(object: any): FileSystem {
    return {
      files: isObject(object.files)
        ? Object.entries(object.files).reduce<{ [key: string]: File }>((acc, [key, value]) => {
          acc[key] = File.fromJSON(value);
          return acc;
        }, {})
        : {},
    };
  },

  toJSON(message: FileSystem): unknown {
    const obj: any = {};
    if (message.files) {
      const entries = Object.entries(message.files);
      if (entries.length > 0) {
        obj.files = {};
        entries.forEach(([k, v]) => {
          obj.files[k] = File.toJSON(v);
        });
      }
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<FileSystem>, I>>(base?: I): FileSystem {
    return FileSystem.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<FileSystem>, I>>(object: I): FileSystem {
    const message = createBaseFileSystem();
    message.files = Object.entries(object.files ?? {}).reduce<{ [key: string]: File }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = File.fromPartial(value);
      }
      return acc;
    }, {});
    return message;
  },
};

function createBaseFileSystem_FilesEntry(): FileSystem_FilesEntry {
  return { key: "", value: undefined };
}

export const FileSystem_FilesEntry = {
  encode(message: FileSystem_FilesEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      File.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FileSystem_FilesEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFileSystem_FilesEntry();
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

          message.value = File.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): FileSystem_FilesEntry {
    return {
      key: isSet(object.key) ? gt.String(object.key) : "",
      value: isSet(object.value) ? File.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: FileSystem_FilesEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = File.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<FileSystem_FilesEntry>, I>>(base?: I): FileSystem_FilesEntry {
    return FileSystem_FilesEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<FileSystem_FilesEntry>, I>>(object: I): FileSystem_FilesEntry {
    const message = createBaseFileSystem_FilesEntry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null) ? File.fromPartial(object.value) : undefined;
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

function bytesFromBase64(b64: string): Uint8Array {
  if (gt.Buffer) {
    return Uint8Array.from(gt.Buffer.from(b64, "base64"));
  } else {
    const bin = gt.atob(b64);
    const arr = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; ++i) {
      arr[i] = bin.charCodeAt(i);
    }
    return arr;
  }
}

function base64FromBytes(arr: Uint8Array): string {
  if (gt.Buffer) {
    return gt.Buffer.from(arr).toString("base64");
  } else {
    const bin: string[] = [];
    arr.forEach((byte) => {
      bin.push(gt.String.fromCharCode(byte));
    });
    return gt.btoa(bin.join(""));
  }
}

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
