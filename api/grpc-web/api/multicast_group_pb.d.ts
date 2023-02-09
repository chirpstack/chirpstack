import * as jspb from 'google-protobuf'

import * as google_api_annotations_pb from '../google/api/annotations_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as common_common_pb from '../common/common_pb';


export class MulticastGroup extends jspb.Message {
  getId(): string;
  setId(value: string): MulticastGroup;

  getName(): string;
  setName(value: string): MulticastGroup;

  getApplicationId(): string;
  setApplicationId(value: string): MulticastGroup;

  getRegion(): common_common_pb.Region;
  setRegion(value: common_common_pb.Region): MulticastGroup;

  getMcAddr(): string;
  setMcAddr(value: string): MulticastGroup;

  getMcNwkSKey(): string;
  setMcNwkSKey(value: string): MulticastGroup;

  getMcAppSKey(): string;
  setMcAppSKey(value: string): MulticastGroup;

  getFCnt(): number;
  setFCnt(value: number): MulticastGroup;

  getGroupType(): MulticastGroupType;
  setGroupType(value: MulticastGroupType): MulticastGroup;

  getDr(): number;
  setDr(value: number): MulticastGroup;

  getFrequency(): number;
  setFrequency(value: number): MulticastGroup;

  getClassBPingSlotPeriod(): number;
  setClassBPingSlotPeriod(value: number): MulticastGroup;

  getClassCSchedulingType(): MulticastGroupSchedulingType;
  setClassCSchedulingType(value: MulticastGroupSchedulingType): MulticastGroup;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MulticastGroup.AsObject;
  static toObject(includeInstance: boolean, msg: MulticastGroup): MulticastGroup.AsObject;
  static serializeBinaryToWriter(message: MulticastGroup, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MulticastGroup;
  static deserializeBinaryFromReader(message: MulticastGroup, reader: jspb.BinaryReader): MulticastGroup;
}

export namespace MulticastGroup {
  export type AsObject = {
    id: string,
    name: string,
    applicationId: string,
    region: common_common_pb.Region,
    mcAddr: string,
    mcNwkSKey: string,
    mcAppSKey: string,
    fCnt: number,
    groupType: MulticastGroupType,
    dr: number,
    frequency: number,
    classBPingSlotPeriod: number,
    classCSchedulingType: MulticastGroupSchedulingType,
  }
}

export class MulticastGroupListItem extends jspb.Message {
  getId(): string;
  setId(value: string): MulticastGroupListItem;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): MulticastGroupListItem;
  hasCreatedAt(): boolean;
  clearCreatedAt(): MulticastGroupListItem;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): MulticastGroupListItem;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): MulticastGroupListItem;

  getName(): string;
  setName(value: string): MulticastGroupListItem;

  getRegion(): common_common_pb.Region;
  setRegion(value: common_common_pb.Region): MulticastGroupListItem;

  getGroupType(): MulticastGroupType;
  setGroupType(value: MulticastGroupType): MulticastGroupListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MulticastGroupListItem.AsObject;
  static toObject(includeInstance: boolean, msg: MulticastGroupListItem): MulticastGroupListItem.AsObject;
  static serializeBinaryToWriter(message: MulticastGroupListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MulticastGroupListItem;
  static deserializeBinaryFromReader(message: MulticastGroupListItem, reader: jspb.BinaryReader): MulticastGroupListItem;
}

export namespace MulticastGroupListItem {
  export type AsObject = {
    id: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    name: string,
    region: common_common_pb.Region,
    groupType: MulticastGroupType,
  }
}

export class CreateMulticastGroupRequest extends jspb.Message {
  getMulticastGroup(): MulticastGroup | undefined;
  setMulticastGroup(value?: MulticastGroup): CreateMulticastGroupRequest;
  hasMulticastGroup(): boolean;
  clearMulticastGroup(): CreateMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateMulticastGroupRequest): CreateMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: CreateMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateMulticastGroupRequest;
  static deserializeBinaryFromReader(message: CreateMulticastGroupRequest, reader: jspb.BinaryReader): CreateMulticastGroupRequest;
}

export namespace CreateMulticastGroupRequest {
  export type AsObject = {
    multicastGroup?: MulticastGroup.AsObject,
  }
}

