import * as jspb from 'google-protobuf'



export class Endpoint extends jspb.Message {
  getName(): string;
  setName(value: string): Endpoint;

  getAliasesList(): Array<string>;
  setAliasesList(value: Array<string>): Endpoint;
  clearAliasesList(): Endpoint;
  addAliases(value: string, index?: number): Endpoint;

  getTarget(): string;
  setTarget(value: string): Endpoint;

  getAllowCors(): boolean;
  setAllowCors(value: boolean): Endpoint;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Endpoint.AsObject;
  static toObject(includeInstance: boolean, msg: Endpoint): Endpoint.AsObject;
  static serializeBinaryToWriter(message: Endpoint, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Endpoint;
  static deserializeBinaryFromReader(message: Endpoint, reader: jspb.BinaryReader): Endpoint;
}

export namespace Endpoint {
  export type AsObject = {
    name: string,
    aliasesList: Array<string>,
    target: string,
    allowCors: boolean,
  }
}

