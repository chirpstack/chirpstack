// source: api/application.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

var google_api_annotations_pb = require('../google/api/annotations_pb.js');
goog.object.extend(proto, google_api_annotations_pb);
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
goog.object.extend(proto, google_protobuf_timestamp_pb);
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
goog.object.extend(proto, google_protobuf_empty_pb);
goog.exportSymbol('proto.api.Application', null, global);
goog.exportSymbol('proto.api.ApplicationListItem', null, global);
goog.exportSymbol('proto.api.AwsSnsIntegration', null, global);
goog.exportSymbol('proto.api.AzureServiceBusIntegration', null, global);
goog.exportSymbol('proto.api.CreateApplicationRequest', null, global);
goog.exportSymbol('proto.api.CreateApplicationResponse', null, global);
goog.exportSymbol('proto.api.CreateAwsSnsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateAzureServiceBusIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateGcpPubSubIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateHttpIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateIftttIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateInfluxDbIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateLoraCloudIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateMyDevicesIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreatePilotThingsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.CreateThingsBoardIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteApplicationRequest', null, global);
goog.exportSymbol('proto.api.DeleteAwsSnsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteAzureServiceBusIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteGcpPubSubIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteHttpIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteIftttIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteInfluxDbIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteLoraCloudIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteMyDevicesIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeletePilotThingsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.DeleteThingsBoardIntegrationRequest', null, global);
goog.exportSymbol('proto.api.Encoding', null, global);
goog.exportSymbol('proto.api.GcpPubSubIntegration', null, global);
goog.exportSymbol('proto.api.GenerateMqttIntegrationClientCertificateRequest', null, global);
goog.exportSymbol('proto.api.GenerateMqttIntegrationClientCertificateResponse', null, global);
goog.exportSymbol('proto.api.GetApplicationRequest', null, global);
goog.exportSymbol('proto.api.GetApplicationResponse', null, global);
goog.exportSymbol('proto.api.GetAwsSnsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetAwsSnsIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetAzureServiceBusIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetAzureServiceBusIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetGcpPubSubIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetGcpPubSubIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetHttpIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetHttpIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetIftttIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetIftttIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetInfluxDbIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetInfluxDbIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetLoraCloudIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetLoraCloudIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetMyDevicesIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetMyDevicesIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetPilotThingsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetPilotThingsIntegrationResponse', null, global);
goog.exportSymbol('proto.api.GetThingsBoardIntegrationRequest', null, global);
goog.exportSymbol('proto.api.GetThingsBoardIntegrationResponse', null, global);
goog.exportSymbol('proto.api.HttpIntegration', null, global);
goog.exportSymbol('proto.api.IftttIntegration', null, global);
goog.exportSymbol('proto.api.InfluxDbIntegration', null, global);
goog.exportSymbol('proto.api.InfluxDbPrecision', null, global);
goog.exportSymbol('proto.api.InfluxDbVersion', null, global);
goog.exportSymbol('proto.api.IntegrationKind', null, global);
goog.exportSymbol('proto.api.IntegrationListItem', null, global);
goog.exportSymbol('proto.api.ListApplicationsRequest', null, global);
goog.exportSymbol('proto.api.ListApplicationsResponse', null, global);
goog.exportSymbol('proto.api.ListIntegrationsRequest', null, global);
goog.exportSymbol('proto.api.ListIntegrationsResponse', null, global);
goog.exportSymbol('proto.api.LoraCloudIntegration', null, global);
goog.exportSymbol('proto.api.LoraCloudModemGeolocationServices', null, global);
goog.exportSymbol('proto.api.MyDevicesIntegration', null, global);
goog.exportSymbol('proto.api.PilotThingsIntegration', null, global);
goog.exportSymbol('proto.api.ThingsBoardIntegration', null, global);
goog.exportSymbol('proto.api.UpdateApplicationRequest', null, global);
goog.exportSymbol('proto.api.UpdateAwsSnsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateAzureServiceBusIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateGcpPubSubIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateHttpIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateIftttIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateInfluxDbIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateLoraCloudIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateMyDevicesIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdatePilotThingsIntegrationRequest', null, global);
goog.exportSymbol('proto.api.UpdateThingsBoardIntegrationRequest', null, global);
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.Application = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.Application, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.Application.displayName = 'proto.api.Application';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.ApplicationListItem = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.ApplicationListItem, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.ApplicationListItem.displayName = 'proto.api.ApplicationListItem';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateApplicationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateApplicationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateApplicationRequest.displayName = 'proto.api.CreateApplicationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateApplicationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateApplicationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateApplicationResponse.displayName = 'proto.api.CreateApplicationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetApplicationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetApplicationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetApplicationRequest.displayName = 'proto.api.GetApplicationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetApplicationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.api.GetApplicationResponse.repeatedFields_, null);
};
goog.inherits(proto.api.GetApplicationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetApplicationResponse.displayName = 'proto.api.GetApplicationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateApplicationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateApplicationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateApplicationRequest.displayName = 'proto.api.UpdateApplicationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteApplicationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteApplicationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteApplicationRequest.displayName = 'proto.api.DeleteApplicationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.ListApplicationsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.ListApplicationsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.ListApplicationsRequest.displayName = 'proto.api.ListApplicationsRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.ListApplicationsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.api.ListApplicationsResponse.repeatedFields_, null);
};
goog.inherits(proto.api.ListApplicationsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.ListApplicationsResponse.displayName = 'proto.api.ListApplicationsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.ListIntegrationsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.ListIntegrationsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.ListIntegrationsRequest.displayName = 'proto.api.ListIntegrationsRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.IntegrationListItem = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.IntegrationListItem, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.IntegrationListItem.displayName = 'proto.api.IntegrationListItem';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.ListIntegrationsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.api.ListIntegrationsResponse.repeatedFields_, null);
};
goog.inherits(proto.api.ListIntegrationsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.ListIntegrationsResponse.displayName = 'proto.api.ListIntegrationsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.HttpIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.HttpIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.HttpIntegration.displayName = 'proto.api.HttpIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateHttpIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateHttpIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateHttpIntegrationRequest.displayName = 'proto.api.CreateHttpIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetHttpIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetHttpIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetHttpIntegrationRequest.displayName = 'proto.api.GetHttpIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetHttpIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetHttpIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetHttpIntegrationResponse.displayName = 'proto.api.GetHttpIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateHttpIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateHttpIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateHttpIntegrationRequest.displayName = 'proto.api.UpdateHttpIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteHttpIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteHttpIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteHttpIntegrationRequest.displayName = 'proto.api.DeleteHttpIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.InfluxDbIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.InfluxDbIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.InfluxDbIntegration.displayName = 'proto.api.InfluxDbIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateInfluxDbIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateInfluxDbIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateInfluxDbIntegrationRequest.displayName = 'proto.api.CreateInfluxDbIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetInfluxDbIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetInfluxDbIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetInfluxDbIntegrationRequest.displayName = 'proto.api.GetInfluxDbIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetInfluxDbIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetInfluxDbIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetInfluxDbIntegrationResponse.displayName = 'proto.api.GetInfluxDbIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateInfluxDbIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateInfluxDbIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateInfluxDbIntegrationRequest.displayName = 'proto.api.UpdateInfluxDbIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteInfluxDbIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteInfluxDbIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteInfluxDbIntegrationRequest.displayName = 'proto.api.DeleteInfluxDbIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.ThingsBoardIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.ThingsBoardIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.ThingsBoardIntegration.displayName = 'proto.api.ThingsBoardIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateThingsBoardIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateThingsBoardIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateThingsBoardIntegrationRequest.displayName = 'proto.api.CreateThingsBoardIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetThingsBoardIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetThingsBoardIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetThingsBoardIntegrationRequest.displayName = 'proto.api.GetThingsBoardIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetThingsBoardIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetThingsBoardIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetThingsBoardIntegrationResponse.displayName = 'proto.api.GetThingsBoardIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateThingsBoardIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateThingsBoardIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateThingsBoardIntegrationRequest.displayName = 'proto.api.UpdateThingsBoardIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteThingsBoardIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteThingsBoardIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteThingsBoardIntegrationRequest.displayName = 'proto.api.DeleteThingsBoardIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.MyDevicesIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.MyDevicesIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.MyDevicesIntegration.displayName = 'proto.api.MyDevicesIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateMyDevicesIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateMyDevicesIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateMyDevicesIntegrationRequest.displayName = 'proto.api.CreateMyDevicesIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetMyDevicesIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetMyDevicesIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetMyDevicesIntegrationRequest.displayName = 'proto.api.GetMyDevicesIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetMyDevicesIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetMyDevicesIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetMyDevicesIntegrationResponse.displayName = 'proto.api.GetMyDevicesIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateMyDevicesIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateMyDevicesIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateMyDevicesIntegrationRequest.displayName = 'proto.api.UpdateMyDevicesIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteMyDevicesIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteMyDevicesIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteMyDevicesIntegrationRequest.displayName = 'proto.api.DeleteMyDevicesIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.LoraCloudIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.LoraCloudIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.LoraCloudIntegration.displayName = 'proto.api.LoraCloudIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.LoraCloudModemGeolocationServices = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.api.LoraCloudModemGeolocationServices.repeatedFields_, null);
};
goog.inherits(proto.api.LoraCloudModemGeolocationServices, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.LoraCloudModemGeolocationServices.displayName = 'proto.api.LoraCloudModemGeolocationServices';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateLoraCloudIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateLoraCloudIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateLoraCloudIntegrationRequest.displayName = 'proto.api.CreateLoraCloudIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetLoraCloudIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetLoraCloudIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetLoraCloudIntegrationRequest.displayName = 'proto.api.GetLoraCloudIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetLoraCloudIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetLoraCloudIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetLoraCloudIntegrationResponse.displayName = 'proto.api.GetLoraCloudIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateLoraCloudIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateLoraCloudIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateLoraCloudIntegrationRequest.displayName = 'proto.api.UpdateLoraCloudIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteLoraCloudIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteLoraCloudIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteLoraCloudIntegrationRequest.displayName = 'proto.api.DeleteLoraCloudIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GcpPubSubIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GcpPubSubIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GcpPubSubIntegration.displayName = 'proto.api.GcpPubSubIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateGcpPubSubIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateGcpPubSubIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateGcpPubSubIntegrationRequest.displayName = 'proto.api.CreateGcpPubSubIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetGcpPubSubIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetGcpPubSubIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetGcpPubSubIntegrationRequest.displayName = 'proto.api.GetGcpPubSubIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetGcpPubSubIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetGcpPubSubIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetGcpPubSubIntegrationResponse.displayName = 'proto.api.GetGcpPubSubIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateGcpPubSubIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateGcpPubSubIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateGcpPubSubIntegrationRequest.displayName = 'proto.api.UpdateGcpPubSubIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteGcpPubSubIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteGcpPubSubIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteGcpPubSubIntegrationRequest.displayName = 'proto.api.DeleteGcpPubSubIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.AwsSnsIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.AwsSnsIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.AwsSnsIntegration.displayName = 'proto.api.AwsSnsIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateAwsSnsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateAwsSnsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateAwsSnsIntegrationRequest.displayName = 'proto.api.CreateAwsSnsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetAwsSnsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetAwsSnsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetAwsSnsIntegrationRequest.displayName = 'proto.api.GetAwsSnsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetAwsSnsIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetAwsSnsIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetAwsSnsIntegrationResponse.displayName = 'proto.api.GetAwsSnsIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateAwsSnsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateAwsSnsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateAwsSnsIntegrationRequest.displayName = 'proto.api.UpdateAwsSnsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteAwsSnsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteAwsSnsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteAwsSnsIntegrationRequest.displayName = 'proto.api.DeleteAwsSnsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.AzureServiceBusIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.AzureServiceBusIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.AzureServiceBusIntegration.displayName = 'proto.api.AzureServiceBusIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateAzureServiceBusIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateAzureServiceBusIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateAzureServiceBusIntegrationRequest.displayName = 'proto.api.CreateAzureServiceBusIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetAzureServiceBusIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetAzureServiceBusIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetAzureServiceBusIntegrationRequest.displayName = 'proto.api.GetAzureServiceBusIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetAzureServiceBusIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetAzureServiceBusIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetAzureServiceBusIntegrationResponse.displayName = 'proto.api.GetAzureServiceBusIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateAzureServiceBusIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateAzureServiceBusIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateAzureServiceBusIntegrationRequest.displayName = 'proto.api.UpdateAzureServiceBusIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteAzureServiceBusIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteAzureServiceBusIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteAzureServiceBusIntegrationRequest.displayName = 'proto.api.DeleteAzureServiceBusIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.PilotThingsIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.PilotThingsIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.PilotThingsIntegration.displayName = 'proto.api.PilotThingsIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreatePilotThingsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreatePilotThingsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreatePilotThingsIntegrationRequest.displayName = 'proto.api.CreatePilotThingsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetPilotThingsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetPilotThingsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetPilotThingsIntegrationRequest.displayName = 'proto.api.GetPilotThingsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetPilotThingsIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetPilotThingsIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetPilotThingsIntegrationResponse.displayName = 'proto.api.GetPilotThingsIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdatePilotThingsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdatePilotThingsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdatePilotThingsIntegrationRequest.displayName = 'proto.api.UpdatePilotThingsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeletePilotThingsIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeletePilotThingsIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeletePilotThingsIntegrationRequest.displayName = 'proto.api.DeletePilotThingsIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.IftttIntegration = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.api.IftttIntegration.repeatedFields_, null);
};
goog.inherits(proto.api.IftttIntegration, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.IftttIntegration.displayName = 'proto.api.IftttIntegration';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.CreateIftttIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.CreateIftttIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.CreateIftttIntegrationRequest.displayName = 'proto.api.CreateIftttIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetIftttIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetIftttIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetIftttIntegrationRequest.displayName = 'proto.api.GetIftttIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GetIftttIntegrationResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GetIftttIntegrationResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GetIftttIntegrationResponse.displayName = 'proto.api.GetIftttIntegrationResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.UpdateIftttIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.UpdateIftttIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.UpdateIftttIntegrationRequest.displayName = 'proto.api.UpdateIftttIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.DeleteIftttIntegrationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.DeleteIftttIntegrationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.DeleteIftttIntegrationRequest.displayName = 'proto.api.DeleteIftttIntegrationRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GenerateMqttIntegrationClientCertificateRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GenerateMqttIntegrationClientCertificateRequest.displayName = 'proto.api.GenerateMqttIntegrationClientCertificateRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.api.GenerateMqttIntegrationClientCertificateResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.api.GenerateMqttIntegrationClientCertificateResponse.displayName = 'proto.api.GenerateMqttIntegrationClientCertificateResponse';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.Application.prototype.toObject = function(opt_includeInstance) {
  return proto.api.Application.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.Application} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.Application.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, ""),
    name: jspb.Message.getFieldWithDefault(msg, 2, ""),
    description: jspb.Message.getFieldWithDefault(msg, 3, ""),
    tenantId: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.Application}
 */
