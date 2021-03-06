/**
 * @fileoverview
 * @enhanceable
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

goog.exportSymbol('proto.google.api.Monitoring', null, global);
goog.exportSymbol('proto.google.api.Monitoring.MonitoringDestination', null, global);

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
proto.google.api.Monitoring = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.google.api.Monitoring.repeatedFields_, null);
};
goog.inherits(proto.google.api.Monitoring, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.google.api.Monitoring.displayName = 'proto.google.api.Monitoring';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.google.api.Monitoring.repeatedFields_ = [1,2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.google.api.Monitoring.prototype.toObject = function(opt_includeInstance) {
  return proto.google.api.Monitoring.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.google.api.Monitoring} msg The msg instance to transform.
 * @return {!Object}
 */
proto.google.api.Monitoring.toObject = function(includeInstance, msg) {
  var f, obj = {
    producerDestinationsList: jspb.Message.toObjectList(msg.getProducerDestinationsList(),
    proto.google.api.Monitoring.MonitoringDestination.toObject, includeInstance),
    consumerDestinationsList: jspb.Message.toObjectList(msg.getConsumerDestinationsList(),
    proto.google.api.Monitoring.MonitoringDestination.toObject, includeInstance)
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
 * @return {!proto.google.api.Monitoring}
 */
proto.google.api.Monitoring.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.google.api.Monitoring;
  return proto.google.api.Monitoring.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.google.api.Monitoring} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.google.api.Monitoring}
 */
proto.google.api.Monitoring.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.google.api.Monitoring.MonitoringDestination;
      reader.readMessage(value,proto.google.api.Monitoring.MonitoringDestination.deserializeBinaryFromReader);
      msg.getProducerDestinationsList().push(value);
      msg.setProducerDestinationsList(msg.getProducerDestinationsList());
      break;
    case 2:
      var value = new proto.google.api.Monitoring.MonitoringDestination;
      reader.readMessage(value,proto.google.api.Monitoring.MonitoringDestination.deserializeBinaryFromReader);
      msg.getConsumerDestinationsList().push(value);
      msg.setConsumerDestinationsList(msg.getConsumerDestinationsList());
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.google.api.Monitoring} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.Monitoring.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.google.api.Monitoring.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.Monitoring.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getProducerDestinationsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.google.api.Monitoring.MonitoringDestination.serializeBinaryToWriter
    );
  }
  f = this.getConsumerDestinationsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.google.api.Monitoring.MonitoringDestination.serializeBinaryToWriter
    );
  }
};


/**
 * Creates a deep clone of this proto. No data is shared with the original.
 * @return {!proto.google.api.Monitoring} The clone.
 */
proto.google.api.Monitoring.prototype.cloneMessage = function() {
  return /** @type {!proto.google.api.Monitoring} */ (jspb.Message.cloneMessage(this));
};


/**
 * repeated MonitoringDestination producer_destinations = 1;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.google.api.Monitoring.MonitoringDestination>}
 */
proto.google.api.Monitoring.prototype.getProducerDestinationsList = function() {
  return /** @type{!Array.<!proto.google.api.Monitoring.MonitoringDestination>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.google.api.Monitoring.MonitoringDestination, 1));
};


/** @param {Array.<!proto.google.api.Monitoring.MonitoringDestination>} value  */
proto.google.api.Monitoring.prototype.setProducerDestinationsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 1, value);
};


proto.google.api.Monitoring.prototype.clearProducerDestinationsList = function() {
  this.setProducerDestinationsList([]);
};


/**
 * repeated MonitoringDestination consumer_destinations = 2;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.google.api.Monitoring.MonitoringDestination>}
 */
proto.google.api.Monitoring.prototype.getConsumerDestinationsList = function() {
  return /** @type{!Array.<!proto.google.api.Monitoring.MonitoringDestination>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.google.api.Monitoring.MonitoringDestination, 2));
};


/** @param {Array.<!proto.google.api.Monitoring.MonitoringDestination>} value  */
proto.google.api.Monitoring.prototype.setConsumerDestinationsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 2, value);
};


proto.google.api.Monitoring.prototype.clearConsumerDestinationsList = function() {
  this.setConsumerDestinationsList([]);
};



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
proto.google.api.Monitoring.MonitoringDestination = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.google.api.Monitoring.MonitoringDestination.repeatedFields_, null);
};
goog.inherits(proto.google.api.Monitoring.MonitoringDestination, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.google.api.Monitoring.MonitoringDestination.displayName = 'proto.google.api.Monitoring.MonitoringDestination';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.google.api.Monitoring.MonitoringDestination.repeatedFields_ = [2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.google.api.Monitoring.MonitoringDestination.prototype.toObject = function(opt_includeInstance) {
  return proto.google.api.Monitoring.MonitoringDestination.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.google.api.Monitoring.MonitoringDestination} msg The msg instance to transform.
 * @return {!Object}
 */
proto.google.api.Monitoring.MonitoringDestination.toObject = function(includeInstance, msg) {
  var f, obj = {
    monitoredResource: msg.getMonitoredResource(),
    metricsList: jspb.Message.getField(msg, 2)
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
 * @return {!proto.google.api.Monitoring.MonitoringDestination}
 */
proto.google.api.Monitoring.MonitoringDestination.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.google.api.Monitoring.MonitoringDestination;
  return proto.google.api.Monitoring.MonitoringDestination.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.google.api.Monitoring.MonitoringDestination} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.google.api.Monitoring.MonitoringDestination}
 */
proto.google.api.Monitoring.MonitoringDestination.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setMonitoredResource(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.getMetricsList().push(value);
      msg.setMetricsList(msg.getMetricsList());
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.google.api.Monitoring.MonitoringDestination} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.Monitoring.MonitoringDestination.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.google.api.Monitoring.MonitoringDestination.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.Monitoring.MonitoringDestination.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getMonitoredResource();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = this.getMetricsList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      2,
      f
    );
  }
};


/**
 * Creates a deep clone of this proto. No data is shared with the original.
 * @return {!proto.google.api.Monitoring.MonitoringDestination} The clone.
 */
proto.google.api.Monitoring.MonitoringDestination.prototype.cloneMessage = function() {
  return /** @type {!proto.google.api.Monitoring.MonitoringDestination} */ (jspb.Message.cloneMessage(this));
};


/**
 * optional string monitored_resource = 1;
 * @return {string}
 */
proto.google.api.Monitoring.MonitoringDestination.prototype.getMonitoredResource = function() {
  return /** @type {string} */ (jspb.Message.getFieldProto3(this, 1, ""));
};


/** @param {string} value  */
proto.google.api.Monitoring.MonitoringDestination.prototype.setMonitoredResource = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * repeated string metrics = 2;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<string>}
 */
proto.google.api.Monitoring.MonitoringDestination.prototype.getMetricsList = function() {
  return /** @type {!Array.<string>} */ (jspb.Message.getField(this, 2));
};


/** @param {Array.<string>} value  */
proto.google.api.Monitoring.MonitoringDestination.prototype.setMetricsList = function(value) {
  jspb.Message.setField(this, 2, value || []);
};


proto.google.api.Monitoring.MonitoringDestination.prototype.clearMetricsList = function() {
  jspb.Message.setField(this, 2, []);
};


goog.object.extend(exports, proto.google.api);
