// package: google.api
// file: google/api/metric.proto

import * as jspb from "google-protobuf";
import * as google_api_label_pb from "../../google/api/label_pb";
import * as google_api_launch_stage_pb from "../../google/api/launch_stage_pb";
import * as google_protobuf_duration_pb from "google-protobuf/google/protobuf/duration_pb";

export class MetricDescriptor extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getType(): string;
  setType(value: string): void;

  clearLabelsList(): void;
  getLabelsList(): Array<google_api_label_pb.LabelDescriptor>;
  setLabelsList(value: Array<google_api_label_pb.LabelDescriptor>): void;
  addLabels(value?: google_api_label_pb.LabelDescriptor, index?: number): google_api_label_pb.LabelDescriptor;

  getMetricKind(): MetricDescriptor.MetricKindMap[keyof MetricDescriptor.MetricKindMap];
  setMetricKind(value: MetricDescriptor.MetricKindMap[keyof MetricDescriptor.MetricKindMap]): void;

  getValueType(): MetricDescriptor.ValueTypeMap[keyof MetricDescriptor.ValueTypeMap];
  setValueType(value: MetricDescriptor.ValueTypeMap[keyof MetricDescriptor.ValueTypeMap]): void;

  getUnit(): string;
  setUnit(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  getDisplayName(): string;
  setDisplayName(value: string): void;

  hasMetadata(): boolean;
  clearMetadata(): void;
  getMetadata(): MetricDescriptor.MetricDescriptorMetadata | undefined;
  setMetadata(value?: MetricDescriptor.MetricDescriptorMetadata): void;

  getLaunchStage(): google_api_launch_stage_pb.LaunchStageMap[keyof google_api_launch_stage_pb.LaunchStageMap];
  setLaunchStage(value: google_api_launch_stage_pb.LaunchStageMap[keyof google_api_launch_stage_pb.LaunchStageMap]): void;

  clearMonitoredResourceTypesList(): void;
  getMonitoredResourceTypesList(): Array<string>;
  setMonitoredResourceTypesList(value: Array<string>): void;
  addMonitoredResourceTypes(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MetricDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: MetricDescriptor): MetricDescriptor.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: MetricDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MetricDescriptor;
  static deserializeBinaryFromReader(message: MetricDescriptor, reader: jspb.BinaryReader): MetricDescriptor;
}

export namespace MetricDescriptor {
  export type AsObject = {
    name: string,
    type: string,
    labelsList: Array<google_api_label_pb.LabelDescriptor.AsObject>,
    metricKind: MetricDescriptor.MetricKindMap[keyof MetricDescriptor.MetricKindMap],
    valueType: MetricDescriptor.ValueTypeMap[keyof MetricDescriptor.ValueTypeMap],
    unit: string,
    description: string,
    displayName: string,
    metadata?: MetricDescriptor.MetricDescriptorMetadata.AsObject,
    launchStage: google_api_launch_stage_pb.LaunchStageMap[keyof google_api_launch_stage_pb.LaunchStageMap],
    monitoredResourceTypesList: Array<string>,
  }

  export class MetricDescriptorMetadata extends jspb.Message {
    getLaunchStage(): google_api_launch_stage_pb.LaunchStageMap[keyof google_api_launch_stage_pb.LaunchStageMap];
    setLaunchStage(value: google_api_launch_stage_pb.LaunchStageMap[keyof google_api_launch_stage_pb.LaunchStageMap]): void;

    hasSamplePeriod(): boolean;
    clearSamplePeriod(): void;
    getSamplePeriod(): google_protobuf_duration_pb.Duration | undefined;
    setSamplePeriod(value?: google_protobuf_duration_pb.Duration): void;

    hasIngestDelay(): boolean;
    clearIngestDelay(): void;
    getIngestDelay(): google_protobuf_duration_pb.Duration | undefined;
    setIngestDelay(value?: google_protobuf_duration_pb.Duration): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): MetricDescriptorMetadata.AsObject;
    static toObject(includeInstance: boolean, msg: MetricDescriptorMetadata): MetricDescriptorMetadata.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: MetricDescriptorMetadata, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): MetricDescriptorMetadata;
    static deserializeBinaryFromReader(message: MetricDescriptorMetadata, reader: jspb.BinaryReader): MetricDescriptorMetadata;
  }

  export namespace MetricDescriptorMetadata {
    export type AsObject = {
      launchStage: google_api_launch_stage_pb.LaunchStageMap[keyof google_api_launch_stage_pb.LaunchStageMap],
      samplePeriod?: google_protobuf_duration_pb.Duration.AsObject,
      ingestDelay?: google_protobuf_duration_pb.Duration.AsObject,
    }
  }

  export interface MetricKindMap {
    METRIC_KIND_UNSPECIFIED: 0;
    GAUGE: 1;
    DELTA: 2;
    CUMULATIVE: 3;
  }

  export const MetricKind: MetricKindMap;

  export interface ValueTypeMap {
    VALUE_TYPE_UNSPECIFIED: 0;
    BOOL: 1;
    INT64: 2;
    DOUBLE: 3;
    STRING: 4;
    DISTRIBUTION: 5;
    MONEY: 6;
  }

  export const ValueType: ValueTypeMap;
}

export class Metric extends jspb.Message {
  getType(): string;
  setType(value: string): void;

  getLabelsMap(): jspb.Map<string, string>;
  clearLabelsMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Metric.AsObject;
  static toObject(includeInstance: boolean, msg: Metric): Metric.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Metric, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Metric;
  static deserializeBinaryFromReader(message: Metric, reader: jspb.BinaryReader): Metric;
}

export namespace Metric {
  export type AsObject = {
    type: string,
    labelsMap: Array<[string, string]>,
  }
}

