import * as jspb from 'google-protobuf'



export class ProjectProperties extends jspb.Message {
  getPropertiesList(): Array<Property>;
  setPropertiesList(value: Array<Property>): ProjectProperties;
  clearPropertiesList(): ProjectProperties;
  addProperties(value?: Property, index?: number): Property;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ProjectProperties.AsObject;
  static toObject(includeInstance: boolean, msg: ProjectProperties): ProjectProperties.AsObject;
  static serializeBinaryToWriter(message: ProjectProperties, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ProjectProperties;
  static deserializeBinaryFromReader(message: ProjectProperties, reader: jspb.BinaryReader): ProjectProperties;
}

export namespace ProjectProperties {
  export type AsObject = {
    propertiesList: Array<Property.AsObject>,
  }
}

export class Property extends jspb.Message {
  getName(): string;
  setName(value: string): Property;

  getType(): Property.PropertyType;
  setType(value: Property.PropertyType): Property;

  getDescription(): string;
  setDescription(value: string): Property;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Property.AsObject;
  static toObject(includeInstance: boolean, msg: Property): Property.AsObject;
  static serializeBinaryToWriter(message: Property, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Property;
  static deserializeBinaryFromReader(message: Property, reader: jspb.BinaryReader): Property;
}

export namespace Property {
  export type AsObject = {
    name: string,
    type: Property.PropertyType,
    description: string,
  }

  export enum PropertyType { 
    UNSPECIFIED = 0,
    INT64 = 1,
    BOOL = 2,
    STRING = 3,
    DOUBLE = 4,
  }
}

