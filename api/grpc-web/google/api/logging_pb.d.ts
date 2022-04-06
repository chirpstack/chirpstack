import * as jspb from 'google-protobuf'



export class Logging extends jspb.Message {
  getProducerDestinationsList(): Array<Logging.LoggingDestination>;
  setProducerDestinationsList(value: Array<Logging.LoggingDestination>): Logging;
  clearProducerDestinationsList(): Logging;
  addProducerDestinations(value?: Logging.LoggingDestination, index?: number): Logging.LoggingDestination;

  getConsumerDestinationsList(): Array<Logging.LoggingDestination>;
  setConsumerDestinationsList(value: Array<Logging.LoggingDestination>): Logging;
  clearConsumerDestinationsList(): Logging;
  addConsumerDestinations(value?: Logging.LoggingDestination, index?: number): Logging.LoggingDestination;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Logging.AsObject;
  static toObject(includeInstance: boolean, msg: Logging): Logging.AsObject;
  static serializeBinaryToWriter(message: Logging, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Logging;
  static deserializeBinaryFromReader(message: Logging, reader: jspb.BinaryReader): Logging;
}

export namespace Logging {
  export type AsObject = {
    producerDestinationsList: Array<Logging.LoggingDestination.AsObject>,
    consumerDestinationsList: Array<Logging.LoggingDestination.AsObject>,
  }

  export class LoggingDestination extends jspb.Message {
    getMonitoredResource(): string;
    setMonitoredResource(value: string): LoggingDestination;

    getLogsList(): Array<string>;
    setLogsList(value: Array<string>): LoggingDestination;
    clearLogsList(): LoggingDestination;
    addLogs(value: string, index?: number): LoggingDestination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): LoggingDestination.AsObject;
    static toObject(includeInstance: boolean, msg: LoggingDestination): LoggingDestination.AsObject;
    static serializeBinaryToWriter(message: LoggingDestination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): LoggingDestination;
    static deserializeBinaryFromReader(message: LoggingDestination, reader: jspb.BinaryReader): LoggingDestination;
  }

  export namespace LoggingDestination {
    export type AsObject = {
      monitoredResource: string,
      logsList: Array<string>,
    }
  }

}