proto.api.Application.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.Application;
  return proto.api.Application.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.Application} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.Application}
 */
proto.api.Application.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setDescription(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setTenantId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.Application.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.Application.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.Application} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.Application.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getDescription();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getTenantId();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.api.Application.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.Application} returns this
 */
proto.api.Application.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.api.Application.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.Application} returns this
 */
proto.api.Application.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string description = 3;
 * @return {string}
 */
proto.api.Application.prototype.getDescription = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.Application} returns this
 */
proto.api.Application.prototype.setDescription = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string tenant_id = 4;
 * @return {string}
 */
proto.api.Application.prototype.getTenantId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.Application} returns this
 */
proto.api.Application.prototype.setTenantId = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.ApplicationListItem.prototype.toObject = function(opt_includeInstance) {
  return proto.api.ApplicationListItem.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.ApplicationListItem} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ApplicationListItem.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, ""),
    createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    name: jspb.Message.getFieldWithDefault(msg, 4, ""),
    description: jspb.Message.getFieldWithDefault(msg, 5, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.ApplicationListItem}
 */
proto.api.ApplicationListItem.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.ApplicationListItem;
  return proto.api.ApplicationListItem.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.ApplicationListItem} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.ApplicationListItem}
 */
proto.api.ApplicationListItem.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    case 3:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setDescription(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.ApplicationListItem.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.ApplicationListItem.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.ApplicationListItem} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ApplicationListItem.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getDescription();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.api.ApplicationListItem.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ApplicationListItem} returns this
 */
proto.api.ApplicationListItem.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional google.protobuf.Timestamp created_at = 2;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.api.ApplicationListItem.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 2));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.api.ApplicationListItem} returns this
*/
proto.api.ApplicationListItem.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.ApplicationListItem} returns this
 */
proto.api.ApplicationListItem.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.ApplicationListItem.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional google.protobuf.Timestamp updated_at = 3;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.api.ApplicationListItem.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 3));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.api.ApplicationListItem} returns this
*/
proto.api.ApplicationListItem.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 3, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.ApplicationListItem} returns this
 */
proto.api.ApplicationListItem.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.ApplicationListItem.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional string name = 4;
 * @return {string}
 */
proto.api.ApplicationListItem.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ApplicationListItem} returns this
 */
proto.api.ApplicationListItem.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string description = 5;
 * @return {string}
 */
proto.api.ApplicationListItem.prototype.getDescription = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ApplicationListItem} returns this
 */
proto.api.ApplicationListItem.prototype.setDescription = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateApplicationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateApplicationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateApplicationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateApplicationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    application: (f = msg.getApplication()) && proto.api.Application.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateApplicationRequest}
 */
proto.api.CreateApplicationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateApplicationRequest;
  return proto.api.CreateApplicationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateApplicationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateApplicationRequest}
 */
proto.api.CreateApplicationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.Application;
      reader.readMessage(value,proto.api.Application.deserializeBinaryFromReader);
      msg.setApplication(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateApplicationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateApplicationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateApplicationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateApplicationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplication();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.Application.serializeBinaryToWriter
    );
  }
};


/**
 * optional Application application = 1;
 * @return {?proto.api.Application}
 */
proto.api.CreateApplicationRequest.prototype.getApplication = function() {
  return /** @type{?proto.api.Application} */ (
    jspb.Message.getWrapperField(this, proto.api.Application, 1));
};


/**
 * @param {?proto.api.Application|undefined} value
 * @return {!proto.api.CreateApplicationRequest} returns this
*/
proto.api.CreateApplicationRequest.prototype.setApplication = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateApplicationRequest} returns this
 */
proto.api.CreateApplicationRequest.prototype.clearApplication = function() {
  return this.setApplication(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateApplicationRequest.prototype.hasApplication = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateApplicationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateApplicationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateApplicationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateApplicationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateApplicationResponse}
 */
proto.api.CreateApplicationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateApplicationResponse;
  return proto.api.CreateApplicationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateApplicationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateApplicationResponse}
 */
proto.api.CreateApplicationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateApplicationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateApplicationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateApplicationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateApplicationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.api.CreateApplicationResponse.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.CreateApplicationResponse} returns this
 */
proto.api.CreateApplicationResponse.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetApplicationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetApplicationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetApplicationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetApplicationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetApplicationRequest}
 */
proto.api.GetApplicationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetApplicationRequest;
  return proto.api.GetApplicationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetApplicationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetApplicationRequest}
 */
proto.api.GetApplicationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetApplicationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetApplicationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetApplicationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetApplicationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.api.GetApplicationRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetApplicationRequest} returns this
 */
proto.api.GetApplicationRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.api.GetApplicationResponse.repeatedFields_ = [4];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetApplicationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetApplicationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetApplicationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetApplicationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    application: (f = msg.getApplication()) && proto.api.Application.toObject(includeInstance, f),
    createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    measurementKeysList: (f = jspb.Message.getRepeatedField(msg, 4)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetApplicationResponse}
 */
proto.api.GetApplicationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetApplicationResponse;
  return proto.api.GetApplicationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetApplicationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetApplicationResponse}
 */
proto.api.GetApplicationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.Application;
      reader.readMessage(value,proto.api.Application.deserializeBinaryFromReader);
      msg.setApplication(value);
      break;
    case 2:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    case 3:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.addMeasurementKeys(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetApplicationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetApplicationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetApplicationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetApplicationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplication();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.Application.serializeBinaryToWriter
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getMeasurementKeysList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      4,
      f
    );
  }
};


/**
 * optional Application application = 1;
 * @return {?proto.api.Application}
 */
proto.api.GetApplicationResponse.prototype.getApplication = function() {
  return /** @type{?proto.api.Application} */ (
    jspb.Message.getWrapperField(this, proto.api.Application, 1));
};


/**
 * @param {?proto.api.Application|undefined} value
 * @return {!proto.api.GetApplicationResponse} returns this
*/
proto.api.GetApplicationResponse.prototype.setApplication = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetApplicationResponse} returns this
 */
proto.api.GetApplicationResponse.prototype.clearApplication = function() {
  return this.setApplication(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetApplicationResponse.prototype.hasApplication = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional google.protobuf.Timestamp created_at = 2;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.api.GetApplicationResponse.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 2));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.api.GetApplicationResponse} returns this
*/
proto.api.GetApplicationResponse.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetApplicationResponse} returns this
 */
proto.api.GetApplicationResponse.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetApplicationResponse.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional google.protobuf.Timestamp updated_at = 3;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.api.GetApplicationResponse.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 3));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.api.GetApplicationResponse} returns this
*/
proto.api.GetApplicationResponse.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 3, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetApplicationResponse} returns this
 */
proto.api.GetApplicationResponse.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetApplicationResponse.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * repeated string measurement_keys = 4;
 * @return {!Array<string>}
 */
proto.api.GetApplicationResponse.prototype.getMeasurementKeysList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 4));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.api.GetApplicationResponse} returns this
 */
proto.api.GetApplicationResponse.prototype.setMeasurementKeysList = function(value) {
  return jspb.Message.setField(this, 4, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.api.GetApplicationResponse} returns this
 */
proto.api.GetApplicationResponse.prototype.addMeasurementKeys = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 4, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.api.GetApplicationResponse} returns this
 */
proto.api.GetApplicationResponse.prototype.clearMeasurementKeysList = function() {
  return this.setMeasurementKeysList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateApplicationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateApplicationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateApplicationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateApplicationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    application: (f = msg.getApplication()) && proto.api.Application.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateApplicationRequest}
 */
proto.api.UpdateApplicationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateApplicationRequest;
  return proto.api.UpdateApplicationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateApplicationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateApplicationRequest}
 */
proto.api.UpdateApplicationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.Application;
      reader.readMessage(value,proto.api.Application.deserializeBinaryFromReader);
      msg.setApplication(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateApplicationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateApplicationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateApplicationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateApplicationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplication();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.Application.serializeBinaryToWriter
    );
  }
};


/**
 * optional Application application = 1;
 * @return {?proto.api.Application}
 */
proto.api.UpdateApplicationRequest.prototype.getApplication = function() {
  return /** @type{?proto.api.Application} */ (
    jspb.Message.getWrapperField(this, proto.api.Application, 1));
};


/**
 * @param {?proto.api.Application|undefined} value
 * @return {!proto.api.UpdateApplicationRequest} returns this
*/
proto.api.UpdateApplicationRequest.prototype.setApplication = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateApplicationRequest} returns this
 */
proto.api.UpdateApplicationRequest.prototype.clearApplication = function() {
  return this.setApplication(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateApplicationRequest.prototype.hasApplication = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteApplicationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteApplicationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteApplicationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteApplicationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteApplicationRequest}
 */
proto.api.DeleteApplicationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteApplicationRequest;
  return proto.api.DeleteApplicationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteApplicationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteApplicationRequest}
 */
proto.api.DeleteApplicationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteApplicationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteApplicationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteApplicationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteApplicationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.api.DeleteApplicationRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteApplicationRequest} returns this
 */
proto.api.DeleteApplicationRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.ListApplicationsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.ListApplicationsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.ListApplicationsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListApplicationsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    limit: jspb.Message.getFieldWithDefault(msg, 1, 0),
    offset: jspb.Message.getFieldWithDefault(msg, 2, 0),
    search: jspb.Message.getFieldWithDefault(msg, 3, ""),
    tenantId: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.ListApplicationsRequest}
 */
proto.api.ListApplicationsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.ListApplicationsRequest;
  return proto.api.ListApplicationsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.ListApplicationsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.ListApplicationsRequest}
 */
proto.api.ListApplicationsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setLimit(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setOffset(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setSearch(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setTenantId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.ListApplicationsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.ListApplicationsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.ListApplicationsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListApplicationsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getLimit();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getOffset();
  if (f !== 0) {
    writer.writeUint32(
      2,
      f
    );
  }
  f = message.getSearch();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getTenantId();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional uint32 limit = 1;
 * @return {number}
 */
proto.api.ListApplicationsRequest.prototype.getLimit = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.api.ListApplicationsRequest} returns this
 */
proto.api.ListApplicationsRequest.prototype.setLimit = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional uint32 offset = 2;
 * @return {number}
 */
proto.api.ListApplicationsRequest.prototype.getOffset = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.api.ListApplicationsRequest} returns this
 */
proto.api.ListApplicationsRequest.prototype.setOffset = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional string search = 3;
 * @return {string}
 */
proto.api.ListApplicationsRequest.prototype.getSearch = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ListApplicationsRequest} returns this
 */
proto.api.ListApplicationsRequest.prototype.setSearch = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string tenant_id = 4;
 * @return {string}
 */
proto.api.ListApplicationsRequest.prototype.getTenantId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ListApplicationsRequest} returns this
 */
proto.api.ListApplicationsRequest.prototype.setTenantId = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.api.ListApplicationsResponse.repeatedFields_ = [2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.ListApplicationsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.ListApplicationsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.ListApplicationsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListApplicationsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    totalCount: jspb.Message.getFieldWithDefault(msg, 1, 0),
    resultList: jspb.Message.toObjectList(msg.getResultList(),
    proto.api.ApplicationListItem.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.ListApplicationsResponse}
 */
proto.api.ListApplicationsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.ListApplicationsResponse;
  return proto.api.ListApplicationsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.ListApplicationsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.ListApplicationsResponse}
 */
proto.api.ListApplicationsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setTotalCount(value);
      break;
    case 2:
      var value = new proto.api.ApplicationListItem;
      reader.readMessage(value,proto.api.ApplicationListItem.deserializeBinaryFromReader);
      msg.addResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.ListApplicationsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.ListApplicationsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.ListApplicationsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListApplicationsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTotalCount();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getResultList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.api.ApplicationListItem.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 total_count = 1;
 * @return {number}
 */
proto.api.ListApplicationsResponse.prototype.getTotalCount = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.api.ListApplicationsResponse} returns this
 */
proto.api.ListApplicationsResponse.prototype.setTotalCount = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * repeated ApplicationListItem result = 2;
 * @return {!Array<!proto.api.ApplicationListItem>}
 */
proto.api.ListApplicationsResponse.prototype.getResultList = function() {
  return /** @type{!Array<!proto.api.ApplicationListItem>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.api.ApplicationListItem, 2));
};


/**
 * @param {!Array<!proto.api.ApplicationListItem>} value
 * @return {!proto.api.ListApplicationsResponse} returns this
*/
proto.api.ListApplicationsResponse.prototype.setResultList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.api.ApplicationListItem=} opt_value
 * @param {number=} opt_index
 * @return {!proto.api.ApplicationListItem}
 */