export class CreateMulticastGroupResponse extends jspb.Message {
  getId(): string;
  setId(value: string): CreateMulticastGroupResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateMulticastGroupResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateMulticastGroupResponse): CreateMulticastGroupResponse.AsObject;
  static serializeBinaryToWriter(message: CreateMulticastGroupResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateMulticastGroupResponse;
  static deserializeBinaryFromReader(message: CreateMulticastGroupResponse, reader: jspb.BinaryReader): CreateMulticastGroupResponse;
}

export namespace CreateMulticastGroupResponse {
  export type AsObject = {
    id: string,
  }
}

export class GetMulticastGroupRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetMulticastGroupRequest): GetMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: GetMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMulticastGroupRequest;
  static deserializeBinaryFromReader(message: GetMulticastGroupRequest, reader: jspb.BinaryReader): GetMulticastGroupRequest;
}

export namespace GetMulticastGroupRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetMulticastGroupResponse extends jspb.Message {
  getMulticastGroup(): MulticastGroup | undefined;
  setMulticastGroup(value?: MulticastGroup): GetMulticastGroupResponse;
  hasMulticastGroup(): boolean;
  clearMulticastGroup(): GetMulticastGroupResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetMulticastGroupResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): GetMulticastGroupResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetMulticastGroupResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): GetMulticastGroupResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMulticastGroupResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetMulticastGroupResponse): GetMulticastGroupResponse.AsObject;
  static serializeBinaryToWriter(message: GetMulticastGroupResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMulticastGroupResponse;
  static deserializeBinaryFromReader(message: GetMulticastGroupResponse, reader: jspb.BinaryReader): GetMulticastGroupResponse;
}

export namespace GetMulticastGroupResponse {
  export type AsObject = {
    multicastGroup?: MulticastGroup.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class UpdateMulticastGroupRequest extends jspb.Message {
  getMulticastGroup(): MulticastGroup | undefined;
  setMulticastGroup(value?: MulticastGroup): UpdateMulticastGroupRequest;
  hasMulticastGroup(): boolean;
  clearMulticastGroup(): UpdateMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateMulticastGroupRequest): UpdateMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateMulticastGroupRequest;
  static deserializeBinaryFromReader(message: UpdateMulticastGroupRequest, reader: jspb.BinaryReader): UpdateMulticastGroupRequest;
}

export namespace UpdateMulticastGroupRequest {
  export type AsObject = {
    multicastGroup?: MulticastGroup.AsObject,
  }
}

export class DeleteMulticastGroupRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteMulticastGroupRequest): DeleteMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteMulticastGroupRequest;
  static deserializeBinaryFromReader(message: DeleteMulticastGroupRequest, reader: jspb.BinaryReader): DeleteMulticastGroupRequest;
}

export namespace DeleteMulticastGroupRequest {
  export type AsObject = {
    id: string,
  }
}

export class ListMulticastGroupsRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): ListMulticastGroupsRequest;

  getOffset(): number;
  setOffset(value: number): ListMulticastGroupsRequest;

  getSearch(): string;
  setSearch(value: string): ListMulticastGroupsRequest;

  getApplicationId(): string;
  setApplicationId(value: string): ListMulticastGroupsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListMulticastGroupsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListMulticastGroupsRequest): ListMulticastGroupsRequest.AsObject;
  static serializeBinaryToWriter(message: ListMulticastGroupsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListMulticastGroupsRequest;
  static deserializeBinaryFromReader(message: ListMulticastGroupsRequest, reader: jspb.BinaryReader): ListMulticastGroupsRequest;
}

export namespace ListMulticastGroupsRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    search: string,
    applicationId: string,
  }
}

export class ListMulticastGroupsResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): ListMulticastGroupsResponse;

  getResultList(): Array<MulticastGroupListItem>;
  setResultList(value: Array<MulticastGroupListItem>): ListMulticastGroupsResponse;
  clearResultList(): ListMulticastGroupsResponse;
  addResult(value?: MulticastGroupListItem, index?: number): MulticastGroupListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListMulticastGroupsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListMulticastGroupsResponse): ListMulticastGroupsResponse.AsObject;
  static serializeBinaryToWriter(message: ListMulticastGroupsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListMulticastGroupsResponse;
  static deserializeBinaryFromReader(message: ListMulticastGroupsResponse, reader: jspb.BinaryReader): ListMulticastGroupsResponse;
}

