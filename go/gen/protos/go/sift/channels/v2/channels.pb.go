// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        (unknown)
// source: sift/channels/v2/channels.proto

package channelsv2

import (
	_ "github.com/sift-stack/sift/go/gen/protos/go/google/api"
	_ "github.com/sift-stack/sift/go/gen/protos/go/protoc-gen-openapiv2/options"
	v1 "github.com/sift-stack/sift/go/gen/protos/go/sift/common/type/v1"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
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

type Channel struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ChannelId        string                       `protobuf:"bytes,1,opt,name=channel_id,json=channelId,proto3" json:"channel_id,omitempty"`
	Name             string                       `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	Component        string                       `protobuf:"bytes,3,opt,name=component,proto3" json:"component,omitempty"`
	AssetId          string                       `protobuf:"bytes,4,opt,name=asset_id,json=assetId,proto3" json:"asset_id,omitempty"`
	Description      string                       `protobuf:"bytes,5,opt,name=description,proto3" json:"description,omitempty"`
	UnitId           string                       `protobuf:"bytes,6,opt,name=unit_id,json=unitId,proto3" json:"unit_id,omitempty"`
	CreatedDate      *timestamppb.Timestamp       `protobuf:"bytes,7,opt,name=created_date,json=createdDate,proto3" json:"created_date,omitempty"`
	ModifiedDate     *timestamppb.Timestamp       `protobuf:"bytes,8,opt,name=modified_date,json=modifiedDate,proto3" json:"modified_date,omitempty"`
	CreatedByUserId  string                       `protobuf:"bytes,9,opt,name=created_by_user_id,json=createdByUserId,proto3" json:"created_by_user_id,omitempty"`
	ModifiedByUserId string                       `protobuf:"bytes,10,opt,name=modified_by_user_id,json=modifiedByUserId,proto3" json:"modified_by_user_id,omitempty"`
	OrganizationId   string                       `protobuf:"bytes,11,opt,name=organization_id,json=organizationId,proto3" json:"organization_id,omitempty"`
	DataType         v1.ChannelDataType           `protobuf:"varint,12,opt,name=data_type,json=dataType,proto3,enum=sift.common.type.v1.ChannelDataType" json:"data_type,omitempty"`
	EnumTypes        []*v1.ChannelEnumType        `protobuf:"bytes,13,rep,name=enum_types,json=enumTypes,proto3" json:"enum_types,omitempty"`
	BitFieldElements []*v1.ChannelBitFieldElement `protobuf:"bytes,14,rep,name=bit_field_elements,json=bitFieldElements,proto3" json:"bit_field_elements,omitempty"`
}

func (x *Channel) Reset() {
	*x = Channel{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_channels_v2_channels_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Channel) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Channel) ProtoMessage() {}

func (x *Channel) ProtoReflect() protoreflect.Message {
	mi := &file_sift_channels_v2_channels_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Channel.ProtoReflect.Descriptor instead.
func (*Channel) Descriptor() ([]byte, []int) {
	return file_sift_channels_v2_channels_proto_rawDescGZIP(), []int{0}
}

func (x *Channel) GetChannelId() string {
	if x != nil {
		return x.ChannelId
	}
	return ""
}

func (x *Channel) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Channel) GetComponent() string {
	if x != nil {
		return x.Component
	}
	return ""
}

func (x *Channel) GetAssetId() string {
	if x != nil {
		return x.AssetId
	}
	return ""
}

func (x *Channel) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Channel) GetUnitId() string {
	if x != nil {
		return x.UnitId
	}
	return ""
}

func (x *Channel) GetCreatedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.CreatedDate
	}
	return nil
}

func (x *Channel) GetModifiedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.ModifiedDate
	}
	return nil
}

func (x *Channel) GetCreatedByUserId() string {
	if x != nil {
		return x.CreatedByUserId
	}
	return ""
}

func (x *Channel) GetModifiedByUserId() string {
	if x != nil {
		return x.ModifiedByUserId
	}
	return ""
}

func (x *Channel) GetOrganizationId() string {
	if x != nil {
		return x.OrganizationId
	}
	return ""
}

func (x *Channel) GetDataType() v1.ChannelDataType {
	if x != nil {
		return x.DataType
	}
	return v1.ChannelDataType(0)
}

func (x *Channel) GetEnumTypes() []*v1.ChannelEnumType {
	if x != nil {
		return x.EnumTypes
	}
	return nil
}

func (x *Channel) GetBitFieldElements() []*v1.ChannelBitFieldElement {
	if x != nil {
		return x.BitFieldElements
	}
	return nil
}

// The request for a call to `ChannelService_GetChannel`.
type GetChannelRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ChannelId string `protobuf:"bytes,1,opt,name=channel_id,json=channelId,proto3" json:"channel_id,omitempty"`
}

func (x *GetChannelRequest) Reset() {
	*x = GetChannelRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_channels_v2_channels_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetChannelRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetChannelRequest) ProtoMessage() {}

func (x *GetChannelRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_channels_v2_channels_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetChannelRequest.ProtoReflect.Descriptor instead.
func (*GetChannelRequest) Descriptor() ([]byte, []int) {
	return file_sift_channels_v2_channels_proto_rawDescGZIP(), []int{1}
}

func (x *GetChannelRequest) GetChannelId() string {
	if x != nil {
		return x.ChannelId
	}
	return ""
}

// The response of a call to `ChannelService_GetChannel`.
type GetChannelResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Channel *Channel `protobuf:"bytes,1,opt,name=channel,proto3" json:"channel,omitempty"`
}

func (x *GetChannelResponse) Reset() {
	*x = GetChannelResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_channels_v2_channels_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetChannelResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetChannelResponse) ProtoMessage() {}

func (x *GetChannelResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_channels_v2_channels_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetChannelResponse.ProtoReflect.Descriptor instead.
func (*GetChannelResponse) Descriptor() ([]byte, []int) {
	return file_sift_channels_v2_channels_proto_rawDescGZIP(), []int{2}
}

func (x *GetChannelResponse) GetChannel() *Channel {
	if x != nil {
		return x.Channel
	}
	return nil
}

// The request for a call to `ChannelService_ListChannels` to retrieve channels.
type ListChannelsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The maximum number of channels to return. The service may return fewer than this value.
	// If unspecified, at most 50 channels will be returned. The maximum value is 10,000; values above
	// 10,000 will be coerced to 10,000. Optional.
	PageSize uint32 `protobuf:"varint,1,opt,name=page_size,json=pageSize,proto3" json:"page_size,omitempty"`
	// A page token, received from a previous `ListChannels` call.
	// Provide this to retrieve the subsequent page.
	// When paginating, all other parameters provided to `ListChannels` must match
	// the call that provided the page token. Optional.
	PageToken string `protobuf:"bytes,2,opt,name=page_token,json=pageToken,proto3" json:"page_token,omitempty"`
	// A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
	// Available fields to filter by are `channel_id`, `asset_id`, `name`, `component`, `description`, `active`, `created_date`, and `modified_date`.
	// For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
	// For more information about the fields used for filtering, please refer to [this definition](/api/grpc/protocol_buffers/channels#channel). Optional.
	Filter string `protobuf:"bytes,3,opt,name=filter,proto3" json:"filter,omitempty"`
	// This field is only required if your user belongs to multiple organizations.
	OrganizationId string `protobuf:"bytes,4,opt,name=organization_id,json=organizationId,proto3" json:"organization_id,omitempty"`
	// How to order the retrieved channels. Formatted as a comma-separated string i.e. "<field_name>[ desc],...".
	// Available fields to order_by are `created_date` and `modified_date`.
	// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
	// For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
	// Example: "created_date desc,modified_date"
	OrderBy string `protobuf:"bytes,5,opt,name=order_by,json=orderBy,proto3" json:"order_by,omitempty"`
}

func (x *ListChannelsRequest) Reset() {
	*x = ListChannelsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_channels_v2_channels_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListChannelsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListChannelsRequest) ProtoMessage() {}

func (x *ListChannelsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_channels_v2_channels_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListChannelsRequest.ProtoReflect.Descriptor instead.
func (*ListChannelsRequest) Descriptor() ([]byte, []int) {
	return file_sift_channels_v2_channels_proto_rawDescGZIP(), []int{3}
}

func (x *ListChannelsRequest) GetPageSize() uint32 {
	if x != nil {
		return x.PageSize
	}
	return 0
}

func (x *ListChannelsRequest) GetPageToken() string {
	if x != nil {
		return x.PageToken
	}
	return ""
}

func (x *ListChannelsRequest) GetFilter() string {
	if x != nil {
		return x.Filter
	}
	return ""
}

func (x *ListChannelsRequest) GetOrganizationId() string {
	if x != nil {
		return x.OrganizationId
	}
	return ""
}

func (x *ListChannelsRequest) GetOrderBy() string {
	if x != nil {
		return x.OrderBy
	}
	return ""
}

// The result of a call to `ChannelService_ListChannels`.
type ListChannelsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Channels      []*Channel `protobuf:"bytes,1,rep,name=channels,proto3" json:"channels,omitempty"`
	NextPageToken string     `protobuf:"bytes,2,opt,name=next_page_token,json=nextPageToken,proto3" json:"next_page_token,omitempty"`
}

func (x *ListChannelsResponse) Reset() {
	*x = ListChannelsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_channels_v2_channels_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListChannelsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListChannelsResponse) ProtoMessage() {}

func (x *ListChannelsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_channels_v2_channels_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListChannelsResponse.ProtoReflect.Descriptor instead.
func (*ListChannelsResponse) Descriptor() ([]byte, []int) {
	return file_sift_channels_v2_channels_proto_rawDescGZIP(), []int{4}
}

func (x *ListChannelsResponse) GetChannels() []*Channel {
	if x != nil {
		return x.Channels
	}
	return nil
}

func (x *ListChannelsResponse) GetNextPageToken() string {
	if x != nil {
		return x.NextPageToken
	}
	return ""
}

var File_sift_channels_v2_channels_proto protoreflect.FileDescriptor

var file_sift_channels_v2_channels_proto_rawDesc = []byte{
	0x0a, 0x1f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2f,
	0x76, 0x32, 0x2f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x12, 0x10, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73,
	0x2e, 0x76, 0x32, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f,
	0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x66, 0x69,
	0x65, 0x6c, 0x64, 0x5f, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x1a, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x2d, 0x67, 0x65, 0x6e, 0x2d,
	0x6f, 0x70, 0x65, 0x6e, 0x61, 0x70, 0x69, 0x76, 0x32, 0x2f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
	0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x1a, 0x33, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
	0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
	0x5f, 0x62, 0x69, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x65, 0x6c, 0x65, 0x6d, 0x65,
	0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2b, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x63,
	0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x68,
	0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2b, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x63, 0x6f, 0x6d, 0x6d,
	0x6f, 0x6e, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0xd4, 0x05, 0x0a, 0x07, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x22,
	0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
	0x49, 0x64, 0x12, 0x17, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x09, 0x63,
	0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03,
	0xe0, 0x41, 0x02, 0x52, 0x09, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x12, 0x1e,
	0x0a, 0x08, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
	0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x07, 0x61, 0x73, 0x73, 0x65, 0x74, 0x49, 0x64, 0x12, 0x25,
	0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20,
	0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
	0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1c, 0x0a, 0x07, 0x75, 0x6e, 0x69, 0x74, 0x5f, 0x69, 0x64,
	0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x06, 0x75, 0x6e, 0x69,
	0x74, 0x49, 0x64, 0x12, 0x42, 0x0a, 0x0c, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x64,
	0x61, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65,
	0x73, 0x74, 0x61, 0x6d, 0x70, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0b, 0x63, 0x72, 0x65, 0x61,
	0x74, 0x65, 0x64, 0x44, 0x61, 0x74, 0x65, 0x12, 0x44, 0x0a, 0x0d, 0x6d, 0x6f, 0x64, 0x69, 0x66,
	0x69, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
	0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
	0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52,
	0x0c, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x44, 0x61, 0x74, 0x65, 0x12, 0x30, 0x0a,
	0x12, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x75, 0x73, 0x65, 0x72,
	0x5f, 0x69, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0f,
	0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x42, 0x79, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12,
	0x32, 0x0a, 0x13, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x75,
	0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41,
	0x02, 0x52, 0x10, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x42, 0x79, 0x55, 0x73, 0x65,
	0x72, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x0f, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41,
	0x02, 0x52, 0x0e, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49,
	0x64, 0x12, 0x46, 0x0a, 0x09, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0c,
	0x20, 0x01, 0x28, 0x0e, 0x32, 0x24, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x63, 0x6f, 0x6d, 0x6d,
	0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52,
	0x08, 0x64, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x43, 0x0a, 0x0a, 0x65, 0x6e, 0x75,
	0x6d, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e,
	0x73, 0x69, 0x66, 0x74, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65,
	0x2e, 0x76, 0x31, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x45, 0x6e, 0x75, 0x6d, 0x54,
	0x79, 0x70, 0x65, 0x52, 0x09, 0x65, 0x6e, 0x75, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x73, 0x12, 0x59,
	0x0a, 0x12, 0x62, 0x69, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x65, 0x6c, 0x65, 0x6d,
	0x65, 0x6e, 0x74, 0x73, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x73, 0x69, 0x66,
	0x74, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x31,
	0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x42, 0x69, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64,
	0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x10, 0x62, 0x69, 0x74, 0x46, 0x69, 0x65, 0x6c,
	0x64, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x22, 0x37, 0x0a, 0x11, 0x47, 0x65, 0x74,
	0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x22,
	0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
	0x49, 0x64, 0x22, 0x4e, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x38, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e,
	0x6e, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x73, 0x69, 0x66, 0x74,
	0x2e, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x22, 0xc6, 0x01, 0x0a, 0x13, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x20, 0x0a, 0x09, 0x70, 0x61,
	0x67, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x03, 0xe0,
	0x41, 0x01, 0x52, 0x08, 0x70, 0x61, 0x67, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x22, 0x0a, 0x0a,
	0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x09, 0x70, 0x61, 0x67, 0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e,
	0x12, 0x1b, 0x0a, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
	0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x12, 0x2c, 0x0a,
	0x0f, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64,
	0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x0e, 0x6f, 0x72, 0x67,
	0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1e, 0x0a, 0x08, 0x6f,
	0x72, 0x64, 0x65, 0x72, 0x5f, 0x62, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0,
	0x41, 0x01, 0x52, 0x07, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x42, 0x79, 0x22, 0x75, 0x0a, 0x14, 0x4c,
	0x69, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x35, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x18,
	0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x63, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
	0x52, 0x08, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x12, 0x26, 0x0a, 0x0f, 0x6e, 0x65,
	0x78, 0x74, 0x5f, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x0d, 0x6e, 0x65, 0x78, 0x74, 0x50, 0x61, 0x67, 0x65, 0x54, 0x6f, 0x6b,
	0x65, 0x6e, 0x32, 0xbc, 0x04, 0x0a, 0x0e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x53, 0x65,
	0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0xa2, 0x01, 0x0a, 0x0a, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x23, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x63, 0x68, 0x61, 0x6e,
	0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x24, 0x2e, 0x73, 0x69, 0x66, 0x74,
	0x2e, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x47, 0x65, 0x74,
	0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
	0x49, 0x92, 0x41, 0x21, 0x12, 0x0a, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
	0x1a, 0x13, 0x52, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x20, 0x61, 0x20, 0x63, 0x68, 0x61,
	0x6e, 0x6e, 0x65, 0x6c, 0x2e, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x1f, 0x12, 0x1d, 0x2f, 0x61, 0x70,
	0x69, 0x2f, 0x76, 0x32, 0x2f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2f, 0x7b, 0x63,
	0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x7d, 0x12, 0xb5, 0x01, 0x0a, 0x0c, 0x4c,
	0x69, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x12, 0x25, 0x2e, 0x73, 0x69,
	0x66, 0x74, 0x2e, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x4c,
	0x69, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
	0x73, 0x74, 0x1a, 0x26, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
	0x6c, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65,
	0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x56, 0x92, 0x41, 0x3b, 0x12,
	0x0c, 0x4c, 0x69, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x1a, 0x2b, 0x52,
	0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73,
	0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
	0x61, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x12,
	0x12, 0x10, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x32, 0x2f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
	0x6c, 0x73, 0x1a, 0xcc, 0x01, 0x92, 0x41, 0xc8, 0x01, 0x12, 0x48, 0x53, 0x65, 0x72, 0x76, 0x69,
	0x63, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x6d, 0x61, 0x74,
	0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74, 0x20,
	0x77, 0x69, 0x74, 0x68, 0x20, 0x5b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x5d, 0x28,
	0x2f, 0x67, 0x6c, 0x6f, 0x73, 0x73, 0x61, 0x72, 0x79, 0x23, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
	0x6c, 0x29, 0x2e, 0x1a, 0x7c, 0x0a, 0x22, 0x52, 0x65, 0x61, 0x64, 0x20, 0x6d, 0x6f, 0x72, 0x65,
	0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x63, 0x68, 0x61, 0x6e,
	0x6e, 0x65, 0x6c, 0x73, 0x20, 0x61, 0x72, 0x65, 0x2e, 0x12, 0x56, 0x68, 0x74, 0x74, 0x70, 0x73,
	0x3a, 0x2f, 0x2f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x65, 0x72, 0x2e, 0x73, 0x75, 0x70, 0x70,
	0x6f, 0x72, 0x74, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x63, 0x6f,
	0x6d, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x64, 0x65, 0x73, 0x6b, 0x2f, 0x63, 0x75,
	0x73, 0x74, 0x6f, 0x6d, 0x65, 0x72, 0x2f, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x6c, 0x2f, 0x32, 0x2f,
	0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x2f, 0x32, 0x36, 0x35, 0x34, 0x35, 0x33, 0x39, 0x34,
	0x33, 0x42, 0xe6, 0x01, 0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x63,
	0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x76, 0x32, 0x42, 0x0d, 0x43, 0x68, 0x61, 0x6e,
	0x6e, 0x65, 0x6c, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x47, 0x67, 0x69, 0x74,
	0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2d, 0x73, 0x74, 0x61,
	0x63, 0x6b, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x67, 0x6f, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x67, 0x6f, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x63, 0x68,
	0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2f, 0x76, 0x32, 0x3b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
	0x6c, 0x73, 0x76, 0x32, 0xa2, 0x02, 0x03, 0x53, 0x43, 0x58, 0xaa, 0x02, 0x10, 0x53, 0x69, 0x66,
	0x74, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x2e, 0x56, 0x32, 0xca, 0x02, 0x10,
	0x53, 0x69, 0x66, 0x74, 0x5c, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x5c, 0x56, 0x32,
	0xe2, 0x02, 0x1c, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73,
	0x5c, 0x56, 0x32, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
	0x02, 0x12, 0x53, 0x69, 0x66, 0x74, 0x3a, 0x3a, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73,
	0x3a, 0x3a, 0x56, 0x32, 0x92, 0x41, 0x13, 0x12, 0x11, 0x0a, 0x0f, 0x43, 0x68, 0x61, 0x6e, 0x6e,
	0x65, 0x6c, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x33,
}

var (
	file_sift_channels_v2_channels_proto_rawDescOnce sync.Once
	file_sift_channels_v2_channels_proto_rawDescData = file_sift_channels_v2_channels_proto_rawDesc
)

func file_sift_channels_v2_channels_proto_rawDescGZIP() []byte {
	file_sift_channels_v2_channels_proto_rawDescOnce.Do(func() {
		file_sift_channels_v2_channels_proto_rawDescData = protoimpl.X.CompressGZIP(file_sift_channels_v2_channels_proto_rawDescData)
	})
	return file_sift_channels_v2_channels_proto_rawDescData
}

var file_sift_channels_v2_channels_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_sift_channels_v2_channels_proto_goTypes = []interface{}{
	(*Channel)(nil),                   // 0: sift.channels.v2.Channel
	(*GetChannelRequest)(nil),         // 1: sift.channels.v2.GetChannelRequest
	(*GetChannelResponse)(nil),        // 2: sift.channels.v2.GetChannelResponse
	(*ListChannelsRequest)(nil),       // 3: sift.channels.v2.ListChannelsRequest
	(*ListChannelsResponse)(nil),      // 4: sift.channels.v2.ListChannelsResponse
	(*timestamppb.Timestamp)(nil),     // 5: google.protobuf.Timestamp
	(v1.ChannelDataType)(0),           // 6: sift.common.type.v1.ChannelDataType
	(*v1.ChannelEnumType)(nil),        // 7: sift.common.type.v1.ChannelEnumType
	(*v1.ChannelBitFieldElement)(nil), // 8: sift.common.type.v1.ChannelBitFieldElement
}
var file_sift_channels_v2_channels_proto_depIdxs = []int32{
	5, // 0: sift.channels.v2.Channel.created_date:type_name -> google.protobuf.Timestamp
	5, // 1: sift.channels.v2.Channel.modified_date:type_name -> google.protobuf.Timestamp
	6, // 2: sift.channels.v2.Channel.data_type:type_name -> sift.common.type.v1.ChannelDataType
	7, // 3: sift.channels.v2.Channel.enum_types:type_name -> sift.common.type.v1.ChannelEnumType
	8, // 4: sift.channels.v2.Channel.bit_field_elements:type_name -> sift.common.type.v1.ChannelBitFieldElement
	0, // 5: sift.channels.v2.GetChannelResponse.channel:type_name -> sift.channels.v2.Channel
	0, // 6: sift.channels.v2.ListChannelsResponse.channels:type_name -> sift.channels.v2.Channel
	1, // 7: sift.channels.v2.ChannelService.GetChannel:input_type -> sift.channels.v2.GetChannelRequest
	3, // 8: sift.channels.v2.ChannelService.ListChannels:input_type -> sift.channels.v2.ListChannelsRequest
	2, // 9: sift.channels.v2.ChannelService.GetChannel:output_type -> sift.channels.v2.GetChannelResponse
	4, // 10: sift.channels.v2.ChannelService.ListChannels:output_type -> sift.channels.v2.ListChannelsResponse
	9, // [9:11] is the sub-list for method output_type
	7, // [7:9] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_sift_channels_v2_channels_proto_init() }
func file_sift_channels_v2_channels_proto_init() {
	if File_sift_channels_v2_channels_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sift_channels_v2_channels_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Channel); i {
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
		file_sift_channels_v2_channels_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetChannelRequest); i {
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
		file_sift_channels_v2_channels_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetChannelResponse); i {
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
		file_sift_channels_v2_channels_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListChannelsRequest); i {
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
		file_sift_channels_v2_channels_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListChannelsResponse); i {
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
			RawDescriptor: file_sift_channels_v2_channels_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_sift_channels_v2_channels_proto_goTypes,
		DependencyIndexes: file_sift_channels_v2_channels_proto_depIdxs,
		MessageInfos:      file_sift_channels_v2_channels_proto_msgTypes,
	}.Build()
	File_sift_channels_v2_channels_proto = out.File
	file_sift_channels_v2_channels_proto_rawDesc = nil
	file_sift_channels_v2_channels_proto_goTypes = nil
	file_sift_channels_v2_channels_proto_depIdxs = nil
}