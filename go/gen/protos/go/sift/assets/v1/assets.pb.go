// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        (unknown)
// source: sift/assets/v1/assets.proto

package assetsv1

import (
	_ "github.com/sift-stack/sift/go/gen/protos/go/google/api"
	_ "github.com/sift-stack/sift/go/gen/protos/go/protoc-gen-openapiv2/options"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	fieldmaskpb "google.golang.org/protobuf/types/known/fieldmaskpb"
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

type Asset struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	AssetId          string                 `protobuf:"bytes,1,opt,name=asset_id,json=assetId,proto3" json:"asset_id,omitempty"`
	Name             string                 `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	OrganizationId   string                 `protobuf:"bytes,4,opt,name=organization_id,json=organizationId,proto3" json:"organization_id,omitempty"`
	CreatedDate      *timestamppb.Timestamp `protobuf:"bytes,5,opt,name=created_date,json=createdDate,proto3" json:"created_date,omitempty"`
	CreatedByUserId  string                 `protobuf:"bytes,6,opt,name=created_by_user_id,json=createdByUserId,proto3" json:"created_by_user_id,omitempty"`
	ModifiedDate     *timestamppb.Timestamp `protobuf:"bytes,7,opt,name=modified_date,json=modifiedDate,proto3" json:"modified_date,omitempty"`
	ModifiedByUserId string                 `protobuf:"bytes,8,opt,name=modified_by_user_id,json=modifiedByUserId,proto3" json:"modified_by_user_id,omitempty"`
	// The names of the tags to associate with this asset.
	Tags []string `protobuf:"bytes,9,rep,name=tags,proto3" json:"tags,omitempty"`
}

func (x *Asset) Reset() {
	*x = Asset{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Asset) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Asset) ProtoMessage() {}

func (x *Asset) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Asset.ProtoReflect.Descriptor instead.
func (*Asset) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{0}
}

func (x *Asset) GetAssetId() string {
	if x != nil {
		return x.AssetId
	}
	return ""
}

func (x *Asset) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Asset) GetOrganizationId() string {
	if x != nil {
		return x.OrganizationId
	}
	return ""
}

func (x *Asset) GetCreatedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.CreatedDate
	}
	return nil
}

func (x *Asset) GetCreatedByUserId() string {
	if x != nil {
		return x.CreatedByUserId
	}
	return ""
}

func (x *Asset) GetModifiedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.ModifiedDate
	}
	return nil
}

func (x *Asset) GetModifiedByUserId() string {
	if x != nil {
		return x.ModifiedByUserId
	}
	return ""
}

func (x *Asset) GetTags() []string {
	if x != nil {
		return x.Tags
	}
	return nil
}

// The request for a call to `AssetService_ListAssets`.
type ListAssetsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The maximum number of assets to return.
	// The service may return fewer than this value.
	// If unspecified, at most 50 assets will be returned.
	// The maximum value is 1000; values above 1000 will be coerced to 1000.
	PageSize uint32 `protobuf:"varint,1,opt,name=page_size,json=pageSize,proto3" json:"page_size,omitempty"`
	// A page token, received from a previous `ListAssets` call.
	// Provide this to retrieve the subsequent page.
	// When paginating, all other parameters provided to `ListAssets` must match
	// the call that provided the page token.
	PageToken string `protobuf:"bytes,2,opt,name=page_token,json=pageToken,proto3" json:"page_token,omitempty"`
	// A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
	// Available fields to filter by are `asset_id`, `created_by_user_id`, `modified_by_user_id`,
	// `created_date`, `modified_date`, and `name`.
	// For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
	// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#asset). Optional.
	Filter string `protobuf:"bytes,3,opt,name=filter,proto3" json:"filter,omitempty"`
	// How to order the retrieved assets. Formatted as a comma-separated string i.e. "<field_name>[ desc],...".
	// Available fields to order_by are `created_date` and `modified_date`.
	// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
	// For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
	// Example: "created_date desc,modified_date"
	OrderBy string `protobuf:"bytes,4,opt,name=order_by,json=orderBy,proto3" json:"order_by,omitempty"`
}

func (x *ListAssetsRequest) Reset() {
	*x = ListAssetsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListAssetsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListAssetsRequest) ProtoMessage() {}

func (x *ListAssetsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListAssetsRequest.ProtoReflect.Descriptor instead.
func (*ListAssetsRequest) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{1}
}

func (x *ListAssetsRequest) GetPageSize() uint32 {
	if x != nil {
		return x.PageSize
	}
	return 0
}

func (x *ListAssetsRequest) GetPageToken() string {
	if x != nil {
		return x.PageToken
	}
	return ""
}

func (x *ListAssetsRequest) GetFilter() string {
	if x != nil {
		return x.Filter
	}
	return ""
}

func (x *ListAssetsRequest) GetOrderBy() string {
	if x != nil {
		return x.OrderBy
	}
	return ""
}

// The result of a call to `AssetService_ListAssets`.
type ListAssetsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Assets        []*Asset `protobuf:"bytes,1,rep,name=assets,proto3" json:"assets,omitempty"`
	NextPageToken string   `protobuf:"bytes,5,opt,name=next_page_token,json=nextPageToken,proto3" json:"next_page_token,omitempty"`
}

func (x *ListAssetsResponse) Reset() {
	*x = ListAssetsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListAssetsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListAssetsResponse) ProtoMessage() {}

func (x *ListAssetsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListAssetsResponse.ProtoReflect.Descriptor instead.
func (*ListAssetsResponse) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{2}
}

func (x *ListAssetsResponse) GetAssets() []*Asset {
	if x != nil {
		return x.Assets
	}
	return nil
}

func (x *ListAssetsResponse) GetNextPageToken() string {
	if x != nil {
		return x.NextPageToken
	}
	return ""
}

// The request for a call to `AssetService_DeleteAsset` to delete a single existing annotation by its asset_id.
type DeleteAssetRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The id of the asset to be deleted. Required.
	AssetId string `protobuf:"bytes,1,opt,name=asset_id,json=assetId,proto3" json:"asset_id,omitempty"`
}

func (x *DeleteAssetRequest) Reset() {
	*x = DeleteAssetRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DeleteAssetRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeleteAssetRequest) ProtoMessage() {}

func (x *DeleteAssetRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DeleteAssetRequest.ProtoReflect.Descriptor instead.
func (*DeleteAssetRequest) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{3}
}

func (x *DeleteAssetRequest) GetAssetId() string {
	if x != nil {
		return x.AssetId
	}
	return ""
}

// The response of a call to `AssetService_DeleteAsset`.
type DeleteAssetResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *DeleteAssetResponse) Reset() {
	*x = DeleteAssetResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DeleteAssetResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DeleteAssetResponse) ProtoMessage() {}

func (x *DeleteAssetResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DeleteAssetResponse.ProtoReflect.Descriptor instead.
func (*DeleteAssetResponse) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{4}
}

// The request for a call to `AssetService_GetAsset` to retrieve a single existing asset by its asset_id.
type GetAssetRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The id of the asset to be retrieved. Required.
	AssetId string `protobuf:"bytes,1,opt,name=asset_id,json=assetId,proto3" json:"asset_id,omitempty"`
}

func (x *GetAssetRequest) Reset() {
	*x = GetAssetRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetAssetRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetAssetRequest) ProtoMessage() {}

func (x *GetAssetRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetAssetRequest.ProtoReflect.Descriptor instead.
func (*GetAssetRequest) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{5}
}

func (x *GetAssetRequest) GetAssetId() string {
	if x != nil {
		return x.AssetId
	}
	return ""
}

type GetAssetResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Asset *Asset `protobuf:"bytes,1,opt,name=asset,proto3" json:"asset,omitempty"`
}

func (x *GetAssetResponse) Reset() {
	*x = GetAssetResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetAssetResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetAssetResponse) ProtoMessage() {}

func (x *GetAssetResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetAssetResponse.ProtoReflect.Descriptor instead.
func (*GetAssetResponse) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{6}
}

func (x *GetAssetResponse) GetAsset() *Asset {
	if x != nil {
		return x.Asset
	}
	return nil
}

type UpdateAssetRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The asset to update. The asset's `asset_id` field is used to identify asset run to update
	// and is required.
	Asset *Asset `protobuf:"bytes,1,opt,name=asset,proto3" json:"asset,omitempty"`
	// The list of fields to be updated. Currently, the only field available to be updated is `tags`.
	UpdateMask *fieldmaskpb.FieldMask `protobuf:"bytes,2,opt,name=update_mask,json=updateMask,proto3" json:"update_mask,omitempty"`
}

func (x *UpdateAssetRequest) Reset() {
	*x = UpdateAssetRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UpdateAssetRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateAssetRequest) ProtoMessage() {}

func (x *UpdateAssetRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UpdateAssetRequest.ProtoReflect.Descriptor instead.
func (*UpdateAssetRequest) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{7}
}

func (x *UpdateAssetRequest) GetAsset() *Asset {
	if x != nil {
		return x.Asset
	}
	return nil
}

func (x *UpdateAssetRequest) GetUpdateMask() *fieldmaskpb.FieldMask {
	if x != nil {
		return x.UpdateMask
	}
	return nil
}

type UpdateAssetResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Asset *Asset `protobuf:"bytes,1,opt,name=asset,proto3" json:"asset,omitempty"`
}

func (x *UpdateAssetResponse) Reset() {
	*x = UpdateAssetResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_assets_v1_assets_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UpdateAssetResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateAssetResponse) ProtoMessage() {}

func (x *UpdateAssetResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_assets_v1_assets_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UpdateAssetResponse.ProtoReflect.Descriptor instead.
func (*UpdateAssetResponse) Descriptor() ([]byte, []int) {
	return file_sift_assets_v1_assets_proto_rawDescGZIP(), []int{8}
}

func (x *UpdateAssetResponse) GetAsset() *Asset {
	if x != nil {
		return x.Asset
	}
	return nil
}

var File_sift_assets_v1_assets_proto protoreflect.FileDescriptor

var file_sift_assets_v1_assets_proto_rawDesc = []byte{
	0x0a, 0x1b, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2f, 0x76, 0x31,
	0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x73,
	0x69, 0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x1a, 0x1c, 0x67,
	0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f,
	0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x62, 0x65,
	0x68, 0x61, 0x76, 0x69, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f,
	0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x66, 0x69,
	0x65, 0x6c, 0x64, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f,
	0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f,
	0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x2d, 0x67, 0x65, 0x6e, 0x2d, 0x6f, 0x70, 0x65, 0x6e,
	0x61, 0x70, 0x69, 0x76, 0x32, 0x2f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x61, 0x6e,
	0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
	0xf7, 0x02, 0x0a, 0x05, 0x41, 0x73, 0x73, 0x65, 0x74, 0x12, 0x1e, 0x0a, 0x08, 0x61, 0x73, 0x73,
	0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02,
	0x52, 0x07, 0x61, 0x73, 0x73, 0x65, 0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x04, 0x6e, 0x61,
	0x6d, 0x65, 0x12, 0x2c, 0x0a, 0x0f, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02,
	0x52, 0x0e, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64,
	0x12, 0x42, 0x0a, 0x0c, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65,
	0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
	0x6d, 0x70, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0b, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
	0x44, 0x61, 0x74, 0x65, 0x12, 0x30, 0x0a, 0x12, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f,
	0x62, 0x79, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09,
	0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x42, 0x79,
	0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x44, 0x0a, 0x0d, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69,
	0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
	0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e,
	0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0c,
	0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x44, 0x61, 0x74, 0x65, 0x12, 0x32, 0x0a, 0x13,
	0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x75, 0x73, 0x65, 0x72,
	0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x10,
	0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x42, 0x79, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64,
	0x12, 0x17, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x09, 0x20, 0x03, 0x28, 0x09, 0x42, 0x03,
	0xe0, 0x41, 0x02, 0x52, 0x04, 0x74, 0x61, 0x67, 0x73, 0x22, 0x96, 0x01, 0x0a, 0x11, 0x4c, 0x69,
	0x73, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
	0x20, 0x0a, 0x09, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0d, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x08, 0x70, 0x61, 0x67, 0x65, 0x53, 0x69, 0x7a,
	0x65, 0x12, 0x22, 0x0a, 0x0a, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x09, 0x70, 0x61, 0x67, 0x65,
	0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x1b, 0x0a, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x06, 0x66, 0x69, 0x6c, 0x74,
	0x65, 0x72, 0x12, 0x1e, 0x0a, 0x08, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x5f, 0x62, 0x79, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x07, 0x6f, 0x72, 0x64, 0x65, 0x72,
	0x42, 0x79, 0x22, 0x6b, 0x0a, 0x12, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x73,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2d, 0x0a, 0x06, 0x61, 0x73, 0x73, 0x65,
	0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e,
	0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52,
	0x06, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x12, 0x26, 0x0a, 0x0f, 0x6e, 0x65, 0x78, 0x74, 0x5f,
	0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x0d, 0x6e, 0x65, 0x78, 0x74, 0x50, 0x61, 0x67, 0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22,
	0x34, 0x0a, 0x12, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1e, 0x0a, 0x08, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69,
	0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x07, 0x61, 0x73,
	0x73, 0x65, 0x74, 0x49, 0x64, 0x22, 0x15, 0x0a, 0x13, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x41,
	0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x31, 0x0a, 0x0f,
	0x47, 0x65, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
	0x1e, 0x0a, 0x08, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x07, 0x61, 0x73, 0x73, 0x65, 0x74, 0x49, 0x64, 0x22,
	0x3f, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x2b, 0x0a, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x15, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73,
	0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74,
	0x22, 0x83, 0x01, 0x0a, 0x12, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2b, 0x0a, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x61, 0x73,
	0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x05, 0x61,
	0x73, 0x73, 0x65, 0x74, 0x12, 0x40, 0x0a, 0x0b, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x6d,
	0x61, 0x73, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x46, 0x69, 0x65, 0x6c,
	0x64, 0x4d, 0x61, 0x73, 0x6b, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0a, 0x75, 0x70, 0x64, 0x61,
	0x74, 0x65, 0x4d, 0x61, 0x73, 0x6b, 0x22, 0x42, 0x0a, 0x13, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
	0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2b, 0x0a,
	0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x73,
	0x69, 0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73,
	0x73, 0x65, 0x74, 0x52, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x32, 0x87, 0x06, 0x0a, 0x0c, 0x41,
	0x73, 0x73, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x9c, 0x01, 0x0a, 0x0b,
	0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x12, 0x22, 0x2e, 0x73, 0x69,
	0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x65, 0x6c,
	0x65, 0x74, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
	0x23, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31,
	0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x22, 0x44, 0x92, 0x41, 0x20, 0x12, 0x0b, 0x44, 0x65, 0x6c, 0x65, 0x74,
	0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x1a, 0x11, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x73, 0x20,
	0x61, 0x6e, 0x20, 0x61, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x1b, 0x2a,
	0x19, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2f,
	0x7b, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x7d, 0x12, 0x92, 0x01, 0x0a, 0x08, 0x47,
	0x65, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x12, 0x1f, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x61,
	0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x41, 0x73, 0x73, 0x65,
	0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x20, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e,
	0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x41, 0x73, 0x73,
	0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x43, 0x92, 0x41, 0x1f, 0x12,
	0x08, 0x47, 0x65, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x1a, 0x13, 0x52, 0x65, 0x74, 0x72, 0x69,
	0x65, 0x76, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x82, 0xd3,
	0xe4, 0x93, 0x02, 0x1b, 0x12, 0x19, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x73,
	0x73, 0x65, 0x74, 0x73, 0x2f, 0x7b, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x7d, 0x12,
	0xa6, 0x01, 0x0a, 0x0a, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x12, 0x21,
	0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e,
	0x4c, 0x69, 0x73, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x1a, 0x22, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e,
	0x76, 0x31, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x51, 0x92, 0x41, 0x38, 0x12, 0x0a, 0x4c, 0x69, 0x73, 0x74,
	0x41, 0x73, 0x73, 0x65, 0x74, 0x73, 0x1a, 0x2a, 0x52, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65,
	0x73, 0x20, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61,
	0x6e, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65,
	0x72, 0x2e, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x10, 0x12, 0x0e, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76,
	0x31, 0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x12, 0x9d, 0x01, 0x0a, 0x0b, 0x55, 0x70, 0x64,
	0x61, 0x74, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x12, 0x22, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e,
	0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
	0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e, 0x73,
	0x69, 0x66, 0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x70,
	0x64, 0x61, 0x74, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x22, 0x45, 0x92, 0x41, 0x29, 0x12, 0x0b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x41, 0x73,
	0x73, 0x65, 0x74, 0x1a, 0x1a, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c,
	0x64, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x73, 0x73, 0x65, 0x74, 0x2e, 0x82,
	0xd3, 0xe4, 0x93, 0x02, 0x13, 0x3a, 0x01, 0x2a, 0x32, 0x0e, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76,
	0x31, 0x2f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x1a, 0x7a, 0x92, 0x41, 0x77, 0x12, 0x44, 0x53,
	0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61,
	0x6d, 0x6d, 0x61, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72,
	0x61, 0x63, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x5b, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73,
	0x5d, 0x28, 0x2f, 0x67, 0x6c, 0x6f, 0x73, 0x73, 0x61, 0x72, 0x79, 0x23, 0x61, 0x73, 0x73, 0x65,
	0x74, 0x29, 0x2e, 0x1a, 0x2f, 0x0a, 0x20, 0x52, 0x65, 0x61, 0x64, 0x20, 0x6d, 0x6f, 0x72, 0x65,
	0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x61, 0x73, 0x73, 0x65,
	0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x2e, 0x12, 0x0b, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2d, 0x6d,
	0x6f, 0x64, 0x65, 0x6c, 0x42, 0xd4, 0x01, 0x0a, 0x12, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x69, 0x66,
	0x74, 0x2e, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x42, 0x0b, 0x41, 0x73, 0x73,
	0x65, 0x74, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x43, 0x67, 0x69, 0x74, 0x68,
	0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2d, 0x73, 0x74, 0x61, 0x63,
	0x6b, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x67, 0x6f, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x67, 0x6f, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x61, 0x73, 0x73,
	0x65, 0x74, 0x73, 0x2f, 0x76, 0x31, 0x3b, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x76, 0x31, 0xa2,
	0x02, 0x03, 0x53, 0x41, 0x58, 0xaa, 0x02, 0x0e, 0x53, 0x69, 0x66, 0x74, 0x2e, 0x41, 0x73, 0x73,
	0x65, 0x74, 0x73, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x0e, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x41, 0x73,
	0x73, 0x65, 0x74, 0x73, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1a, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x41,
	0x73, 0x73, 0x65, 0x74, 0x73, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61,
	0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x10, 0x53, 0x69, 0x66, 0x74, 0x3a, 0x3a, 0x41, 0x73, 0x73,
	0x65, 0x74, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x92, 0x41, 0x11, 0x12, 0x0f, 0x0a, 0x0d, 0x41, 0x73,
	0x73, 0x65, 0x74, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x62, 0x06, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x33,
}

var (
	file_sift_assets_v1_assets_proto_rawDescOnce sync.Once
	file_sift_assets_v1_assets_proto_rawDescData = file_sift_assets_v1_assets_proto_rawDesc
)

func file_sift_assets_v1_assets_proto_rawDescGZIP() []byte {
	file_sift_assets_v1_assets_proto_rawDescOnce.Do(func() {
		file_sift_assets_v1_assets_proto_rawDescData = protoimpl.X.CompressGZIP(file_sift_assets_v1_assets_proto_rawDescData)
	})
	return file_sift_assets_v1_assets_proto_rawDescData
}

var file_sift_assets_v1_assets_proto_msgTypes = make([]protoimpl.MessageInfo, 9)
var file_sift_assets_v1_assets_proto_goTypes = []interface{}{
	(*Asset)(nil),                 // 0: sift.assets.v1.Asset
	(*ListAssetsRequest)(nil),     // 1: sift.assets.v1.ListAssetsRequest
	(*ListAssetsResponse)(nil),    // 2: sift.assets.v1.ListAssetsResponse
	(*DeleteAssetRequest)(nil),    // 3: sift.assets.v1.DeleteAssetRequest
	(*DeleteAssetResponse)(nil),   // 4: sift.assets.v1.DeleteAssetResponse
	(*GetAssetRequest)(nil),       // 5: sift.assets.v1.GetAssetRequest
	(*GetAssetResponse)(nil),      // 6: sift.assets.v1.GetAssetResponse
	(*UpdateAssetRequest)(nil),    // 7: sift.assets.v1.UpdateAssetRequest
	(*UpdateAssetResponse)(nil),   // 8: sift.assets.v1.UpdateAssetResponse
	(*timestamppb.Timestamp)(nil), // 9: google.protobuf.Timestamp
	(*fieldmaskpb.FieldMask)(nil), // 10: google.protobuf.FieldMask
}
var file_sift_assets_v1_assets_proto_depIdxs = []int32{
	9,  // 0: sift.assets.v1.Asset.created_date:type_name -> google.protobuf.Timestamp
	9,  // 1: sift.assets.v1.Asset.modified_date:type_name -> google.protobuf.Timestamp
	0,  // 2: sift.assets.v1.ListAssetsResponse.assets:type_name -> sift.assets.v1.Asset
	0,  // 3: sift.assets.v1.GetAssetResponse.asset:type_name -> sift.assets.v1.Asset
	0,  // 4: sift.assets.v1.UpdateAssetRequest.asset:type_name -> sift.assets.v1.Asset
	10, // 5: sift.assets.v1.UpdateAssetRequest.update_mask:type_name -> google.protobuf.FieldMask
	0,  // 6: sift.assets.v1.UpdateAssetResponse.asset:type_name -> sift.assets.v1.Asset
	3,  // 7: sift.assets.v1.AssetService.DeleteAsset:input_type -> sift.assets.v1.DeleteAssetRequest
	5,  // 8: sift.assets.v1.AssetService.GetAsset:input_type -> sift.assets.v1.GetAssetRequest
	1,  // 9: sift.assets.v1.AssetService.ListAssets:input_type -> sift.assets.v1.ListAssetsRequest
	7,  // 10: sift.assets.v1.AssetService.UpdateAsset:input_type -> sift.assets.v1.UpdateAssetRequest
	4,  // 11: sift.assets.v1.AssetService.DeleteAsset:output_type -> sift.assets.v1.DeleteAssetResponse
	6,  // 12: sift.assets.v1.AssetService.GetAsset:output_type -> sift.assets.v1.GetAssetResponse
	2,  // 13: sift.assets.v1.AssetService.ListAssets:output_type -> sift.assets.v1.ListAssetsResponse
	8,  // 14: sift.assets.v1.AssetService.UpdateAsset:output_type -> sift.assets.v1.UpdateAssetResponse
	11, // [11:15] is the sub-list for method output_type
	7,  // [7:11] is the sub-list for method input_type
	7,  // [7:7] is the sub-list for extension type_name
	7,  // [7:7] is the sub-list for extension extendee
	0,  // [0:7] is the sub-list for field type_name
}

func init() { file_sift_assets_v1_assets_proto_init() }
func file_sift_assets_v1_assets_proto_init() {
	if File_sift_assets_v1_assets_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sift_assets_v1_assets_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Asset); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListAssetsRequest); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListAssetsResponse); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*DeleteAssetRequest); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*DeleteAssetResponse); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetAssetRequest); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetAssetResponse); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UpdateAssetRequest); i {
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
		file_sift_assets_v1_assets_proto_msgTypes[8].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UpdateAssetResponse); i {
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
			RawDescriptor: file_sift_assets_v1_assets_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   9,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_sift_assets_v1_assets_proto_goTypes,
		DependencyIndexes: file_sift_assets_v1_assets_proto_depIdxs,
		MessageInfos:      file_sift_assets_v1_assets_proto_msgTypes,
	}.Build()
	File_sift_assets_v1_assets_proto = out.File
	file_sift_assets_v1_assets_proto_rawDesc = nil
	file_sift_assets_v1_assets_proto_goTypes = nil
	file_sift_assets_v1_assets_proto_depIdxs = nil
}