proto.api.ListApplicationsResponse.prototype.addResult = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.api.ApplicationListItem, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.api.ListApplicationsResponse} returns this
 */
proto.api.ListApplicationsResponse.prototype.clearResultList = function() {
  return this.setResultList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.ListIntegrationsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.ListIntegrationsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.ListIntegrationsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListIntegrationsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.ListIntegrationsRequest}
 */
proto.api.ListIntegrationsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.ListIntegrationsRequest;
  return proto.api.ListIntegrationsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.ListIntegrationsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.ListIntegrationsRequest}
 */
proto.api.ListIntegrationsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.ListIntegrationsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.ListIntegrationsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.ListIntegrationsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListIntegrationsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.ListIntegrationsRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ListIntegrationsRequest} returns this
 */
proto.api.ListIntegrationsRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.IntegrationListItem.prototype.toObject = function(opt_includeInstance) {
  return proto.api.IntegrationListItem.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.IntegrationListItem} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.IntegrationListItem.toObject = function(includeInstance, msg) {
  var f, obj = {
    kind: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.IntegrationListItem}
 */
proto.api.IntegrationListItem.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.IntegrationListItem;
  return proto.api.IntegrationListItem.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.IntegrationListItem} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.IntegrationListItem}
 */
proto.api.IntegrationListItem.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.api.IntegrationKind} */ (reader.readEnum());
      msg.setKind(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.IntegrationListItem.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.IntegrationListItem.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.IntegrationListItem} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.IntegrationListItem.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKind();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
};


/**
 * optional IntegrationKind kind = 1;
 * @return {!proto.api.IntegrationKind}
 */
proto.api.IntegrationListItem.prototype.getKind = function() {
  return /** @type {!proto.api.IntegrationKind} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.api.IntegrationKind} value
 * @return {!proto.api.IntegrationListItem} returns this
 */
proto.api.IntegrationListItem.prototype.setKind = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.api.ListIntegrationsResponse.repeatedFields_ = [2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.ListIntegrationsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.ListIntegrationsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.ListIntegrationsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListIntegrationsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    totalCount: jspb.Message.getFieldWithDefault(msg, 1, 0),
    resultList: jspb.Message.toObjectList(msg.getResultList(),
    proto.api.IntegrationListItem.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.ListIntegrationsResponse}
 */
proto.api.ListIntegrationsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.ListIntegrationsResponse;
  return proto.api.ListIntegrationsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.ListIntegrationsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.ListIntegrationsResponse}
 */
proto.api.ListIntegrationsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setTotalCount(value);
      break;
    case 2:
      var value = new proto.api.IntegrationListItem;
      reader.readMessage(value,proto.api.IntegrationListItem.deserializeBinaryFromReader);
      msg.addResult(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.ListIntegrationsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.ListIntegrationsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.ListIntegrationsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ListIntegrationsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTotalCount();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getResultList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.api.IntegrationListItem.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 total_count = 1;
 * @return {number}
 */
proto.api.ListIntegrationsResponse.prototype.getTotalCount = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.api.ListIntegrationsResponse} returns this
 */
proto.api.ListIntegrationsResponse.prototype.setTotalCount = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * repeated IntegrationListItem result = 2;
 * @return {!Array<!proto.api.IntegrationListItem>}
 */
proto.api.ListIntegrationsResponse.prototype.getResultList = function() {
  return /** @type{!Array<!proto.api.IntegrationListItem>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.api.IntegrationListItem, 2));
};


/**
 * @param {!Array<!proto.api.IntegrationListItem>} value
 * @return {!proto.api.ListIntegrationsResponse} returns this
*/
proto.api.ListIntegrationsResponse.prototype.setResultList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.api.IntegrationListItem=} opt_value
 * @param {number=} opt_index
 * @return {!proto.api.IntegrationListItem}
 */
proto.api.ListIntegrationsResponse.prototype.addResult = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.api.IntegrationListItem, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.api.ListIntegrationsResponse} returns this
 */
proto.api.ListIntegrationsResponse.prototype.clearResultList = function() {
  return this.setResultList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.HttpIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.HttpIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.HttpIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.HttpIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    headersMap: (f = msg.getHeadersMap()) ? f.toObject(includeInstance, undefined) : [],
    encoding: jspb.Message.getFieldWithDefault(msg, 3, 0),
    eventEndpointUrl: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.HttpIntegration}
 */
proto.api.HttpIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.HttpIntegration;
  return proto.api.HttpIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.HttpIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.HttpIntegration}
 */
proto.api.HttpIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = msg.getHeadersMap();
      reader.readMessage(value, function(message, reader) {
        jspb.Map.deserializeBinary(message, reader, jspb.BinaryReader.prototype.readString, jspb.BinaryReader.prototype.readString, null, "", "");
         });
      break;
    case 3:
      var value = /** @type {!proto.api.Encoding} */ (reader.readEnum());
      msg.setEncoding(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setEventEndpointUrl(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.HttpIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.HttpIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.HttpIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.HttpIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getHeadersMap(true);
  if (f && f.getLength() > 0) {
    f.serializeBinary(2, writer, jspb.BinaryWriter.prototype.writeString, jspb.BinaryWriter.prototype.writeString);
  }
  f = message.getEncoding();
  if (f !== 0.0) {
    writer.writeEnum(
      3,
      f
    );
  }
  f = message.getEventEndpointUrl();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.HttpIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.HttpIntegration} returns this
 */
proto.api.HttpIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * map<string, string> headers = 2;
 * @param {boolean=} opt_noLazyCreate Do not create the map if
 * empty, instead returning `undefined`
 * @return {!jspb.Map<string,string>}
 */
proto.api.HttpIntegration.prototype.getHeadersMap = function(opt_noLazyCreate) {
  return /** @type {!jspb.Map<string,string>} */ (
      jspb.Message.getMapField(this, 2, opt_noLazyCreate,
      null));
};


/**
 * Clears values from the map. The map will be non-null.
 * @return {!proto.api.HttpIntegration} returns this
 */
proto.api.HttpIntegration.prototype.clearHeadersMap = function() {
  this.getHeadersMap().clear();
  return this;};


/**
 * optional Encoding encoding = 3;
 * @return {!proto.api.Encoding}
 */
proto.api.HttpIntegration.prototype.getEncoding = function() {
  return /** @type {!proto.api.Encoding} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {!proto.api.Encoding} value
 * @return {!proto.api.HttpIntegration} returns this
 */
proto.api.HttpIntegration.prototype.setEncoding = function(value) {
  return jspb.Message.setProto3EnumField(this, 3, value);
};


/**
 * optional string event_endpoint_url = 4;
 * @return {string}
 */
proto.api.HttpIntegration.prototype.getEventEndpointUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.HttpIntegration} returns this
 */
proto.api.HttpIntegration.prototype.setEventEndpointUrl = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateHttpIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateHttpIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateHttpIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateHttpIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.HttpIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateHttpIntegrationRequest}
 */
proto.api.CreateHttpIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateHttpIntegrationRequest;
  return proto.api.CreateHttpIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateHttpIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateHttpIntegrationRequest}
 */
proto.api.CreateHttpIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.HttpIntegration;
      reader.readMessage(value,proto.api.HttpIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateHttpIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateHttpIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateHttpIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateHttpIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.HttpIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional HttpIntegration integration = 1;
 * @return {?proto.api.HttpIntegration}
 */
proto.api.CreateHttpIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.HttpIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.HttpIntegration, 1));
};


/**
 * @param {?proto.api.HttpIntegration|undefined} value
 * @return {!proto.api.CreateHttpIntegrationRequest} returns this
*/
proto.api.CreateHttpIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateHttpIntegrationRequest} returns this
 */
proto.api.CreateHttpIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateHttpIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetHttpIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetHttpIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetHttpIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetHttpIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetHttpIntegrationRequest}
 */
proto.api.GetHttpIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetHttpIntegrationRequest;
  return proto.api.GetHttpIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetHttpIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetHttpIntegrationRequest}
 */
proto.api.GetHttpIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetHttpIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetHttpIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetHttpIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetHttpIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetHttpIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetHttpIntegrationRequest} returns this
 */
proto.api.GetHttpIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetHttpIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetHttpIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetHttpIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetHttpIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.HttpIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetHttpIntegrationResponse}
 */
proto.api.GetHttpIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetHttpIntegrationResponse;
  return proto.api.GetHttpIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetHttpIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetHttpIntegrationResponse}
 */
proto.api.GetHttpIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.HttpIntegration;
      reader.readMessage(value,proto.api.HttpIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetHttpIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetHttpIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetHttpIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetHttpIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.HttpIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional HttpIntegration integration = 1;
 * @return {?proto.api.HttpIntegration}
 */
proto.api.GetHttpIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.HttpIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.HttpIntegration, 1));
};


/**
 * @param {?proto.api.HttpIntegration|undefined} value
 * @return {!proto.api.GetHttpIntegrationResponse} returns this
*/
proto.api.GetHttpIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetHttpIntegrationResponse} returns this
 */
proto.api.GetHttpIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetHttpIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateHttpIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateHttpIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateHttpIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateHttpIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.HttpIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateHttpIntegrationRequest}
 */
proto.api.UpdateHttpIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateHttpIntegrationRequest;
  return proto.api.UpdateHttpIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateHttpIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateHttpIntegrationRequest}
 */
proto.api.UpdateHttpIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.HttpIntegration;
      reader.readMessage(value,proto.api.HttpIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateHttpIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateHttpIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateHttpIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateHttpIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.HttpIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional HttpIntegration integration = 1;
 * @return {?proto.api.HttpIntegration}
 */
proto.api.UpdateHttpIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.HttpIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.HttpIntegration, 1));
};


/**
 * @param {?proto.api.HttpIntegration|undefined} value
 * @return {!proto.api.UpdateHttpIntegrationRequest} returns this
*/
proto.api.UpdateHttpIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateHttpIntegrationRequest} returns this
 */
proto.api.UpdateHttpIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateHttpIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteHttpIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteHttpIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteHttpIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteHttpIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteHttpIntegrationRequest}
 */
proto.api.DeleteHttpIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteHttpIntegrationRequest;
  return proto.api.DeleteHttpIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteHttpIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteHttpIntegrationRequest}
 */
proto.api.DeleteHttpIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteHttpIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteHttpIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteHttpIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteHttpIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteHttpIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteHttpIntegrationRequest} returns this
 */
proto.api.DeleteHttpIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.InfluxDbIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.InfluxDbIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.InfluxDbIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.InfluxDbIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    endpoint: jspb.Message.getFieldWithDefault(msg, 2, ""),
    db: jspb.Message.getFieldWithDefault(msg, 3, ""),
    username: jspb.Message.getFieldWithDefault(msg, 4, ""),
    password: jspb.Message.getFieldWithDefault(msg, 5, ""),
    retentionPolicyName: jspb.Message.getFieldWithDefault(msg, 6, ""),
    precision: jspb.Message.getFieldWithDefault(msg, 7, 0),
    version: jspb.Message.getFieldWithDefault(msg, 8, 0),
    token: jspb.Message.getFieldWithDefault(msg, 9, ""),
    organization: jspb.Message.getFieldWithDefault(msg, 10, ""),
    bucket: jspb.Message.getFieldWithDefault(msg, 11, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.InfluxDbIntegration}
 */
proto.api.InfluxDbIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.InfluxDbIntegration;
  return proto.api.InfluxDbIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.InfluxDbIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.InfluxDbIntegration}
 */
proto.api.InfluxDbIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setEndpoint(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setDb(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setUsername(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setPassword(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setRetentionPolicyName(value);
      break;
    case 7:
      var value = /** @type {!proto.api.InfluxDbPrecision} */ (reader.readEnum());
      msg.setPrecision(value);
      break;
    case 8:
      var value = /** @type {!proto.api.InfluxDbVersion} */ (reader.readEnum());
      msg.setVersion(value);
      break;
    case 9:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    case 10:
      var value = /** @type {string} */ (reader.readString());
      msg.setOrganization(value);
      break;
    case 11:
      var value = /** @type {string} */ (reader.readString());
      msg.setBucket(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.InfluxDbIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.InfluxDbIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.InfluxDbIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.InfluxDbIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEndpoint();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getDb();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getUsername();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getPassword();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = message.getRetentionPolicyName();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
  f = message.getPrecision();
  if (f !== 0.0) {
    writer.writeEnum(
      7,
      f
    );
  }
  f = message.getVersion();
  if (f !== 0.0) {
    writer.writeEnum(
      8,
      f
    );
  }
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      9,
      f
    );
  }
  f = message.getOrganization();
  if (f.length > 0) {
    writer.writeString(
      10,
      f
    );
  }
  f = message.getBucket();
  if (f.length > 0) {
    writer.writeString(
      11,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string endpoint = 2;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getEndpoint = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setEndpoint = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string db = 3;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getDb = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setDb = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string username = 4;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getUsername = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setUsername = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string password = 5;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional string retention_policy_name = 6;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getRetentionPolicyName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setRetentionPolicyName = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


/**
 * optional InfluxDbPrecision precision = 7;
 * @return {!proto.api.InfluxDbPrecision}
 */
proto.api.InfluxDbIntegration.prototype.getPrecision = function() {
  return /** @type {!proto.api.InfluxDbPrecision} */ (jspb.Message.getFieldWithDefault(this, 7, 0));
};


/**
 * @param {!proto.api.InfluxDbPrecision} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setPrecision = function(value) {
  return jspb.Message.setProto3EnumField(this, 7, value);
};


/**
 * optional InfluxDbVersion version = 8;
 * @return {!proto.api.InfluxDbVersion}
 */
proto.api.InfluxDbIntegration.prototype.getVersion = function() {
  return /** @type {!proto.api.InfluxDbVersion} */ (jspb.Message.getFieldWithDefault(this, 8, 0));
};


/**
 * @param {!proto.api.InfluxDbVersion} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setVersion = function(value) {
  return jspb.Message.setProto3EnumField(this, 8, value);
};


/**
 * optional string token = 9;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 9, value);
};


/**
 * optional string organization = 10;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getOrganization = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 10, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setOrganization = function(value) {
  return jspb.Message.setProto3StringField(this, 10, value);
};


/**
 * optional string bucket = 11;
 * @return {string}
 */
proto.api.InfluxDbIntegration.prototype.getBucket = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 11, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.InfluxDbIntegration} returns this
 */
proto.api.InfluxDbIntegration.prototype.setBucket = function(value) {
  return jspb.Message.setProto3StringField(this, 11, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateInfluxDbIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateInfluxDbIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateInfluxDbIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateInfluxDbIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.InfluxDbIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateInfluxDbIntegrationRequest}
 */
proto.api.CreateInfluxDbIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateInfluxDbIntegrationRequest;
  return proto.api.CreateInfluxDbIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateInfluxDbIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateInfluxDbIntegrationRequest}
 */
proto.api.CreateInfluxDbIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.InfluxDbIntegration;
      reader.readMessage(value,proto.api.InfluxDbIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateInfluxDbIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateInfluxDbIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateInfluxDbIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateInfluxDbIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.InfluxDbIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional InfluxDbIntegration integration = 1;
 * @return {?proto.api.InfluxDbIntegration}
 */
proto.api.CreateInfluxDbIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.InfluxDbIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.InfluxDbIntegration, 1));
};


/**
 * @param {?proto.api.InfluxDbIntegration|undefined} value
 * @return {!proto.api.CreateInfluxDbIntegrationRequest} returns this
*/
proto.api.CreateInfluxDbIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateInfluxDbIntegrationRequest} returns this
 */
proto.api.CreateInfluxDbIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateInfluxDbIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetInfluxDbIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetInfluxDbIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetInfluxDbIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetInfluxDbIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetInfluxDbIntegrationRequest}
 */
