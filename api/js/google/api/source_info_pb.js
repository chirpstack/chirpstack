/**
 * @fileoverview
 * @enhanceable
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

var google_protobuf_any_pb = require('google-protobuf/google/protobuf/any_pb.js');
goog.exportSymbol('proto.google.api.SourceInfo', null, global);

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
proto.google.api.SourceInfo = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.google.api.SourceInfo.repeatedFields_, null);
};
goog.inherits(proto.google.api.SourceInfo, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.google.api.SourceInfo.displayName = 'proto.google.api.SourceInfo';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.google.api.SourceInfo.repeatedFields_ = [1];



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
proto.google.api.SourceInfo.prototype.toObject = function(opt_includeInstance) {
  return proto.google.api.SourceInfo.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.google.api.SourceInfo} msg The msg instance to transform.
 * @return {!Object}
 */
proto.google.api.SourceInfo.toObject = function(includeInstance, msg) {
  var f, obj = {
    sourceFilesList: jspb.Message.toObjectList(msg.getSourceFilesList(),
    google_protobuf_any_pb.Any.toObject, includeInstance)
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
 * @return {!proto.google.api.SourceInfo}
 */
proto.google.api.SourceInfo.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.google.api.SourceInfo;
  return proto.google.api.SourceInfo.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.google.api.SourceInfo} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.google.api.SourceInfo}
 */
proto.google.api.SourceInfo.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new google_protobuf_any_pb.Any;
      reader.readMessage(value,google_protobuf_any_pb.Any.deserializeBinaryFromReader);
      msg.getSourceFilesList().push(value);
      msg.setSourceFilesList(msg.getSourceFilesList());
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
 * @param {!proto.google.api.SourceInfo} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.SourceInfo.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.google.api.SourceInfo.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.google.api.SourceInfo.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getSourceFilesList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      google_protobuf_any_pb.Any.serializeBinaryToWriter
    );
  }
};


/**
 * Creates a deep clone of this proto. No data is shared with the original.
 * @return {!proto.google.api.SourceInfo} The clone.
 */
proto.google.api.SourceInfo.prototype.cloneMessage = function() {
  return /** @type {!proto.google.api.SourceInfo} */ (jspb.Message.cloneMessage(this));
};


/**
 * repeated google.protobuf.Any source_files = 1;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.google.protobuf.Any>}
 */
proto.google.api.SourceInfo.prototype.getSourceFilesList = function() {
  return /** @type{!Array.<!proto.google.protobuf.Any>} */ (
    jspb.Message.getRepeatedWrapperField(this, google_protobuf_any_pb.Any, 1));
};


/** @param {Array.<!proto.google.protobuf.Any>} value  */
proto.google.api.SourceInfo.prototype.setSourceFilesList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 1, value);
};


proto.google.api.SourceInfo.prototype.clearSourceFilesList = function() {
  this.setSourceFilesList([]);
};


goog.object.extend(exports, proto.google.api);
