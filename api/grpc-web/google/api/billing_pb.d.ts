import * as jspb from 'google-protobuf'



export class Billing extends jspb.Message {
  getConsumerDestinationsList(): Array<Billing.BillingDestination>;
  setConsumerDestinationsList(value: Array<Billing.BillingDestination>): Billing;
  clearConsumerDestinationsList(): Billing;
  addConsumerDestinations(value?: Billing.BillingDestination, index?: number): Billing.BillingDestination;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Billing.AsObject;
  static toObject(includeInstance: boolean, msg: Billing): Billing.AsObject;
  static serializeBinaryToWriter(message: Billing, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Billing;
  static deserializeBinaryFromReader(message: Billing, reader: jspb.BinaryReader): Billing;
}

export namespace Billing {
  export type AsObject = {
    consumerDestinationsList: Array<Billing.BillingDestination.AsObject>,
  }

  export class BillingDestination extends jspb.Message {
    getMonitoredResource(): string;
    setMonitoredResource(value: string): BillingDestination;

    getMetricsList(): Array<string>;
    setMetricsList(value: Array<string>): BillingDestination;
    clearMetricsList(): BillingDestination;
    addMetrics(value: string, index?: number): BillingDestination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BillingDestination.AsObject;
    static toObject(includeInstance: boolean, msg: BillingDestination): BillingDestination.AsObject;
    static serializeBinaryToWriter(message: BillingDestination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BillingDestination;
    static deserializeBinaryFromReader(message: BillingDestination, reader: jspb.BinaryReader): BillingDestination;
  }

  export namespace BillingDestination {
    export type AsObject = {
      monitoredResource: string,
      metricsList: Array<string>,
    }
  }

}

