# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/data/v1/data.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.api import annotations_pb2 as google_dot_api_dot_annotations__pb2
from google.api import field_behavior_pb2 as google_dot_api_dot_field__behavior__pb2
from google.protobuf import any_pb2 as google_dot_protobuf_dot_any__pb2
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
from protoc_gen_openapiv2.options import annotations_pb2 as protoc__gen__openapiv2_dot_options_dot_annotations__pb2
from sift.calculated_channels.v1 import calculated_channels_pb2 as sift_dot_calculated__channels_dot_v1_dot_calculated__channels__pb2
from sift.common.type.v1 import channel_bit_field_element_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__bit__field__element__pb2
from sift.common.type.v1 import channel_data_type_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__data__type__pb2
from sift.common.type.v1 import channel_enum_type_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__enum__type__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x17sift/data/v1/data.proto\x12\x0csift.data.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a\x19google/protobuf/any.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\x1a\x35sift/calculated_channels/v1/calculated_channels.proto\x1a\x33sift/common/type/v1/channel_bit_field_element.proto\x1a+sift/common/type/v1/channel_data_type.proto\x1a+sift/common/type/v1/channel_enum_type.proto\"\x8a\x02\n\x0eGetDataRequest\x12-\n\x07queries\x18\x01 \x03(\x0b\x32\x13.sift.data.v1.QueryR\x07queries\x12\x39\n\nstart_time\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\tstartTime\x12\x35\n\x08\x65nd_time\x18\x03 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\x07\x65ndTime\x12\x1b\n\tsample_ms\x18\x04 \x01(\rR\x08sampleMs\x12\x1b\n\tpage_size\x18\x05 \x01(\rR\x08pageSize\x12\x1d\n\npage_token\x18\x06 \x01(\tR\tpageToken\"\x9f\x01\n\x05Query\x12\x36\n\x07\x63hannel\x18\x01 \x01(\x0b\x32\x1a.sift.data.v1.ChannelQueryH\x00R\x07\x63hannel\x12U\n\x12\x63\x61lculated_channel\x18\x02 \x01(\x0b\x32$.sift.data.v1.CalculatedChannelQueryH\x00R\x11\x63\x61lculatedChannelB\x07\n\x05query\"T\n\x0c\x43hannelQuery\x12\x1d\n\nchannel_id\x18\x01 \x01(\tR\tchannelId\x12\x1a\n\x06run_id\x18\x02 \x01(\tH\x00R\x05runId\x88\x01\x01\x42\t\n\x07_run_id\"\x89\x02\n\x16\x43\x61lculatedChannelQuery\x12$\n\x0b\x63hannel_key\x18\x01 \x01(\tB\x03\xe0\x41\x02R\nchannelKey\x12S\n\nexpression\x18\x02 \x01(\x0b\x32..sift.calculated_channels.v1.ExpressionRequestB\x03\xe0\x41\x02R\nexpression\x12\x1a\n\x06run_id\x18\x03 \x01(\tH\x00R\x05runId\x88\x01\x01\x12\x44\n\x04mode\x18\x04 \x01(\x0e\x32+.sift.calculated_channels.v1.ExpressionModeH\x01R\x04mode\x88\x01\x01\x42\t\n\x07_run_idB\x07\n\x05_mode\"h\n\x0fGetDataResponse\x12&\n\x0fnext_page_token\x18\x01 \x01(\tR\rnextPageToken\x12-\n\x04\x64\x61ta\x18\x02 \x03(\x0b\x32\x14.google.protobuf.AnyB\x03\xe0\x41\x02R\x04\x64\x61ta\"\xbf\x06\n\x08Metadata\x12\x46\n\tdata_type\x18\x01 \x01(\x0e\x32$.sift.common.type.v1.ChannelDataTypeB\x03\xe0\x41\x02R\x08\x64\x61taType\x12\"\n\nsampled_ms\x18\x02 \x01(\rB\x03\xe0\x41\x02R\tsampledMs\x12\x37\n\x05\x61sset\x18\x03 \x01(\x0b\x32\x1c.sift.data.v1.Metadata.AssetB\x03\xe0\x41\x02R\x05\x61sset\x12\x31\n\x03run\x18\x04 \x01(\x0b\x32\x1a.sift.data.v1.Metadata.RunH\x00R\x03run\x88\x01\x01\x12=\n\x07\x63hannel\x18\x05 \x01(\x0b\x32\x1e.sift.data.v1.Metadata.ChannelH\x01R\x07\x63hannel\x88\x01\x01\x1a@\n\x05\x41sset\x12\x1e\n\x08\x61sset_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x07\x61ssetId\x12\x17\n\x04name\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x04name\x1a:\n\x03Run\x12\x1a\n\x06run_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x05runId\x12\x17\n\x04name\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x04name\x1a\x89\x03\n\x07\x43hannel\x12\"\n\nchannel_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\tchannelId\x12\x1c\n\tcomponent\x18\x02 \x01(\tR\tcomponent\x12\x12\n\x04name\x18\x03 \x01(\tR\x04name\x12\x37\n\x04unit\x18\x04 \x01(\x0b\x32#.sift.data.v1.Metadata.Channel.UnitR\x04unit\x12\x43\n\nenum_types\x18\x05 \x03(\x0b\x32$.sift.common.type.v1.ChannelEnumTypeR\tenumTypes\x12Y\n\x12\x62it_field_elements\x18\x06 \x03(\x0b\x32+.sift.common.type.v1.ChannelBitFieldElementR\x10\x62itFieldElements\x1aO\n\x04Unit\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12.\n\x10\x61\x62\x62reviated_name\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x0f\x61\x62\x62reviatedNameB\x06\n\x04_runB\n\n\x08_channel\"g\n\x0b\x44oubleValue\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\x01\x42\x03\xe0\x41\x02R\x05value\"\x7f\n\x0c\x44oubleValues\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x36\n\x06values\x18\x02 \x03(\x0b\x32\x19.sift.data.v1.DoubleValueB\x03\xe0\x41\x02R\x06values\"g\n\x0bStringValue\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x05value\"\x7f\n\x0cStringValues\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x36\n\x06values\x18\x02 \x03(\x0b\x32\x19.sift.data.v1.StringValueB\x03\xe0\x41\x02R\x06values\"e\n\tEnumValue\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\rB\x03\xe0\x41\x02R\x05value\"{\n\nEnumValues\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x34\n\x06values\x18\x02 \x03(\x0b\x32\x17.sift.data.v1.EnumValueB\x03\xe0\x41\x02R\x06values\"i\n\rBitFieldValue\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\rB\x03\xe0\x41\x02R\x05value\"j\n\x15\x42itFieldElementValues\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12\x38\n\x06values\x18\x02 \x03(\x0b\x32\x1b.sift.data.v1.BitFieldValueB\x03\xe0\x41\x02R\x06values\"\x8b\x01\n\x0e\x42itFieldValues\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12@\n\x06values\x18\x02 \x03(\x0b\x32#.sift.data.v1.BitFieldElementValuesB\x03\xe0\x41\x02R\x06values\"e\n\tBoolValue\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\x08\x42\x03\xe0\x41\x02R\x05value\"{\n\nBoolValues\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x34\n\x06values\x18\x02 \x03(\x0b\x32\x17.sift.data.v1.BoolValueB\x03\xe0\x41\x02R\x06values\"f\n\nFloatValue\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\x02\x42\x03\xe0\x41\x02R\x05value\"}\n\x0b\x46loatValues\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x35\n\x06values\x18\x02 \x03(\x0b\x32\x18.sift.data.v1.FloatValueB\x03\xe0\x41\x02R\x06values\"f\n\nInt32Value\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\x05\x42\x03\xe0\x41\x02R\x05value\"}\n\x0bInt32Values\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x35\n\x06values\x18\x02 \x03(\x0b\x32\x18.sift.data.v1.Int32ValueB\x03\xe0\x41\x02R\x06values\"g\n\x0bUint32Value\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\rB\x03\xe0\x41\x02R\x05value\"\x7f\n\x0cUint32Values\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x36\n\x06values\x18\x02 \x03(\x0b\x32\x19.sift.data.v1.Uint32ValueB\x03\xe0\x41\x02R\x06values\"f\n\nInt64Value\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\x03\x42\x03\xe0\x41\x02R\x05value\"}\n\x0bInt64Values\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x35\n\x06values\x18\x02 \x03(\x0b\x32\x18.sift.data.v1.Int64ValueB\x03\xe0\x41\x02R\x06values\"g\n\x0bUint64Value\x12=\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\ttimestamp\x12\x19\n\x05value\x18\x02 \x01(\x04\x42\x03\xe0\x41\x02R\x05value\"\x7f\n\x0cUint64Values\x12\x37\n\x08metadata\x18\x01 \x01(\x0b\x32\x16.sift.data.v1.MetadataB\x03\xe0\x41\x02R\x08metadata\x12\x36\n\x06values\x18\x02 \x03(\x0b\x32\x19.sift.data.v1.Uint64ValueB\x03\xe0\x41\x02R\x06values2\xa2\x01\n\x0b\x44\x61taService\x12w\n\x07GetData\x12\x1c.sift.data.v1.GetDataRequest\x1a\x1d.sift.data.v1.GetDataResponse\"/\x92\x41\x15\x12\x07GetData\x1a\nQuery data\x82\xd3\xe4\x93\x02\x11\"\x0c/api/v1/data:\x01*\x1a\x1a\x92\x41\x17\x12\x15Service to query dataB\xaf\x01\n\x10\x63om.sift.data.v1B\tDataProtoP\x01Z+azimuth/gen/protos/go/sift/data/v1;datav1pb\xa2\x02\x03SDX\xaa\x02\x0cSift.Data.V1\xca\x02\x0cSift\\Data\\V1\xe2\x02\x18Sift\\Data\\V1\\GPBMetadata\xea\x02\x0eSift::Data::V1\x92\x41\x10\x12\x0e\n\x0c\x44\x61ta Serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.data.v1.data_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\020com.sift.data.v1B\tDataProtoP\001Z+azimuth/gen/protos/go/sift/data/v1;datav1pb\242\002\003SDX\252\002\014Sift.Data.V1\312\002\014Sift\\Data\\V1\342\002\030Sift\\Data\\V1\\GPBMetadata\352\002\016Sift::Data::V1\222A\020\022\016\n\014Data Service'
  _globals['_CALCULATEDCHANNELQUERY'].fields_by_name['channel_key']._loaded_options = None
  _globals['_CALCULATEDCHANNELQUERY'].fields_by_name['channel_key']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELQUERY'].fields_by_name['expression']._loaded_options = None
  _globals['_CALCULATEDCHANNELQUERY'].fields_by_name['expression']._serialized_options = b'\340A\002'
  _globals['_GETDATARESPONSE'].fields_by_name['data']._loaded_options = None
  _globals['_GETDATARESPONSE'].fields_by_name['data']._serialized_options = b'\340A\002'
  _globals['_METADATA_ASSET'].fields_by_name['asset_id']._loaded_options = None
  _globals['_METADATA_ASSET'].fields_by_name['asset_id']._serialized_options = b'\340A\002'
  _globals['_METADATA_ASSET'].fields_by_name['name']._loaded_options = None
  _globals['_METADATA_ASSET'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_METADATA_RUN'].fields_by_name['run_id']._loaded_options = None
  _globals['_METADATA_RUN'].fields_by_name['run_id']._serialized_options = b'\340A\002'
  _globals['_METADATA_RUN'].fields_by_name['name']._loaded_options = None
  _globals['_METADATA_RUN'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_METADATA_CHANNEL_UNIT'].fields_by_name['name']._loaded_options = None
  _globals['_METADATA_CHANNEL_UNIT'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_METADATA_CHANNEL_UNIT'].fields_by_name['abbreviated_name']._loaded_options = None
  _globals['_METADATA_CHANNEL_UNIT'].fields_by_name['abbreviated_name']._serialized_options = b'\340A\002'
  _globals['_METADATA_CHANNEL'].fields_by_name['channel_id']._loaded_options = None
  _globals['_METADATA_CHANNEL'].fields_by_name['channel_id']._serialized_options = b'\340A\002'
  _globals['_METADATA'].fields_by_name['data_type']._loaded_options = None
  _globals['_METADATA'].fields_by_name['data_type']._serialized_options = b'\340A\002'
  _globals['_METADATA'].fields_by_name['sampled_ms']._loaded_options = None
  _globals['_METADATA'].fields_by_name['sampled_ms']._serialized_options = b'\340A\002'
  _globals['_METADATA'].fields_by_name['asset']._loaded_options = None
  _globals['_METADATA'].fields_by_name['asset']._serialized_options = b'\340A\002'
  _globals['_DOUBLEVALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_DOUBLEVALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_DOUBLEVALUE'].fields_by_name['value']._loaded_options = None
  _globals['_DOUBLEVALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_DOUBLEVALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_DOUBLEVALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_DOUBLEVALUES'].fields_by_name['values']._loaded_options = None
  _globals['_DOUBLEVALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_STRINGVALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_STRINGVALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_STRINGVALUE'].fields_by_name['value']._loaded_options = None
  _globals['_STRINGVALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_STRINGVALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_STRINGVALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_STRINGVALUES'].fields_by_name['values']._loaded_options = None
  _globals['_STRINGVALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_ENUMVALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_ENUMVALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_ENUMVALUE'].fields_by_name['value']._loaded_options = None
  _globals['_ENUMVALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_ENUMVALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_ENUMVALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_ENUMVALUES'].fields_by_name['values']._loaded_options = None
  _globals['_ENUMVALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_BITFIELDVALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_BITFIELDVALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_BITFIELDVALUE'].fields_by_name['value']._loaded_options = None
  _globals['_BITFIELDVALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_BITFIELDELEMENTVALUES'].fields_by_name['name']._loaded_options = None
  _globals['_BITFIELDELEMENTVALUES'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_BITFIELDELEMENTVALUES'].fields_by_name['values']._loaded_options = None
  _globals['_BITFIELDELEMENTVALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_BITFIELDVALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_BITFIELDVALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_BITFIELDVALUES'].fields_by_name['values']._loaded_options = None
  _globals['_BITFIELDVALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_BOOLVALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_BOOLVALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_BOOLVALUE'].fields_by_name['value']._loaded_options = None
  _globals['_BOOLVALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_BOOLVALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_BOOLVALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_BOOLVALUES'].fields_by_name['values']._loaded_options = None
  _globals['_BOOLVALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_FLOATVALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_FLOATVALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_FLOATVALUE'].fields_by_name['value']._loaded_options = None
  _globals['_FLOATVALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_FLOATVALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_FLOATVALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_FLOATVALUES'].fields_by_name['values']._loaded_options = None
  _globals['_FLOATVALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_INT32VALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_INT32VALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_INT32VALUE'].fields_by_name['value']._loaded_options = None
  _globals['_INT32VALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_INT32VALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_INT32VALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_INT32VALUES'].fields_by_name['values']._loaded_options = None
  _globals['_INT32VALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_UINT32VALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_UINT32VALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_UINT32VALUE'].fields_by_name['value']._loaded_options = None
  _globals['_UINT32VALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_UINT32VALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_UINT32VALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_UINT32VALUES'].fields_by_name['values']._loaded_options = None
  _globals['_UINT32VALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_INT64VALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_INT64VALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_INT64VALUE'].fields_by_name['value']._loaded_options = None
  _globals['_INT64VALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_INT64VALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_INT64VALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_INT64VALUES'].fields_by_name['values']._loaded_options = None
  _globals['_INT64VALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_UINT64VALUE'].fields_by_name['timestamp']._loaded_options = None
  _globals['_UINT64VALUE'].fields_by_name['timestamp']._serialized_options = b'\340A\002'
  _globals['_UINT64VALUE'].fields_by_name['value']._loaded_options = None
  _globals['_UINT64VALUE'].fields_by_name['value']._serialized_options = b'\340A\002'
  _globals['_UINT64VALUES'].fields_by_name['metadata']._loaded_options = None
  _globals['_UINT64VALUES'].fields_by_name['metadata']._serialized_options = b'\340A\002'
  _globals['_UINT64VALUES'].fields_by_name['values']._loaded_options = None
  _globals['_UINT64VALUES'].fields_by_name['values']._serialized_options = b'\340A\002'
  _globals['_DATASERVICE']._loaded_options = None
  _globals['_DATASERVICE']._serialized_options = b'\222A\027\022\025Service to query data'
  _globals['_DATASERVICE'].methods_by_name['GetData']._loaded_options = None
  _globals['_DATASERVICE'].methods_by_name['GetData']._serialized_options = b'\222A\025\022\007GetData\032\nQuery data\202\323\344\223\002\021\"\014/api/v1/data:\001*'
  _globals['_GETDATAREQUEST']._serialized_start=411
  _globals['_GETDATAREQUEST']._serialized_end=677
  _globals['_QUERY']._serialized_start=680
  _globals['_QUERY']._serialized_end=839
  _globals['_CHANNELQUERY']._serialized_start=841
  _globals['_CHANNELQUERY']._serialized_end=925
  _globals['_CALCULATEDCHANNELQUERY']._serialized_start=928
  _globals['_CALCULATEDCHANNELQUERY']._serialized_end=1193
  _globals['_GETDATARESPONSE']._serialized_start=1195
  _globals['_GETDATARESPONSE']._serialized_end=1299
  _globals['_METADATA']._serialized_start=1302
  _globals['_METADATA']._serialized_end=2133
  _globals['_METADATA_ASSET']._serialized_start=1593
  _globals['_METADATA_ASSET']._serialized_end=1657
  _globals['_METADATA_RUN']._serialized_start=1659
  _globals['_METADATA_RUN']._serialized_end=1717
  _globals['_METADATA_CHANNEL']._serialized_start=1720
  _globals['_METADATA_CHANNEL']._serialized_end=2113
  _globals['_METADATA_CHANNEL_UNIT']._serialized_start=2034
  _globals['_METADATA_CHANNEL_UNIT']._serialized_end=2113
  _globals['_DOUBLEVALUE']._serialized_start=2135
  _globals['_DOUBLEVALUE']._serialized_end=2238
  _globals['_DOUBLEVALUES']._serialized_start=2240
  _globals['_DOUBLEVALUES']._serialized_end=2367
  _globals['_STRINGVALUE']._serialized_start=2369
  _globals['_STRINGVALUE']._serialized_end=2472
  _globals['_STRINGVALUES']._serialized_start=2474
  _globals['_STRINGVALUES']._serialized_end=2601
  _globals['_ENUMVALUE']._serialized_start=2603
  _globals['_ENUMVALUE']._serialized_end=2704
  _globals['_ENUMVALUES']._serialized_start=2706
  _globals['_ENUMVALUES']._serialized_end=2829
  _globals['_BITFIELDVALUE']._serialized_start=2831
  _globals['_BITFIELDVALUE']._serialized_end=2936
  _globals['_BITFIELDELEMENTVALUES']._serialized_start=2938
  _globals['_BITFIELDELEMENTVALUES']._serialized_end=3044
  _globals['_BITFIELDVALUES']._serialized_start=3047
  _globals['_BITFIELDVALUES']._serialized_end=3186
  _globals['_BOOLVALUE']._serialized_start=3188
  _globals['_BOOLVALUE']._serialized_end=3289
  _globals['_BOOLVALUES']._serialized_start=3291
  _globals['_BOOLVALUES']._serialized_end=3414
  _globals['_FLOATVALUE']._serialized_start=3416
  _globals['_FLOATVALUE']._serialized_end=3518
  _globals['_FLOATVALUES']._serialized_start=3520
  _globals['_FLOATVALUES']._serialized_end=3645
  _globals['_INT32VALUE']._serialized_start=3647
  _globals['_INT32VALUE']._serialized_end=3749
  _globals['_INT32VALUES']._serialized_start=3751
  _globals['_INT32VALUES']._serialized_end=3876
  _globals['_UINT32VALUE']._serialized_start=3878
  _globals['_UINT32VALUE']._serialized_end=3981
  _globals['_UINT32VALUES']._serialized_start=3983
  _globals['_UINT32VALUES']._serialized_end=4110
  _globals['_INT64VALUE']._serialized_start=4112
  _globals['_INT64VALUE']._serialized_end=4214
  _globals['_INT64VALUES']._serialized_start=4216
  _globals['_INT64VALUES']._serialized_end=4341
  _globals['_UINT64VALUE']._serialized_start=4343
  _globals['_UINT64VALUE']._serialized_end=4446
  _globals['_UINT64VALUES']._serialized_start=4448
  _globals['_UINT64VALUES']._serialized_end=4575
  _globals['_DATASERVICE']._serialized_start=4578
  _globals['_DATASERVICE']._serialized_end=4740
# @@protoc_insertion_point(module_scope)
