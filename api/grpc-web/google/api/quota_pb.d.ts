import * as jspb from 'google-protobuf'



export class Quota extends jspb.Message {
  getLimitsList(): Array<QuotaLimit>;
  setLimitsList(value: Array<QuotaLimit>): Quota;
  clearLimitsList(): Quota;
  addLimits(value?: QuotaLimit, index?: number): QuotaLimit;

  getMetricRulesList(): Array<MetricRule>;
  setMetricRulesList(value: Array<MetricRule>): Quota;
  clearMetricRulesList(): Quota;
  addMetricRules(value?: MetricRule, index?: number): MetricRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Quota.AsObject;
  static toObject(includeInstance: boolean, msg: Quota): Quota.AsObject;
  static serializeBinaryToWriter(message: Quota, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Quota;
  static deserializeBinaryFromReader(message: Quota, reader: jspb.BinaryReader): Quota;
}

export namespace Quota {
  export type AsObject = {
    limitsList: Array<QuotaLimit.AsObject>,
    metricRulesList: Array<MetricRule.AsObject>,
  }
}

export class MetricRule extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): MetricRule;

  getMetricCostsMap(): jspb.Map<string, number>;
  clearMetricCostsMap(): MetricRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MetricRule.AsObject;
  static toObject(includeInstance: boolean, msg: MetricRule): MetricRule.AsObject;
  static serializeBinaryToWriter(message: MetricRule, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MetricRule;
  static deserializeBinaryFromReader(message: MetricRule, reader: jspb.BinaryReader): MetricRule;
}

export namespace MetricRule {
  export type AsObject = {
    selector: string,
    metricCostsMap: Array<[string, number]>,
  }
}

export class QuotaLimit extends jspb.Message {
  getName(): string;
  setName(value: string): QuotaLimit;

  getDescription(): string;
  setDescription(value: string): QuotaLimit;

  getDefaultLimit(): number;
  setDefaultLimit(value: number): QuotaLimit;

  getMaxLimit(): number;
  setMaxLimit(value: number): QuotaLimit;

  getFreeTier(): number;
  setFreeTier(value: number): QuotaLimit;

  getDuration(): string;
  setDuration(value: string): QuotaLimit;

  getMetric(): string;
  setMetric(value: string): QuotaLimit;

  getUnit(): string;
  setUnit(value: string): QuotaLimit;

  getValuesMap(): jspb.Map<string, number>;
  clearValuesMap(): QuotaLimit;

  getDisplayName(): string;
  setDisplayName(value: string): QuotaLimit;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): QuotaLimit.AsObject;
  static toObject(includeInstance: boolean, msg: QuotaLimit): QuotaLimit.AsObject;
  static serializeBinaryToWriter(message: QuotaLimit, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): QuotaLimit;
  static deserializeBinaryFromReader(message: QuotaLimit, reader: jspb.BinaryReader): QuotaLimit;
}

export namespace QuotaLimit {
  export type AsObject = {
    name: string,
    description: string,
    defaultLimit: number,
    maxLimit: number,
    freeTier: number,
    duration: string,
    metric: string,
    unit: string,
    valuesMap: Array<[string, number]>,
    displayName: string,
  }
}

