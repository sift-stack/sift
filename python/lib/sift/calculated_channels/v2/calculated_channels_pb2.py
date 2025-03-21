# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/calculated_channels/v2/calculated_channels.proto
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
from google.protobuf import field_mask_pb2 as google_dot_protobuf_dot_field__mask__pb2
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
from protoc_gen_openapiv2.options import annotations_pb2 as protoc__gen__openapiv2_dot_options_dot_annotations__pb2
from sift.calculated_channels.v1 import calculated_channels_pb2 as sift_dot_calculated__channels_dot_v1_dot_calculated__channels__pb2
from sift.common.type.v1 import channel_data_type_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__data__type__pb2
from sift.common.type.v1 import resource_identifier_pb2 as sift_dot_common_dot_type_dot_v1_dot_resource__identifier__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n5sift/calculated_channels/v2/calculated_channels.proto\x12\x1bsift.calculated_channels.v2\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a google/protobuf/field_mask.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\x1a\x35sift/calculated_channels/v1/calculated_channels.proto\x1a+sift/common/type/v1/channel_data_type.proto\x1a-sift/common/type/v1/resource_identifier.proto\"\x89\x07\n\x11\x43\x61lculatedChannel\x12\x37\n\x15\x63\x61lculated_channel_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x13\x63\x61lculatedChannelId\x12,\n\x0forganization_id\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x0eorganizationId\x12\'\n\nclient_key\x18\x03 \x01(\tB\x03\xe0\x41\x01H\x00R\tclientKey\x88\x01\x01\x12I\n\rarchived_date\x18\x04 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01H\x01R\x0c\x61rchivedDate\x88\x01\x01\x12\"\n\nversion_id\x18\x05 \x01(\tB\x03\xe0\x41\x02R\tversionId\x12\x1d\n\x07version\x18\x06 \x01(\rB\x03\xe0\x41\x02R\x07version\x12\x17\n\x04name\x18\x07 \x01(\tB\x03\xe0\x41\x02R\x04name\x12%\n\x0b\x64\x65scription\x18\x08 \x01(\tB\x03\xe0\x41\x02R\x0b\x64\x65scription\x12*\n\x0e\x63hange_message\x18\t \x01(\tB\x03\xe0\x41\x02R\rchangeMessage\x12\"\n\nuser_notes\x18\n \x01(\tB\x03\xe0\x41\x02R\tuserNotes\x12\x1e\n\x05units\x18\x12 \x01(\tB\x03\xe0\x41\x01H\x02R\x05units\x88\x01\x01\x12\x42\n\x0c\x63reated_date\x18\x0b \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0b\x63reatedDate\x12\x44\n\rmodified_date\x18\x0c \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0cmodifiedDate\x12\x8a\x01\n calculated_channel_configuration\x18\x0f \x01(\x0b\x32;.sift.calculated_channels.v2.CalculatedChannelConfigurationB\x03\xe0\x41\x02R\x1e\x63\x61lculatedChannelConfiguration\x12\x30\n\x12\x63reated_by_user_id\x18\x10 \x01(\tB\x03\xe0\x41\x02R\x0f\x63reatedByUserId\x12\x32\n\x13modified_by_user_id\x18\x11 \x01(\tB\x03\xe0\x41\x02R\x10modifiedByUserIdB\r\n\x0b_client_keyB\x10\n\x0e_archived_dateB\x08\n\x06_units\"\x90\x02\n\x1e\x43\x61lculatedChannelConfiguration\x12v\n\x13\x61sset_configuration\x18\x01 \x01(\x0b\x32@.sift.calculated_channels.v2.CalculatedChannelAssetConfigurationB\x03\xe0\x41\x02R\x12\x61ssetConfiguration\x12v\n\x13query_configuration\x18\x02 \x01(\x0b\x32@.sift.calculated_channels.v2.CalculatedChannelQueryConfigurationB\x03\xe0\x41\x02R\x12queryConfiguration\"\x98\x02\n#CalculatedChannelAssetConfiguration\x12\x1f\n\nall_assets\x18\x01 \x01(\x08H\x00R\tallAssets\x12o\n\tselection\x18\x02 \x01(\x0b\x32O.sift.calculated_channels.v2.CalculatedChannelAssetConfiguration.AssetSelectionH\x00R\tselection\x1aP\n\x0e\x41ssetSelection\x12 \n\tasset_ids\x18\x01 \x03(\tB\x03\xe0\x41\x02R\x08\x61ssetIds\x12\x1c\n\x07tag_ids\x18\x02 \x03(\tB\x03\xe0\x41\x02R\x06tagIdsB\r\n\x0b\x61sset_scope\"\xc7\x02\n#CalculatedChannelQueryConfiguration\x12X\n\x03sel\x18\x01 \x01(\x0b\x32\x44.sift.calculated_channels.v2.CalculatedChannelQueryConfiguration.SelH\x00R\x03sel\x1a\xbc\x01\n\x03Sel\x12#\n\nexpression\x18\x01 \x01(\tB\x03\xe0\x41\x02R\nexpression\x12\x8f\x01\n\x1d\x65xpression_channel_references\x18\x02 \x03(\x0b\x32\x46.sift.calculated_channels.v2.CalculatedChannelAbstractChannelReferenceB\x03\xe0\x41\x02R\x1b\x65xpressionChannelReferencesB\x07\n\x05query\"\x91\x01\n)CalculatedChannelAbstractChannelReference\x12\x30\n\x11\x63hannel_reference\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x10\x63hannelReference\x12\x32\n\x12\x63hannel_identifier\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x11\x63hannelIdentifier\"\xcd\x01\n!CalculatedChannelValidationResult\x12\x1e\n\x08\x61sset_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x07\x61ssetId\x12\'\n\nasset_name\x18\x02 \x01(\tB\x03\xe0\x41\x01H\x00R\tassetName\x88\x01\x01\x12 \n\ttag_names\x18\x03 \x03(\tB\x03\xe0\x41\x02R\x08tagNames\x12.\n\x10missing_channels\x18\x04 \x03(\tB\x03\xe0\x41\x02R\x0fmissingChannelsB\r\n\x0b_asset_name\"\xa8\x01\n\x1bGetCalculatedChannelRequest\x12\x37\n\x15\x63\x61lculated_channel_id\x18\x01 \x01(\tB\x03\xe0\x41\x01R\x13\x63\x61lculatedChannelId\x12\"\n\nclient_key\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tclientKey\x12,\n\x0forganization_id\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\"\x82\x01\n\x1cGetCalculatedChannelResponse\x12\x62\n\x12\x63\x61lculated_channel\x18\x01 \x01(\x0b\x32..sift.calculated_channels.v2.CalculatedChannelB\x03\xe0\x41\x02R\x11\x63\x61lculatedChannel\"\xf3\x02\n\x1e\x43reateCalculatedChannelRequest\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12%\n\x0b\x64\x65scription\x18\x02 \x01(\tB\x03\xe0\x41\x01R\x0b\x64\x65scription\x12\"\n\nuser_notes\x18\x03 \x01(\tB\x03\xe0\x41\x01R\tuserNotes\x12\x1e\n\x05units\x18\x07 \x01(\tB\x03\xe0\x41\x01H\x00R\x05units\x88\x01\x01\x12\'\n\nclient_key\x18\x04 \x01(\tB\x03\xe0\x41\x01H\x01R\tclientKey\x88\x01\x01\x12\x8a\x01\n calculated_channel_configuration\x18\x05 \x01(\x0b\x32;.sift.calculated_channels.v2.CalculatedChannelConfigurationB\x03\xe0\x41\x02R\x1e\x63\x61lculatedChannelConfigurationB\x08\n\x06_unitsB\r\n\x0b_client_key\"\xfb\x01\n\x1f\x43reateCalculatedChannelResponse\x12\x62\n\x12\x63\x61lculated_channel\x18\x01 \x01(\x0b\x32..sift.calculated_channels.v2.CalculatedChannelB\x03\xe0\x41\x02R\x11\x63\x61lculatedChannel\x12t\n\x13inapplicable_assets\x18\x02 \x03(\x0b\x32>.sift.calculated_channels.v2.CalculatedChannelValidationResultB\x03\xe0\x41\x02R\x12inapplicableAssets\"\xd0\x01\n\x1dListCalculatedChannelsRequest\x12 \n\tpage_size\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\x12,\n\x0forganization_id\x18\x04 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\x12\x1e\n\x08order_by\x18\x05 \x01(\tB\x03\xe0\x41\x01R\x07orderBy\"\xb3\x01\n\x1eListCalculatedChannelsResponse\x12\x64\n\x13\x63\x61lculated_channels\x18\x01 \x03(\x0b\x32..sift.calculated_channels.v2.CalculatedChannelB\x03\xe0\x41\x02R\x12\x63\x61lculatedChannels\x12+\n\x0fnext_page_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\rnextPageToken\"\xfe\x01\n\x1eUpdateCalculatedChannelRequest\x12\x62\n\x12\x63\x61lculated_channel\x18\x01 \x01(\x0b\x32..sift.calculated_channels.v2.CalculatedChannelB\x03\xe0\x41\x02R\x11\x63\x61lculatedChannel\x12@\n\x0bupdate_mask\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.FieldMaskB\x03\xe0\x41\x02R\nupdateMask\x12\'\n\nuser_notes\x18\x03 \x01(\tB\x03\xe0\x41\x01H\x00R\tuserNotes\x88\x01\x01\x42\r\n\x0b_user_notes\"\xfb\x01\n\x1fUpdateCalculatedChannelResponse\x12\x62\n\x12\x63\x61lculated_channel\x18\x01 \x01(\x0b\x32..sift.calculated_channels.v2.CalculatedChannelB\x03\xe0\x41\x02R\x11\x63\x61lculatedChannel\x12t\n\x13inapplicable_assets\x18\x02 \x03(\x0b\x32>.sift.calculated_channels.v2.CalculatedChannelValidationResultB\x03\xe0\x41\x02R\x12inapplicableAssets\"\xb4\x02\n$ListCalculatedChannelVersionsRequest\x12\x37\n\x15\x63\x61lculated_channel_id\x18\x01 \x01(\tB\x03\xe0\x41\x01R\x13\x63\x61lculatedChannelId\x12\"\n\nclient_key\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tclientKey\x12 \n\tpage_size\x18\x03 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x04 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x05 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\x12,\n\x0forganization_id\x18\x06 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\x12\x1e\n\x08order_by\x18\x07 \x01(\tB\x03\xe0\x41\x01R\x07orderBy\"\xc9\x01\n%ListCalculatedChannelVersionsResponse\x12s\n\x1b\x63\x61lculated_channel_versions\x18\x01 \x03(\x0b\x32..sift.calculated_channels.v2.CalculatedChannelB\x03\xe0\x41\x02R\x19\x63\x61lculatedChannelVersions\x12+\n\x0fnext_page_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\rnextPageToken\"\xd3\x03\n\x1fResolveCalculatedChannelRequest\x12N\n\nidentifier\x18\x01 \x01(\x0b\x32\'.sift.common.type.v1.ResourceIdentifierB\x03\xe0\x41\x01H\x00R\nidentifier\x12\x8c\x01\n calculated_channel_configuration\x18\x02 \x01(\x0b\x32;.sift.calculated_channels.v2.CalculatedChannelConfigurationB\x03\xe0\x41\x01H\x00R\x1e\x63\x61lculatedChannelConfiguration\x12,\n\x0forganization_id\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\x12@\n\x06\x61ssets\x18\x04 \x01(\x0b\x32#.sift.common.type.v1.NamedResourcesB\x03\xe0\x41\x01R\x06\x61ssets\x12\x43\n\x03run\x18\x05 \x01(\x0b\x32\'.sift.common.type.v1.ResourceIdentifierB\x03\xe0\x41\x01H\x01R\x03run\x88\x01\x01\x42\x14\n\x12\x63\x61lculated_channelB\x06\n\x04_run\"\xdc\x05\n ResolveCalculatedChannelResponse\x12<\n\x15\x63\x61lculated_channel_id\x18\x01 \x01(\tB\x03\xe0\x41\x01H\x00R\x13\x63\x61lculatedChannelId\x88\x01\x01\x12x\n\x08resolved\x18\x02 \x03(\x0b\x32W.sift.calculated_channels.v2.ResolveCalculatedChannelResponse.ResolvedCalculatedChannelB\x03\xe0\x41\x02R\x08resolved\x12~\n\nunresolved\x18\x03 \x03(\x0b\x32Y.sift.calculated_channels.v2.ResolveCalculatedChannelResponse.UnresolvedCalculatedChannelB\x03\xe0\x41\x02R\nunresolved\x1a\xf8\x01\n\x19ResolvedCalculatedChannel\x12\"\n\nasset_name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\tassetName\x12\x62\n\x12\x65xpression_request\x18\x02 \x01(\x0b\x32..sift.calculated_channels.v1.ExpressionRequestB\x03\xe0\x41\x02R\x11\x65xpressionRequest\x12S\n\x10output_data_type\x18\x03 \x01(\x0e\x32$.sift.common.type.v1.ChannelDataTypeB\x03\xe0\x41\x02R\x0eoutputDataType\x1ak\n\x1bUnresolvedCalculatedChannel\x12\"\n\nasset_name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\tassetName\x12(\n\rerror_message\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x0c\x65rrorMessageB\x18\n\x16_calculated_channel_id\"\x86\x01\n%BatchResolveCalculatedChannelsRequest\x12]\n\x08requests\x18\x01 \x03(\x0b\x32<.sift.calculated_channels.v2.ResolveCalculatedChannelRequestB\x03\xe0\x41\x02R\x08requests\"\x8a\x01\n&BatchResolveCalculatedChannelsResponse\x12`\n\tresponses\x18\x01 \x03(\x0b\x32=.sift.calculated_channels.v2.ResolveCalculatedChannelResponseB\x03\xe0\x41\x02R\tresponses2\xd8\x11\n\x18\x43\x61lculatedChannelService\x12\xe0\x02\n\x14GetCalculatedChannel\x12\x38.sift.calculated_channels.v2.GetCalculatedChannelRequest\x1a\x39.sift.calculated_channels.v2.GetCalculatedChannelResponse\"\xd2\x01\x92\x41L\x12\x14GetCalculatedChannel\x1a\x34Retrieve the latest version of a calculated channel.\x82\xd3\xe4\x93\x02}\x12\x33/api/v2/calculated-channels/{calculated_channel_id}ZF\x12\x44/v2/organizations/{organization_id}/calculated-channels/{client_key}\x12\xf6\x01\n\x17\x43reateCalculatedChannel\x12;.sift.calculated_channels.v2.CreateCalculatedChannelRequest\x1a<.sift.calculated_channels.v2.CreateCalculatedChannelResponse\"`\x92\x41\x37\x12\x17\x43reateCalculatedChannel\x1a\x1c\x43reate a calculated channel.\x82\xd3\xe4\x93\x02 \"\x1b/api/v2/calculated-channels:\x01*\x12\xa4\x02\n\x16ListCalculatedChannels\x12:.sift.calculated_channels.v2.ListCalculatedChannelsRequest\x1a;.sift.calculated_channels.v2.ListCalculatedChannelsResponse\"\x90\x01\x92\x41j\x12\x16ListCalculatedChannels\x1aPRetrieve the latest versions of calculated channels based on an optional filter.\x82\xd3\xe4\x93\x02\x1d\x12\x1b/api/v2/calculated-channels\x12\x92\x02\n\x17UpdateCalculatedChannel\x12;.sift.calculated_channels.v2.UpdateCalculatedChannelRequest\x1a<.sift.calculated_channels.v2.UpdateCalculatedChannelResponse\"|\x92\x41S\x12\x17UpdateCalculatedChannel\x1a\x38Update and create a new version of a calculated channel.\x82\xd3\xe4\x93\x02 2\x1b/api/v2/calculated-channels:\x01*\x12\xac\x03\n\x1dListCalculatedChannelVersions\x12\x41.sift.calculated_channels.v2.ListCalculatedChannelVersionsRequest\x1a\x42.sift.calculated_channels.v2.ListCalculatedChannelVersionsResponse\"\x83\x02\x92\x41j\x12\x1dListCalculatedChannelVersions\x1aIList versions of a particular calculated channel with an optional filter.\x82\xd3\xe4\x93\x02\x8f\x01\x12</api/v2/calculated-channels/{calculated_channel_id}/versionsZO\x12M/v2/organizations/{organization_id}/calculated-channels/{client_key}/versions\x12\xa6\x02\n\x18ResolveCalculatedChannel\x12<.sift.calculated_channels.v2.ResolveCalculatedChannelRequest\x1a=.sift.calculated_channels.v2.ResolveCalculatedChannelResponse\"\x8c\x01\x92\x41[\x12\x18ResolveCalculatedChannel\x1a?Resolve a calculated channel into an expression with references\x82\xd3\xe4\x93\x02(\"#/api/v2/calculated-channels/resolve:\x01*\x12\xcb\x02\n\x1e\x42\x61tchResolveCalculatedChannels\x12\x42.sift.calculated_channels.v2.BatchResolveCalculatedChannelsRequest\x1a\x43.sift.calculated_channels.v2.BatchResolveCalculatedChannelsResponse\"\x9f\x01\x92\x41h\x12\x1d\x42\x61tchResolveCalculatedChannel\x1aGResolve a batch of calculated channels into expressions with references\x82\xd3\xe4\x93\x02.\")/api/v2/calculated-channels/resolve:batch:\x01*B\xc4\x01\n\x1f\x63om.sift.calculated_channels.v2B\x17\x43\x61lculatedChannelsProtoP\x01\xa2\x02\x03SCX\xaa\x02\x1aSift.CalculatedChannels.V2\xca\x02\x1aSift\\CalculatedChannels\\V2\xe2\x02&Sift\\CalculatedChannels\\V2\\GPBMetadata\xea\x02\x1cSift::CalculatedChannels::V2b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.calculated_channels.v2.calculated_channels_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\037com.sift.calculated_channels.v2B\027CalculatedChannelsProtoP\001\242\002\003SCX\252\002\032Sift.CalculatedChannels.V2\312\002\032Sift\\CalculatedChannels\\V2\342\002&Sift\\CalculatedChannels\\V2\\GPBMetadata\352\002\034Sift::CalculatedChannels::V2'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['calculated_channel_id']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['calculated_channel_id']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['organization_id']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['organization_id']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['client_key']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['archived_date']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['archived_date']._serialized_options = b'\340A\001'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['version_id']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['version_id']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['version']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['version']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['name']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['description']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['description']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['change_message']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['change_message']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['user_notes']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['user_notes']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['units']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['units']._serialized_options = b'\340A\001'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['created_date']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['created_date']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['modified_date']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['modified_date']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['calculated_channel_configuration']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['calculated_channel_configuration']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['created_by_user_id']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['created_by_user_id']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNEL'].fields_by_name['modified_by_user_id']._loaded_options = None
  _globals['_CALCULATEDCHANNEL'].fields_by_name['modified_by_user_id']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELCONFIGURATION'].fields_by_name['asset_configuration']._loaded_options = None
  _globals['_CALCULATEDCHANNELCONFIGURATION'].fields_by_name['asset_configuration']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELCONFIGURATION'].fields_by_name['query_configuration']._loaded_options = None
  _globals['_CALCULATEDCHANNELCONFIGURATION'].fields_by_name['query_configuration']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION_ASSETSELECTION'].fields_by_name['asset_ids']._loaded_options = None
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION_ASSETSELECTION'].fields_by_name['asset_ids']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION_ASSETSELECTION'].fields_by_name['tag_ids']._loaded_options = None
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION_ASSETSELECTION'].fields_by_name['tag_ids']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION_SEL'].fields_by_name['expression']._loaded_options = None
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION_SEL'].fields_by_name['expression']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION_SEL'].fields_by_name['expression_channel_references']._loaded_options = None
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION_SEL'].fields_by_name['expression_channel_references']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELABSTRACTCHANNELREFERENCE'].fields_by_name['channel_reference']._loaded_options = None
  _globals['_CALCULATEDCHANNELABSTRACTCHANNELREFERENCE'].fields_by_name['channel_reference']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELABSTRACTCHANNELREFERENCE'].fields_by_name['channel_identifier']._loaded_options = None
  _globals['_CALCULATEDCHANNELABSTRACTCHANNELREFERENCE'].fields_by_name['channel_identifier']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['asset_id']._loaded_options = None
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['asset_id']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['asset_name']._loaded_options = None
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['asset_name']._serialized_options = b'\340A\001'
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['tag_names']._loaded_options = None
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['tag_names']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['missing_channels']._loaded_options = None
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT'].fields_by_name['missing_channels']._serialized_options = b'\340A\002'
  _globals['_GETCALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel_id']._loaded_options = None
  _globals['_GETCALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel_id']._serialized_options = b'\340A\001'
  _globals['_GETCALCULATEDCHANNELREQUEST'].fields_by_name['client_key']._loaded_options = None
  _globals['_GETCALCULATEDCHANNELREQUEST'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_GETCALCULATEDCHANNELREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_GETCALCULATEDCHANNELREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_GETCALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel']._loaded_options = None
  _globals['_GETCALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel']._serialized_options = b'\340A\002'
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['name']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['description']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['description']._serialized_options = b'\340A\001'
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['user_notes']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['user_notes']._serialized_options = b'\340A\001'
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['units']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['units']._serialized_options = b'\340A\001'
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['client_key']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel_configuration']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel_configuration']._serialized_options = b'\340A\002'
  _globals['_CREATECALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel']._serialized_options = b'\340A\002'
  _globals['_CREATECALCULATEDCHANNELRESPONSE'].fields_by_name['inapplicable_assets']._loaded_options = None
  _globals['_CREATECALCULATEDCHANNELRESPONSE'].fields_by_name['inapplicable_assets']._serialized_options = b'\340A\002'
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['order_by']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELSREQUEST'].fields_by_name['order_by']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELSRESPONSE'].fields_by_name['calculated_channels']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELSRESPONSE'].fields_by_name['calculated_channels']._serialized_options = b'\340A\002'
  _globals['_LISTCALCULATEDCHANNELSRESPONSE'].fields_by_name['next_page_token']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELSRESPONSE'].fields_by_name['next_page_token']._serialized_options = b'\340A\001'
  _globals['_UPDATECALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel']._loaded_options = None
  _globals['_UPDATECALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel']._serialized_options = b'\340A\002'
  _globals['_UPDATECALCULATEDCHANNELREQUEST'].fields_by_name['update_mask']._loaded_options = None
  _globals['_UPDATECALCULATEDCHANNELREQUEST'].fields_by_name['update_mask']._serialized_options = b'\340A\002'
  _globals['_UPDATECALCULATEDCHANNELREQUEST'].fields_by_name['user_notes']._loaded_options = None
  _globals['_UPDATECALCULATEDCHANNELREQUEST'].fields_by_name['user_notes']._serialized_options = b'\340A\001'
  _globals['_UPDATECALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel']._loaded_options = None
  _globals['_UPDATECALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel']._serialized_options = b'\340A\002'
  _globals['_UPDATECALCULATEDCHANNELRESPONSE'].fields_by_name['inapplicable_assets']._loaded_options = None
  _globals['_UPDATECALCULATEDCHANNELRESPONSE'].fields_by_name['inapplicable_assets']._serialized_options = b'\340A\002'
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['calculated_channel_id']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['calculated_channel_id']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['client_key']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['order_by']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST'].fields_by_name['order_by']._serialized_options = b'\340A\001'
  _globals['_LISTCALCULATEDCHANNELVERSIONSRESPONSE'].fields_by_name['calculated_channel_versions']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSRESPONSE'].fields_by_name['calculated_channel_versions']._serialized_options = b'\340A\002'
  _globals['_LISTCALCULATEDCHANNELVERSIONSRESPONSE'].fields_by_name['next_page_token']._loaded_options = None
  _globals['_LISTCALCULATEDCHANNELVERSIONSRESPONSE'].fields_by_name['next_page_token']._serialized_options = b'\340A\001'
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['identifier']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['identifier']._serialized_options = b'\340A\001'
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel_configuration']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['calculated_channel_configuration']._serialized_options = b'\340A\001'
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['assets']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['assets']._serialized_options = b'\340A\001'
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['run']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELREQUEST'].fields_by_name['run']._serialized_options = b'\340A\001'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL'].fields_by_name['asset_name']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL'].fields_by_name['asset_name']._serialized_options = b'\340A\002'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL'].fields_by_name['expression_request']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL'].fields_by_name['expression_request']._serialized_options = b'\340A\002'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL'].fields_by_name['output_data_type']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL'].fields_by_name['output_data_type']._serialized_options = b'\340A\002'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_UNRESOLVEDCALCULATEDCHANNEL'].fields_by_name['asset_name']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_UNRESOLVEDCALCULATEDCHANNEL'].fields_by_name['asset_name']._serialized_options = b'\340A\002'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_UNRESOLVEDCALCULATEDCHANNEL'].fields_by_name['error_message']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_UNRESOLVEDCALCULATEDCHANNEL'].fields_by_name['error_message']._serialized_options = b'\340A\002'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel_id']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE'].fields_by_name['calculated_channel_id']._serialized_options = b'\340A\001'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE'].fields_by_name['resolved']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE'].fields_by_name['resolved']._serialized_options = b'\340A\002'
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE'].fields_by_name['unresolved']._loaded_options = None
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE'].fields_by_name['unresolved']._serialized_options = b'\340A\002'
  _globals['_BATCHRESOLVECALCULATEDCHANNELSREQUEST'].fields_by_name['requests']._loaded_options = None
  _globals['_BATCHRESOLVECALCULATEDCHANNELSREQUEST'].fields_by_name['requests']._serialized_options = b'\340A\002'
  _globals['_BATCHRESOLVECALCULATEDCHANNELSRESPONSE'].fields_by_name['responses']._loaded_options = None
  _globals['_BATCHRESOLVECALCULATEDCHANNELSRESPONSE'].fields_by_name['responses']._serialized_options = b'\340A\002'
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['GetCalculatedChannel']._loaded_options = None
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['GetCalculatedChannel']._serialized_options = b'\222AL\022\024GetCalculatedChannel\0324Retrieve the latest version of a calculated channel.\202\323\344\223\002}\0223/api/v2/calculated-channels/{calculated_channel_id}ZF\022D/v2/organizations/{organization_id}/calculated-channels/{client_key}'
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['CreateCalculatedChannel']._loaded_options = None
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['CreateCalculatedChannel']._serialized_options = b'\222A7\022\027CreateCalculatedChannel\032\034Create a calculated channel.\202\323\344\223\002 \"\033/api/v2/calculated-channels:\001*'
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['ListCalculatedChannels']._loaded_options = None
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['ListCalculatedChannels']._serialized_options = b'\222Aj\022\026ListCalculatedChannels\032PRetrieve the latest versions of calculated channels based on an optional filter.\202\323\344\223\002\035\022\033/api/v2/calculated-channels'
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['UpdateCalculatedChannel']._loaded_options = None
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['UpdateCalculatedChannel']._serialized_options = b'\222AS\022\027UpdateCalculatedChannel\0328Update and create a new version of a calculated channel.\202\323\344\223\002 2\033/api/v2/calculated-channels:\001*'
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['ListCalculatedChannelVersions']._loaded_options = None
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['ListCalculatedChannelVersions']._serialized_options = b'\222Aj\022\035ListCalculatedChannelVersions\032IList versions of a particular calculated channel with an optional filter.\202\323\344\223\002\217\001\022</api/v2/calculated-channels/{calculated_channel_id}/versionsZO\022M/v2/organizations/{organization_id}/calculated-channels/{client_key}/versions'
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['ResolveCalculatedChannel']._loaded_options = None
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['ResolveCalculatedChannel']._serialized_options = b'\222A[\022\030ResolveCalculatedChannel\032?Resolve a calculated channel into an expression with references\202\323\344\223\002(\"#/api/v2/calculated-channels/resolve:\001*'
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['BatchResolveCalculatedChannels']._loaded_options = None
  _globals['_CALCULATEDCHANNELSERVICE'].methods_by_name['BatchResolveCalculatedChannels']._serialized_options = b'\222Ah\022\035BatchResolveCalculatedChannel\032GResolve a batch of calculated channels into expressions with references\202\323\344\223\002.\")/api/v2/calculated-channels/resolve:batch:\001*'
  _globals['_CALCULATEDCHANNEL']._serialized_start=412
  _globals['_CALCULATEDCHANNEL']._serialized_end=1317
  _globals['_CALCULATEDCHANNELCONFIGURATION']._serialized_start=1320
  _globals['_CALCULATEDCHANNELCONFIGURATION']._serialized_end=1592
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION']._serialized_start=1595
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION']._serialized_end=1875
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION_ASSETSELECTION']._serialized_start=1780
  _globals['_CALCULATEDCHANNELASSETCONFIGURATION_ASSETSELECTION']._serialized_end=1860
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION']._serialized_start=1878
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION']._serialized_end=2205
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION_SEL']._serialized_start=2008
  _globals['_CALCULATEDCHANNELQUERYCONFIGURATION_SEL']._serialized_end=2196
  _globals['_CALCULATEDCHANNELABSTRACTCHANNELREFERENCE']._serialized_start=2208
  _globals['_CALCULATEDCHANNELABSTRACTCHANNELREFERENCE']._serialized_end=2353
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT']._serialized_start=2356
  _globals['_CALCULATEDCHANNELVALIDATIONRESULT']._serialized_end=2561
  _globals['_GETCALCULATEDCHANNELREQUEST']._serialized_start=2564
  _globals['_GETCALCULATEDCHANNELREQUEST']._serialized_end=2732
  _globals['_GETCALCULATEDCHANNELRESPONSE']._serialized_start=2735
  _globals['_GETCALCULATEDCHANNELRESPONSE']._serialized_end=2865
  _globals['_CREATECALCULATEDCHANNELREQUEST']._serialized_start=2868
  _globals['_CREATECALCULATEDCHANNELREQUEST']._serialized_end=3239
  _globals['_CREATECALCULATEDCHANNELRESPONSE']._serialized_start=3242
  _globals['_CREATECALCULATEDCHANNELRESPONSE']._serialized_end=3493
  _globals['_LISTCALCULATEDCHANNELSREQUEST']._serialized_start=3496
  _globals['_LISTCALCULATEDCHANNELSREQUEST']._serialized_end=3704
  _globals['_LISTCALCULATEDCHANNELSRESPONSE']._serialized_start=3707
  _globals['_LISTCALCULATEDCHANNELSRESPONSE']._serialized_end=3886
  _globals['_UPDATECALCULATEDCHANNELREQUEST']._serialized_start=3889
  _globals['_UPDATECALCULATEDCHANNELREQUEST']._serialized_end=4143
  _globals['_UPDATECALCULATEDCHANNELRESPONSE']._serialized_start=4146
  _globals['_UPDATECALCULATEDCHANNELRESPONSE']._serialized_end=4397
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST']._serialized_start=4400
  _globals['_LISTCALCULATEDCHANNELVERSIONSREQUEST']._serialized_end=4708
  _globals['_LISTCALCULATEDCHANNELVERSIONSRESPONSE']._serialized_start=4711
  _globals['_LISTCALCULATEDCHANNELVERSIONSRESPONSE']._serialized_end=4912
  _globals['_RESOLVECALCULATEDCHANNELREQUEST']._serialized_start=4915
  _globals['_RESOLVECALCULATEDCHANNELREQUEST']._serialized_end=5382
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE']._serialized_start=5385
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE']._serialized_end=6117
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL']._serialized_start=5734
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_RESOLVEDCALCULATEDCHANNEL']._serialized_end=5982
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_UNRESOLVEDCALCULATEDCHANNEL']._serialized_start=5984
  _globals['_RESOLVECALCULATEDCHANNELRESPONSE_UNRESOLVEDCALCULATEDCHANNEL']._serialized_end=6091
  _globals['_BATCHRESOLVECALCULATEDCHANNELSREQUEST']._serialized_start=6120
  _globals['_BATCHRESOLVECALCULATEDCHANNELSREQUEST']._serialized_end=6254
  _globals['_BATCHRESOLVECALCULATEDCHANNELSRESPONSE']._serialized_start=6257
  _globals['_BATCHRESOLVECALCULATEDCHANNELSRESPONSE']._serialized_end=6395
  _globals['_CALCULATEDCHANNELSERVICE']._serialized_start=6398
  _globals['_CALCULATEDCHANNELSERVICE']._serialized_end=8662
# @@protoc_insertion_point(module_scope)