proto.api.GetInfluxDbIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetInfluxDbIntegrationRequest;
  return proto.api.GetInfluxDbIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetInfluxDbIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetInfluxDbIntegrationRequest}
 */
proto.api.GetInfluxDbIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetInfluxDbIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetInfluxDbIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetInfluxDbIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetInfluxDbIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetInfluxDbIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetInfluxDbIntegrationRequest} returns this
 */
proto.api.GetInfluxDbIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetInfluxDbIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetInfluxDbIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetInfluxDbIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetInfluxDbIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.InfluxDbIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetInfluxDbIntegrationResponse}
 */
proto.api.GetInfluxDbIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetInfluxDbIntegrationResponse;
  return proto.api.GetInfluxDbIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetInfluxDbIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetInfluxDbIntegrationResponse}
 */
proto.api.GetInfluxDbIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.InfluxDbIntegration;
      reader.readMessage(value,proto.api.InfluxDbIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetInfluxDbIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetInfluxDbIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetInfluxDbIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetInfluxDbIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.InfluxDbIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional InfluxDbIntegration integration = 1;
 * @return {?proto.api.InfluxDbIntegration}
 */
proto.api.GetInfluxDbIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.InfluxDbIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.InfluxDbIntegration, 1));
};


/**
 * @param {?proto.api.InfluxDbIntegration|undefined} value
 * @return {!proto.api.GetInfluxDbIntegrationResponse} returns this
*/
proto.api.GetInfluxDbIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetInfluxDbIntegrationResponse} returns this
 */
proto.api.GetInfluxDbIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetInfluxDbIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateInfluxDbIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateInfluxDbIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateInfluxDbIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateInfluxDbIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.InfluxDbIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateInfluxDbIntegrationRequest}
 */
proto.api.UpdateInfluxDbIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateInfluxDbIntegrationRequest;
  return proto.api.UpdateInfluxDbIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateInfluxDbIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateInfluxDbIntegrationRequest}
 */
proto.api.UpdateInfluxDbIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.InfluxDbIntegration;
      reader.readMessage(value,proto.api.InfluxDbIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateInfluxDbIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateInfluxDbIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateInfluxDbIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateInfluxDbIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.InfluxDbIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional InfluxDbIntegration integration = 1;
 * @return {?proto.api.InfluxDbIntegration}
 */
proto.api.UpdateInfluxDbIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.InfluxDbIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.InfluxDbIntegration, 1));
};


/**
 * @param {?proto.api.InfluxDbIntegration|undefined} value
 * @return {!proto.api.UpdateInfluxDbIntegrationRequest} returns this
*/
proto.api.UpdateInfluxDbIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateInfluxDbIntegrationRequest} returns this
 */
proto.api.UpdateInfluxDbIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateInfluxDbIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteInfluxDbIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteInfluxDbIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteInfluxDbIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteInfluxDbIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteInfluxDbIntegrationRequest}
 */
proto.api.DeleteInfluxDbIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteInfluxDbIntegrationRequest;
  return proto.api.DeleteInfluxDbIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteInfluxDbIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteInfluxDbIntegrationRequest}
 */
proto.api.DeleteInfluxDbIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteInfluxDbIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteInfluxDbIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteInfluxDbIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteInfluxDbIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteInfluxDbIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteInfluxDbIntegrationRequest} returns this
 */
proto.api.DeleteInfluxDbIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.ThingsBoardIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.ThingsBoardIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.ThingsBoardIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ThingsBoardIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    server: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.ThingsBoardIntegration}
 */
proto.api.ThingsBoardIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.ThingsBoardIntegration;
  return proto.api.ThingsBoardIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.ThingsBoardIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.ThingsBoardIntegration}
 */
proto.api.ThingsBoardIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setServer(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.ThingsBoardIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.ThingsBoardIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.ThingsBoardIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.ThingsBoardIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getServer();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.ThingsBoardIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ThingsBoardIntegration} returns this
 */
proto.api.ThingsBoardIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string server = 2;
 * @return {string}
 */
proto.api.ThingsBoardIntegration.prototype.getServer = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.ThingsBoardIntegration} returns this
 */
proto.api.ThingsBoardIntegration.prototype.setServer = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateThingsBoardIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateThingsBoardIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateThingsBoardIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateThingsBoardIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.ThingsBoardIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateThingsBoardIntegrationRequest}
 */
proto.api.CreateThingsBoardIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateThingsBoardIntegrationRequest;
  return proto.api.CreateThingsBoardIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateThingsBoardIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateThingsBoardIntegrationRequest}
 */
proto.api.CreateThingsBoardIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.ThingsBoardIntegration;
      reader.readMessage(value,proto.api.ThingsBoardIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateThingsBoardIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateThingsBoardIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateThingsBoardIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateThingsBoardIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.ThingsBoardIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional ThingsBoardIntegration integration = 1;
 * @return {?proto.api.ThingsBoardIntegration}
 */
proto.api.CreateThingsBoardIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.ThingsBoardIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.ThingsBoardIntegration, 1));
};


/**
 * @param {?proto.api.ThingsBoardIntegration|undefined} value
 * @return {!proto.api.CreateThingsBoardIntegrationRequest} returns this
*/
proto.api.CreateThingsBoardIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateThingsBoardIntegrationRequest} returns this
 */
proto.api.CreateThingsBoardIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateThingsBoardIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetThingsBoardIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetThingsBoardIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetThingsBoardIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetThingsBoardIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetThingsBoardIntegrationRequest}
 */
proto.api.GetThingsBoardIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetThingsBoardIntegrationRequest;
  return proto.api.GetThingsBoardIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetThingsBoardIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetThingsBoardIntegrationRequest}
 */
proto.api.GetThingsBoardIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetThingsBoardIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetThingsBoardIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetThingsBoardIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetThingsBoardIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetThingsBoardIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetThingsBoardIntegrationRequest} returns this
 */
proto.api.GetThingsBoardIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetThingsBoardIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetThingsBoardIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetThingsBoardIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetThingsBoardIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.ThingsBoardIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetThingsBoardIntegrationResponse}
 */
proto.api.GetThingsBoardIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetThingsBoardIntegrationResponse;
  return proto.api.GetThingsBoardIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetThingsBoardIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetThingsBoardIntegrationResponse}
 */
proto.api.GetThingsBoardIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.ThingsBoardIntegration;
      reader.readMessage(value,proto.api.ThingsBoardIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetThingsBoardIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetThingsBoardIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetThingsBoardIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetThingsBoardIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.ThingsBoardIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional ThingsBoardIntegration integration = 1;
 * @return {?proto.api.ThingsBoardIntegration}
 */
proto.api.GetThingsBoardIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.ThingsBoardIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.ThingsBoardIntegration, 1));
};


/**
 * @param {?proto.api.ThingsBoardIntegration|undefined} value
 * @return {!proto.api.GetThingsBoardIntegrationResponse} returns this
*/
proto.api.GetThingsBoardIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetThingsBoardIntegrationResponse} returns this
 */
proto.api.GetThingsBoardIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetThingsBoardIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateThingsBoardIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateThingsBoardIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateThingsBoardIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateThingsBoardIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.ThingsBoardIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateThingsBoardIntegrationRequest}
 */
proto.api.UpdateThingsBoardIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateThingsBoardIntegrationRequest;
  return proto.api.UpdateThingsBoardIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateThingsBoardIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateThingsBoardIntegrationRequest}
 */
proto.api.UpdateThingsBoardIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.ThingsBoardIntegration;
      reader.readMessage(value,proto.api.ThingsBoardIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateThingsBoardIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateThingsBoardIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateThingsBoardIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateThingsBoardIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.ThingsBoardIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional ThingsBoardIntegration integration = 1;
 * @return {?proto.api.ThingsBoardIntegration}
 */
proto.api.UpdateThingsBoardIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.ThingsBoardIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.ThingsBoardIntegration, 1));
};


/**
 * @param {?proto.api.ThingsBoardIntegration|undefined} value
 * @return {!proto.api.UpdateThingsBoardIntegrationRequest} returns this
*/
proto.api.UpdateThingsBoardIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateThingsBoardIntegrationRequest} returns this
 */
proto.api.UpdateThingsBoardIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateThingsBoardIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteThingsBoardIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteThingsBoardIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteThingsBoardIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteThingsBoardIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteThingsBoardIntegrationRequest}
 */
proto.api.DeleteThingsBoardIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteThingsBoardIntegrationRequest;
  return proto.api.DeleteThingsBoardIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteThingsBoardIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteThingsBoardIntegrationRequest}
 */
proto.api.DeleteThingsBoardIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteThingsBoardIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteThingsBoardIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteThingsBoardIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteThingsBoardIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteThingsBoardIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteThingsBoardIntegrationRequest} returns this
 */
proto.api.DeleteThingsBoardIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.MyDevicesIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.MyDevicesIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.MyDevicesIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.MyDevicesIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    endpoint: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.MyDevicesIntegration}
 */
proto.api.MyDevicesIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.MyDevicesIntegration;
  return proto.api.MyDevicesIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.MyDevicesIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.MyDevicesIntegration}
 */
proto.api.MyDevicesIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setEndpoint(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.MyDevicesIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.MyDevicesIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.MyDevicesIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.MyDevicesIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEndpoint();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.MyDevicesIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.MyDevicesIntegration} returns this
 */
proto.api.MyDevicesIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string endpoint = 2;
 * @return {string}
 */
proto.api.MyDevicesIntegration.prototype.getEndpoint = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.MyDevicesIntegration} returns this
 */
proto.api.MyDevicesIntegration.prototype.setEndpoint = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateMyDevicesIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateMyDevicesIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateMyDevicesIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateMyDevicesIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.MyDevicesIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateMyDevicesIntegrationRequest}
 */
proto.api.CreateMyDevicesIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateMyDevicesIntegrationRequest;
  return proto.api.CreateMyDevicesIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateMyDevicesIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateMyDevicesIntegrationRequest}
 */
proto.api.CreateMyDevicesIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.MyDevicesIntegration;
      reader.readMessage(value,proto.api.MyDevicesIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateMyDevicesIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateMyDevicesIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateMyDevicesIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateMyDevicesIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.MyDevicesIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional MyDevicesIntegration integration = 1;
 * @return {?proto.api.MyDevicesIntegration}
 */
proto.api.CreateMyDevicesIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.MyDevicesIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.MyDevicesIntegration, 1));
};


/**
 * @param {?proto.api.MyDevicesIntegration|undefined} value
 * @return {!proto.api.CreateMyDevicesIntegrationRequest} returns this
*/
proto.api.CreateMyDevicesIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateMyDevicesIntegrationRequest} returns this
 */
proto.api.CreateMyDevicesIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateMyDevicesIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetMyDevicesIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetMyDevicesIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetMyDevicesIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetMyDevicesIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetMyDevicesIntegrationRequest}
 */
proto.api.GetMyDevicesIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetMyDevicesIntegrationRequest;
  return proto.api.GetMyDevicesIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetMyDevicesIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetMyDevicesIntegrationRequest}
 */
proto.api.GetMyDevicesIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetMyDevicesIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetMyDevicesIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetMyDevicesIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetMyDevicesIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetMyDevicesIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetMyDevicesIntegrationRequest} returns this
 */
proto.api.GetMyDevicesIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetMyDevicesIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetMyDevicesIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetMyDevicesIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetMyDevicesIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.MyDevicesIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetMyDevicesIntegrationResponse}
 */
proto.api.GetMyDevicesIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetMyDevicesIntegrationResponse;
  return proto.api.GetMyDevicesIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetMyDevicesIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetMyDevicesIntegrationResponse}
 */
