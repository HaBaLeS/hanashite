// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.25.0
// 	protoc        v3.14.0
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

	MessageId []byte `protobuf:"bytes,10,opt,name=message_id,json=messageId,proto3" json:"message_id,omitempty"`
	// Types that are assignable to Msg:
	//	*HanMessage_Auth
	//	*HanMessage_AuthResult
	//	*HanMessage_ChanLst
	//	*HanMessage_ChanLstResult
	//	*HanMessage_ChanJoin
	//	*HanMessage_ChanJoinResult
	//	*HanMessage_ChanPart
	//	*HanMessage_ChanPartResult
	//	*HanMessage_ChanStatus
	//	*HanMessage_ChanStatusResult
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

func (x *HanMessage) GetMessageId() []byte {
	if x != nil {
		return x.MessageId
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

func (x *HanMessage) GetChanLst() *ChannelList {
	if x, ok := x.GetMsg().(*HanMessage_ChanLst); ok {
		return x.ChanLst
	}
	return nil
}

func (x *HanMessage) GetChanLstResult() *ChannelListResult {
	if x, ok := x.GetMsg().(*HanMessage_ChanLstResult); ok {
		return x.ChanLstResult
	}
	return nil
}

func (x *HanMessage) GetChanJoin() *ChannelJoin {
	if x, ok := x.GetMsg().(*HanMessage_ChanJoin); ok {
		return x.ChanJoin
	}
	return nil
}

func (x *HanMessage) GetChanJoinResult() *ChannelJoinResult {
	if x, ok := x.GetMsg().(*HanMessage_ChanJoinResult); ok {
		return x.ChanJoinResult
	}
	return nil
}

func (x *HanMessage) GetChanPart() *ChannelPart {
	if x, ok := x.GetMsg().(*HanMessage_ChanPart); ok {
		return x.ChanPart
	}
	return nil
}

func (x *HanMessage) GetChanPartResult() *ChannelPartResult {
	if x, ok := x.GetMsg().(*HanMessage_ChanPartResult); ok {
		return x.ChanPartResult
	}
	return nil
}

func (x *HanMessage) GetChanStatus() *ChannelStatus {
	if x, ok := x.GetMsg().(*HanMessage_ChanStatus); ok {
		return x.ChanStatus
	}
	return nil
}

func (x *HanMessage) GetChanStatusResult() *ChannelStatusResult {
	if x, ok := x.GetMsg().(*HanMessage_ChanStatusResult); ok {
		return x.ChanStatusResult
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

type HanMessage_ChanLst struct {
	ChanLst *ChannelList `protobuf:"bytes,13,opt,name=chan_lst,json=chanLst,proto3,oneof"`
}

type HanMessage_ChanLstResult struct {
	ChanLstResult *ChannelListResult `protobuf:"bytes,14,opt,name=chan_lst_result,json=chanLstResult,proto3,oneof"`
}

type HanMessage_ChanJoin struct {
	ChanJoin *ChannelJoin `protobuf:"bytes,15,opt,name=chan_join,json=chanJoin,proto3,oneof"`
}

type HanMessage_ChanJoinResult struct {
	ChanJoinResult *ChannelJoinResult `protobuf:"bytes,16,opt,name=chan_join_result,json=chanJoinResult,proto3,oneof"`
}

type HanMessage_ChanPart struct {
	ChanPart *ChannelPart `protobuf:"bytes,17,opt,name=chan_part,json=chanPart,proto3,oneof"`
}

type HanMessage_ChanPartResult struct {
	ChanPartResult *ChannelPartResult `protobuf:"bytes,18,opt,name=chan_part_result,json=chanPartResult,proto3,oneof"`
}

type HanMessage_ChanStatus struct {
	ChanStatus *ChannelStatus `protobuf:"bytes,19,opt,name=chan_status,json=chanStatus,proto3,oneof"`
}

type HanMessage_ChanStatusResult struct {
	ChanStatusResult *ChannelStatusResult `protobuf:"bytes,20,opt,name=chan_status_result,json=chanStatusResult,proto3,oneof"`
}

func (*HanMessage_Auth) isHanMessage_Msg() {}

func (*HanMessage_AuthResult) isHanMessage_Msg() {}

func (*HanMessage_ChanLst) isHanMessage_Msg() {}

func (*HanMessage_ChanLstResult) isHanMessage_Msg() {}

func (*HanMessage_ChanJoin) isHanMessage_Msg() {}

func (*HanMessage_ChanJoinResult) isHanMessage_Msg() {}

func (*HanMessage_ChanPart) isHanMessage_Msg() {}

func (*HanMessage_ChanPartResult) isHanMessage_Msg() {}

func (*HanMessage_ChanStatus) isHanMessage_Msg() {}

func (*HanMessage_ChanStatusResult) isHanMessage_Msg() {}

type Auth struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Username string `protobuf:"bytes,20,opt,name=username,proto3" json:"username,omitempty"`
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

	Success      bool   `protobuf:"varint,30,opt,name=success,proto3" json:"success,omitempty"`
	ConnectionId []byte `protobuf:"bytes,31,opt,name=connection_id,json=connectionId,proto3" json:"connection_id,omitempty"`
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

func (x *AuthResult) GetConnectionId() []byte {
	if x != nil {
		return x.ConnectionId
	}
	return nil
}

type ChannelList struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *ChannelList) Reset() {
	*x = ChannelList{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelList) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelList) ProtoMessage() {}

func (x *ChannelList) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelList.ProtoReflect.Descriptor instead.
func (*ChannelList) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{4}
}

type ChannelListentry struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name string `protobuf:"bytes,40,opt,name=name,proto3" json:"name,omitempty"`
	Id   []byte `protobuf:"bytes,41,opt,name=id,proto3" json:"id,omitempty"`
}

func (x *ChannelListentry) Reset() {
	*x = ChannelListentry{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelListentry) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelListentry) ProtoMessage() {}

func (x *ChannelListentry) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelListentry.ProtoReflect.Descriptor instead.
func (*ChannelListentry) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{5}
}

