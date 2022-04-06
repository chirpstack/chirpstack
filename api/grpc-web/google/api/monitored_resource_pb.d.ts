import * as jspb from 'google-protobuf'

import * as google_api_label_pb from '../../google/api/label_pb';
import * as google_api_launch_stage_pb from '../../google/api/launch_stage_pb';
import * as google_protobuf_struct_pb from 'google-protobuf/google/protobuf/struct_pb';


export class MonitoredResourceDescriptor extends jspb.Message {
  getName(): string;
  setName(value: string): MonitoredResourceDescriptor;

  getType(): string;
  setType(value: string): MonitoredResourceDescriptor;

  getDisplayName(): string;
  setDisplayName(value: string): MonitoredResourceDescriptor;

  getDescription(): string;
  setDescription(value: string): MonitoredResourceDescriptor;

  getLabelsList(): Array<google_api_label_pb.LabelDescriptor>;
  setLabelsList(value: Array<google_api_label_pb.LabelDescriptor>): MonitoredResourceDescriptor;
  clearLabelsList(): MonitoredResourceDescriptor;
  addLabels(value?: google_api_label_pb.LabelDescriptor, index?: number): google_api_label_pb.LabelDescriptor;

  getLaunchStage(): google_api_launch_stage_pb.LaunchStage;
  setLaunchStage(value: google_api_launch_stage_pb.LaunchStage): MonitoredResourceDescriptor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MonitoredResourceDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: MonitoredResourceDescriptor): MonitoredResourceDescriptor.AsObject;
  static serializeBinaryToWriter(message: MonitoredResourceDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MonitoredResourceDescriptor;
  static deserializeBinaryFromReader(message: MonitoredResourceDescriptor, reader: jspb.BinaryReader): MonitoredResourceDescriptor;
}

export namespace MonitoredResourceDescriptor {
  export type AsObject = {
    name: string,
    type: string,
    displayName: string,
    description: string,
    labelsList: Array<google_api_label_pb.LabelDescriptor.AsObject>,
    launchStage: google_api_launch_stage_pb.LaunchStage,
  }
}

export class MonitoredResource extends jspb.Message {
  getType(): string;
  setType(value: string): MonitoredResource;

  getLabelsMap(): jspb.Map<string, string>;
  clearLabelsMap(): MonitoredResource;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MonitoredResource.AsObject;
  static toObject(includeInstance: boolean, msg: MonitoredResource): MonitoredResource.AsObject;
  static serializeBinaryToWriter(message: MonitoredResource, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MonitoredResource;
  static deserializeBinaryFromReader(message: MonitoredResource, reader: jspb.BinaryReader): MonitoredResource;
}

export namespace MonitoredResource {
  export type AsObject = {
    type: string,
    labelsMap: Array<[string, string]>,
  }
}

export class MonitoredResourceMetadata extends jspb.Message {
  getSystemLabels(): google_protobuf_struct_pb.Struct | undefined;
  setSystemLabels(value?: google_protobuf_struct_pb.Struct): MonitoredResourceMetadata;
  hasSystemLabels(): boolean;
  clearSystemLabels(): MonitoredResourceMetadata;

  getUserLabelsMap(): jspb.Map<string, string>;
  clearUserLabelsMap(): MonitoredResourceMetadata;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MonitoredResourceMetadata.AsObject;
  static toObject(includeInstance: boolean, msg: MonitoredResourceMetadata): MonitoredResourceMetadata.AsObject;
  static serializeBinaryToWriter(message: MonitoredResourceMetadata, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MonitoredResourceMetadata;
  static deserializeBinaryFromReader(message: MonitoredResourceMetadata, reader: jspb.BinaryReader): MonitoredResourceMetadata;
}

export namespace MonitoredResourceMetadata {
  export type AsObject = {
    systemLabels?: google_protobuf_struct_pb.Struct.AsObject,
    userLabelsMap: Array<[string, string]>,
  }
}

