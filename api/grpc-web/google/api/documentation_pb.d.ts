import * as jspb from 'google-protobuf'



export class Documentation extends jspb.Message {
  getSummary(): string;
  setSummary(value: string): Documentation;

  getPagesList(): Array<Page>;
  setPagesList(value: Array<Page>): Documentation;
  clearPagesList(): Documentation;
  addPages(value?: Page, index?: number): Page;

  getRulesList(): Array<DocumentationRule>;
  setRulesList(value: Array<DocumentationRule>): Documentation;
  clearRulesList(): Documentation;
  addRules(value?: DocumentationRule, index?: number): DocumentationRule;

  getDocumentationRootUrl(): string;
  setDocumentationRootUrl(value: string): Documentation;

  getServiceRootUrl(): string;
  setServiceRootUrl(value: string): Documentation;

  getOverview(): string;
  setOverview(value: string): Documentation;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Documentation.AsObject;
  static toObject(includeInstance: boolean, msg: Documentation): Documentation.AsObject;
  static serializeBinaryToWriter(message: Documentation, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Documentation;
  static deserializeBinaryFromReader(message: Documentation, reader: jspb.BinaryReader): Documentation;
}

export namespace Documentation {
  export type AsObject = {
    summary: string,
    pagesList: Array<Page.AsObject>,
    rulesList: Array<DocumentationRule.AsObject>,
    documentationRootUrl: string,
    serviceRootUrl: string,
    overview: string,
  }
}

export class DocumentationRule extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): DocumentationRule;

  getDescription(): string;
  setDescription(value: string): DocumentationRule;

  getDeprecationDescription(): string;
  setDeprecationDescription(value: string): DocumentationRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DocumentationRule.AsObject;
  static toObject(includeInstance: boolean, msg: DocumentationRule): DocumentationRule.AsObject;
  static serializeBinaryToWriter(message: DocumentationRule, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DocumentationRule;
  static deserializeBinaryFromReader(message: DocumentationRule, reader: jspb.BinaryReader): DocumentationRule;
}

export namespace DocumentationRule {
  export type AsObject = {
    selector: string,
    description: string,
    deprecationDescription: string,
  }
}

export class Page extends jspb.Message {
  getName(): string;
  setName(value: string): Page;

  getContent(): string;
  setContent(value: string): Page;

  getSubpagesList(): Array<Page>;
  setSubpagesList(value: Array<Page>): Page;
  clearSubpagesList(): Page;
  addSubpages(value?: Page, index?: number): Page;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Page.AsObject;
  static toObject(includeInstance: boolean, msg: Page): Page.AsObject;
  static serializeBinaryToWriter(message: Page, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Page;
  static deserializeBinaryFromReader(message: Page, reader: jspb.BinaryReader): Page;
}

export namespace Page {
  export type AsObject = {
    name: string,
    content: string,
    subpagesList: Array<Page.AsObject>,
  }
}

