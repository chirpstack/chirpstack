import * as jspb from 'google-protobuf'

import * as google_api_policy_pb from '../../google/api/policy_pb';


export class Control extends jspb.Message {
  getEnvironment(): string;
  setEnvironment(value: string): Control;

  getMethodPoliciesList(): Array<google_api_policy_pb.MethodPolicy>;
  setMethodPoliciesList(value: Array<google_api_policy_pb.MethodPolicy>): Control;
  clearMethodPoliciesList(): Control;
  addMethodPolicies(value?: google_api_policy_pb.MethodPolicy, index?: number): google_api_policy_pb.MethodPolicy;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Control.AsObject;
  static toObject(includeInstance: boolean, msg: Control): Control.AsObject;
  static serializeBinaryToWriter(message: Control, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Control;
  static deserializeBinaryFromReader(message: Control, reader: jspb.BinaryReader): Control;
}

export namespace Control {
  export type AsObject = {
    environment: string,
    methodPoliciesList: Array<google_api_policy_pb.MethodPolicy.AsObject>,
  }
}