func (x *ChannelListentry) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *ChannelListentry) GetId() []byte {
	if x != nil {
		return x.Id
	}
	return nil
}

type ChannelListResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Channel []*ChannelListentry `protobuf:"bytes,45,rep,name=channel,proto3" json:"channel,omitempty"`
}

func (x *ChannelListResult) Reset() {
	*x = ChannelListResult{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelListResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelListResult) ProtoMessage() {}

func (x *ChannelListResult) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelListResult.ProtoReflect.Descriptor instead.
func (*ChannelListResult) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{6}
}

func (x *ChannelListResult) GetChannel() []*ChannelListentry {
	if x != nil {
		return x.Channel
	}
	return nil
}

type ChannelJoin struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name      string `protobuf:"bytes,50,opt,name=name,proto3" json:"name,omitempty"`
	ChannelId []byte `protobuf:"bytes,51,opt,name=channel_id,json=channelId,proto3" json:"channel_id,omitempty"`
}

func (x *ChannelJoin) Reset() {
	*x = ChannelJoin{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelJoin) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelJoin) ProtoMessage() {}

func (x *ChannelJoin) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelJoin.ProtoReflect.Descriptor instead.
func (*ChannelJoin) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{7}
}

func (x *ChannelJoin) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *ChannelJoin) GetChannelId() []byte {
	if x != nil {
		return x.ChannelId
	}
	return nil
}

type ChannelJoinResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Success   bool   `protobuf:"varint,60,opt,name=success,proto3" json:"success,omitempty"`
	ChannelId []byte `protobuf:"bytes,61,opt,name=channel_id,json=channelId,proto3" json:"channel_id,omitempty"`
}

func (x *ChannelJoinResult) Reset() {
	*x = ChannelJoinResult{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelJoinResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelJoinResult) ProtoMessage() {}

func (x *ChannelJoinResult) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelJoinResult.ProtoReflect.Descriptor instead.
func (*ChannelJoinResult) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{8}
}

