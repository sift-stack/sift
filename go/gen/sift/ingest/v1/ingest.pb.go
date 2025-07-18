// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        (unknown)
// source: sift/ingest/v1/ingest.proto

package ingestv1

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	emptypb "google.golang.org/protobuf/types/known/emptypb"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type IngestWithConfigDataStreamRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	IngestionConfigId string                              `protobuf:"bytes,1,opt,name=ingestion_config_id,json=ingestionConfigId,proto3" json:"ingestion_config_id,omitempty"`
	Flow              string                              `protobuf:"bytes,2,opt,name=flow,proto3" json:"flow,omitempty"`
	Timestamp         *timestamppb.Timestamp              `protobuf:"bytes,3,opt,name=timestamp,proto3" json:"timestamp,omitempty"`
	ChannelValues     []*IngestWithConfigDataChannelValue `protobuf:"bytes,4,rep,name=channel_values,json=channelValues,proto3" json:"channel_values,omitempty"`
	// The run_id MUST be included if this data is part of a run.
	RunId string `protobuf:"bytes,5,opt,name=run_id,json=runId,proto3" json:"run_id,omitempty"`
	// By default, if this request contains any channel values that do not match
	// the supplied ingestion config, the request is stored in an error queue and
	// the stream continues to accept data. This ensures all data is saved, but
	// only valid data is fully ingested. If this is set to `true`, any validation
	// errors end the stream and return the error to the client.
	EndStreamOnValidationError bool   `protobuf:"varint,6,opt,name=end_stream_on_validation_error,json=endStreamOnValidationError,proto3" json:"end_stream_on_validation_error,omitempty"`
	OrganizationId             string `protobuf:"bytes,7,opt,name=organization_id,json=organizationId,proto3" json:"organization_id,omitempty"`
}

func (x *IngestWithConfigDataStreamRequest) Reset() {
	*x = IngestWithConfigDataStreamRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_ingest_v1_ingest_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IngestWithConfigDataStreamRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IngestWithConfigDataStreamRequest) ProtoMessage() {}

func (x *IngestWithConfigDataStreamRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_ingest_v1_ingest_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IngestWithConfigDataStreamRequest.ProtoReflect.Descriptor instead.
func (*IngestWithConfigDataStreamRequest) Descriptor() ([]byte, []int) {
	return file_sift_ingest_v1_ingest_proto_rawDescGZIP(), []int{0}
}

func (x *IngestWithConfigDataStreamRequest) GetIngestionConfigId() string {
	if x != nil {
		return x.IngestionConfigId
	}
	return ""
}

func (x *IngestWithConfigDataStreamRequest) GetFlow() string {
	if x != nil {
		return x.Flow
	}
	return ""
}

func (x *IngestWithConfigDataStreamRequest) GetTimestamp() *timestamppb.Timestamp {
	if x != nil {
		return x.Timestamp
	}
	return nil
}

func (x *IngestWithConfigDataStreamRequest) GetChannelValues() []*IngestWithConfigDataChannelValue {
	if x != nil {
		return x.ChannelValues
	}
	return nil
}

func (x *IngestWithConfigDataStreamRequest) GetRunId() string {
	if x != nil {
		return x.RunId
	}
	return ""
}

func (x *IngestWithConfigDataStreamRequest) GetEndStreamOnValidationError() bool {
	if x != nil {
		return x.EndStreamOnValidationError
	}
	return false
}

func (x *IngestWithConfigDataStreamRequest) GetOrganizationId() string {
	if x != nil {
		return x.OrganizationId
	}
	return ""
}

type IngestWithConfigDataStreamResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *IngestWithConfigDataStreamResponse) Reset() {
	*x = IngestWithConfigDataStreamResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_ingest_v1_ingest_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IngestWithConfigDataStreamResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IngestWithConfigDataStreamResponse) ProtoMessage() {}

func (x *IngestWithConfigDataStreamResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_ingest_v1_ingest_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IngestWithConfigDataStreamResponse.ProtoReflect.Descriptor instead.
func (*IngestWithConfigDataStreamResponse) Descriptor() ([]byte, []int) {
	return file_sift_ingest_v1_ingest_proto_rawDescGZIP(), []int{1}
}

type IngestWithConfigDataChannelValue struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Type:
	//
	//	*IngestWithConfigDataChannelValue_String_
	//	*IngestWithConfigDataChannelValue_Double
	//	*IngestWithConfigDataChannelValue_Float
	//	*IngestWithConfigDataChannelValue_Bool
	//	*IngestWithConfigDataChannelValue_Int32
	//	*IngestWithConfigDataChannelValue_Uint32
	//	*IngestWithConfigDataChannelValue_Int64
	//	*IngestWithConfigDataChannelValue_Uint64
	//	*IngestWithConfigDataChannelValue_BitField
	//	*IngestWithConfigDataChannelValue_Enum
	//	*IngestWithConfigDataChannelValue_Empty
	//	*IngestWithConfigDataChannelValue_Bytes
	Type isIngestWithConfigDataChannelValue_Type `protobuf_oneof:"type"`
}

func (x *IngestWithConfigDataChannelValue) Reset() {
	*x = IngestWithConfigDataChannelValue{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_ingest_v1_ingest_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IngestWithConfigDataChannelValue) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IngestWithConfigDataChannelValue) ProtoMessage() {}

func (x *IngestWithConfigDataChannelValue) ProtoReflect() protoreflect.Message {
	mi := &file_sift_ingest_v1_ingest_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IngestWithConfigDataChannelValue.ProtoReflect.Descriptor instead.
func (*IngestWithConfigDataChannelValue) Descriptor() ([]byte, []int) {
	return file_sift_ingest_v1_ingest_proto_rawDescGZIP(), []int{2}
}

func (m *IngestWithConfigDataChannelValue) GetType() isIngestWithConfigDataChannelValue_Type {
	if m != nil {
		return m.Type
	}
	return nil
}

func (x *IngestWithConfigDataChannelValue) GetString_() string {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_String_); ok {
		return x.String_
	}
	return ""
}

func (x *IngestWithConfigDataChannelValue) GetDouble() float64 {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Double); ok {
		return x.Double
	}
	return 0
}

func (x *IngestWithConfigDataChannelValue) GetFloat() float32 {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Float); ok {
		return x.Float
	}
	return 0
}

func (x *IngestWithConfigDataChannelValue) GetBool() bool {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Bool); ok {
		return x.Bool
	}
	return false
}

func (x *IngestWithConfigDataChannelValue) GetInt32() int32 {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Int32); ok {
		return x.Int32
	}
	return 0
}

func (x *IngestWithConfigDataChannelValue) GetUint32() uint32 {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Uint32); ok {
		return x.Uint32
	}
	return 0
}

func (x *IngestWithConfigDataChannelValue) GetInt64() int64 {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Int64); ok {
		return x.Int64
	}
	return 0
}

func (x *IngestWithConfigDataChannelValue) GetUint64() uint64 {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Uint64); ok {
		return x.Uint64
	}
	return 0
}

func (x *IngestWithConfigDataChannelValue) GetBitField() []byte {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_BitField); ok {
		return x.BitField
	}
	return nil
}

func (x *IngestWithConfigDataChannelValue) GetEnum() uint32 {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Enum); ok {
		return x.Enum
	}
	return 0
}

func (x *IngestWithConfigDataChannelValue) GetEmpty() *emptypb.Empty {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Empty); ok {
		return x.Empty
	}
	return nil
}

func (x *IngestWithConfigDataChannelValue) GetBytes() []byte {
	if x, ok := x.GetType().(*IngestWithConfigDataChannelValue_Bytes); ok {
		return x.Bytes
	}
	return nil
}

