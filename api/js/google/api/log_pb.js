/**
 * @fileoverview
 * @enhanceable
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

var google_api_label_pb = require('../../google/api/label_pb.js');
goog.exportSymbol('proto.google.api.LogDescriptor', null, global);

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
proto.google.api.LogDescriptor = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.google.api.LogDescriptor.repeatedFields_, null);
};
goog.inherits(proto.google.api.LogDescriptor, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.google.api.LogDescriptor.displayName = 'proto.google.api.LogDescriptor';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.google.api.LogDescriptor.repeatedFields_ = [2];



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
proto.google.api.LogDescriptor.prototype.toObject = function(opt_includeInstance) {
  return proto.google.api.LogDescriptor.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.google.api.LogDescriptor} msg The msg instance to transform.
 * @return {!Object}
 */
proto.google.api.LogDescriptor.toObject = function(includeInstance, msg) {
  var f, obj = {
    name: msg.getName(),
    labelsList: jspb.Message.toObjectList(msg.getLabelsList(),
    google_api_label_pb.LabelDescriptor.toObject, includeInstance),
    description: msg.getDescription(),
    displayName: msg.getDisplayName()
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
 * @return {!proto.google.api.LogDescriptor}
 */
proto.google.api.LogDescriptor.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.google.api.LogDescriptor;
  return proto.google.api.LogDescriptor.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.google.api.LogDescriptor} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.google.api.LogDescriptor}
 */
proto.google.api.LogDescriptor.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 2:
      var value = new google_api_label_pb.LabelDescriptor;
      reader.readMessage(value,google_api_label_pb.LabelDescriptor.deserializeBinaryFromReader);
      msg.getLabelsList().push(value);
      msg.setLabelsList(msg.getLabelsList());
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setDescription(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setDisplayName(value);
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
 * @param {!proto.google.api.LogDescriptor} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.LogDescriptor.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.google.api.LogDescriptor.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.LogDescriptor.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = this.getLabelsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      google_api_label_pb.LabelDescriptor.serializeBinaryToWriter
    );
  }
  f = this.getDescription();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = this.getDisplayName();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * Creates a deep clone of this proto. No data is shared with the original.
 * @return {!proto.google.api.LogDescriptor} The clone.
 */
proto.google.api.LogDescriptor.prototype.cloneMessage = function() {
  return /** @type {!proto.google.api.LogDescriptor} */ (jspb.Message.cloneMessage(this));
};


/**
 * optional string name = 1;
 * @return {string}
 */
proto.google.api.LogDescriptor.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldProto3(this, 1, ""));
};


/** @param {string} value  */
proto.google.api.LogDescriptor.prototype.setName = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * repeated LabelDescriptor labels = 2;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.google.api.LabelDescriptor>}
 */
proto.google.api.LogDescriptor.prototype.getLabelsList = function() {
  return /** @type{!Array.<!proto.google.api.LabelDescriptor>} */ (
    jspb.Message.getRepeatedWrapperField(this, google_api_label_pb.LabelDescriptor, 2));
};


/** @param {Array.<!proto.google.api.LabelDescriptor>} value  */
proto.google.api.LogDescriptor.prototype.setLabelsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 2, value);
};


proto.google.api.LogDescriptor.prototype.clearLabelsList = function() {
  this.setLabelsList([]);
};


/**
 * optional string description = 3;
 * @return {string}
 */
proto.google.api.LogDescriptor.prototype.getDescription = function() {
  return /** @type {string} */ (jspb.Message.getFieldProto3(this, 3, ""));
};


/** @param {string} value  */
proto.google.api.LogDescriptor.prototype.setDescription = function(value) {
  jspb.Message.setField(this, 3, value);
};


/**
 * optional string display_name = 4;
 * @return {string}
 */
proto.google.api.LogDescriptor.prototype.getDisplayName = function() {
  return /** @type {string} */ (jspb.Message.getFieldProto3(this, 4, ""));
};


/** @param {string} value  */
proto.google.api.LogDescriptor.prototype.setDisplayName = function(value) {
  jspb.Message.setField(this, 4, value);
};


goog.object.extend(exports, proto.google.api);