func (x *ChannelJoinResult) GetSuccess() bool {
	if x != nil {
		return x.Success
	}
	return false
}

func (x *ChannelJoinResult) GetChannelId() []byte {
	if x != nil {
		return x.ChannelId
	}
	return nil
}

type ChannelPart struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *ChannelPart) Reset() {
	*x = ChannelPart{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[9]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelPart) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelPart) ProtoMessage() {}

func (x *ChannelPart) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[9]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelPart.ProtoReflect.Descriptor instead.
func (*ChannelPart) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{9}
}

type ChannelPartResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Success bool `protobuf:"varint,80,opt,name=success,proto3" json:"success,omitempty"`
}

func (x *ChannelPartResult) Reset() {
	*x = ChannelPartResult{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[10]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelPartResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelPartResult) ProtoMessage() {}

func (x *ChannelPartResult) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[10]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelPartResult.ProtoReflect.Descriptor instead.
func (*ChannelPartResult) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{10}
}

func (x *ChannelPartResult) GetSuccess() bool {
	if x != nil {
		return x.Success
	}
	return false
}

type ChannelStatus struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ChannelId []byte `protobuf:"bytes,90,opt,name=channel_id,json=channelId,proto3" json:"channel_id,omitempty"`
}

func (x *ChannelStatus) Reset() {
	*x = ChannelStatus{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[11]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelStatus) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelStatus) ProtoMessage() {}

func (x *ChannelStatus) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[11]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelStatus.ProtoReflect.Descriptor instead.
func (*ChannelStatus) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{11}
}

func (x *ChannelStatus) GetChannelId() []byte {
	if x != nil {
		return x.ChannelId
	}
	return nil
}

type ChannelStatusResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *ChannelStatusResult) Reset() {
	*x = ChannelStatusResult{}
	if protoimpl.UnsafeEnabled {
		mi := &file_hanmessage_proto_msgTypes[12]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ChannelStatusResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ChannelStatusResult) ProtoMessage() {}

func (x *ChannelStatusResult) ProtoReflect() protoreflect.Message {
	mi := &file_hanmessage_proto_msgTypes[12]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ChannelStatusResult.ProtoReflect.Descriptor instead.
func (*ChannelStatusResult) Descriptor() ([]byte, []int) {
	return file_hanmessage_proto_rawDescGZIP(), []int{12}
}

var File_hanmessage_proto protoreflect.FileDescriptor

var file_hanmessage_proto_rawDesc = []byte{
	0x0a, 0x10, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x0a, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x3c,
	0x0a, 0x0c, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x14,
	0x0a, 0x05, 0x6d, 0x61, 0x67, 0x69, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x05, 0x6d,
	0x61, 0x67, 0x69, 0x63, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x07, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x22, 0xa9, 0x05, 0x0a,
	0x0a, 0x48, 0x61, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6d,
	0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x52,
	0x09, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x04, 0x61, 0x75,
	0x74, 0x68, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65,
	0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x48, 0x00, 0x52, 0x04, 0x61, 0x75,
	0x74, 0x68, 0x12, 0x39, 0x0a, 0x0b, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c,
	0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73,
	0x73, 0x61, 0x67, 0x65, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48,
	0x00, 0x52, 0x0a, 0x61, 0x75, 0x74, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x34, 0x0a,
	0x08, 0x63, 0x68, 0x61, 0x6e, 0x5f, 0x6c, 0x73, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x17, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x43, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74, 0x48, 0x00, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e,
	0x4c, 0x73, 0x74, 0x12, 0x47, 0x0a, 0x0f, 0x63, 0x68, 0x61, 0x6e, 0x5f, 0x6c, 0x73, 0x74, 0x5f,
	0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x68,
	0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65,
	0x6c, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x52, 0x0d, 0x63,
	0x68, 0x61, 0x6e, 0x4c, 0x73, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x36, 0x0a, 0x09,
	0x63, 0x68, 0x61, 0x6e, 0x5f, 0x6a, 0x6f, 0x69, 0x6e, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x17, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x43, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x4a, 0x6f, 0x69, 0x6e, 0x48, 0x00, 0x52, 0x08, 0x63, 0x68, 0x61, 0x6e,
	0x4a, 0x6f, 0x69, 0x6e, 0x12, 0x49, 0x0a, 0x10, 0x63, 0x68, 0x61, 0x6e, 0x5f, 0x6a, 0x6f, 0x69,
	0x6e, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d,
	0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x6e,
	0x6e, 0x65, 0x6c, 0x4a, 0x6f, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x52,
	0x0e, 0x63, 0x68, 0x61, 0x6e, 0x4a, 0x6f, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
	0x36, 0x0a, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x18, 0x11, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x17, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e,
	0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x50, 0x61, 0x72, 0x74, 0x48, 0x00, 0x52, 0x08, 0x63,
	0x68, 0x61, 0x6e, 0x50, 0x61, 0x72, 0x74, 0x12, 0x49, 0x0a, 0x10, 0x63, 0x68, 0x61, 0x6e, 0x5f,
	0x70, 0x61, 0x72, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x12, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x43,
	0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x50, 0x61, 0x72, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
	0x48, 0x00, 0x52, 0x0e, 0x63, 0x68, 0x61, 0x6e, 0x50, 0x61, 0x72, 0x74, 0x52, 0x65, 0x73, 0x75,
	0x6c, 0x74, 0x12, 0x3c, 0x0a, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73,
	0x73, 0x61, 0x67, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x53, 0x74, 0x61, 0x74,
	0x75, 0x73, 0x48, 0x00, 0x52, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
	0x12, 0x4f, 0x0a, 0x12, 0x63, 0x68, 0x61, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f,
	0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x68,
	0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65,
	0x6c, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x52,
	0x10, 0x63, 0x68, 0x61, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c,
	0x74, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x22, 0x0a, 0x04, 0x41, 0x75, 0x74, 0x68,
	0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x14, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x4b, 0x0a, 0x0a,
	0x41, 0x75, 0x74, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75,
	0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63,
	0x63, 0x65, 0x73, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
	0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x1f, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x63, 0x6f, 0x6e,
	0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x0d, 0x0a, 0x0b, 0x43, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74, 0x22, 0x36, 0x0a, 0x10, 0x43, 0x68, 0x61, 0x6e,
	0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x12, 0x0a, 0x04,
	0x6e, 0x61, 0x6d, 0x65, 0x18, 0x28, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
	0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x29, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x02, 0x69, 0x64,
	0x22, 0x4b, 0x0a, 0x11, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74, 0x52,
	0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x36, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
	0x18, 0x2d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x61, 0x6e, 0x6d, 0x65, 0x73, 0x73,
	0x61, 0x67, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74, 0x65,
	0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x22, 0x40, 0x0a,
	0x0b, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4a, 0x6f, 0x69, 0x6e, 0x12, 0x12, 0x0a, 0x04,
	0x6e, 0x61, 0x6d, 0x65, 0x18, 0x32, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
	0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x33,
	0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x22,
	0x4c, 0x0a, 0x11, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4a, 0x6f, 0x69, 0x6e, 0x52, 0x65,
	0x73, 0x75, 0x6c, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18,
	0x3c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x1d,
	0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x3d, 0x20, 0x01,
	0x28, 0x0c, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x22, 0x0d, 0x0a,
	0x0b, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x50, 0x61, 0x72, 0x74, 0x22, 0x2d, 0x0a, 0x11,
	0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x50, 0x61, 0x72, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c,
	0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x50, 0x20, 0x01,
	0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x22, 0x2e, 0x0a, 0x0d, 0x43,
	0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x1d, 0x0a, 0x0a,
	0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x5a, 0x20, 0x01, 0x28, 0x0c,
	0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x22, 0x15, 0x0a, 0x13, 0x43,
	0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x75,
	0x6c, 0x74, 0x42, 0x0d, 0x5a, 0x0b, 0x2e, 0x3b, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a,
	0x65, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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

var file_hanmessage_proto_msgTypes = make([]protoimpl.MessageInfo, 13)
var file_hanmessage_proto_goTypes = []interface{}{
	(*StreamHeader)(nil),        // 0: hanmessage.StreamHeader
	(*HanMessage)(nil),          // 1: hanmessage.HanMessage
	(*Auth)(nil),                // 2: hanmessage.Auth
	(*AuthResult)(nil),          // 3: hanmessage.AuthResult
	(*ChannelList)(nil),         // 4: hanmessage.ChannelList
	(*ChannelListentry)(nil),    // 5: hanmessage.ChannelListentry
	(*ChannelListResult)(nil),   // 6: hanmessage.ChannelListResult
	(*ChannelJoin)(nil),         // 7: hanmessage.ChannelJoin
	(*ChannelJoinResult)(nil),   // 8: hanmessage.ChannelJoinResult
	(*ChannelPart)(nil),         // 9: hanmessage.ChannelPart
	(*ChannelPartResult)(nil),   // 10: hanmessage.ChannelPartResult
	(*ChannelStatus)(nil),       // 11: hanmessage.ChannelStatus
	(*ChannelStatusResult)(nil), // 12: hanmessage.ChannelStatusResult
}
var file_hanmessage_proto_depIdxs = []int32{
	2,  // 0: hanmessage.HanMessage.auth:type_name -> hanmessage.Auth
	3,  // 1: hanmessage.HanMessage.auth_result:type_name -> hanmessage.AuthResult
	4,  // 2: hanmessage.HanMessage.chan_lst:type_name -> hanmessage.ChannelList
	6,  // 3: hanmessage.HanMessage.chan_lst_result:type_name -> hanmessage.ChannelListResult
	7,  // 4: hanmessage.HanMessage.chan_join:type_name -> hanmessage.ChannelJoin
	8,  // 5: hanmessage.HanMessage.chan_join_result:type_name -> hanmessage.ChannelJoinResult
	9,  // 6: hanmessage.HanMessage.chan_part:type_name -> hanmessage.ChannelPart
	10, // 7: hanmessage.HanMessage.chan_part_result:type_name -> hanmessage.ChannelPartResult
	11, // 8: hanmessage.HanMessage.chan_status:type_name -> hanmessage.ChannelStatus
	12, // 9: hanmessage.HanMessage.chan_status_result:type_name -> hanmessage.ChannelStatusResult
	5,  // 10: hanmessage.ChannelListResult.channel:type_name -> hanmessage.ChannelListentry
	11, // [11:11] is the sub-list for method output_type
	11, // [11:11] is the sub-list for method input_type
	11, // [11:11] is the sub-list for extension type_name
	11, // [11:11] is the sub-list for extension extendee
	0,  // [0:11] is the sub-list for field type_name
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
		file_hanmessage_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelList); i {
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
		file_hanmessage_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelListentry); i {
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
		file_hanmessage_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelListResult); i {
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
		file_hanmessage_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelJoin); i {
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
		file_hanmessage_proto_msgTypes[8].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelJoinResult); i {
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
		file_hanmessage_proto_msgTypes[9].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelPart); i {
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
		file_hanmessage_proto_msgTypes[10].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelPartResult); i {
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
		file_hanmessage_proto_msgTypes[11].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelStatus); i {
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
		file_hanmessage_proto_msgTypes[12].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ChannelStatusResult); i {
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
		(*HanMessage_ChanLst)(nil),
		(*HanMessage_ChanLstResult)(nil),
		(*HanMessage_ChanJoin)(nil),
		(*HanMessage_ChanJoinResult)(nil),
		(*HanMessage_ChanPart)(nil),
		(*HanMessage_ChanPartResult)(nil),
		(*HanMessage_ChanStatus)(nil),
		(*HanMessage_ChanStatusResult)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_hanmessage_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   13,
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