type isIngestWithConfigDataChannelValue_Type interface {
	isIngestWithConfigDataChannelValue_Type()
}

type IngestWithConfigDataChannelValue_String_ struct {
	String_ string `protobuf:"bytes,1,opt,name=string,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Double struct {
	Double float64 `protobuf:"fixed64,2,opt,name=double,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Float struct {
	Float float32 `protobuf:"fixed32,3,opt,name=float,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Bool struct {
	Bool bool `protobuf:"varint,4,opt,name=bool,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Int32 struct {
	Int32 int32 `protobuf:"varint,5,opt,name=int32,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Uint32 struct {
	Uint32 uint32 `protobuf:"varint,6,opt,name=uint32,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Int64 struct {
	Int64 int64 `protobuf:"varint,7,opt,name=int64,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Uint64 struct {
	Uint64 uint64 `protobuf:"varint,8,opt,name=uint64,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_BitField struct {
	BitField []byte `protobuf:"bytes,9,opt,name=bit_field,json=bitField,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Enum struct {
	Enum uint32 `protobuf:"varint,10,opt,name=enum,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Empty struct {
	// If there's not a new data point for a channel at the given timestamp, pass empty to skip it
	Empty *emptypb.Empty `protobuf:"bytes,11,opt,name=empty,proto3,oneof"`
}

type IngestWithConfigDataChannelValue_Bytes struct {
	Bytes []byte `protobuf:"bytes,12,opt,name=bytes,proto3,oneof"`
}

func (*IngestWithConfigDataChannelValue_String_) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Double) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Float) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Bool) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Int32) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Uint32) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Int64) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Uint64) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_BitField) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Enum) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Empty) isIngestWithConfigDataChannelValue_Type() {}

func (*IngestWithConfigDataChannelValue_Bytes) isIngestWithConfigDataChannelValue_Type() {}

type IngestArbitraryProtobufDataStreamRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	MessageTypeIdentifier  string                 `protobuf:"bytes,1,opt,name=message_type_identifier,json=messageTypeIdentifier,proto3" json:"message_type_identifier,omitempty"`
	MessageTypeDisplayName *string                `protobuf:"bytes,2,opt,name=message_type_display_name,json=messageTypeDisplayName,proto3,oneof" json:"message_type_display_name,omitempty"`
	AssetName              string                 `protobuf:"bytes,3,opt,name=asset_name,json=assetName,proto3" json:"asset_name,omitempty"`
	Timestamp              *timestamppb.Timestamp `protobuf:"bytes,4,opt,name=timestamp,proto3" json:"timestamp,omitempty"`
	Value                  []byte                 `protobuf:"bytes,5,opt,name=value,proto3" json:"value,omitempty"`
	RunId                  string                 `protobuf:"bytes,6,opt,name=run_id,json=runId,proto3" json:"run_id,omitempty"`
	Namespace              string                 `protobuf:"bytes,7,opt,name=namespace,proto3" json:"namespace,omitempty"`
	OrganizationId         string                 `protobuf:"bytes,8,opt,name=organization_id,json=organizationId,proto3" json:"organization_id,omitempty"`
	// By default, if this request fails to parse for any reason, the request is
	// stored in an error queue and the stream continues to accept data. This
	// ensures all data is saved, but only valid data is fully ingested. If this
	// is set to `true`, any validation errors end the stream and return the error to the client.
	EndStreamOnValidationError bool `protobuf:"varint,9,opt,name=end_stream_on_validation_error,json=endStreamOnValidationError,proto3" json:"end_stream_on_validation_error,omitempty"`
}

func (x *IngestArbitraryProtobufDataStreamRequest) Reset() {
	*x = IngestArbitraryProtobufDataStreamRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_ingest_v1_ingest_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IngestArbitraryProtobufDataStreamRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IngestArbitraryProtobufDataStreamRequest) ProtoMessage() {}

