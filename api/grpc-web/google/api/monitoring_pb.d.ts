import * as jspb from 'google-protobuf'



export class Monitoring extends jspb.Message {
  getProducerDestinationsList(): Array<Monitoring.MonitoringDestination>;
  setProducerDestinationsList(value: Array<Monitoring.MonitoringDestination>): Monitoring;
  clearProducerDestinationsList(): Monitoring;
  addProducerDestinations(value?: Monitoring.MonitoringDestination, index?: number): Monitoring.MonitoringDestination;

  getConsumerDestinationsList(): Array<Monitoring.MonitoringDestination>;
  setConsumerDestinationsList(value: Array<Monitoring.MonitoringDestination>): Monitoring;
  clearConsumerDestinationsList(): Monitoring;
  addConsumerDestinations(value?: Monitoring.MonitoringDestination, index?: number): Monitoring.MonitoringDestination;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Monitoring.AsObject;
  static toObject(includeInstance: boolean, msg: Monitoring): Monitoring.AsObject;
  static serializeBinaryToWriter(message: Monitoring, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Monitoring;
  static deserializeBinaryFromReader(message: Monitoring, reader: jspb.BinaryReader): Monitoring;
}

export namespace Monitoring {
  export type AsObject = {
    producerDestinationsList: Array<Monitoring.MonitoringDestination.AsObject>,
    consumerDestinationsList: Array<Monitoring.MonitoringDestination.AsObject>,
  }

  export class MonitoringDestination extends jspb.Message {
    getMonitoredResource(): string;
    setMonitoredResource(value: string): MonitoringDestination;

    getMetricsList(): Array<string>;
    setMetricsList(value: Array<string>): MonitoringDestination;
    clearMetricsList(): MonitoringDestination;
    addMetrics(value: string, index?: number): MonitoringDestination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): MonitoringDestination.AsObject;
    static toObject(includeInstance: boolean, msg: MonitoringDestination): MonitoringDestination.AsObject;
    static serializeBinaryToWriter(message: MonitoringDestination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): MonitoringDestination;
    static deserializeBinaryFromReader(message: MonitoringDestination, reader: jspb.BinaryReader): MonitoringDestination;
  }

  export namespace MonitoringDestination {
    export type AsObject = {
      monitoredResource: string,
      metricsList: Array<string>,
    }
  }

}

