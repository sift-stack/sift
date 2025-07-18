// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        (unknown)
// source: sift/roles/v2/roles.proto

package rolesv2

import (
	_ "github.com/sift-stack/sift/go/gen/google/api"
	_ "github.com/sift-stack/sift/go/gen/protoc-gen-openapiv2/options"
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

type Role struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	RoleId      string `protobuf:"bytes,1,opt,name=role_id,json=roleId,proto3" json:"role_id,omitempty"`
	Name        string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	Description string `protobuf:"bytes,3,opt,name=description,proto3" json:"description,omitempty"`
}

func (x *Role) Reset() {
	*x = Role{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_roles_v2_roles_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Role) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Role) ProtoMessage() {}

func (x *Role) ProtoReflect() protoreflect.Message {
	mi := &file_sift_roles_v2_roles_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Role.ProtoReflect.Descriptor instead.
func (*Role) Descriptor() ([]byte, []int) {
	return file_sift_roles_v2_roles_proto_rawDescGZIP(), []int{0}
}

func (x *Role) GetRoleId() string {
	if x != nil {
		return x.RoleId
	}
	return ""
}

func (x *Role) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Role) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

type ListRolesRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The maximum number of roles to return.
	// The service may return fewer than this value.
	// If unspecified, at most 50 roles will be returned.
	// The maximum value is 1000; values above 1000 will be coerced to 1000.
	PageSize uint32 `protobuf:"varint,1,opt,name=page_size,json=pageSize,proto3" json:"page_size,omitempty"`
	// A page token, received from a previous `ListRoles` call.
	// Provide this to retrieve the subsequent page.
	//
	// When paginating, all other parameters provided to `ListRoles` must match
	// the call that provided the page token.
	PageToken string `protobuf:"bytes,2,opt,name=page_token,json=pageToken,proto3" json:"page_token,omitempty"`
	// A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
	// Available fields to filter by are `role_id`, `name`, and `description`.
	// For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
	// For more information about the fields used for filtering, please refer to [this definition](/docs/api/grpc/protocol-buffers/channels#channel). Optional.
	Filter string `protobuf:"bytes,3,opt,name=filter,proto3" json:"filter,omitempty"`
	// How to order the retrieved channels. Formatted as a comma-separated string i.e. "FIELD_NAME[ desc],...".
	// Available fields to order_by are `name` and `description`.
	// If left empty, items are ordered by `name` in ascending order (oldest-first).
	// For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
	// Example: "name desc,description"
	OrderBy string `protobuf:"bytes,4,opt,name=order_by,json=orderBy,proto3" json:"order_by,omitempty"`
}

func (x *ListRolesRequest) Reset() {
	*x = ListRolesRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_roles_v2_roles_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListRolesRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListRolesRequest) ProtoMessage() {}

func (x *ListRolesRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_roles_v2_roles_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListRolesRequest.ProtoReflect.Descriptor instead.
func (*ListRolesRequest) Descriptor() ([]byte, []int) {
	return file_sift_roles_v2_roles_proto_rawDescGZIP(), []int{1}
}

func (x *ListRolesRequest) GetPageSize() uint32 {
	if x != nil {
		return x.PageSize
	}
	return 0
}

func (x *ListRolesRequest) GetPageToken() string {
	if x != nil {
		return x.PageToken
	}
	return ""
}

func (x *ListRolesRequest) GetFilter() string {
	if x != nil {
		return x.Filter
	}
	return ""
}

func (x *ListRolesRequest) GetOrderBy() string {
	if x != nil {
		return x.OrderBy
	}
	return ""
}

type ListRolesResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Roles         []*Role `protobuf:"bytes,1,rep,name=roles,proto3" json:"roles,omitempty"`
	NextPageToken string  `protobuf:"bytes,2,opt,name=next_page_token,json=nextPageToken,proto3" json:"next_page_token,omitempty"`
}

