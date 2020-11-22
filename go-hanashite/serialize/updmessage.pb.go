// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.25.0
// 	protoc        v3.12.4
// source: updmessage.proto

package serialize

import (
	proto "github.com/golang/protobuf/proto"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// This is a compile-time assertion that a sufficiently up-to-date version
// of the legacy proto package is being used.
const _ = proto.ProtoPackageIsVersion4

//*
// Udp Message Envelope
type HanUdpMessage struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	UserId     []byte       `protobuf:"bytes,111,opt,name=user_id,json=userId,proto3" json:"user_id,omitempty"`
	AudioFrame *AudioPacket `protobuf:"bytes,100,opt,name=audio_frame,json=audioFrame,proto3" json:"audio_frame,omitempty"`
}

func (x *HanUdpMessage) Reset() {
	*x = HanUdpMessage{}
	if protoimpl.UnsafeEnabled {
		mi := &file_updmessage_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *HanUdpMessage) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*HanUdpMessage) ProtoMessage() {}

func (x *HanUdpMessage) ProtoReflect() protoreflect.Message {
	mi := &file_updmessage_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use HanUdpMessage.ProtoReflect.Descriptor instead.
func (*HanUdpMessage) Descriptor() ([]byte, []int) {
	return file_updmessage_proto_rawDescGZIP(), []int{0}
}

func (x *HanUdpMessage) GetUserId() []byte {
	if x != nil {
		return x.UserId
	}
	return nil
}

func (x *HanUdpMessage) GetAudioFrame() *AudioPacket {
	if x != nil {
		return x.AudioFrame
	}
	return nil
}

type AudioPacket struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ChannelId   []byte `protobuf:"bytes,110,opt,name=channel_id,json=channelId,proto3" json:"channel_id,omitempty"`
	SequernceId uint64 `protobuf:"varint,112,opt,name=sequernce_id,json=sequernceId,proto3" json:"sequernce_id,omitempty"`
	Data        []byte `protobuf:"bytes,113,opt,name=data,proto3" json:"data,omitempty"`
}

func (x *AudioPacket) Reset() {
	*x = AudioPacket{}
	if protoimpl.UnsafeEnabled {
		mi := &file_updmessage_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AudioPacket) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AudioPacket) ProtoMessage() {}

func (x *AudioPacket) ProtoReflect() protoreflect.Message {
	mi := &file_updmessage_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AudioPacket.ProtoReflect.Descriptor instead.
func (*AudioPacket) Descriptor() ([]byte, []int) {
	return file_updmessage_proto_rawDescGZIP(), []int{1}
}

func (x *AudioPacket) GetChannelId() []byte {
	if x != nil {
		return x.ChannelId
	}
	return nil
}

func (x *AudioPacket) GetSequernceId() uint64 {
	if x != nil {
		return x.SequernceId
	}
	return 0
}

func (x *AudioPacket) GetData() []byte {
	if x != nil {
		return x.Data
	}
	return nil
}

var File_updmessage_proto protoreflect.FileDescriptor

var file_updmessage_proto_rawDesc = []byte{
	0x0a, 0x10, 0x75, 0x70, 0x64, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0x57, 0x0a, 0x0d, 0x48, 0x61, 0x6e, 0x55, 0x64, 0x70, 0x4d, 0x65, 0x73, 0x73,
	0x61, 0x67, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x6f,
	0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x75, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x2d, 0x0a, 0x0b,
	0x61, 0x75, 0x64, 0x69, 0x6f, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x18, 0x64, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x0c, 0x2e, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x52,
	0x0a, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x22, 0x63, 0x0a, 0x0b, 0x41,
	0x75, 0x64, 0x69, 0x6f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68,
	0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x6e, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09,
	0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x65, 0x71,
	0x75, 0x65, 0x72, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x70, 0x20, 0x01, 0x28, 0x04, 0x52,
	0x0b, 0x73, 0x65, 0x71, 0x75, 0x65, 0x72, 0x6e, 0x63, 0x65, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04,
	0x64, 0x61, 0x74, 0x61, 0x18, 0x71, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61,
	0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_updmessage_proto_rawDescOnce sync.Once
	file_updmessage_proto_rawDescData = file_updmessage_proto_rawDesc
)

func file_updmessage_proto_rawDescGZIP() []byte {
	file_updmessage_proto_rawDescOnce.Do(func() {
		file_updmessage_proto_rawDescData = protoimpl.X.CompressGZIP(file_updmessage_proto_rawDescData)
	})
	return file_updmessage_proto_rawDescData
}

var file_updmessage_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_updmessage_proto_goTypes = []interface{}{
	(*HanUdpMessage)(nil), // 0: HanUdpMessage
	(*AudioPacket)(nil),   // 1: AudioPacket
}
var file_updmessage_proto_depIdxs = []int32{
	1, // 0: HanUdpMessage.audio_frame:type_name -> AudioPacket
	1, // [1:1] is the sub-list for method output_type
	1, // [1:1] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_updmessage_proto_init() }
func file_updmessage_proto_init() {
	if File_updmessage_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_updmessage_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*HanUdpMessage); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_updmessage_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AudioPacket); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_updmessage_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_updmessage_proto_goTypes,
		DependencyIndexes: file_updmessage_proto_depIdxs,
		MessageInfos:      file_updmessage_proto_msgTypes,
	}.Build()
	File_updmessage_proto = out.File
	file_updmessage_proto_rawDesc = nil
	file_updmessage_proto_goTypes = nil
	file_updmessage_proto_depIdxs = nil
}
