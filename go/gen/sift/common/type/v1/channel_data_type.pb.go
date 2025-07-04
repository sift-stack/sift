// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        (unknown)
// source: sift/common/type/v1/channel_data_type.proto

package typev1

import (
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

type ChannelDataType int32

const (
	// Deprecated: Do not use.
	ChannelDataType_CHANNEL_DATA_TYPE_UNSPECIFIED ChannelDataType = 0
	ChannelDataType_CHANNEL_DATA_TYPE_DOUBLE      ChannelDataType = 1
	ChannelDataType_CHANNEL_DATA_TYPE_STRING      ChannelDataType = 2
	ChannelDataType_CHANNEL_DATA_TYPE_ENUM        ChannelDataType = 3
	ChannelDataType_CHANNEL_DATA_TYPE_BIT_FIELD   ChannelDataType = 4
	ChannelDataType_CHANNEL_DATA_TYPE_BOOL        ChannelDataType = 5
	ChannelDataType_CHANNEL_DATA_TYPE_FLOAT       ChannelDataType = 6
	ChannelDataType_CHANNEL_DATA_TYPE_INT_32      ChannelDataType = 7
	ChannelDataType_CHANNEL_DATA_TYPE_UINT_32     ChannelDataType = 8
	ChannelDataType_CHANNEL_DATA_TYPE_INT_64      ChannelDataType = 9
	ChannelDataType_CHANNEL_DATA_TYPE_UINT_64     ChannelDataType = 10
	ChannelDataType_CHANNEL_DATA_TYPE_BYTES       ChannelDataType = 11
)

// Enum value maps for ChannelDataType.
var (
	ChannelDataType_name = map[int32]string{
		0:  "CHANNEL_DATA_TYPE_UNSPECIFIED",
		1:  "CHANNEL_DATA_TYPE_DOUBLE",
		2:  "CHANNEL_DATA_TYPE_STRING",
		3:  "CHANNEL_DATA_TYPE_ENUM",
		4:  "CHANNEL_DATA_TYPE_BIT_FIELD",
		5:  "CHANNEL_DATA_TYPE_BOOL",
		6:  "CHANNEL_DATA_TYPE_FLOAT",
		7:  "CHANNEL_DATA_TYPE_INT_32",
		8:  "CHANNEL_DATA_TYPE_UINT_32",
		9:  "CHANNEL_DATA_TYPE_INT_64",
		10: "CHANNEL_DATA_TYPE_UINT_64",
		11: "CHANNEL_DATA_TYPE_BYTES",
	}
	ChannelDataType_value = map[string]int32{
		"CHANNEL_DATA_TYPE_UNSPECIFIED": 0,
		"CHANNEL_DATA_TYPE_DOUBLE":      1,
		"CHANNEL_DATA_TYPE_STRING":      2,
		"CHANNEL_DATA_TYPE_ENUM":        3,
		"CHANNEL_DATA_TYPE_BIT_FIELD":   4,
		"CHANNEL_DATA_TYPE_BOOL":        5,
		"CHANNEL_DATA_TYPE_FLOAT":       6,
		"CHANNEL_DATA_TYPE_INT_32":      7,
		"CHANNEL_DATA_TYPE_UINT_32":     8,
		"CHANNEL_DATA_TYPE_INT_64":      9,
		"CHANNEL_DATA_TYPE_UINT_64":     10,
		"CHANNEL_DATA_TYPE_BYTES":       11,
	}
)

func (x ChannelDataType) Enum() *ChannelDataType {
	p := new(ChannelDataType)
	*p = x
	return p
}

func (x ChannelDataType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (ChannelDataType) Descriptor() protoreflect.EnumDescriptor {
	return file_sift_common_type_v1_channel_data_type_proto_enumTypes[0].Descriptor()
}

func (ChannelDataType) Type() protoreflect.EnumType {
	return &file_sift_common_type_v1_channel_data_type_proto_enumTypes[0]
}

func (x ChannelDataType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use ChannelDataType.Descriptor instead.
func (ChannelDataType) EnumDescriptor() ([]byte, []int) {
	return file_sift_common_type_v1_channel_data_type_proto_rawDescGZIP(), []int{0}
}

var File_sift_common_type_v1_channel_data_type_proto protoreflect.FileDescriptor

var file_sift_common_type_v1_channel_data_type_proto_rawDesc = []byte{
	0x0a, 0x2b, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x74, 0x79,
	0x70, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x64, 0x61,
	0x74, 0x61, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x73,
	0x69, 0x66, 0x74, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e,
	0x76, 0x31, 0x2a, 0x81, 0x03, 0x0a, 0x0f, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x44, 0x61,
	0x74, 0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x25, 0x0a, 0x1d, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45,
	0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50,
	0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x1a, 0x02, 0x08, 0x01, 0x12, 0x1c, 0x0a,
	0x18, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x44, 0x4f, 0x55, 0x42, 0x4c, 0x45, 0x10, 0x01, 0x12, 0x1c, 0x0a, 0x18, 0x43,
	0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45,
	0x5f, 0x53, 0x54, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48, 0x41,
	0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x45,
	0x4e, 0x55, 0x4d, 0x10, 0x03, 0x12, 0x1f, 0x0a, 0x1b, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c,
	0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x42, 0x49, 0x54, 0x5f, 0x46,
	0x49, 0x45, 0x4c, 0x44, 0x10, 0x04, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45,
	0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x42, 0x4f, 0x4f, 0x4c,
	0x10, 0x05, 0x12, 0x1b, 0x0a, 0x17, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41,
	0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x46, 0x4c, 0x4f, 0x41, 0x54, 0x10, 0x06, 0x12,
	0x1c, 0x0a, 0x18, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f,
	0x54, 0x59, 0x50, 0x45, 0x5f, 0x49, 0x4e, 0x54, 0x5f, 0x33, 0x32, 0x10, 0x07, 0x12, 0x1d, 0x0a,
	0x19, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x55, 0x49, 0x4e, 0x54, 0x5f, 0x33, 0x32, 0x10, 0x08, 0x12, 0x1c, 0x0a, 0x18,
	0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50,
	0x45, 0x5f, 0x49, 0x4e, 0x54, 0x5f, 0x36, 0x34, 0x10, 0x09, 0x12, 0x1d, 0x0a, 0x19, 0x43, 0x48,
	0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f,
	0x55, 0x49, 0x4e, 0x54, 0x5f, 0x36, 0x34, 0x10, 0x0a, 0x12, 0x1b, 0x0a, 0x17, 0x43, 0x48, 0x41,
	0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x42,
	0x59, 0x54, 0x45, 0x53, 0x10, 0x0b, 0x42, 0xdc, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x73,
	0x69, 0x66, 0x74, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e,
	0x76, 0x31, 0x42, 0x14, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x44, 0x61, 0x74, 0x61, 0x54,
	0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x3c, 0x67, 0x69, 0x74, 0x68,
	0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2d, 0x73, 0x74, 0x61, 0x63,
	0x6b, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x67, 0x6f, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x73, 0x69,
	0x66, 0x74, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76,
	0x31, 0x3b, 0x74, 0x79, 0x70, 0x65, 0x76, 0x31, 0xa2, 0x02, 0x03, 0x53, 0x43, 0x54, 0xaa, 0x02,
	0x13, 0x53, 0x69, 0x66, 0x74, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x54, 0x79, 0x70,
	0x65, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x13, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x43, 0x6f, 0x6d, 0x6d,
	0x6f, 0x6e, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1f, 0x53, 0x69, 0x66,
	0x74, 0x5c, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x5c, 0x56, 0x31,
	0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x16, 0x53,
	0x69, 0x66, 0x74, 0x3a, 0x3a, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x3a, 0x3a, 0x54, 0x79, 0x70,
	0x65, 0x3a, 0x3a, 0x56, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_sift_common_type_v1_channel_data_type_proto_rawDescOnce sync.Once
	file_sift_common_type_v1_channel_data_type_proto_rawDescData = file_sift_common_type_v1_channel_data_type_proto_rawDesc
)

func file_sift_common_type_v1_channel_data_type_proto_rawDescGZIP() []byte {
	file_sift_common_type_v1_channel_data_type_proto_rawDescOnce.Do(func() {
		file_sift_common_type_v1_channel_data_type_proto_rawDescData = protoimpl.X.CompressGZIP(file_sift_common_type_v1_channel_data_type_proto_rawDescData)
	})
	return file_sift_common_type_v1_channel_data_type_proto_rawDescData
}

var file_sift_common_type_v1_channel_data_type_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_sift_common_type_v1_channel_data_type_proto_goTypes = []interface{}{
	(ChannelDataType)(0), // 0: sift.common.type.v1.ChannelDataType
}
var file_sift_common_type_v1_channel_data_type_proto_depIdxs = []int32{
	0, // [0:0] is the sub-list for method output_type
	0, // [0:0] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_sift_common_type_v1_channel_data_type_proto_init() }
func file_sift_common_type_v1_channel_data_type_proto_init() {
	if File_sift_common_type_v1_channel_data_type_proto != nil {
		return
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_sift_common_type_v1_channel_data_type_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   0,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_sift_common_type_v1_channel_data_type_proto_goTypes,
		DependencyIndexes: file_sift_common_type_v1_channel_data_type_proto_depIdxs,
		EnumInfos:         file_sift_common_type_v1_channel_data_type_proto_enumTypes,
	}.Build()
	File_sift_common_type_v1_channel_data_type_proto = out.File
	file_sift_common_type_v1_channel_data_type_proto_rawDesc = nil
	file_sift_common_type_v1_channel_data_type_proto_goTypes = nil
	file_sift_common_type_v1_channel_data_type_proto_depIdxs = nil
}