func (x *IngestArbitraryProtobufDataStreamRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_ingest_v1_ingest_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IngestArbitraryProtobufDataStreamRequest.ProtoReflect.Descriptor instead.
func (*IngestArbitraryProtobufDataStreamRequest) Descriptor() ([]byte, []int) {
	return file_sift_ingest_v1_ingest_proto_rawDescGZIP(), []int{3}
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetMessageTypeIdentifier() string {
	if x != nil {
		return x.MessageTypeIdentifier
	}
	return ""
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetMessageTypeDisplayName() string {
	if x != nil && x.MessageTypeDisplayName != nil {
		return *x.MessageTypeDisplayName
	}
	return ""
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetAssetName() string {
	if x != nil {
		return x.AssetName
	}
	return ""
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetTimestamp() *timestamppb.Timestamp {
	if x != nil {
		return x.Timestamp
	}
	return nil
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetValue() []byte {
	if x != nil {
		return x.Value
	}
	return nil
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetRunId() string {
	if x != nil {
		return x.RunId
	}
	return ""
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetNamespace() string {
	if x != nil {
		return x.Namespace
	}
	return ""
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetOrganizationId() string {
	if x != nil {
		return x.OrganizationId
	}
	return ""
}

func (x *IngestArbitraryProtobufDataStreamRequest) GetEndStreamOnValidationError() bool {
	if x != nil {
		return x.EndStreamOnValidationError
	}
	return false
}

type IngestArbitraryProtobufDataStreamResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *IngestArbitraryProtobufDataStreamResponse) Reset() {
	*x = IngestArbitraryProtobufDataStreamResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_ingest_v1_ingest_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IngestArbitraryProtobufDataStreamResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IngestArbitraryProtobufDataStreamResponse) ProtoMessage() {}

func (x *IngestArbitraryProtobufDataStreamResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_ingest_v1_ingest_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IngestArbitraryProtobufDataStreamResponse.ProtoReflect.Descriptor instead.
func (*IngestArbitraryProtobufDataStreamResponse) Descriptor() ([]byte, []int) {
	return file_sift_ingest_v1_ingest_proto_rawDescGZIP(), []int{4}
}

var File_sift_ingest_v1_ingest_proto protoreflect.FileDescriptor

var file_sift_ingest_v1_ingest_proto_rawDesc = []byte{
	0x0a, 0x1b, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x2f, 0x76, 0x31,
	0x2f, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x73,
	0x69, 0x66, 0x74, 0x2e, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x2e, 0x76, 0x31, 0x1a, 0x1b, 0x67,
	0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x65,
	0x6d, 0x70, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65,
	0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfe, 0x02, 0x0a, 0x21,
	0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x57, 0x69, 0x74, 0x68, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
	0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x12, 0x2e, 0x0a, 0x13, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x63,
	0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11,
	0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x49,
	0x64, 0x12, 0x12, 0x0a, 0x04, 0x66, 0x6c, 0x6f, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x04, 0x66, 0x6c, 0x6f, 0x77, 0x12, 0x38, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
	0x6d, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
	0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73,
	0x74, 0x61, 0x6d, 0x70, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12,
	0x57, 0x0a, 0x0e, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65,
	0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x69,
	0x6e, 0x67, 0x65, 0x73, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x57,
	0x69, 0x74, 0x68, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x44, 0x61, 0x74, 0x61, 0x43, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x12, 0x15, 0x0a, 0x06, 0x72, 0x75, 0x6e, 0x5f,
	0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x72, 0x75, 0x6e, 0x49, 0x64, 0x12,
	0x42, 0x0a, 0x1e, 0x65, 0x6e, 0x64, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x6f, 0x6e,
	0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x72, 0x72, 0x6f,
	0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x1a, 0x65, 0x6e, 0x64, 0x53, 0x74, 0x72, 0x65,
	0x61, 0x6d, 0x4f, 0x6e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x72,
	0x72, 0x6f, 0x72, 0x12, 0x27, 0x0a, 0x0f, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x6f, 0x72,
	0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x24, 0x0a, 0x22,
	0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x57, 0x69, 0x74, 0x68, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
	0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
	0x73, 0x65, 0x22, 0xed, 0x02, 0x0a, 0x20, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x57, 0x69, 0x74,
	0x68, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x44, 0x61, 0x74, 0x61, 0x43, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x18, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e,
	0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e,
	0x67, 0x12, 0x18, 0x0a, 0x06, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x01, 0x48, 0x00, 0x52, 0x06, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x12, 0x16, 0x0a, 0x05, 0x66,
	0x6c, 0x6f, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x48, 0x00, 0x52, 0x05, 0x66, 0x6c,
	0x6f, 0x61, 0x74, 0x12, 0x14, 0x0a, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28,
	0x08, 0x48, 0x00, 0x52, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x12, 0x16, 0x0a, 0x05, 0x69, 0x6e, 0x74,
	0x33, 0x32, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x48, 0x00, 0x52, 0x05, 0x69, 0x6e, 0x74, 0x33,
	0x32, 0x12, 0x18, 0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x06, 0x20, 0x01, 0x28,
	0x0d, 0x48, 0x00, 0x52, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x12, 0x16, 0x0a, 0x05, 0x69,
	0x6e, 0x74, 0x36, 0x34, 0x18, 0x07, 0x20, 0x01, 0x28, 0x03, 0x48, 0x00, 0x52, 0x05, 0x69, 0x6e,
	0x74, 0x36, 0x34, 0x12, 0x18, 0x0a, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x08, 0x20,
	0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x06, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x12, 0x1d, 0x0a,
	0x09, 0x62, 0x69, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0c,
	0x48, 0x00, 0x52, 0x08, 0x62, 0x69, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x14, 0x0a, 0x04,
	0x65, 0x6e, 0x75, 0x6d, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x00, 0x52, 0x04, 0x65, 0x6e,
	0x75, 0x6d, 0x12, 0x2e, 0x0a, 0x05, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x18, 0x0b, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x48, 0x00, 0x52, 0x05, 0x65, 0x6d, 0x70,
	0x74, 0x79, 0x12, 0x16, 0x0a, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28,
	0x0c, 0x48, 0x00, 0x52, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73, 0x42, 0x06, 0x0a, 0x04, 0x74, 0x79,
	0x70, 0x65, 0x22, 0xd1, 0x03, 0x0a, 0x28, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x41, 0x72, 0x62,
	0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x44, 0x61,
	0x74, 0x61, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
	0x36, 0x0a, 0x17, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x5f,
	0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x15, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x49, 0x64, 0x65,
	0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x3e, 0x0a, 0x19, 0x6d, 0x65, 0x73, 0x73, 0x61,
	0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x64, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x5f,
	0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x16, 0x6d, 0x65,
	0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x44, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79,
	0x4e, 0x61, 0x6d, 0x65, 0x88, 0x01, 0x01, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x73, 0x73, 0x65, 0x74,
	0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x61, 0x73, 0x73,
	0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x38, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
	0x61, 0x6d, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65,
	0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
	0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52,
	0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x15, 0x0a, 0x06, 0x72, 0x75, 0x6e, 0x5f, 0x69, 0x64,
	0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x72, 0x75, 0x6e, 0x49, 0x64, 0x12, 0x1c, 0x0a,
	0x09, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x09, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x6f,
	0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x08,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x49, 0x64, 0x12, 0x42, 0x0a, 0x1e, 0x65, 0x6e, 0x64, 0x5f, 0x73, 0x74, 0x72, 0x65,
	0x61, 0x6d, 0x5f, 0x6f, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x1a, 0x65, 0x6e,
	0x64, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4f, 0x6e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x42, 0x1c, 0x0a, 0x1a, 0x5f, 0x6d, 0x65, 0x73,
	0x73, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x64, 0x69, 0x73, 0x70, 0x6c, 0x61,
	0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x2b, 0x0a, 0x29, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74,
	0x41, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
	0x66, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x32, 0xb4, 0x02, 0x0a, 0x0d, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x53, 0x65,
	0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x85, 0x01, 0x0a, 0x1a, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74,
	0x57, 0x69, 0x74, 0x68, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74,
	0x72, 0x65, 0x61, 0x6d, 0x12, 0x31, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x69, 0x6e, 0x67, 0x65,
	0x73, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x57, 0x69, 0x74, 0x68,
	0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x32, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x69,
	0x6e, 0x67, 0x65, 0x73, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x57,
	0x69, 0x74, 0x68, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x28, 0x01, 0x12, 0x9a, 0x01,
	0x0a, 0x21, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x41, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72,
	0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x72,
	0x65, 0x61, 0x6d, 0x12, 0x38, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x69, 0x6e, 0x67, 0x65, 0x73,
	0x74, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x41, 0x72, 0x62, 0x69, 0x74,
	0x72, 0x61, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x44, 0x61, 0x74, 0x61,
	0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x39, 0x2e,
	0x73, 0x69, 0x66, 0x74, 0x2e, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x49,
	0x6e, 0x67, 0x65, 0x73, 0x74, 0x41, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x50, 0x72,
	0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x28, 0x01, 0x42, 0xb6, 0x01, 0x0a, 0x12, 0x63,
	0x6f, 0x6d, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x2e, 0x76,
	0x31, 0x42, 0x0b, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
	0x5a, 0x39, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x69, 0x66,
	0x74, 0x2d, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x67, 0x6f, 0x2f,
	0x67, 0x65, 0x6e, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x2f,
	0x76, 0x31, 0x3b, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x76, 0x31, 0xa2, 0x02, 0x03, 0x53, 0x49,
	0x58, 0xaa, 0x02, 0x0e, 0x53, 0x69, 0x66, 0x74, 0x2e, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x2e,
	0x56, 0x31, 0xca, 0x02, 0x0e, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74,
	0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1a, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x49, 0x6e, 0x67, 0x65, 0x73,
	0x74, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
	0xea, 0x02, 0x10, 0x53, 0x69, 0x66, 0x74, 0x3a, 0x3a, 0x49, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x3a,
	0x3a, 0x56, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sift_ingest_v1_ingest_proto_rawDescOnce sync.Once
	file_sift_ingest_v1_ingest_proto_rawDescData = file_sift_ingest_v1_ingest_proto_rawDesc
)

func file_sift_ingest_v1_ingest_proto_rawDescGZIP() []byte {
	file_sift_ingest_v1_ingest_proto_rawDescOnce.Do(func() {
		file_sift_ingest_v1_ingest_proto_rawDescData = protoimpl.X.CompressGZIP(file_sift_ingest_v1_ingest_proto_rawDescData)
	})
	return file_sift_ingest_v1_ingest_proto_rawDescData
}

var file_sift_ingest_v1_ingest_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_sift_ingest_v1_ingest_proto_goTypes = []interface{}{
	(*IngestWithConfigDataStreamRequest)(nil),         // 0: sift.ingest.v1.IngestWithConfigDataStreamRequest
	(*IngestWithConfigDataStreamResponse)(nil),        // 1: sift.ingest.v1.IngestWithConfigDataStreamResponse
	(*IngestWithConfigDataChannelValue)(nil),          // 2: sift.ingest.v1.IngestWithConfigDataChannelValue
	(*IngestArbitraryProtobufDataStreamRequest)(nil),  // 3: sift.ingest.v1.IngestArbitraryProtobufDataStreamRequest
	(*IngestArbitraryProtobufDataStreamResponse)(nil), // 4: sift.ingest.v1.IngestArbitraryProtobufDataStreamResponse
	(*timestamppb.Timestamp)(nil),                     // 5: google.protobuf.Timestamp
	(*emptypb.Empty)(nil),                             // 6: google.protobuf.Empty
}
var file_sift_ingest_v1_ingest_proto_depIdxs = []int32{
	5, // 0: sift.ingest.v1.IngestWithConfigDataStreamRequest.timestamp:type_name -> google.protobuf.Timestamp
	2, // 1: sift.ingest.v1.IngestWithConfigDataStreamRequest.channel_values:type_name -> sift.ingest.v1.IngestWithConfigDataChannelValue
	6, // 2: sift.ingest.v1.IngestWithConfigDataChannelValue.empty:type_name -> google.protobuf.Empty
	5, // 3: sift.ingest.v1.IngestArbitraryProtobufDataStreamRequest.timestamp:type_name -> google.protobuf.Timestamp
	0, // 4: sift.ingest.v1.IngestService.IngestWithConfigDataStream:input_type -> sift.ingest.v1.IngestWithConfigDataStreamRequest
	3, // 5: sift.ingest.v1.IngestService.IngestArbitraryProtobufDataStream:input_type -> sift.ingest.v1.IngestArbitraryProtobufDataStreamRequest
	1, // 6: sift.ingest.v1.IngestService.IngestWithConfigDataStream:output_type -> sift.ingest.v1.IngestWithConfigDataStreamResponse
	4, // 7: sift.ingest.v1.IngestService.IngestArbitraryProtobufDataStream:output_type -> sift.ingest.v1.IngestArbitraryProtobufDataStreamResponse
	6, // [6:8] is the sub-list for method output_type
	4, // [4:6] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_sift_ingest_v1_ingest_proto_init() }
func file_sift_ingest_v1_ingest_proto_init() {
	if File_sift_ingest_v1_ingest_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sift_ingest_v1_ingest_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*IngestWithConfigDataStreamRequest); i {
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
		file_sift_ingest_v1_ingest_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*IngestWithConfigDataStreamResponse); i {
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
		file_sift_ingest_v1_ingest_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*IngestWithConfigDataChannelValue); i {
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
		file_sift_ingest_v1_ingest_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*IngestArbitraryProtobufDataStreamRequest); i {
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
		file_sift_ingest_v1_ingest_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*IngestArbitraryProtobufDataStreamResponse); i {
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
	file_sift_ingest_v1_ingest_proto_msgTypes[2].OneofWrappers = []interface{}{
		(*IngestWithConfigDataChannelValue_String_)(nil),
		(*IngestWithConfigDataChannelValue_Double)(nil),
		(*IngestWithConfigDataChannelValue_Float)(nil),
		(*IngestWithConfigDataChannelValue_Bool)(nil),
		(*IngestWithConfigDataChannelValue_Int32)(nil),
		(*IngestWithConfigDataChannelValue_Uint32)(nil),
		(*IngestWithConfigDataChannelValue_Int64)(nil),
		(*IngestWithConfigDataChannelValue_Uint64)(nil),
		(*IngestWithConfigDataChannelValue_BitField)(nil),
		(*IngestWithConfigDataChannelValue_Enum)(nil),
		(*IngestWithConfigDataChannelValue_Empty)(nil),
		(*IngestWithConfigDataChannelValue_Bytes)(nil),
	}
	file_sift_ingest_v1_ingest_proto_msgTypes[3].OneofWrappers = []interface{}{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_sift_ingest_v1_ingest_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_sift_ingest_v1_ingest_proto_goTypes,
		DependencyIndexes: file_sift_ingest_v1_ingest_proto_depIdxs,
		MessageInfos:      file_sift_ingest_v1_ingest_proto_msgTypes,
	}.Build()
	File_sift_ingest_v1_ingest_proto = out.File
	file_sift_ingest_v1_ingest_proto_rawDesc = nil
	file_sift_ingest_v1_ingest_proto_goTypes = nil
	file_sift_ingest_v1_ingest_proto_depIdxs = nil
}
