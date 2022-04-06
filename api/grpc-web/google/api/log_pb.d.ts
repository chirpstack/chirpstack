import * as jspb from 'google-protobuf'

import * as google_api_label_pb from '../../google/api/label_pb';


export class LogDescriptor extends jspb.Message {
  getName(): string;
  setName(value: string): LogDescriptor;

  getLabelsList(): Array<google_api_label_pb.LabelDescriptor>;
  setLabelsList(value: Array<google_api_label_pb.LabelDescriptor>): LogDescriptor;
  clearLabelsList(): LogDescriptor;
  addLabels(value?: google_api_label_pb.LabelDescriptor, index?: number): google_api_label_pb.LabelDescriptor;

  getDescription(): string;
  setDescription(value: string): LogDescriptor;

  getDisplayName(): string;
  setDisplayName(value: string): LogDescriptor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LogDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: LogDescriptor): LogDescriptor.AsObject;
  static serializeBinaryToWriter(message: LogDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LogDescriptor;
  static deserializeBinaryFromReader(message: LogDescriptor, reader: jspb.BinaryReader): LogDescriptor;
}

export namespace LogDescriptor {
  export type AsObject = {
    name: string,
    labelsList: Array<google_api_label_pb.LabelDescriptor.AsObject>,
    description: string,
    displayName: string,
  }
}

