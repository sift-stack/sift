// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        (unknown)
// source: sift/tags/v1/tags.proto

package tagsv1

import (
	_ "github.com/sift-stack/sift/go/gen/protos/go/google/api"
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

type Tag struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	TagId            string                 `protobuf:"bytes,1,opt,name=tag_id,json=tagId,proto3" json:"tag_id,omitempty"`
	Name             string                 `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	OrganizationId   string                 `protobuf:"bytes,3,opt,name=organization_id,json=organizationId,proto3" json:"organization_id,omitempty"`
	CreatedByUserId  string                 `protobuf:"bytes,4,opt,name=created_by_user_id,json=createdByUserId,proto3" json:"created_by_user_id,omitempty"`
	ModifiedByUserId string                 `protobuf:"bytes,5,opt,name=modified_by_user_id,json=modifiedByUserId,proto3" json:"modified_by_user_id,omitempty"`
	CreatedDate      *timestamppb.Timestamp `protobuf:"bytes,6,opt,name=created_date,json=createdDate,proto3" json:"created_date,omitempty"`
	ModifiedDate     *timestamppb.Timestamp `protobuf:"bytes,7,opt,name=modified_date,json=modifiedDate,proto3" json:"modified_date,omitempty"`
}

func (x *Tag) Reset() {
	*x = Tag{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_tags_v1_tags_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Tag) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Tag) ProtoMessage() {}

func (x *Tag) ProtoReflect() protoreflect.Message {
	mi := &file_sift_tags_v1_tags_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Tag.ProtoReflect.Descriptor instead.
func (*Tag) Descriptor() ([]byte, []int) {
	return file_sift_tags_v1_tags_proto_rawDescGZIP(), []int{0}
}

func (x *Tag) GetTagId() string {
	if x != nil {
		return x.TagId
	}
	return ""
}

func (x *Tag) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Tag) GetOrganizationId() string {
	if x != nil {
		return x.OrganizationId
	}
	return ""
}

func (x *Tag) GetCreatedByUserId() string {
	if x != nil {
		return x.CreatedByUserId
	}
	return ""
}

func (x *Tag) GetModifiedByUserId() string {
	if x != nil {
		return x.ModifiedByUserId
	}
	return ""
}

func (x *Tag) GetCreatedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.CreatedDate
	}
	return nil
}

func (x *Tag) GetModifiedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.ModifiedDate
	}
	return nil
}

// Points to a tag by name or tag_id.
// When this message is used for a request, you can use either name or tag_id to refer to a tag.
// When this message is returned in a response, both of the fields will be populated and valid.
type TagRef struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	TagId string `protobuf:"bytes,1,opt,name=tag_id,json=tagId,proto3" json:"tag_id,omitempty"`
	Name  string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
}

func (x *TagRef) Reset() {
	*x = TagRef{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_tags_v1_tags_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TagRef) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TagRef) ProtoMessage() {}

func (x *TagRef) ProtoReflect() protoreflect.Message {
	mi := &file_sift_tags_v1_tags_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TagRef.ProtoReflect.Descriptor instead.
func (*TagRef) Descriptor() ([]byte, []int) {
	return file_sift_tags_v1_tags_proto_rawDescGZIP(), []int{1}
}

func (x *TagRef) GetTagId() string {
	if x != nil {
		return x.TagId
	}
	return ""
}

func (x *TagRef) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

var File_sift_tags_v1_tags_proto protoreflect.FileDescriptor

var file_sift_tags_v1_tags_proto_rawDesc = []byte{
	0x0a, 0x17, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x74, 0x61, 0x67, 0x73, 0x2f, 0x76, 0x31, 0x2f, 0x74,
	0x61, 0x67, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x73, 0x69, 0x66, 0x74, 0x2e,
	0x74, 0x61, 0x67, 0x73, 0x2e, 0x76, 0x31, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f,
	0x61, 0x70, 0x69, 0x2f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69,
	0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
	0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xd8, 0x02, 0x0a, 0x03, 0x54, 0x61,
	0x67, 0x12, 0x1a, 0x0a, 0x06, 0x74, 0x61, 0x67, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x05, 0x74, 0x61, 0x67, 0x49, 0x64, 0x12, 0x17, 0x0a,
	0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02,
	0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2c, 0x0a, 0x0f, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69,
	0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x42,
	0x03, 0xe0, 0x41, 0x02, 0x52, 0x0e, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x49, 0x64, 0x12, 0x30, 0x0a, 0x12, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f,
	0x62, 0x79, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
	0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x42, 0x79,
	0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x32, 0x0a, 0x13, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69,
	0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20,
	0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x10, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69,
	0x65, 0x64, 0x42, 0x79, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x42, 0x0a, 0x0c, 0x63, 0x72,
	0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
	0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x42, 0x03, 0xe0, 0x41,
	0x02, 0x52, 0x0b, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x44, 0x61, 0x74, 0x65, 0x12, 0x44,
	0x0a, 0x0d, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18,
	0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
	0x70, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0c, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64,
	0x44, 0x61, 0x74, 0x65, 0x22, 0x3d, 0x0a, 0x06, 0x54, 0x61, 0x67, 0x52, 0x65, 0x66, 0x12, 0x1a,
	0x0a, 0x06, 0x74, 0x61, 0x67, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03,
	0xe0, 0x41, 0x01, 0x52, 0x05, 0x74, 0x61, 0x67, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x04, 0x6e, 0x61,
	0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x04, 0x6e,
	0x61, 0x6d, 0x65, 0x42, 0xb0, 0x01, 0x0a, 0x10, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x69, 0x66, 0x74,
	0x2e, 0x74, 0x61, 0x67, 0x73, 0x2e, 0x76, 0x31, 0x42, 0x09, 0x54, 0x61, 0x67, 0x73, 0x50, 0x72,
	0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x3f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
	0x6d, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2d, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2f, 0x73, 0x69, 0x66,
	0x74, 0x2f, 0x67, 0x6f, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f,
	0x67, 0x6f, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x74, 0x61, 0x67, 0x73, 0x2f, 0x76, 0x31, 0x3b,
	0x74, 0x61, 0x67, 0x73, 0x76, 0x31, 0xa2, 0x02, 0x03, 0x53, 0x54, 0x58, 0xaa, 0x02, 0x0c, 0x53,
	0x69, 0x66, 0x74, 0x2e, 0x54, 0x61, 0x67, 0x73, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x0c, 0x53, 0x69,
	0x66, 0x74, 0x5c, 0x54, 0x61, 0x67, 0x73, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x18, 0x53, 0x69, 0x66,
	0x74, 0x5c, 0x54, 0x61, 0x67, 0x73, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74,
	0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x0e, 0x53, 0x69, 0x66, 0x74, 0x3a, 0x3a, 0x54, 0x61,
	0x67, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sift_tags_v1_tags_proto_rawDescOnce sync.Once
	file_sift_tags_v1_tags_proto_rawDescData = file_sift_tags_v1_tags_proto_rawDesc
)

func file_sift_tags_v1_tags_proto_rawDescGZIP() []byte {
	file_sift_tags_v1_tags_proto_rawDescOnce.Do(func() {
		file_sift_tags_v1_tags_proto_rawDescData = protoimpl.X.CompressGZIP(file_sift_tags_v1_tags_proto_rawDescData)
	})
	return file_sift_tags_v1_tags_proto_rawDescData
}

var file_sift_tags_v1_tags_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_sift_tags_v1_tags_proto_goTypes = []interface{}{
	(*Tag)(nil),                   // 0: sift.tags.v1.Tag
	(*TagRef)(nil),                // 1: sift.tags.v1.TagRef
	(*timestamppb.Timestamp)(nil), // 2: google.protobuf.Timestamp
}
var file_sift_tags_v1_tags_proto_depIdxs = []int32{
	2, // 0: sift.tags.v1.Tag.created_date:type_name -> google.protobuf.Timestamp
	2, // 1: sift.tags.v1.Tag.modified_date:type_name -> google.protobuf.Timestamp
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_sift_tags_v1_tags_proto_init() }
func file_sift_tags_v1_tags_proto_init() {
	if File_sift_tags_v1_tags_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sift_tags_v1_tags_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Tag); i {
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
		file_sift_tags_v1_tags_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TagRef); i {
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
			RawDescriptor: file_sift_tags_v1_tags_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_sift_tags_v1_tags_proto_goTypes,
		DependencyIndexes: file_sift_tags_v1_tags_proto_depIdxs,
		MessageInfos:      file_sift_tags_v1_tags_proto_msgTypes,
	}.Build()
	File_sift_tags_v1_tags_proto = out.File
	file_sift_tags_v1_tags_proto_rawDesc = nil
	file_sift_tags_v1_tags_proto_goTypes = nil
	file_sift_tags_v1_tags_proto_depIdxs = nil
}
