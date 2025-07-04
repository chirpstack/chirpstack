// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.36.6
// 	protoc        v6.30.2
// source: stream/frame.proto

package stream

import (
	common "github.com/chirpstack/chirpstack/api/go/v4/common"
	gw "github.com/chirpstack/chirpstack/api/go/v4/gw"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
	reflect "reflect"
	sync "sync"
	unsafe "unsafe"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type UplinkFrameLog struct {
	state protoimpl.MessageState `protogen:"open.v1"`
	// PHYPayload.
	PhyPayload []byte `protobuf:"bytes,1,opt,name=phy_payload,json=phyPayload,proto3" json:"phy_payload,omitempty"`
	// TX meta-data.
	TxInfo *gw.UplinkTxInfo `protobuf:"bytes,2,opt,name=tx_info,json=txInfo,proto3" json:"tx_info,omitempty"`
	// RX meta-data.
	RxInfo []*gw.UplinkRxInfo `protobuf:"bytes,3,rep,name=rx_info,json=rxInfo,proto3" json:"rx_info,omitempty"`
	// Frame type.
	FType common.FType `protobuf:"varint,4,opt,name=f_type,json=fType,proto3,enum=common.FType" json:"f_type,omitempty"`
	// Device address (optional).
	DevAddr string `protobuf:"bytes,5,opt,name=dev_addr,json=devAddr,proto3" json:"dev_addr,omitempty"`
	// Device EUI (optional).
	DevEui string `protobuf:"bytes,6,opt,name=dev_eui,json=devEui,proto3" json:"dev_eui,omitempty"`
	// Time.
	Time *timestamppb.Timestamp `protobuf:"bytes,7,opt,name=time,proto3" json:"time,omitempty"`
	// Plaintext f_opts mac-commands.
	PlaintextFOpts bool `protobuf:"varint,8,opt,name=plaintext_f_opts,json=plaintextFOpts,proto3" json:"plaintext_f_opts,omitempty"`
	// Plaintext frm_payload.
	PlaintextFrmPayload bool `protobuf:"varint,9,opt,name=plaintext_frm_payload,json=plaintextFrmPayload,proto3" json:"plaintext_frm_payload,omitempty"`
	unknownFields       protoimpl.UnknownFields
	sizeCache           protoimpl.SizeCache
}

func (x *UplinkFrameLog) Reset() {
	*x = UplinkFrameLog{}
	mi := &file_stream_frame_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *UplinkFrameLog) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UplinkFrameLog) ProtoMessage() {}

func (x *UplinkFrameLog) ProtoReflect() protoreflect.Message {
	mi := &file_stream_frame_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UplinkFrameLog.ProtoReflect.Descriptor instead.
func (*UplinkFrameLog) Descriptor() ([]byte, []int) {
	return file_stream_frame_proto_rawDescGZIP(), []int{0}
}

func (x *UplinkFrameLog) GetPhyPayload() []byte {
	if x != nil {
		return x.PhyPayload
	}
	return nil
}

func (x *UplinkFrameLog) GetTxInfo() *gw.UplinkTxInfo {
	if x != nil {
		return x.TxInfo
	}
	return nil
}

func (x *UplinkFrameLog) GetRxInfo() []*gw.UplinkRxInfo {
	if x != nil {
		return x.RxInfo
	}
	return nil
}

func (x *UplinkFrameLog) GetFType() common.FType {
	if x != nil {
		return x.FType
	}
	return common.FType(0)
}

func (x *UplinkFrameLog) GetDevAddr() string {
	if x != nil {
		return x.DevAddr
	}
	return ""
}

func (x *UplinkFrameLog) GetDevEui() string {
	if x != nil {
		return x.DevEui
	}
	return ""
}

func (x *UplinkFrameLog) GetTime() *timestamppb.Timestamp {
	if x != nil {
		return x.Time
	}
	return nil
}

func (x *UplinkFrameLog) GetPlaintextFOpts() bool {
	if x != nil {
		return x.PlaintextFOpts
	}
	return false
}

func (x *UplinkFrameLog) GetPlaintextFrmPayload() bool {
	if x != nil {
		return x.PlaintextFrmPayload
	}
	return false
}

type DownlinkFrameLog struct {
	state protoimpl.MessageState `protogen:"open.v1"`
	// Time.
	Time *timestamppb.Timestamp `protobuf:"bytes,1,opt,name=time,proto3" json:"time,omitempty"`
	// PHYPayload.
	PhyPayload []byte `protobuf:"bytes,2,opt,name=phy_payload,json=phyPayload,proto3" json:"phy_payload,omitempty"`
	// TX meta-data.
	TxInfo *gw.DownlinkTxInfo `protobuf:"bytes,3,opt,name=tx_info,json=txInfo,proto3" json:"tx_info,omitempty"`
	// Downlink ID.
	DownlinkId uint32 `protobuf:"varint,4,opt,name=downlink_id,json=downlinkId,proto3" json:"downlink_id,omitempty"`
	// Gateway ID (EUI64).
	GatewayId string `protobuf:"bytes,5,opt,name=gateway_id,json=gatewayId,proto3" json:"gateway_id,omitempty"`
	// Frame type.
	FType common.FType `protobuf:"varint,6,opt,name=f_type,json=fType,proto3,enum=common.FType" json:"f_type,omitempty"`
	// Device address (optional).
	DevAddr string `protobuf:"bytes,7,opt,name=dev_addr,json=devAddr,proto3" json:"dev_addr,omitempty"`
	// Device EUI (optional).
	DevEui string `protobuf:"bytes,8,opt,name=dev_eui,json=devEui,proto3" json:"dev_eui,omitempty"`
	// Plaintext f_opts mac-commands.
	PlaintextFOpts bool `protobuf:"varint,9,opt,name=plaintext_f_opts,json=plaintextFOpts,proto3" json:"plaintext_f_opts,omitempty"`
	// Plaintext frm_payload.
	PlaintextFrmPayload bool `protobuf:"varint,10,opt,name=plaintext_frm_payload,json=plaintextFrmPayload,proto3" json:"plaintext_frm_payload,omitempty"`
	unknownFields       protoimpl.UnknownFields
	sizeCache           protoimpl.SizeCache
}

func (x *DownlinkFrameLog) Reset() {
	*x = DownlinkFrameLog{}
	mi := &file_stream_frame_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *DownlinkFrameLog) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DownlinkFrameLog) ProtoMessage() {}