proto.api.GetMyDevicesIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.MyDevicesIntegration;
      reader.readMessage(value,proto.api.MyDevicesIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetMyDevicesIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetMyDevicesIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetMyDevicesIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetMyDevicesIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.MyDevicesIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional MyDevicesIntegration integration = 1;
 * @return {?proto.api.MyDevicesIntegration}
 */
proto.api.GetMyDevicesIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.MyDevicesIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.MyDevicesIntegration, 1));
};


/**
 * @param {?proto.api.MyDevicesIntegration|undefined} value
 * @return {!proto.api.GetMyDevicesIntegrationResponse} returns this
*/
proto.api.GetMyDevicesIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetMyDevicesIntegrationResponse} returns this
 */
proto.api.GetMyDevicesIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetMyDevicesIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateMyDevicesIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateMyDevicesIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateMyDevicesIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateMyDevicesIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.MyDevicesIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateMyDevicesIntegrationRequest}
 */
proto.api.UpdateMyDevicesIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateMyDevicesIntegrationRequest;
  return proto.api.UpdateMyDevicesIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateMyDevicesIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateMyDevicesIntegrationRequest}
 */
proto.api.UpdateMyDevicesIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.MyDevicesIntegration;
      reader.readMessage(value,proto.api.MyDevicesIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateMyDevicesIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateMyDevicesIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateMyDevicesIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateMyDevicesIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.MyDevicesIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional MyDevicesIntegration integration = 1;
 * @return {?proto.api.MyDevicesIntegration}
 */
proto.api.UpdateMyDevicesIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.MyDevicesIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.MyDevicesIntegration, 1));
};


/**
 * @param {?proto.api.MyDevicesIntegration|undefined} value
 * @return {!proto.api.UpdateMyDevicesIntegrationRequest} returns this
*/
proto.api.UpdateMyDevicesIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateMyDevicesIntegrationRequest} returns this
 */
proto.api.UpdateMyDevicesIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateMyDevicesIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteMyDevicesIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteMyDevicesIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteMyDevicesIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteMyDevicesIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteMyDevicesIntegrationRequest}
 */
proto.api.DeleteMyDevicesIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteMyDevicesIntegrationRequest;
  return proto.api.DeleteMyDevicesIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteMyDevicesIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteMyDevicesIntegrationRequest}
 */
proto.api.DeleteMyDevicesIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteMyDevicesIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteMyDevicesIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteMyDevicesIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteMyDevicesIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteMyDevicesIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteMyDevicesIntegrationRequest} returns this
 */
proto.api.DeleteMyDevicesIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.LoraCloudIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.LoraCloudIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.LoraCloudIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.LoraCloudIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    modemGeolocationServices: (f = msg.getModemGeolocationServices()) && proto.api.LoraCloudModemGeolocationServices.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.LoraCloudIntegration}
 */
proto.api.LoraCloudIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.LoraCloudIntegration;
  return proto.api.LoraCloudIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.LoraCloudIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.LoraCloudIntegration}
 */
proto.api.LoraCloudIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = new proto.api.LoraCloudModemGeolocationServices;
      reader.readMessage(value,proto.api.LoraCloudModemGeolocationServices.deserializeBinaryFromReader);
      msg.setModemGeolocationServices(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.LoraCloudIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.LoraCloudIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.LoraCloudIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.LoraCloudIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getModemGeolocationServices();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.api.LoraCloudModemGeolocationServices.serializeBinaryToWriter
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.LoraCloudIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.LoraCloudIntegration} returns this
 */
proto.api.LoraCloudIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional LoraCloudModemGeolocationServices modem_geolocation_services = 2;
 * @return {?proto.api.LoraCloudModemGeolocationServices}
 */
proto.api.LoraCloudIntegration.prototype.getModemGeolocationServices = function() {
  return /** @type{?proto.api.LoraCloudModemGeolocationServices} */ (
    jspb.Message.getWrapperField(this, proto.api.LoraCloudModemGeolocationServices, 2));
};


/**
 * @param {?proto.api.LoraCloudModemGeolocationServices|undefined} value
 * @return {!proto.api.LoraCloudIntegration} returns this
*/
proto.api.LoraCloudIntegration.prototype.setModemGeolocationServices = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.LoraCloudIntegration} returns this
 */
proto.api.LoraCloudIntegration.prototype.clearModemGeolocationServices = function() {
  return this.setModemGeolocationServices(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.LoraCloudIntegration.prototype.hasModemGeolocationServices = function() {
  return jspb.Message.getField(this, 2) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.api.LoraCloudModemGeolocationServices.repeatedFields_ = [16];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.toObject = function(opt_includeInstance) {
  return proto.api.LoraCloudModemGeolocationServices.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.LoraCloudModemGeolocationServices} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.LoraCloudModemGeolocationServices.toObject = function(includeInstance, msg) {
  var f, obj = {
    token: jspb.Message.getFieldWithDefault(msg, 1, ""),
    modemEnabled: jspb.Message.getBooleanFieldWithDefault(msg, 2, false),
    forwardFPortsList: (f = jspb.Message.getRepeatedField(msg, 16)) == null ? undefined : f,
    gnssUseRxTime: jspb.Message.getBooleanFieldWithDefault(msg, 5, false),
    gnssUseGatewayLocation: jspb.Message.getBooleanFieldWithDefault(msg, 17, false),
    parseTlv: jspb.Message.getBooleanFieldWithDefault(msg, 6, false),
    geolocationBufferTtl: jspb.Message.getFieldWithDefault(msg, 7, 0),
    geolocationMinBufferSize: jspb.Message.getFieldWithDefault(msg, 8, 0),
    geolocationTdoa: jspb.Message.getBooleanFieldWithDefault(msg, 9, false),
    geolocationRssi: jspb.Message.getBooleanFieldWithDefault(msg, 10, false),
    geolocationGnss: jspb.Message.getBooleanFieldWithDefault(msg, 11, false),
    geolocationGnssPayloadField: jspb.Message.getFieldWithDefault(msg, 12, ""),
    geolocationGnssUseRxTime: jspb.Message.getBooleanFieldWithDefault(msg, 13, false),
    geolocationWifi: jspb.Message.getBooleanFieldWithDefault(msg, 14, false),
    geolocationWifiPayloadField: jspb.Message.getFieldWithDefault(msg, 15, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.LoraCloudModemGeolocationServices}
 */
proto.api.LoraCloudModemGeolocationServices.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.LoraCloudModemGeolocationServices;
  return proto.api.LoraCloudModemGeolocationServices.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.LoraCloudModemGeolocationServices} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.LoraCloudModemGeolocationServices}
 */
proto.api.LoraCloudModemGeolocationServices.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    case 2:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setModemEnabled(value);
      break;
    case 16:
      var value = /** @type {!Array<number>} */ (reader.readPackedUint32());
      msg.setForwardFPortsList(value);
      break;
    case 5:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setGnssUseRxTime(value);
      break;
    case 17:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setGnssUseGatewayLocation(value);
      break;
    case 6:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setParseTlv(value);
      break;
    case 7:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setGeolocationBufferTtl(value);
      break;
    case 8:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setGeolocationMinBufferSize(value);
      break;
    case 9:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setGeolocationTdoa(value);
      break;
    case 10:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setGeolocationRssi(value);
      break;
    case 11:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setGeolocationGnss(value);
      break;
    case 12:
      var value = /** @type {string} */ (reader.readString());
      msg.setGeolocationGnssPayloadField(value);
      break;
    case 13:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setGeolocationGnssUseRxTime(value);
      break;
    case 14:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setGeolocationWifi(value);
      break;
    case 15:
      var value = /** @type {string} */ (reader.readString());
      msg.setGeolocationWifiPayloadField(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.LoraCloudModemGeolocationServices.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.LoraCloudModemGeolocationServices} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.LoraCloudModemGeolocationServices.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getModemEnabled();
  if (f) {
    writer.writeBool(
      2,
      f
    );
  }
  f = message.getForwardFPortsList();
  if (f.length > 0) {
    writer.writePackedUint32(
      16,
      f
    );
  }
  f = message.getGnssUseRxTime();
  if (f) {
    writer.writeBool(
      5,
      f
    );
  }
  f = message.getGnssUseGatewayLocation();
  if (f) {
    writer.writeBool(
      17,
      f
    );
  }
  f = message.getParseTlv();
  if (f) {
    writer.writeBool(
      6,
      f
    );
  }
  f = message.getGeolocationBufferTtl();
  if (f !== 0) {
    writer.writeUint32(
      7,
      f
    );
  }
  f = message.getGeolocationMinBufferSize();
  if (f !== 0) {
    writer.writeUint32(
      8,
      f
    );
  }
  f = message.getGeolocationTdoa();
  if (f) {
    writer.writeBool(
      9,
      f
    );
  }
  f = message.getGeolocationRssi();
  if (f) {
    writer.writeBool(
      10,
      f
    );
  }
  f = message.getGeolocationGnss();
  if (f) {
    writer.writeBool(
      11,
      f
    );
  }
  f = message.getGeolocationGnssPayloadField();
  if (f.length > 0) {
    writer.writeString(
      12,
      f
    );
  }
  f = message.getGeolocationGnssUseRxTime();
  if (f) {
    writer.writeBool(
      13,
      f
    );
  }
  f = message.getGeolocationWifi();
  if (f) {
    writer.writeBool(
      14,
      f
    );
  }
  f = message.getGeolocationWifiPayloadField();
  if (f.length > 0) {
    writer.writeString(
      15,
      f
    );
  }
};


/**
 * optional string token = 1;
 * @return {string}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional bool modem_enabled = 2;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getModemEnabled = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 2, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setModemEnabled = function(value) {
  return jspb.Message.setProto3BooleanField(this, 2, value);
};


/**
 * repeated uint32 forward_f_ports = 16;
 * @return {!Array<number>}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getForwardFPortsList = function() {
  return /** @type {!Array<number>} */ (jspb.Message.getRepeatedField(this, 16));
};


/**
 * @param {!Array<number>} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setForwardFPortsList = function(value) {
  return jspb.Message.setField(this, 16, value || []);
};


/**
 * @param {number} value
 * @param {number=} opt_index
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.addForwardFPorts = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 16, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.clearForwardFPortsList = function() {
  return this.setForwardFPortsList([]);
};


/**
 * optional bool gnss_use_rx_time = 5;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGnssUseRxTime = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 5, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGnssUseRxTime = function(value) {
  return jspb.Message.setProto3BooleanField(this, 5, value);
};


/**
 * optional bool gnss_use_gateway_location = 17;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGnssUseGatewayLocation = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 17, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGnssUseGatewayLocation = function(value) {
  return jspb.Message.setProto3BooleanField(this, 17, value);
};


/**
 * optional bool parse_tlv = 6;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getParseTlv = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 6, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setParseTlv = function(value) {
  return jspb.Message.setProto3BooleanField(this, 6, value);
};


/**
 * optional uint32 geolocation_buffer_ttl = 7;
 * @return {number}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationBufferTtl = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 7, 0));
};


/**
 * @param {number} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationBufferTtl = function(value) {
  return jspb.Message.setProto3IntField(this, 7, value);
};


/**
 * optional uint32 geolocation_min_buffer_size = 8;
 * @return {number}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationMinBufferSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 8, 0));
};


/**
 * @param {number} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationMinBufferSize = function(value) {
  return jspb.Message.setProto3IntField(this, 8, value);
};


/**
 * optional bool geolocation_tdoa = 9;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationTdoa = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 9, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationTdoa = function(value) {
  return jspb.Message.setProto3BooleanField(this, 9, value);
};


/**
 * optional bool geolocation_rssi = 10;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationRssi = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 10, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationRssi = function(value) {
  return jspb.Message.setProto3BooleanField(this, 10, value);
};


/**
 * optional bool geolocation_gnss = 11;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationGnss = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 11, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationGnss = function(value) {
  return jspb.Message.setProto3BooleanField(this, 11, value);
};


/**
 * optional string geolocation_gnss_payload_field = 12;
 * @return {string}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationGnssPayloadField = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 12, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationGnssPayloadField = function(value) {
  return jspb.Message.setProto3StringField(this, 12, value);
};


/**
 * optional bool geolocation_gnss_use_rx_time = 13;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationGnssUseRxTime = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 13, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationGnssUseRxTime = function(value) {
  return jspb.Message.setProto3BooleanField(this, 13, value);
};


/**
 * optional bool geolocation_wifi = 14;
 * @return {boolean}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationWifi = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 14, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationWifi = function(value) {
  return jspb.Message.setProto3BooleanField(this, 14, value);
};


/**
 * optional string geolocation_wifi_payload_field = 15;
 * @return {string}
 */
proto.api.LoraCloudModemGeolocationServices.prototype.getGeolocationWifiPayloadField = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 15, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.LoraCloudModemGeolocationServices} returns this
 */
proto.api.LoraCloudModemGeolocationServices.prototype.setGeolocationWifiPayloadField = function(value) {
  return jspb.Message.setProto3StringField(this, 15, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateLoraCloudIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateLoraCloudIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateLoraCloudIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateLoraCloudIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.LoraCloudIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateLoraCloudIntegrationRequest}
 */
proto.api.CreateLoraCloudIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateLoraCloudIntegrationRequest;
  return proto.api.CreateLoraCloudIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateLoraCloudIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateLoraCloudIntegrationRequest}
 */
