/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Position } from "./position";

export const protobufPackage = "quary.service.v1";

/** Range represents a range of positions in a file. */
export interface Range {
  start: Position | undefined;
  end: Position | undefined;
}

function createBaseRange(): Range {
  return { start: undefined, end: undefined };
}

export const Range = {
  encode(message: Range, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.start !== undefined) {
      Position.encode(message.start, writer.uint32(10).fork()).ldelim();
    }
    if (message.end !== undefined) {
      Position.encode(message.end, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Range {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRange();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.start = Position.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.end = Position.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Range {
    return {
      start: isSet(object.start) ? Position.fromJSON(object.start) : undefined,
      end: isSet(object.end) ? Position.fromJSON(object.end) : undefined,
    };
  },

  toJSON(message: Range): unknown {
    const obj: any = {};
    if (message.start !== undefined) {
      obj.start = Position.toJSON(message.start);
    }
    if (message.end !== undefined) {
      obj.end = Position.toJSON(message.end);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Range>, I>>(base?: I): Range {
    return Range.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Range>, I>>(object: I): Range {
    const message = createBaseRange();
    message.start = (object.start !== undefined && object.start !== null)
      ? Position.fromPartial(object.start)
      : undefined;
    message.end = (object.end !== undefined && object.end !== null) ? Position.fromPartial(object.end) : undefined;
    return message;
  },
};

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