export namespace ListMulticastGroupsResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<MulticastGroupListItem.AsObject>,
  }
}

export class AddDeviceToMulticastGroupRequest extends jspb.Message {
  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): AddDeviceToMulticastGroupRequest;

  getDevEui(): string;
  setDevEui(value: string): AddDeviceToMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddDeviceToMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddDeviceToMulticastGroupRequest): AddDeviceToMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: AddDeviceToMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddDeviceToMulticastGroupRequest;
  static deserializeBinaryFromReader(message: AddDeviceToMulticastGroupRequest, reader: jspb.BinaryReader): AddDeviceToMulticastGroupRequest;
}

export namespace AddDeviceToMulticastGroupRequest {
  export type AsObject = {
    multicastGroupId: string,
    devEui: string,
  }
}

export class RemoveDeviceFromMulticastGroupRequest extends jspb.Message {
  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): RemoveDeviceFromMulticastGroupRequest;

  getDevEui(): string;
  setDevEui(value: string): RemoveDeviceFromMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RemoveDeviceFromMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RemoveDeviceFromMulticastGroupRequest): RemoveDeviceFromMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: RemoveDeviceFromMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RemoveDeviceFromMulticastGroupRequest;
  static deserializeBinaryFromReader(message: RemoveDeviceFromMulticastGroupRequest, reader: jspb.BinaryReader): RemoveDeviceFromMulticastGroupRequest;
}

export namespace RemoveDeviceFromMulticastGroupRequest {
  export type AsObject = {
    multicastGroupId: string,
    devEui: string,
  }
}

export class AddGatewayToMulticastGroupRequest extends jspb.Message {
  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): AddGatewayToMulticastGroupRequest;

  getGatewayId(): string;
  setGatewayId(value: string): AddGatewayToMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddGatewayToMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddGatewayToMulticastGroupRequest): AddGatewayToMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: AddGatewayToMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddGatewayToMulticastGroupRequest;
  static deserializeBinaryFromReader(message: AddGatewayToMulticastGroupRequest, reader: jspb.BinaryReader): AddGatewayToMulticastGroupRequest;
}

export namespace AddGatewayToMulticastGroupRequest {
  export type AsObject = {
    multicastGroupId: string,
    gatewayId: string,
  }
}

export class RemoveGatewayFromMulticastGroupRequest extends jspb.Message {
  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): RemoveGatewayFromMulticastGroupRequest;

  getGatewayId(): string;
  setGatewayId(value: string): RemoveGatewayFromMulticastGroupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RemoveGatewayFromMulticastGroupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RemoveGatewayFromMulticastGroupRequest): RemoveGatewayFromMulticastGroupRequest.AsObject;
  static serializeBinaryToWriter(message: RemoveGatewayFromMulticastGroupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RemoveGatewayFromMulticastGroupRequest;
  static deserializeBinaryFromReader(message: RemoveGatewayFromMulticastGroupRequest, reader: jspb.BinaryReader): RemoveGatewayFromMulticastGroupRequest;
}

export namespace RemoveGatewayFromMulticastGroupRequest {
  export type AsObject = {
    multicastGroupId: string,
    gatewayId: string,
  }
}

export class MulticastGroupQueueItem extends jspb.Message {
  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): MulticastGroupQueueItem;

  getFCnt(): number;
  setFCnt(value: number): MulticastGroupQueueItem;

  getFPort(): number;
  setFPort(value: number): MulticastGroupQueueItem;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): MulticastGroupQueueItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MulticastGroupQueueItem.AsObject;
  static toObject(includeInstance: boolean, msg: MulticastGroupQueueItem): MulticastGroupQueueItem.AsObject;
  static serializeBinaryToWriter(message: MulticastGroupQueueItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MulticastGroupQueueItem;
  static deserializeBinaryFromReader(message: MulticastGroupQueueItem, reader: jspb.BinaryReader): MulticastGroupQueueItem;
}

export namespace MulticastGroupQueueItem {
  export type AsObject = {
    multicastGroupId: string,
    fCnt: number,
    fPort: number,
    data: Uint8Array | string,
  }
}