func (x *DownlinkFrameLog) ProtoReflect() protoreflect.Message {
	mi := &file_stream_frame_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DownlinkFrameLog.ProtoReflect.Descriptor instead.
func (*DownlinkFrameLog) Descriptor() ([]byte, []int) {
	return file_stream_frame_proto_rawDescGZIP(), []int{1}
}

func (x *DownlinkFrameLog) GetTime() *timestamppb.Timestamp {
	if x != nil {
		return x.Time
	}
	return nil
}

func (x *DownlinkFrameLog) GetPhyPayload() []byte {
	if x != nil {
		return x.PhyPayload
	}
	return nil
}

func (x *DownlinkFrameLog) GetTxInfo() *gw.DownlinkTxInfo {
	if x != nil {
		return x.TxInfo
	}
	return nil
}

func (x *DownlinkFrameLog) GetDownlinkId() uint32 {
	if x != nil {
		return x.DownlinkId
	}
	return 0
}

func (x *DownlinkFrameLog) GetGatewayId() string {
	if x != nil {
		return x.GatewayId
	}
	return ""
}

func (x *DownlinkFrameLog) GetFType() common.FType {
	if x != nil {
		return x.FType
	}
	return common.FType(0)
}

func (x *DownlinkFrameLog) GetDevAddr() string {
	if x != nil {
		return x.DevAddr
	}
	return ""
}

func (x *DownlinkFrameLog) GetDevEui() string {
	if x != nil {
		return x.DevEui
	}
	return ""
}

func (x *DownlinkFrameLog) GetPlaintextFOpts() bool {
	if x != nil {
		return x.PlaintextFOpts
	}
	return false
}

func (x *DownlinkFrameLog) GetPlaintextFrmPayload() bool {
	if x != nil {
		return x.PlaintextFrmPayload
	}
	return false
}

var File_stream_frame_proto protoreflect.FileDescriptor

const file_stream_frame_proto_rawDesc = "" +
	"\n" +
	"\x12stream/frame.proto\x12\x06stream\x1a\x1fgoogle/protobuf/timestamp.proto\x1a\x13common/common.proto\x1a\vgw/gw.proto\"\xef\x02\n" +
	"\x0eUplinkFrameLog\x12\x1f\n" +
	"\vphy_payload\x18\x01 \x01(\fR\n" +
	"phyPayload\x12)\n" +
	"\atx_info\x18\x02 \x01(\v2\x10.gw.UplinkTxInfoR\x06txInfo\x12)\n" +
	"\arx_info\x18\x03 \x03(\v2\x10.gw.UplinkRxInfoR\x06rxInfo\x12$\n" +
	"\x06f_type\x18\x04 \x01(\x0e2\r.common.FTypeR\x05fType\x12\x19\n" +
	"\bdev_addr\x18\x05 \x01(\tR\adevAddr\x12\x17\n" +
	"\adev_eui\x18\x06 \x01(\tR\x06devEui\x12.\n" +
	"\x04time\x18\a \x01(\v2\x1a.google.protobuf.TimestampR\x04time\x12(\n" +
	"\x10plaintext_f_opts\x18\b \x01(\bR\x0eplaintextFOpts\x122\n" +
	"\x15plaintext_frm_payload\x18\t \x01(\bR\x13plaintextFrmPayload\"\x88\x03\n" +
	"\x10DownlinkFrameLog\x12.\n" +
	"\x04time\x18\x01 \x01(\v2\x1a.google.protobuf.TimestampR\x04time\x12\x1f\n" +
	"\vphy_payload\x18\x02 \x01(\fR\n" +
	"phyPayload\x12+\n" +
	"\atx_info\x18\x03 \x01(\v2\x12.gw.DownlinkTxInfoR\x06txInfo\x12\x1f\n" +
	"\vdownlink_id\x18\x04 \x01(\rR\n" +
	"downlinkId\x12\x1d\n" +
	"\n" +
	"gateway_id\x18\x05 \x01(\tR\tgatewayId\x12$\n" +
	"\x06f_type\x18\x06 \x01(\x0e2\r.common.FTypeR\x05fType\x12\x19\n" +
	"\bdev_addr\x18\a \x01(\tR\adevAddr\x12\x17\n" +
	"\adev_eui\x18\b \x01(\tR\x06devEui\x12(\n" +
	"\x10plaintext_f_opts\x18\t \x01(\bR\x0eplaintextFOpts\x122\n" +
	"\x15plaintext_frm_payload\x18\n" +
	" \x01(\bR\x13plaintextFrmPayloadB\xa3\x01\n" +
	"\x18io.chirpstack.api.streamB\n" +
	"FrameProtoP\x01Z1github.com/chirpstack/chirpstack/api/go/v4/stream\xaa\x02\x11Chirpstack.Stream\xca\x02\x11Chirpstack\\Stream\xe2\x02\x1dGPBMetadata\\Chirpstack\\Streamb\x06proto3"

var (
	file_stream_frame_proto_rawDescOnce sync.Once
	file_stream_frame_proto_rawDescData []byte
)

func file_stream_frame_proto_rawDescGZIP() []byte {
	file_stream_frame_proto_rawDescOnce.Do(func() {
		file_stream_frame_proto_rawDescData = protoimpl.X.CompressGZIP(unsafe.Slice(unsafe.StringData(file_stream_frame_proto_rawDesc), len(file_stream_frame_proto_rawDesc)))
	})
	return file_stream_frame_proto_rawDescData
}

var file_stream_frame_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_stream_frame_proto_goTypes = []any{
	(*UplinkFrameLog)(nil),        // 0: stream.UplinkFrameLog
	(*DownlinkFrameLog)(nil),      // 1: stream.DownlinkFrameLog
	(*gw.UplinkTxInfo)(nil),       // 2: gw.UplinkTxInfo
	(*gw.UplinkRxInfo)(nil),       // 3: gw.UplinkRxInfo
	(common.FType)(0),             // 4: common.FType
	(*timestamppb.Timestamp)(nil), // 5: google.protobuf.Timestamp
	(*gw.DownlinkTxInfo)(nil),     // 6: gw.DownlinkTxInfo
}
var file_stream_frame_proto_depIdxs = []int32{
	2, // 0: stream.UplinkFrameLog.tx_info:type_name -> gw.UplinkTxInfo
	3, // 1: stream.UplinkFrameLog.rx_info:type_name -> gw.UplinkRxInfo
	4, // 2: stream.UplinkFrameLog.f_type:type_name -> common.FType
	5, // 3: stream.UplinkFrameLog.time:type_name -> google.protobuf.Timestamp
	5, // 4: stream.DownlinkFrameLog.time:type_name -> google.protobuf.Timestamp
	6, // 5: stream.DownlinkFrameLog.tx_info:type_name -> gw.DownlinkTxInfo
	4, // 6: stream.DownlinkFrameLog.f_type:type_name -> common.FType
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_stream_frame_proto_init() }
func file_stream_frame_proto_init() {
	if File_stream_frame_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: unsafe.Slice(unsafe.StringData(file_stream_frame_proto_rawDesc), len(file_stream_frame_proto_rawDesc)),
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_stream_frame_proto_goTypes,
		DependencyIndexes: file_stream_frame_proto_depIdxs,
		MessageInfos:      file_stream_frame_proto_msgTypes,
	}.Build()
	File_stream_frame_proto = out.File
	file_stream_frame_proto_goTypes = nil
	file_stream_frame_proto_depIdxs = nil
}