proto.api.CreateLoraCloudIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.LoraCloudIntegration;
      reader.readMessage(value,proto.api.LoraCloudIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateLoraCloudIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateLoraCloudIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateLoraCloudIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateLoraCloudIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.LoraCloudIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional LoraCloudIntegration integration = 1;
 * @return {?proto.api.LoraCloudIntegration}
 */
proto.api.CreateLoraCloudIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.LoraCloudIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.LoraCloudIntegration, 1));
};


/**
 * @param {?proto.api.LoraCloudIntegration|undefined} value
 * @return {!proto.api.CreateLoraCloudIntegrationRequest} returns this
*/
proto.api.CreateLoraCloudIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateLoraCloudIntegrationRequest} returns this
 */
proto.api.CreateLoraCloudIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateLoraCloudIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetLoraCloudIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetLoraCloudIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetLoraCloudIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetLoraCloudIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetLoraCloudIntegrationRequest}
 */
proto.api.GetLoraCloudIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetLoraCloudIntegrationRequest;
  return proto.api.GetLoraCloudIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetLoraCloudIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetLoraCloudIntegrationRequest}
 */
proto.api.GetLoraCloudIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetLoraCloudIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetLoraCloudIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetLoraCloudIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetLoraCloudIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetLoraCloudIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetLoraCloudIntegrationRequest} returns this
 */
proto.api.GetLoraCloudIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetLoraCloudIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetLoraCloudIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetLoraCloudIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetLoraCloudIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.LoraCloudIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetLoraCloudIntegrationResponse}
 */
proto.api.GetLoraCloudIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetLoraCloudIntegrationResponse;
  return proto.api.GetLoraCloudIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetLoraCloudIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetLoraCloudIntegrationResponse}
 */
proto.api.GetLoraCloudIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.LoraCloudIntegration;
      reader.readMessage(value,proto.api.LoraCloudIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetLoraCloudIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetLoraCloudIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetLoraCloudIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetLoraCloudIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.LoraCloudIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional LoraCloudIntegration integration = 1;
 * @return {?proto.api.LoraCloudIntegration}
 */
proto.api.GetLoraCloudIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.LoraCloudIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.LoraCloudIntegration, 1));
};


/**
 * @param {?proto.api.LoraCloudIntegration|undefined} value
 * @return {!proto.api.GetLoraCloudIntegrationResponse} returns this
*/
proto.api.GetLoraCloudIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetLoraCloudIntegrationResponse} returns this
 */
proto.api.GetLoraCloudIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetLoraCloudIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateLoraCloudIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateLoraCloudIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateLoraCloudIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateLoraCloudIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.LoraCloudIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateLoraCloudIntegrationRequest}
 */
proto.api.UpdateLoraCloudIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateLoraCloudIntegrationRequest;
  return proto.api.UpdateLoraCloudIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateLoraCloudIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateLoraCloudIntegrationRequest}
 */
proto.api.UpdateLoraCloudIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.LoraCloudIntegration;
      reader.readMessage(value,proto.api.LoraCloudIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateLoraCloudIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateLoraCloudIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateLoraCloudIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateLoraCloudIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.LoraCloudIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional LoraCloudIntegration integration = 1;
 * @return {?proto.api.LoraCloudIntegration}
 */
proto.api.UpdateLoraCloudIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.LoraCloudIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.LoraCloudIntegration, 1));
};


/**
 * @param {?proto.api.LoraCloudIntegration|undefined} value
 * @return {!proto.api.UpdateLoraCloudIntegrationRequest} returns this
*/
proto.api.UpdateLoraCloudIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateLoraCloudIntegrationRequest} returns this
 */
proto.api.UpdateLoraCloudIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateLoraCloudIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteLoraCloudIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteLoraCloudIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteLoraCloudIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteLoraCloudIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteLoraCloudIntegrationRequest}
 */
proto.api.DeleteLoraCloudIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteLoraCloudIntegrationRequest;
  return proto.api.DeleteLoraCloudIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteLoraCloudIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteLoraCloudIntegrationRequest}
 */
proto.api.DeleteLoraCloudIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteLoraCloudIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteLoraCloudIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteLoraCloudIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteLoraCloudIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteLoraCloudIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteLoraCloudIntegrationRequest} returns this
 */
proto.api.DeleteLoraCloudIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GcpPubSubIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GcpPubSubIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GcpPubSubIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GcpPubSubIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    encoding: jspb.Message.getFieldWithDefault(msg, 2, 0),
    credentialsFile: jspb.Message.getFieldWithDefault(msg, 3, ""),
    projectId: jspb.Message.getFieldWithDefault(msg, 4, ""),
    topicName: jspb.Message.getFieldWithDefault(msg, 5, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GcpPubSubIntegration}
 */
proto.api.GcpPubSubIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GcpPubSubIntegration;
  return proto.api.GcpPubSubIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GcpPubSubIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GcpPubSubIntegration}
 */
proto.api.GcpPubSubIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {!proto.api.Encoding} */ (reader.readEnum());
      msg.setEncoding(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setCredentialsFile(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setProjectId(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setTopicName(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GcpPubSubIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GcpPubSubIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GcpPubSubIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GcpPubSubIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEncoding();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getCredentialsFile();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getProjectId();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getTopicName();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GcpPubSubIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GcpPubSubIntegration} returns this
 */
proto.api.GcpPubSubIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional Encoding encoding = 2;
 * @return {!proto.api.Encoding}
 */
proto.api.GcpPubSubIntegration.prototype.getEncoding = function() {
  return /** @type {!proto.api.Encoding} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.api.Encoding} value
 * @return {!proto.api.GcpPubSubIntegration} returns this
 */
proto.api.GcpPubSubIntegration.prototype.setEncoding = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string credentials_file = 3;
 * @return {string}
 */
proto.api.GcpPubSubIntegration.prototype.getCredentialsFile = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GcpPubSubIntegration} returns this
 */
proto.api.GcpPubSubIntegration.prototype.setCredentialsFile = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string project_id = 4;
 * @return {string}
 */
proto.api.GcpPubSubIntegration.prototype.getProjectId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GcpPubSubIntegration} returns this
 */
proto.api.GcpPubSubIntegration.prototype.setProjectId = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string topic_name = 5;
 * @return {string}
 */
proto.api.GcpPubSubIntegration.prototype.getTopicName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GcpPubSubIntegration} returns this
 */
proto.api.GcpPubSubIntegration.prototype.setTopicName = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateGcpPubSubIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateGcpPubSubIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateGcpPubSubIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateGcpPubSubIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.GcpPubSubIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateGcpPubSubIntegrationRequest}
 */
proto.api.CreateGcpPubSubIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateGcpPubSubIntegrationRequest;
  return proto.api.CreateGcpPubSubIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateGcpPubSubIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateGcpPubSubIntegrationRequest}
 */
proto.api.CreateGcpPubSubIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.GcpPubSubIntegration;
      reader.readMessage(value,proto.api.GcpPubSubIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateGcpPubSubIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateGcpPubSubIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateGcpPubSubIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateGcpPubSubIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.GcpPubSubIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional GcpPubSubIntegration integration = 1;
 * @return {?proto.api.GcpPubSubIntegration}
 */
proto.api.CreateGcpPubSubIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.GcpPubSubIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.GcpPubSubIntegration, 1));
};


/**
 * @param {?proto.api.GcpPubSubIntegration|undefined} value
 * @return {!proto.api.CreateGcpPubSubIntegrationRequest} returns this
*/
proto.api.CreateGcpPubSubIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateGcpPubSubIntegrationRequest} returns this
 */
proto.api.CreateGcpPubSubIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateGcpPubSubIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetGcpPubSubIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetGcpPubSubIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetGcpPubSubIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetGcpPubSubIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetGcpPubSubIntegrationRequest}
 */
proto.api.GetGcpPubSubIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetGcpPubSubIntegrationRequest;
  return proto.api.GetGcpPubSubIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetGcpPubSubIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetGcpPubSubIntegrationRequest}
 */
proto.api.GetGcpPubSubIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetGcpPubSubIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetGcpPubSubIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetGcpPubSubIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetGcpPubSubIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetGcpPubSubIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetGcpPubSubIntegrationRequest} returns this
 */
proto.api.GetGcpPubSubIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetGcpPubSubIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetGcpPubSubIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetGcpPubSubIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetGcpPubSubIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.GcpPubSubIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetGcpPubSubIntegrationResponse}
 */
proto.api.GetGcpPubSubIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetGcpPubSubIntegrationResponse;
  return proto.api.GetGcpPubSubIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetGcpPubSubIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetGcpPubSubIntegrationResponse}
 */
proto.api.GetGcpPubSubIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.GcpPubSubIntegration;
      reader.readMessage(value,proto.api.GcpPubSubIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetGcpPubSubIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetGcpPubSubIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetGcpPubSubIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetGcpPubSubIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.GcpPubSubIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional GcpPubSubIntegration integration = 1;
 * @return {?proto.api.GcpPubSubIntegration}
 */
proto.api.GetGcpPubSubIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.GcpPubSubIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.GcpPubSubIntegration, 1));
};


/**
 * @param {?proto.api.GcpPubSubIntegration|undefined} value
 * @return {!proto.api.GetGcpPubSubIntegrationResponse} returns this
*/
proto.api.GetGcpPubSubIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetGcpPubSubIntegrationResponse} returns this
 */
proto.api.GetGcpPubSubIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetGcpPubSubIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateGcpPubSubIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateGcpPubSubIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateGcpPubSubIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateGcpPubSubIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.GcpPubSubIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateGcpPubSubIntegrationRequest}
 */
proto.api.UpdateGcpPubSubIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateGcpPubSubIntegrationRequest;
  return proto.api.UpdateGcpPubSubIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateGcpPubSubIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateGcpPubSubIntegrationRequest}
 */
proto.api.UpdateGcpPubSubIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.GcpPubSubIntegration;
      reader.readMessage(value,proto.api.GcpPubSubIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateGcpPubSubIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateGcpPubSubIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateGcpPubSubIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateGcpPubSubIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.GcpPubSubIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional GcpPubSubIntegration integration = 1;
 * @return {?proto.api.GcpPubSubIntegration}
 */
proto.api.UpdateGcpPubSubIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.GcpPubSubIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.GcpPubSubIntegration, 1));
};


/**
 * @param {?proto.api.GcpPubSubIntegration|undefined} value
 * @return {!proto.api.UpdateGcpPubSubIntegrationRequest} returns this
*/
proto.api.UpdateGcpPubSubIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateGcpPubSubIntegrationRequest} returns this
 */
proto.api.UpdateGcpPubSubIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateGcpPubSubIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteGcpPubSubIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteGcpPubSubIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteGcpPubSubIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteGcpPubSubIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteGcpPubSubIntegrationRequest}
 */
proto.api.DeleteGcpPubSubIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteGcpPubSubIntegrationRequest;
  return proto.api.DeleteGcpPubSubIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteGcpPubSubIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteGcpPubSubIntegrationRequest}
 */
proto.api.DeleteGcpPubSubIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteGcpPubSubIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteGcpPubSubIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteGcpPubSubIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteGcpPubSubIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteGcpPubSubIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteGcpPubSubIntegrationRequest} returns this
 */
proto.api.DeleteGcpPubSubIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.AwsSnsIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.AwsSnsIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.AwsSnsIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.AwsSnsIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    encoding: jspb.Message.getFieldWithDefault(msg, 2, 0),
    region: jspb.Message.getFieldWithDefault(msg, 3, ""),
    accessKeyId: jspb.Message.getFieldWithDefault(msg, 4, ""),
    secretAccessKey: jspb.Message.getFieldWithDefault(msg, 5, ""),
    topicArn: jspb.Message.getFieldWithDefault(msg, 6, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.AwsSnsIntegration}
 */
proto.api.AwsSnsIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.AwsSnsIntegration;
  return proto.api.AwsSnsIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.AwsSnsIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.AwsSnsIntegration}
 */
proto.api.AwsSnsIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {!proto.api.Encoding} */ (reader.readEnum());
      msg.setEncoding(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setRegion(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setAccessKeyId(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setSecretAccessKey(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setTopicArn(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.AwsSnsIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.AwsSnsIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.AwsSnsIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.AwsSnsIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEncoding();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getRegion();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getAccessKeyId();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getSecretAccessKey();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = message.getTopicArn();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.AwsSnsIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AwsSnsIntegration} returns this
 */
proto.api.AwsSnsIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional Encoding encoding = 2;
 * @return {!proto.api.Encoding}
 */
proto.api.AwsSnsIntegration.prototype.getEncoding = function() {
  return /** @type {!proto.api.Encoding} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.api.Encoding} value
 * @return {!proto.api.AwsSnsIntegration} returns this
 */
proto.api.AwsSnsIntegration.prototype.setEncoding = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string region = 3;
 * @return {string}
 */
proto.api.AwsSnsIntegration.prototype.getRegion = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AwsSnsIntegration} returns this
 */
proto.api.AwsSnsIntegration.prototype.setRegion = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string access_key_id = 4;
 * @return {string}
 */
proto.api.AwsSnsIntegration.prototype.getAccessKeyId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AwsSnsIntegration} returns this
 */
proto.api.AwsSnsIntegration.prototype.setAccessKeyId = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string secret_access_key = 5;
 * @return {string}
 */
proto.api.AwsSnsIntegration.prototype.getSecretAccessKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AwsSnsIntegration} returns this
 */
proto.api.AwsSnsIntegration.prototype.setSecretAccessKey = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional string topic_arn = 6;
 * @return {string}
 */
proto.api.AwsSnsIntegration.prototype.getTopicArn = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AwsSnsIntegration} returns this
 */
proto.api.AwsSnsIntegration.prototype.setTopicArn = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateAwsSnsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateAwsSnsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateAwsSnsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateAwsSnsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.AwsSnsIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateAwsSnsIntegrationRequest}
 */
