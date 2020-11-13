// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.25.0
// 	protoc        v3.12.4
// source: hanmessage.proto

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
// Envelope to enable easy parsing of multiple types of messages
type StreamHeader struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Magic  uint32 `protobuf:"fixed32,1,opt,name=magic,proto3" json:"magic,omitempty"`
	Length uint32 `protobuf:"fixed32,2,opt,name=length,proto3" json:"length,omitempty"`
}

func (x *StreamHeader) Reset() {
	*x = StreamHeader{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *StreamHeader) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*StreamHeader) ProtoMessage() {}

func (x *StreamHeader) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use StreamHeader.ProtoReflect.Descriptor instead.
func (*StreamHeader) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{0}
}

func (x *StreamHeader) GetMagic() uint32 {
	if x != nil {
		return x.Magic
	}
	return 0
}

func (x *StreamHeader) GetLength() uint32 {
	if x != nil {
		return x.Length
	}
	return 0
}

type HanMessage struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Uuid []byte `protobuf:"bytes,10,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// Types that are assignable to Msg:
	//	*HanMessage_Auth
	//	*HanMessage_AuthResult
	Msg isHanMessage_Msg `protobuf_oneof:"msg"`
}

func (x *HanMessage) Reset() {
	*x = HanMessage{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *HanMessage) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*HanMessage) ProtoMessage() {}

func (x *HanMessage) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use HanMessage.ProtoReflect.Descriptor instead.
func (*HanMessage) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{1}
}

func (x *HanMessage) GetUuid() []byte {
	if x != nil {
		return x.Uuid
	}
	return nil
}

func (m *HanMessage) GetMsg() isHanMessage_Msg {
	if m != nil {
		return m.Msg
	}
	return nil
}

func (x *HanMessage) GetAuth() *Auth {
	if x, ok := x.GetMsg().(*HanMessage_Auth); ok {
		return x.Auth
	}
	return nil
}

func (x *HanMessage) GetAuthResult() *AuthResult {
	if x, ok := x.GetMsg().(*HanMessage_AuthResult); ok {
		return x.AuthResult
	}
	return nil
}

type isHanMessage_Msg interface {
	isHanMessage_Msg()
}

type HanMessage_Auth struct {
	Auth *Auth `protobuf:"bytes,11,opt,name=auth,proto3,oneof"`
}

type HanMessage_AuthResult struct {
	AuthResult *AuthResult `protobuf:"bytes,12,opt,name=auth_result,json=authResult,proto3,oneof"`
}

func (*HanMessage_Auth) isHanMessage_Msg() {}

func (*HanMessage_AuthResult) isHanMessage_Msg() {}

type Auth struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Username string `protobuf:"bytes,100,opt,name=username,proto3" json:"username,omitempty"`
}

func (x *Auth) Reset() {
	*x = Auth{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Auth) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Auth) ProtoMessage() {}

func (x *Auth) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Auth.ProtoReflect.Descriptor instead.
func (*Auth) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{2}
}

func (x *Auth) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

type AuthResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Success bool `protobuf:"varint,200,opt,name=success,proto3" json:"success,omitempty"`
}

func (x *AuthResult) Reset() {
	*x = AuthResult{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AuthResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AuthResult) ProtoMessage() {}

func (x *AuthResult) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AuthResult.ProtoReflect.Descriptor instead.
func (*AuthResult) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{3}
}

func (x *AuthResult) GetSuccess() bool {
	if x != nil {
		return x.Success
	}
	return false
}

var File_hanmessage_proto protoreflect.FileDescriptor

var file_hanmessage_proto_rawDesc = []byte{
	0x0a, 0x10, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0x3c, 0x0a, 0x0c, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x48, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x6d, 0x61, 0x67, 0x69, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x07, 0x52, 0x05, 0x6d, 0x61, 0x67, 0x69, 0x63, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67,
	0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x07, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68,
	0x22, 0x74, 0x0a, 0x0a, 0x48, 0x61, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x12,
	0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x75, 0x75,
	0x69, 0x64, 0x12, 0x1b, 0x0a, 0x04, 0x61, 0x75, 0x74, 0x68, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x05, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x48, 0x00, 0x52, 0x04, 0x61, 0x75, 0x74, 0x68, 0x12,
	0x2e, 0x0a, 0x0b, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x0c,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c,
	0x74, 0x48, 0x00, 0x52, 0x0a, 0x61, 0x75, 0x74, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x42,
	0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x22, 0x0a, 0x04, 0x41, 0x75, 0x74, 0x68, 0x12, 0x1a,
	0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x64, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x27, 0x0a, 0x0a, 0x41, 0x75,
	0x74, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x19, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63,
	0x65, 0x73, 0x73, 0x18, 0xc8, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63,
	0x65, 0x73, 0x73, 0x42, 0x0d, 0x5a, 0x0b, 0x2e, 0x3b, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69,
	0x7a, 0x65, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_hanmessage_proto_rawDescOnce sync.Once
	file_hanmessage_proto_rawDescData = file_hanmessage_proto_rawDesc
)

func file_hanmessage_proto_rawDescGZIP() []byte {
	file_hanmessage_proto_rawDescOnce.Do(func() {
		file_hanmessage_proto_rawDescData = protoimpl.X.CompressGZIP(file_hanmessage_proto_rawDescData)
	})
	return file_hanmessage_proto_rawDescData
}

var file_hanmessage_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_hanmessage_proto_goTypes = []interface{}{
	(*StreamHeader)(nil), // 0: StreamHeader
	(*HanMessage)(nil),   // 1: HanMessage
	(*Auth)(nil),         // 2: Auth
	(*AuthResult)(nil),   // 3: AuthResult
}
var file_hanmessage_proto_depIdxs = []int32{
	2, // 0: HanMessage.auth:type_name -> Auth
	3, // 1: HanMessage.auth_result:type_name -> AuthResult
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_hanmessage_proto_init() }
func file_hanmessage_proto_init() {
	if File_hanmessage_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_hanmessage_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*StreamHeader); i {
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
		file_hanmessage_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*HanMessage); i {
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
		file_hanmessage_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Auth); i {
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
		file_hanmessage_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AuthResult); i {
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
	file_hanmessage_proto_msgTypes[1].OneofWrappers = []interface{}{
		(*HanMessage_Auth)(nil),
		(*HanMessage_AuthResult)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_hanmessage_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_hanmessage_proto_goTypes,
		DependencyIndexes: file_hanmessage_proto_depIdxs,
		MessageInfos:      file_hanmessage_proto_msgTypes,
	}.Build()
	File_hanmessage_proto = out.File
	file_hanmessage_proto_rawDesc = nil
	file_hanmessage_proto_goTypes = nil
	file_hanmessage_proto_depIdxs = nil
}