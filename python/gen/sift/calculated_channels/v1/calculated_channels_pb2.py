# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/calculated_channels/v1/calculated_channels.proto
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
from protoc_gen_openapiv2.options import annotations_pb2 as protoc__gen__openapiv2_dot_options_dot_annotations__pb2
from sift.common.type.v1 import channel_data_type_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__data__type__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n5sift/calculated_channels/v1/calculated_channels.proto\x12\x1bsift.calculated_channels.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\x1a+sift/common/type/v1/channel_data_type.proto\"\xf9\x01\n\x11\x45xpressionRequest\x12y\n\x12\x63hannel_references\x18\x01 \x03(\x0b\x32\x45.sift.calculated_channels.v1.ExpressionRequest.ChannelReferencesEntryB\x03\xe0\x41\x02R\x11\x63hannelReferences\x12#\n\nexpression\x18\x02 \x01(\tB\x03\xe0\x41\x02R\nexpression\x1a\x44\n\x16\x43hannelReferencesEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12\x14\n\x05value\x18\x02 \x01(\tR\x05value:\x02\x38\x01\"\xa4\x01\n ListExpressionIdentifiersRequest\x12\x1b\n\tpage_size\x18\x01 \x01(\rR\x08pageSize\x12\x1d\n\npage_token\x18\x02 \x01(\tR\tpageToken\x12\x44\n\x04mode\x18\x03 \x01(\x0e\x32+.sift.calculated_channels.v1.ExpressionModeB\x03\xe0\x41\x02R\x04mode\"}\n!ListExpressionIdentifiersResponse\x12X\n\x0bidentifiers\x18\x01 \x03(\x0b\x32\x31.sift.calculated_channels.v1.ExpressionIdentifierB\x03\xe0\x41\x02R\x0bidentifiers\"\xce\x01\n\x14\x45xpressionIdentifier\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12%\n\x0b\x64\x65scription\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x0b\x64\x65scription\x12N\n\x04type\x18\x03 \x01(\x0e\x32\x35.sift.calculated_channels.v1.ExpressionIdentifierTypeB\x03\xe0\x41\x02R\x04type\x12&\n\x0c\x64isplay_name\x18\x04 \x01(\tB\x03\xe0\x41\x02R\x0b\x64isplayName\"\xb6\x01\n\x19ValidateExpressionRequest\x12S\n\nexpression\x18\x01 \x01(\x0b\x32..sift.calculated_channels.v1.ExpressionRequestB\x03\xe0\x41\x02R\nexpression\x12\x44\n\x04mode\x18\x02 \x01(\x0e\x32+.sift.calculated_channels.v1.ExpressionModeB\x03\xe0\x41\x02R\x04mode\"\xd8\x01\n\x1aValidateExpressionResponse\x12T\n\x05\x65rror\x18\x01 \x01(\x0b\x32<.sift.calculated_channels.v1.ErrorValidatingExpressionResultH\x00R\x05\x65rror\x12Z\n\x07success\x18\x02 \x01(\x0b\x32>.sift.calculated_channels.v1.SuccessValidatingExpressionResultH\x00R\x07successB\x08\n\x06result\"K\n\x1f\x45rrorValidatingExpressionResult\x12(\n\rerror_message\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x0c\x65rrorMessage\"k\n!SuccessValidatingExpressionResult\x12\x46\n\tdata_type\x18\x01 \x01(\x0e\x32$.sift.common.type.v1.ChannelDataTypeB\x03\xe0\x41\x02R\x08\x64\x61taType*\x9b\x01\n\x18\x45xpressionIdentifierType\x12.\n&EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED\x10\x00\x1a\x02\x08\x01\x12\'\n#EXPRESSION_IDENTIFIER_TYPE_FUNCTION\x10\x01\x12&\n\"EXPRESSION_IDENTIFIER_TYPE_CHANNEL\x10\x02*y\n\x0e\x45xpressionMode\x12#\n\x1b\x45XPRESSION_MODE_UNSPECIFIED\x10\x00\x1a\x02\x08\x01\x12\x19\n\x15\x45XPRESSION_MODE_RULES\x10\x01\x12\'\n#EXPRESSION_MODE_CALCULATED_CHANNELS\x10\x02\x32\xf1\x06\n\x19\x43\x61lculatedChannelsService\x12\xd9\x02\n\x19ListExpressionIdentifiers\x12=.sift.calculated_channels.v1.ListExpressionIdentifiersRequest\x1a>.sift.calculated_channels.v1.ListExpressionIdentifiersResponse\"\xbc\x01\x92\x41\x7f\x12\x19ListExpressionIdentifiers\x1a\x62Retrieves a list of valid identifiers that can be used as part of a calculated channel expression.\x82\xd3\xe4\x93\x02\x34\x12\x32/api/v1/calculated-channels:expression-identifiers\x12\xb0\x02\n\x12ValidateExpression\x12\x36.sift.calculated_channels.v1.ValidateExpressionRequest\x1a\x37.sift.calculated_channels.v1.ValidateExpressionResponse\"\xa8\x01\x92\x41k\x12\x12ValidateExpression\x1aUUsed to validate whether or not an expression used for a calculated channel is valid.\x82\xd3\xe4\x93\x02\x34\"//api/v1/calculated-channels:validate-expression:\x01*\x1a\xc4\x01\x92\x41\xc0\x01\x12>Service to programmatically interact with calculated channels.\x1a~\n$Read more about calculated channels.\x12Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/265421153B\xe6\x01\n\x1f\x63om.sift.calculated_channels.v1B\x17\x43\x61lculatedChannelsProtoP\x01\xa2\x02\x03SCX\xaa\x02\x1aSift.CalculatedChannels.V1\xca\x02\x1aSift\\CalculatedChannels\\V1\xe2\x02&Sift\\CalculatedChannels\\V1\\GPBMetadata\xea\x02\x1cSift::CalculatedChannels::V1\x92\x41\x1f\x12\x1d\n\x1b\x43\x61lculated Channels Serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.calculated_channels.v1.calculated_channels_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\037com.sift.calculated_channels.v1B\027CalculatedChannelsProtoP\001\242\002\003SCX\252\002\032Sift.CalculatedChannels.V1\312\002\032Sift\\CalculatedChannels\\V1\342\002&Sift\\CalculatedChannels\\V1\\GPBMetadata\352\002\034Sift::CalculatedChannels::V1\222A\037\022\035\n\033Calculated Channels Service'
  _globals['_EXPRESSIONIDENTIFIERTYPE'].values_by_name["EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED"]._loaded_options = None
  _globals['_EXPRESSIONIDENTIFIERTYPE'].values_by_name["EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED"]._serialized_options = b'\010\001'
  _globals['_EXPRESSIONMODE'].values_by_name["EXPRESSION_MODE_UNSPECIFIED"]._loaded_options = None
  _globals['_EXPRESSIONMODE'].values_by_name["EXPRESSION_MODE_UNSPECIFIED"]._serialized_options = b'\010\001'
  _globals['_EXPRESSIONREQUEST_CHANNELREFERENCESENTRY']._loaded_options = None
  _globals['_EXPRESSIONREQUEST_CHANNELREFERENCESENTRY']._serialized_options = b'8\001'
  _globals['_EXPRESSIONREQUEST'].fields_by_name['channel_references']._loaded_options = None
  _globals['_EXPRESSIONREQUEST'].fields_by_name['channel_references']._serialized_options = b'\340A\002'
  _globals['_EXPRESSIONREQUEST'].fields_by_name['expression']._loaded_options = None
  _globals['_EXPRESSIONREQUEST'].fields_by_name['expression']._serialized_options = b'\340A\002'
  _globals['_LISTEXPRESSIONIDENTIFIERSREQUEST'].fields_by_name['mode']._loaded_options = None
  _globals['_LISTEXPRESSIONIDENTIFIERSREQUEST'].fields_by_name['mode']._serialized_options = b'\340A\002'
  _globals['_LISTEXPRESSIONIDENTIFIERSRESPONSE'].fields_by_name['identifiers']._loaded_options = None
  _globals['_LISTEXPRESSIONIDENTIFIERSRESPONSE'].fields_by_name['identifiers']._serialized_options = b'\340A\002'
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['name']._loaded_options = None
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['description']._loaded_options = None
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['description']._serialized_options = b'\340A\002'
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['type']._loaded_options = None
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['type']._serialized_options = b'\340A\002'
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['display_name']._loaded_options = None
  _globals['_EXPRESSIONIDENTIFIER'].fields_by_name['display_name']._serialized_options = b'\340A\002'
  _globals['_VALIDATEEXPRESSIONREQUEST'].fields_by_name['expression']._loaded_options = None
  _globals['_VALIDATEEXPRESSIONREQUEST'].fields_by_name['expression']._serialized_options = b'\340A\002'
  _globals['_VALIDATEEXPRESSIONREQUEST'].fields_by_name['mode']._loaded_options = None
  _globals['_VALIDATEEXPRESSIONREQUEST'].fields_by_name['mode']._serialized_options = b'\340A\002'
  _globals['_ERRORVALIDATINGEXPRESSIONRESULT'].fields_by_name['error_message']._loaded_options = None
  _globals['_ERRORVALIDATINGEXPRESSIONRESULT'].fields_by_name['error_message']._serialized_options = b'\340A\002'
  _globals['_SUCCESSVALIDATINGEXPRESSIONRESULT'].fields_by_name['data_type']._loaded_options = None
  _globals['_SUCCESSVALIDATINGEXPRESSIONRESULT'].fields_by_name['data_type']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELSSERVICE']._loaded_options = None
  _globals['_CALCULATEDCHANNELSSERVICE']._serialized_options = b'\222A\300\001\022>Service to programmatically interact with calculated channels.\032~\n$Read more about calculated channels.\022Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/265421153'
  _globals['_CALCULATEDCHANNELSSERVICE'].methods_by_name['ListExpressionIdentifiers']._loaded_options = None
  _globals['_CALCULATEDCHANNELSSERVICE'].methods_by_name['ListExpressionIdentifiers']._serialized_options = b'\222A\177\022\031ListExpressionIdentifiers\032bRetrieves a list of valid identifiers that can be used as part of a calculated channel expression.\202\323\344\223\0024\0222/api/v1/calculated-channels:expression-identifiers'
  _globals['_CALCULATEDCHANNELSSERVICE'].methods_by_name['ValidateExpression']._loaded_options = None
  _globals['_CALCULATEDCHANNELSSERVICE'].methods_by_name['ValidateExpression']._serialized_options = b'\222Ak\022\022ValidateExpression\032UUsed to validate whether or not an expression used for a calculated channel is valid.\202\323\344\223\0024\"//api/v1/calculated-channels:validate-expression:\001*'
  _globals['_EXPRESSIONIDENTIFIERTYPE']._serialized_start=1588
  _globals['_EXPRESSIONIDENTIFIERTYPE']._serialized_end=1743
  _globals['_EXPRESSIONMODE']._serialized_start=1745
  _globals['_EXPRESSIONMODE']._serialized_end=1866
  _globals['_EXPRESSIONREQUEST']._serialized_start=243
  _globals['_EXPRESSIONREQUEST']._serialized_end=492
  _globals['_EXPRESSIONREQUEST_CHANNELREFERENCESENTRY']._serialized_start=424
  _globals['_EXPRESSIONREQUEST_CHANNELREFERENCESENTRY']._serialized_end=492
  _globals['_LISTEXPRESSIONIDENTIFIERSREQUEST']._serialized_start=495
  _globals['_LISTEXPRESSIONIDENTIFIERSREQUEST']._serialized_end=659
  _globals['_LISTEXPRESSIONIDENTIFIERSRESPONSE']._serialized_start=661
  _globals['_LISTEXPRESSIONIDENTIFIERSRESPONSE']._serialized_end=786
  _globals['_EXPRESSIONIDENTIFIER']._serialized_start=789
  _globals['_EXPRESSIONIDENTIFIER']._serialized_end=995
  _globals['_VALIDATEEXPRESSIONREQUEST']._serialized_start=998
  _globals['_VALIDATEEXPRESSIONREQUEST']._serialized_end=1180
  _globals['_VALIDATEEXPRESSIONRESPONSE']._serialized_start=1183
  _globals['_VALIDATEEXPRESSIONRESPONSE']._serialized_end=1399
  _globals['_ERRORVALIDATINGEXPRESSIONRESULT']._serialized_start=1401
  _globals['_ERRORVALIDATINGEXPRESSIONRESULT']._serialized_end=1476
  _globals['_SUCCESSVALIDATINGEXPRESSIONRESULT']._serialized_start=1478
  _globals['_SUCCESSVALIDATINGEXPRESSIONRESULT']._serialized_end=1585
  _globals['_CALCULATEDCHANNELSSERVICE']._serialized_start=1869
  _globals['_CALCULATEDCHANNELSSERVICE']._serialized_end=2750
# @@protoc_insertion_point(module_scope)