proto.api.CreateAwsSnsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateAwsSnsIntegrationRequest;
  return proto.api.CreateAwsSnsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateAwsSnsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateAwsSnsIntegrationRequest}
 */
proto.api.CreateAwsSnsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.AwsSnsIntegration;
      reader.readMessage(value,proto.api.AwsSnsIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateAwsSnsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateAwsSnsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateAwsSnsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateAwsSnsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.AwsSnsIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional AwsSnsIntegration integration = 1;
 * @return {?proto.api.AwsSnsIntegration}
 */
proto.api.CreateAwsSnsIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.AwsSnsIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.AwsSnsIntegration, 1));
};


/**
 * @param {?proto.api.AwsSnsIntegration|undefined} value
 * @return {!proto.api.CreateAwsSnsIntegrationRequest} returns this
*/
proto.api.CreateAwsSnsIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateAwsSnsIntegrationRequest} returns this
 */
proto.api.CreateAwsSnsIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateAwsSnsIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetAwsSnsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetAwsSnsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetAwsSnsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAwsSnsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetAwsSnsIntegrationRequest}
 */
proto.api.GetAwsSnsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetAwsSnsIntegrationRequest;
  return proto.api.GetAwsSnsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetAwsSnsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetAwsSnsIntegrationRequest}
 */
proto.api.GetAwsSnsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetAwsSnsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetAwsSnsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetAwsSnsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAwsSnsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetAwsSnsIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetAwsSnsIntegrationRequest} returns this
 */
proto.api.GetAwsSnsIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetAwsSnsIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetAwsSnsIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetAwsSnsIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAwsSnsIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.AwsSnsIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetAwsSnsIntegrationResponse}
 */
proto.api.GetAwsSnsIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetAwsSnsIntegrationResponse;
  return proto.api.GetAwsSnsIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetAwsSnsIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetAwsSnsIntegrationResponse}
 */
proto.api.GetAwsSnsIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.AwsSnsIntegration;
      reader.readMessage(value,proto.api.AwsSnsIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetAwsSnsIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetAwsSnsIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetAwsSnsIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAwsSnsIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.AwsSnsIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional AwsSnsIntegration integration = 1;
 * @return {?proto.api.AwsSnsIntegration}
 */
proto.api.GetAwsSnsIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.AwsSnsIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.AwsSnsIntegration, 1));
};


/**
 * @param {?proto.api.AwsSnsIntegration|undefined} value
 * @return {!proto.api.GetAwsSnsIntegrationResponse} returns this
*/
proto.api.GetAwsSnsIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetAwsSnsIntegrationResponse} returns this
 */
proto.api.GetAwsSnsIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetAwsSnsIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateAwsSnsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateAwsSnsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateAwsSnsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateAwsSnsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.AwsSnsIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateAwsSnsIntegrationRequest}
 */
proto.api.UpdateAwsSnsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateAwsSnsIntegrationRequest;
  return proto.api.UpdateAwsSnsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateAwsSnsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateAwsSnsIntegrationRequest}
 */
proto.api.UpdateAwsSnsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.AwsSnsIntegration;
      reader.readMessage(value,proto.api.AwsSnsIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateAwsSnsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateAwsSnsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateAwsSnsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateAwsSnsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.AwsSnsIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional AwsSnsIntegration integration = 1;
 * @return {?proto.api.AwsSnsIntegration}
 */
proto.api.UpdateAwsSnsIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.AwsSnsIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.AwsSnsIntegration, 1));
};


/**
 * @param {?proto.api.AwsSnsIntegration|undefined} value
 * @return {!proto.api.UpdateAwsSnsIntegrationRequest} returns this
*/
proto.api.UpdateAwsSnsIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateAwsSnsIntegrationRequest} returns this
 */
proto.api.UpdateAwsSnsIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateAwsSnsIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteAwsSnsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteAwsSnsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteAwsSnsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteAwsSnsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteAwsSnsIntegrationRequest}
 */
proto.api.DeleteAwsSnsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteAwsSnsIntegrationRequest;
  return proto.api.DeleteAwsSnsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteAwsSnsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteAwsSnsIntegrationRequest}
 */
proto.api.DeleteAwsSnsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteAwsSnsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteAwsSnsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteAwsSnsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteAwsSnsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteAwsSnsIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteAwsSnsIntegrationRequest} returns this
 */
proto.api.DeleteAwsSnsIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.AzureServiceBusIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.AzureServiceBusIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.AzureServiceBusIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.AzureServiceBusIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    encoding: jspb.Message.getFieldWithDefault(msg, 2, 0),
    connectionString: jspb.Message.getFieldWithDefault(msg, 3, ""),
    publishName: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.AzureServiceBusIntegration}
 */
proto.api.AzureServiceBusIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.AzureServiceBusIntegration;
  return proto.api.AzureServiceBusIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.AzureServiceBusIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.AzureServiceBusIntegration}
 */
proto.api.AzureServiceBusIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {!proto.api.Encoding} */ (reader.readEnum());
      msg.setEncoding(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setConnectionString(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setPublishName(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.AzureServiceBusIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.AzureServiceBusIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.AzureServiceBusIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.AzureServiceBusIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEncoding();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getConnectionString();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getPublishName();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.AzureServiceBusIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AzureServiceBusIntegration} returns this
 */
proto.api.AzureServiceBusIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional Encoding encoding = 2;
 * @return {!proto.api.Encoding}
 */
proto.api.AzureServiceBusIntegration.prototype.getEncoding = function() {
  return /** @type {!proto.api.Encoding} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.api.Encoding} value
 * @return {!proto.api.AzureServiceBusIntegration} returns this
 */
proto.api.AzureServiceBusIntegration.prototype.setEncoding = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string connection_string = 3;
 * @return {string}
 */
proto.api.AzureServiceBusIntegration.prototype.getConnectionString = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AzureServiceBusIntegration} returns this
 */
proto.api.AzureServiceBusIntegration.prototype.setConnectionString = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string publish_name = 4;
 * @return {string}
 */
proto.api.AzureServiceBusIntegration.prototype.getPublishName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.AzureServiceBusIntegration} returns this
 */
proto.api.AzureServiceBusIntegration.prototype.setPublishName = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateAzureServiceBusIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateAzureServiceBusIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateAzureServiceBusIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateAzureServiceBusIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.AzureServiceBusIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateAzureServiceBusIntegrationRequest}
 */
proto.api.CreateAzureServiceBusIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateAzureServiceBusIntegrationRequest;
  return proto.api.CreateAzureServiceBusIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateAzureServiceBusIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateAzureServiceBusIntegrationRequest}
 */
proto.api.CreateAzureServiceBusIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.AzureServiceBusIntegration;
      reader.readMessage(value,proto.api.AzureServiceBusIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateAzureServiceBusIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateAzureServiceBusIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateAzureServiceBusIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateAzureServiceBusIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.AzureServiceBusIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional AzureServiceBusIntegration integration = 1;
 * @return {?proto.api.AzureServiceBusIntegration}
 */
proto.api.CreateAzureServiceBusIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.AzureServiceBusIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.AzureServiceBusIntegration, 1));
};


/**
 * @param {?proto.api.AzureServiceBusIntegration|undefined} value
 * @return {!proto.api.CreateAzureServiceBusIntegrationRequest} returns this
*/
proto.api.CreateAzureServiceBusIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateAzureServiceBusIntegrationRequest} returns this
 */
proto.api.CreateAzureServiceBusIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateAzureServiceBusIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetAzureServiceBusIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetAzureServiceBusIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetAzureServiceBusIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAzureServiceBusIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetAzureServiceBusIntegrationRequest}
 */
proto.api.GetAzureServiceBusIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetAzureServiceBusIntegrationRequest;
  return proto.api.GetAzureServiceBusIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetAzureServiceBusIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetAzureServiceBusIntegrationRequest}
 */
proto.api.GetAzureServiceBusIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetAzureServiceBusIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetAzureServiceBusIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetAzureServiceBusIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAzureServiceBusIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetAzureServiceBusIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetAzureServiceBusIntegrationRequest} returns this
 */
proto.api.GetAzureServiceBusIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetAzureServiceBusIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetAzureServiceBusIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetAzureServiceBusIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAzureServiceBusIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.AzureServiceBusIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetAzureServiceBusIntegrationResponse}
 */
proto.api.GetAzureServiceBusIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetAzureServiceBusIntegrationResponse;
  return proto.api.GetAzureServiceBusIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetAzureServiceBusIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetAzureServiceBusIntegrationResponse}
 */
proto.api.GetAzureServiceBusIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.AzureServiceBusIntegration;
      reader.readMessage(value,proto.api.AzureServiceBusIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetAzureServiceBusIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetAzureServiceBusIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetAzureServiceBusIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetAzureServiceBusIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.AzureServiceBusIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional AzureServiceBusIntegration integration = 1;
 * @return {?proto.api.AzureServiceBusIntegration}
 */
proto.api.GetAzureServiceBusIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.AzureServiceBusIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.AzureServiceBusIntegration, 1));
};


/**
 * @param {?proto.api.AzureServiceBusIntegration|undefined} value
 * @return {!proto.api.GetAzureServiceBusIntegrationResponse} returns this
*/
proto.api.GetAzureServiceBusIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetAzureServiceBusIntegrationResponse} returns this
 */
proto.api.GetAzureServiceBusIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetAzureServiceBusIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateAzureServiceBusIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateAzureServiceBusIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.AzureServiceBusIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateAzureServiceBusIntegrationRequest}
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateAzureServiceBusIntegrationRequest;
  return proto.api.UpdateAzureServiceBusIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateAzureServiceBusIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateAzureServiceBusIntegrationRequest}
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.AzureServiceBusIntegration;
      reader.readMessage(value,proto.api.AzureServiceBusIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateAzureServiceBusIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateAzureServiceBusIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.AzureServiceBusIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional AzureServiceBusIntegration integration = 1;
 * @return {?proto.api.AzureServiceBusIntegration}
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.AzureServiceBusIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.AzureServiceBusIntegration, 1));
};


/**
 * @param {?proto.api.AzureServiceBusIntegration|undefined} value
 * @return {!proto.api.UpdateAzureServiceBusIntegrationRequest} returns this
*/
proto.api.UpdateAzureServiceBusIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateAzureServiceBusIntegrationRequest} returns this
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateAzureServiceBusIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteAzureServiceBusIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteAzureServiceBusIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteAzureServiceBusIntegrationRequest}
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteAzureServiceBusIntegrationRequest;
  return proto.api.DeleteAzureServiceBusIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteAzureServiceBusIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteAzureServiceBusIntegrationRequest}
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteAzureServiceBusIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteAzureServiceBusIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteAzureServiceBusIntegrationRequest} returns this
 */
proto.api.DeleteAzureServiceBusIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.PilotThingsIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.PilotThingsIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.PilotThingsIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.PilotThingsIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    server: jspb.Message.getFieldWithDefault(msg, 2, ""),
    token: jspb.Message.getFieldWithDefault(msg, 3, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.PilotThingsIntegration}
 */
proto.api.PilotThingsIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.PilotThingsIntegration;
  return proto.api.PilotThingsIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.PilotThingsIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.PilotThingsIntegration}
 */
proto.api.PilotThingsIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setServer(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.PilotThingsIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.PilotThingsIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.PilotThingsIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.PilotThingsIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getServer();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.PilotThingsIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.PilotThingsIntegration} returns this
 */
proto.api.PilotThingsIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string server = 2;
 * @return {string}
 */
proto.api.PilotThingsIntegration.prototype.getServer = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.PilotThingsIntegration} returns this
 */
proto.api.PilotThingsIntegration.prototype.setServer = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string token = 3;
 * @return {string}
 */
proto.api.PilotThingsIntegration.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.PilotThingsIntegration} returns this
 */
proto.api.PilotThingsIntegration.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreatePilotThingsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreatePilotThingsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreatePilotThingsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreatePilotThingsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.PilotThingsIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreatePilotThingsIntegrationRequest}
 */
proto.api.CreatePilotThingsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreatePilotThingsIntegrationRequest;
  return proto.api.CreatePilotThingsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreatePilotThingsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreatePilotThingsIntegrationRequest}
 */
proto.api.CreatePilotThingsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.PilotThingsIntegration;
      reader.readMessage(value,proto.api.PilotThingsIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreatePilotThingsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreatePilotThingsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreatePilotThingsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreatePilotThingsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.PilotThingsIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional PilotThingsIntegration integration = 1;
 * @return {?proto.api.PilotThingsIntegration}
 */
proto.api.CreatePilotThingsIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.PilotThingsIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.PilotThingsIntegration, 1));
};


/**
 * @param {?proto.api.PilotThingsIntegration|undefined} value
 * @return {!proto.api.CreatePilotThingsIntegrationRequest} returns this
*/
proto.api.CreatePilotThingsIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreatePilotThingsIntegrationRequest} returns this
 */
proto.api.CreatePilotThingsIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreatePilotThingsIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetPilotThingsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetPilotThingsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetPilotThingsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetPilotThingsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetPilotThingsIntegrationRequest}
 */
proto.api.GetPilotThingsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetPilotThingsIntegrationRequest;
  return proto.api.GetPilotThingsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetPilotThingsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetPilotThingsIntegrationRequest}
 */
proto.api.GetPilotThingsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetPilotThingsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetPilotThingsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetPilotThingsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetPilotThingsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetPilotThingsIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetPilotThingsIntegrationRequest} returns this
 */
proto.api.GetPilotThingsIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetPilotThingsIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetPilotThingsIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetPilotThingsIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetPilotThingsIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.PilotThingsIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetPilotThingsIntegrationResponse}
 */