export class EnqueueMulticastGroupQueueItemRequest extends jspb.Message {
  getQueueItem(): MulticastGroupQueueItem | undefined;
  setQueueItem(value?: MulticastGroupQueueItem): EnqueueMulticastGroupQueueItemRequest;
  hasQueueItem(): boolean;
  clearQueueItem(): EnqueueMulticastGroupQueueItemRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EnqueueMulticastGroupQueueItemRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EnqueueMulticastGroupQueueItemRequest): EnqueueMulticastGroupQueueItemRequest.AsObject;
  static serializeBinaryToWriter(message: EnqueueMulticastGroupQueueItemRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EnqueueMulticastGroupQueueItemRequest;
  static deserializeBinaryFromReader(message: EnqueueMulticastGroupQueueItemRequest, reader: jspb.BinaryReader): EnqueueMulticastGroupQueueItemRequest;
}

export namespace EnqueueMulticastGroupQueueItemRequest {
  export type AsObject = {
    queueItem?: MulticastGroupQueueItem.AsObject,
  }
}

export class EnqueueMulticastGroupQueueItemResponse extends jspb.Message {
  getFCnt(): number;
  setFCnt(value: number): EnqueueMulticastGroupQueueItemResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EnqueueMulticastGroupQueueItemResponse.AsObject;
  static toObject(includeInstance: boolean, msg: EnqueueMulticastGroupQueueItemResponse): EnqueueMulticastGroupQueueItemResponse.AsObject;
  static serializeBinaryToWriter(message: EnqueueMulticastGroupQueueItemResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EnqueueMulticastGroupQueueItemResponse;
  static deserializeBinaryFromReader(message: EnqueueMulticastGroupQueueItemResponse, reader: jspb.BinaryReader): EnqueueMulticastGroupQueueItemResponse;
}

export namespace EnqueueMulticastGroupQueueItemResponse {
  export type AsObject = {
    fCnt: number,
  }
}

export class FlushMulticastGroupQueueRequest extends jspb.Message {
  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): FlushMulticastGroupQueueRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FlushMulticastGroupQueueRequest.AsObject;
  static toObject(includeInstance: boolean, msg: FlushMulticastGroupQueueRequest): FlushMulticastGroupQueueRequest.AsObject;
  static serializeBinaryToWriter(message: FlushMulticastGroupQueueRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FlushMulticastGroupQueueRequest;
  static deserializeBinaryFromReader(message: FlushMulticastGroupQueueRequest, reader: jspb.BinaryReader): FlushMulticastGroupQueueRequest;
}

export namespace FlushMulticastGroupQueueRequest {
  export type AsObject = {
    multicastGroupId: string,
  }
}

export class ListMulticastGroupQueueRequest extends jspb.Message {
  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): ListMulticastGroupQueueRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListMulticastGroupQueueRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListMulticastGroupQueueRequest): ListMulticastGroupQueueRequest.AsObject;
  static serializeBinaryToWriter(message: ListMulticastGroupQueueRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListMulticastGroupQueueRequest;
  static deserializeBinaryFromReader(message: ListMulticastGroupQueueRequest, reader: jspb.BinaryReader): ListMulticastGroupQueueRequest;
}

export namespace ListMulticastGroupQueueRequest {
  export type AsObject = {
    multicastGroupId: string,
  }
}

export class ListMulticastGroupQueueResponse extends jspb.Message {
  getItemsList(): Array<MulticastGroupQueueItem>;
  setItemsList(value: Array<MulticastGroupQueueItem>): ListMulticastGroupQueueResponse;
  clearItemsList(): ListMulticastGroupQueueResponse;
  addItems(value?: MulticastGroupQueueItem, index?: number): MulticastGroupQueueItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListMulticastGroupQueueResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListMulticastGroupQueueResponse): ListMulticastGroupQueueResponse.AsObject;
  static serializeBinaryToWriter(message: ListMulticastGroupQueueResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListMulticastGroupQueueResponse;
  static deserializeBinaryFromReader(message: ListMulticastGroupQueueResponse, reader: jspb.BinaryReader): ListMulticastGroupQueueResponse;
}

export namespace ListMulticastGroupQueueResponse {
  export type AsObject = {
    itemsList: Array<MulticastGroupQueueItem.AsObject>,
  }
}

export enum MulticastGroupType { 
  CLASS_C = 0,
  CLASS_B = 1,
}
export enum MulticastGroupSchedulingType { 
  DELAY = 0,
  GPS_TIME = 1,
}