func (x *ListRolesResponse) Reset() {
	*x = ListRolesResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_roles_v2_roles_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListRolesResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListRolesResponse) ProtoMessage() {}

func (x *ListRolesResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_roles_v2_roles_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListRolesResponse.ProtoReflect.Descriptor instead.
func (*ListRolesResponse) Descriptor() ([]byte, []int) {
	return file_sift_roles_v2_roles_proto_rawDescGZIP(), []int{2}
}

func (x *ListRolesResponse) GetRoles() []*Role {
	if x != nil {
		return x.Roles
	}
	return nil
}

func (x *ListRolesResponse) GetNextPageToken() string {
	if x != nil {
		return x.NextPageToken
	}
	return ""
}

var File_sift_roles_v2_roles_proto protoreflect.FileDescriptor

var file_sift_roles_v2_roles_proto_rawDesc = []byte{
	0x0a, 0x19, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x2f, 0x76, 0x32, 0x2f,
	0x72, 0x6f, 0x6c, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x73, 0x69, 0x66,
	0x74, 0x2e, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x32, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
	0x2f, 0x61, 0x70, 0x69, 0x2f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x62, 0x65, 0x68, 0x61, 0x76,
	0x69, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x63, 0x2d, 0x67, 0x65, 0x6e, 0x2d, 0x6f, 0x70, 0x65, 0x6e, 0x61, 0x70, 0x69, 0x76, 0x32, 0x2f,
	0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x55, 0x0a, 0x04, 0x52, 0x6f, 0x6c,
	0x65, 0x12, 0x17, 0x0a, 0x07, 0x72, 0x6f, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x06, 0x72, 0x6f, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
	0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x20,
	0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
	0x22, 0x8b, 0x01, 0x0a, 0x10, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x69,
	0x7a, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x70, 0x61, 0x67, 0x65, 0x53, 0x69,
	0x7a, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x70, 0x61, 0x67, 0x65, 0x54, 0x6f, 0x6b, 0x65,
	0x6e, 0x12, 0x1b, 0x0a, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28,
	0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x12, 0x1e,
	0x0a, 0x08, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x5f, 0x62, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
	0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x07, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x42, 0x79, 0x22, 0x66,
	0x0a, 0x11, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a, 0x05, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03,
	0x28, 0x0b, 0x32, 0x13, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x2e,
	0x76, 0x32, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x05, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x12, 0x26,
	0x0a, 0x0f, 0x6e, 0x65, 0x78, 0x74, 0x5f, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65,
	0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x6e, 0x65, 0x78, 0x74, 0x50, 0x61, 0x67,
	0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x32, 0xc7, 0x01, 0x0a, 0x0b, 0x52, 0x6f, 0x6c, 0x65, 0x53,
	0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0xb7, 0x01, 0x0a, 0x09, 0x4c, 0x69, 0x73, 0x74, 0x52,
	0x6f, 0x6c, 0x65, 0x73, 0x12, 0x1f, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x72, 0x6f, 0x6c, 0x65,
	0x73, 0x2e, 0x76, 0x32, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x20, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x72, 0x6f, 0x6c,
	0x65, 0x73, 0x2e, 0x76, 0x32, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52,
	0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x67, 0x92, 0x41, 0x4f, 0x12, 0x09, 0x4c, 0x69,
	0x73, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x1a, 0x29, 0x52, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76,
	0x65, 0x73, 0x20, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61,
	0x6e, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65,
	0x72, 0x2e, 0x2a, 0x17, 0x52, 0x6f, 0x6c, 0x65, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f,
	0x4c, 0x69, 0x73, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x56, 0x32, 0x82, 0xd3, 0xe4, 0x93, 0x02,
	0x0f, 0x12, 0x0d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x32, 0x2f, 0x72, 0x6f, 0x6c, 0x65, 0x73,
	0x42, 0xc1, 0x01, 0x0a, 0x11, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x72, 0x6f,
	0x6c, 0x65, 0x73, 0x2e, 0x76, 0x32, 0x42, 0x0a, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x50, 0x72, 0x6f,
	0x74, 0x6f, 0x50, 0x01, 0x5a, 0x37, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d,
	0x2f, 0x73, 0x69, 0x66, 0x74, 0x2d, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2f, 0x73, 0x69, 0x66, 0x74,
	0x2f, 0x67, 0x6f, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x72, 0x6f, 0x6c,
	0x65, 0x73, 0x2f, 0x76, 0x32, 0x3b, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x76, 0x32, 0xa2, 0x02, 0x03,
	0x53, 0x52, 0x58, 0xaa, 0x02, 0x0d, 0x53, 0x69, 0x66, 0x74, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x73,
	0x2e, 0x56, 0x32, 0xca, 0x02, 0x0d, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x52, 0x6f, 0x6c, 0x65, 0x73,
	0x5c, 0x56, 0x32, 0xe2, 0x02, 0x19, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x52, 0x6f, 0x6c, 0x65, 0x73,
	0x5c, 0x56, 0x32, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
	0x02, 0x0f, 0x53, 0x69, 0x66, 0x74, 0x3a, 0x3a, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x3a, 0x3a, 0x56,
	0x32, 0x92, 0x41, 0x10, 0x12, 0x0e, 0x0a, 0x0c, 0x52, 0x6f, 0x6c, 0x65, 0x20, 0x53, 0x65, 0x72,
	0x76, 0x69, 0x63, 0x65, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sift_roles_v2_roles_proto_rawDescOnce sync.Once
	file_sift_roles_v2_roles_proto_rawDescData = file_sift_roles_v2_roles_proto_rawDesc
)

func file_sift_roles_v2_roles_proto_rawDescGZIP() []byte {
	file_sift_roles_v2_roles_proto_rawDescOnce.Do(func() {
		file_sift_roles_v2_roles_proto_rawDescData = protoimpl.X.CompressGZIP(file_sift_roles_v2_roles_proto_rawDescData)
	})
	return file_sift_roles_v2_roles_proto_rawDescData
}

var file_sift_roles_v2_roles_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_sift_roles_v2_roles_proto_goTypes = []interface{}{
	(*Role)(nil),              // 0: sift.roles.v2.Role
	(*ListRolesRequest)(nil),  // 1: sift.roles.v2.ListRolesRequest
	(*ListRolesResponse)(nil), // 2: sift.roles.v2.ListRolesResponse
}
var file_sift_roles_v2_roles_proto_depIdxs = []int32{
	0, // 0: sift.roles.v2.ListRolesResponse.roles:type_name -> sift.roles.v2.Role
	1, // 1: sift.roles.v2.RoleService.ListRoles:input_type -> sift.roles.v2.ListRolesRequest
	2, // 2: sift.roles.v2.RoleService.ListRoles:output_type -> sift.roles.v2.ListRolesResponse
	2, // [2:3] is the sub-list for method output_type
	1, // [1:2] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_sift_roles_v2_roles_proto_init() }
func file_sift_roles_v2_roles_proto_init() {
	if File_sift_roles_v2_roles_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sift_roles_v2_roles_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Role); i {
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
		file_sift_roles_v2_roles_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListRolesRequest); i {
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
		file_sift_roles_v2_roles_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListRolesResponse); i {
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
			RawDescriptor: file_sift_roles_v2_roles_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_sift_roles_v2_roles_proto_goTypes,
		DependencyIndexes: file_sift_roles_v2_roles_proto_depIdxs,
		MessageInfos:      file_sift_roles_v2_roles_proto_msgTypes,
	}.Build()
	File_sift_roles_v2_roles_proto = out.File
	file_sift_roles_v2_roles_proto_rawDesc = nil
	file_sift_roles_v2_roles_proto_goTypes = nil
	file_sift_roles_v2_roles_proto_depIdxs = nil
}