proto.api.GetPilotThingsIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetPilotThingsIntegrationResponse;
  return proto.api.GetPilotThingsIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetPilotThingsIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetPilotThingsIntegrationResponse}
 */
proto.api.GetPilotThingsIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.PilotThingsIntegration;
      reader.readMessage(value,proto.api.PilotThingsIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetPilotThingsIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetPilotThingsIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetPilotThingsIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetPilotThingsIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.PilotThingsIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional PilotThingsIntegration integration = 1;
 * @return {?proto.api.PilotThingsIntegration}
 */
proto.api.GetPilotThingsIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.PilotThingsIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.PilotThingsIntegration, 1));
};


/**
 * @param {?proto.api.PilotThingsIntegration|undefined} value
 * @return {!proto.api.GetPilotThingsIntegrationResponse} returns this
*/
proto.api.GetPilotThingsIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetPilotThingsIntegrationResponse} returns this
 */
proto.api.GetPilotThingsIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetPilotThingsIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdatePilotThingsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdatePilotThingsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdatePilotThingsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdatePilotThingsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.PilotThingsIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdatePilotThingsIntegrationRequest}
 */
proto.api.UpdatePilotThingsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdatePilotThingsIntegrationRequest;
  return proto.api.UpdatePilotThingsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdatePilotThingsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdatePilotThingsIntegrationRequest}
 */
proto.api.UpdatePilotThingsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.PilotThingsIntegration;
      reader.readMessage(value,proto.api.PilotThingsIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdatePilotThingsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdatePilotThingsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdatePilotThingsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdatePilotThingsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.PilotThingsIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional PilotThingsIntegration integration = 1;
 * @return {?proto.api.PilotThingsIntegration}
 */
proto.api.UpdatePilotThingsIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.PilotThingsIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.PilotThingsIntegration, 1));
};


/**
 * @param {?proto.api.PilotThingsIntegration|undefined} value
 * @return {!proto.api.UpdatePilotThingsIntegrationRequest} returns this
*/
proto.api.UpdatePilotThingsIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdatePilotThingsIntegrationRequest} returns this
 */
proto.api.UpdatePilotThingsIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdatePilotThingsIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeletePilotThingsIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeletePilotThingsIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeletePilotThingsIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeletePilotThingsIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeletePilotThingsIntegrationRequest}
 */
proto.api.DeletePilotThingsIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeletePilotThingsIntegrationRequest;
  return proto.api.DeletePilotThingsIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeletePilotThingsIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeletePilotThingsIntegrationRequest}
 */
proto.api.DeletePilotThingsIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeletePilotThingsIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeletePilotThingsIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeletePilotThingsIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeletePilotThingsIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeletePilotThingsIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeletePilotThingsIntegrationRequest} returns this
 */
proto.api.DeletePilotThingsIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.api.IftttIntegration.repeatedFields_ = [3];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.IftttIntegration.prototype.toObject = function(opt_includeInstance) {
  return proto.api.IftttIntegration.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.IftttIntegration} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.IftttIntegration.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, ""),
    key: jspb.Message.getFieldWithDefault(msg, 2, ""),
    uplinkValuesList: (f = jspb.Message.getRepeatedField(msg, 3)) == null ? undefined : f,
    arbitraryJson: jspb.Message.getBooleanFieldWithDefault(msg, 4, false),
    eventPrefix: jspb.Message.getFieldWithDefault(msg, 5, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.IftttIntegration}
 */
proto.api.IftttIntegration.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.IftttIntegration;
  return proto.api.IftttIntegration.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.IftttIntegration} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.IftttIntegration}
 */
proto.api.IftttIntegration.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setKey(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.addUplinkValues(value);
      break;
    case 4:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setArbitraryJson(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setEventPrefix(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.IftttIntegration.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.IftttIntegration.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.IftttIntegration} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.IftttIntegration.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getUplinkValuesList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      3,
      f
    );
  }
  f = message.getArbitraryJson();
  if (f) {
    writer.writeBool(
      4,
      f
    );
  }
  f = message.getEventPrefix();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.IftttIntegration.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.IftttIntegration} returns this
 */
proto.api.IftttIntegration.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string key = 2;
 * @return {string}
 */
proto.api.IftttIntegration.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.IftttIntegration} returns this
 */
proto.api.IftttIntegration.prototype.setKey = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * repeated string uplink_values = 3;
 * @return {!Array<string>}
 */
proto.api.IftttIntegration.prototype.getUplinkValuesList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 3));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.api.IftttIntegration} returns this
 */
proto.api.IftttIntegration.prototype.setUplinkValuesList = function(value) {
  return jspb.Message.setField(this, 3, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.api.IftttIntegration} returns this
 */
proto.api.IftttIntegration.prototype.addUplinkValues = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 3, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.api.IftttIntegration} returns this
 */
proto.api.IftttIntegration.prototype.clearUplinkValuesList = function() {
  return this.setUplinkValuesList([]);
};


/**
 * optional bool arbitrary_json = 4;
 * @return {boolean}
 */
proto.api.IftttIntegration.prototype.getArbitraryJson = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 4, false));
};


/**
 * @param {boolean} value
 * @return {!proto.api.IftttIntegration} returns this
 */
proto.api.IftttIntegration.prototype.setArbitraryJson = function(value) {
  return jspb.Message.setProto3BooleanField(this, 4, value);
};


/**
 * optional string event_prefix = 5;
 * @return {string}
 */
proto.api.IftttIntegration.prototype.getEventPrefix = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.IftttIntegration} returns this
 */
proto.api.IftttIntegration.prototype.setEventPrefix = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.CreateIftttIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.CreateIftttIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.CreateIftttIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateIftttIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.IftttIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.CreateIftttIntegrationRequest}
 */
proto.api.CreateIftttIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.CreateIftttIntegrationRequest;
  return proto.api.CreateIftttIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.CreateIftttIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.CreateIftttIntegrationRequest}
 */
proto.api.CreateIftttIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.IftttIntegration;
      reader.readMessage(value,proto.api.IftttIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.CreateIftttIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.CreateIftttIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.CreateIftttIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.CreateIftttIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.IftttIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional IftttIntegration integration = 1;
 * @return {?proto.api.IftttIntegration}
 */
proto.api.CreateIftttIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.IftttIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.IftttIntegration, 1));
};


/**
 * @param {?proto.api.IftttIntegration|undefined} value
 * @return {!proto.api.CreateIftttIntegrationRequest} returns this
*/
proto.api.CreateIftttIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.CreateIftttIntegrationRequest} returns this
 */
proto.api.CreateIftttIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.CreateIftttIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetIftttIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetIftttIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetIftttIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetIftttIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetIftttIntegrationRequest}
 */
proto.api.GetIftttIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetIftttIntegrationRequest;
  return proto.api.GetIftttIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetIftttIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetIftttIntegrationRequest}
 */
proto.api.GetIftttIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetIftttIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetIftttIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetIftttIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetIftttIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GetIftttIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GetIftttIntegrationRequest} returns this
 */
proto.api.GetIftttIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GetIftttIntegrationResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GetIftttIntegrationResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GetIftttIntegrationResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetIftttIntegrationResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.IftttIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GetIftttIntegrationResponse}
 */
proto.api.GetIftttIntegrationResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GetIftttIntegrationResponse;
  return proto.api.GetIftttIntegrationResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GetIftttIntegrationResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GetIftttIntegrationResponse}
 */
proto.api.GetIftttIntegrationResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.IftttIntegration;
      reader.readMessage(value,proto.api.IftttIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GetIftttIntegrationResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GetIftttIntegrationResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GetIftttIntegrationResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GetIftttIntegrationResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.IftttIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional IftttIntegration integration = 1;
 * @return {?proto.api.IftttIntegration}
 */
proto.api.GetIftttIntegrationResponse.prototype.getIntegration = function() {
  return /** @type{?proto.api.IftttIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.IftttIntegration, 1));
};


/**
 * @param {?proto.api.IftttIntegration|undefined} value
 * @return {!proto.api.GetIftttIntegrationResponse} returns this
*/
proto.api.GetIftttIntegrationResponse.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GetIftttIntegrationResponse} returns this
 */
proto.api.GetIftttIntegrationResponse.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GetIftttIntegrationResponse.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.UpdateIftttIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.UpdateIftttIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.UpdateIftttIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateIftttIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    integration: (f = msg.getIntegration()) && proto.api.IftttIntegration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.UpdateIftttIntegrationRequest}
 */
proto.api.UpdateIftttIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.UpdateIftttIntegrationRequest;
  return proto.api.UpdateIftttIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.UpdateIftttIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.UpdateIftttIntegrationRequest}
 */
proto.api.UpdateIftttIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.api.IftttIntegration;
      reader.readMessage(value,proto.api.IftttIntegration.deserializeBinaryFromReader);
      msg.setIntegration(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.UpdateIftttIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.UpdateIftttIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.UpdateIftttIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.UpdateIftttIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIntegration();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.api.IftttIntegration.serializeBinaryToWriter
    );
  }
};


/**
 * optional IftttIntegration integration = 1;
 * @return {?proto.api.IftttIntegration}
 */
proto.api.UpdateIftttIntegrationRequest.prototype.getIntegration = function() {
  return /** @type{?proto.api.IftttIntegration} */ (
    jspb.Message.getWrapperField(this, proto.api.IftttIntegration, 1));
};


/**
 * @param {?proto.api.IftttIntegration|undefined} value
 * @return {!proto.api.UpdateIftttIntegrationRequest} returns this
*/
proto.api.UpdateIftttIntegrationRequest.prototype.setIntegration = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.UpdateIftttIntegrationRequest} returns this
 */
proto.api.UpdateIftttIntegrationRequest.prototype.clearIntegration = function() {
  return this.setIntegration(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.UpdateIftttIntegrationRequest.prototype.hasIntegration = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.DeleteIftttIntegrationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.DeleteIftttIntegrationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.DeleteIftttIntegrationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteIftttIntegrationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.DeleteIftttIntegrationRequest}
 */
proto.api.DeleteIftttIntegrationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.DeleteIftttIntegrationRequest;
  return proto.api.DeleteIftttIntegrationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.DeleteIftttIntegrationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.DeleteIftttIntegrationRequest}
 */
proto.api.DeleteIftttIntegrationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.DeleteIftttIntegrationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.DeleteIftttIntegrationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.DeleteIftttIntegrationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.DeleteIftttIntegrationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.DeleteIftttIntegrationRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.DeleteIftttIntegrationRequest} returns this
 */
proto.api.DeleteIftttIntegrationRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GenerateMqttIntegrationClientCertificateRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GenerateMqttIntegrationClientCertificateRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    applicationId: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateRequest}
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GenerateMqttIntegrationClientCertificateRequest;
  return proto.api.GenerateMqttIntegrationClientCertificateRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GenerateMqttIntegrationClientCertificateRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateRequest}
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setApplicationId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GenerateMqttIntegrationClientCertificateRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GenerateMqttIntegrationClientCertificateRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getApplicationId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string application_id = 1;
 * @return {string}
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.prototype.getApplicationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateRequest} returns this
 */
proto.api.GenerateMqttIntegrationClientCertificateRequest.prototype.setApplicationId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.api.GenerateMqttIntegrationClientCertificateResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.api.GenerateMqttIntegrationClientCertificateResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    tlsCert: jspb.Message.getFieldWithDefault(msg, 1, ""),
    tlsKey: jspb.Message.getFieldWithDefault(msg, 2, ""),
    caCert: jspb.Message.getFieldWithDefault(msg, 3, ""),
    expiresAt: (f = msg.getExpiresAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateResponse}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.api.GenerateMqttIntegrationClientCertificateResponse;
  return proto.api.GenerateMqttIntegrationClientCertificateResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.api.GenerateMqttIntegrationClientCertificateResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateResponse}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setTlsCert(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setTlsKey(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setCaCert(value);
      break;
    case 4:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setExpiresAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.api.GenerateMqttIntegrationClientCertificateResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.api.GenerateMqttIntegrationClientCertificateResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTlsCert();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getTlsKey();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getCaCert();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getExpiresAt();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional string tls_cert = 1;
 * @return {string}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.getTlsCert = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateResponse} returns this
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.setTlsCert = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string tls_key = 2;
 * @return {string}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.getTlsKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateResponse} returns this
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.setTlsKey = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string ca_cert = 3;
 * @return {string}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.getCaCert = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateResponse} returns this
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.setCaCert = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional google.protobuf.Timestamp expires_at = 4;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.getExpiresAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 4));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateResponse} returns this
*/
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.setExpiresAt = function(value) {
  return jspb.Message.setWrapperField(this, 4, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.api.GenerateMqttIntegrationClientCertificateResponse} returns this
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.clearExpiresAt = function() {
  return this.setExpiresAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.api.GenerateMqttIntegrationClientCertificateResponse.prototype.hasExpiresAt = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * @enum {number}
 */
proto.api.Encoding = {
  JSON: 0,
  PROTOBUF: 1
};

/**
 * @enum {number}
 */
proto.api.IntegrationKind = {
  HTTP: 0,
  INFLUX_DB: 1,
  THINGS_BOARD: 2,
  MY_DEVICES: 3,
  LORA_CLOUD: 4,
  GCP_PUB_SUB: 5,
  AWS_SNS: 6,
  AZURE_SERVICE_BUS: 7,
  PILOT_THINGS: 8,
  MQTT_GLOBAL: 9,
  IFTTT: 10
};

/**
 * @enum {number}
 */
proto.api.InfluxDbPrecision = {
  NS: 0,
  U: 1,
  MS: 2,
  S: 3,
  M: 4,
  H: 5
};

/**
 * @enum {number}
 */
proto.api.InfluxDbVersion = {
  INFLUXDB_1: 0,
  INFLUXDB_2: 1
};

goog.object.extend(exports, proto.api);
