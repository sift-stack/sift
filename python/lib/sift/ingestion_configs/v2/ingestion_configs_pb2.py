# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/ingestion_configs/v2/ingestion_configs.proto
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
from sift.common.type.v1 import channel_bit_field_element_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__bit__field__element__pb2
from sift.common.type.v1 import channel_data_type_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__data__type__pb2
from sift.common.type.v1 import channel_enum_type_pb2 as sift_dot_common_dot_type_dot_v1_dot_channel__enum__type__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n1sift/ingestion_configs/v2/ingestion_configs.proto\x12\x19sift.ingestion_configs.v2\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\x1a\x33sift/common/type/v1/channel_bit_field_element.proto\x1a+sift/common/type/v1/channel_data_type.proto\x1a+sift/common/type/v1/channel_enum_type.proto\"\x8a\x01\n\x0fIngestionConfig\x12\x33\n\x13ingestion_config_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x11ingestionConfigId\x12\x1e\n\x08\x61sset_id\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x07\x61ssetId\x12\"\n\nclient_key\x18\x03 \x01(\tB\x03\xe0\x41\x01R\tclientKey\"k\n\nFlowConfig\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12\x44\n\x08\x63hannels\x18\x02 \x03(\x0b\x32(.sift.ingestion_configs.v2.ChannelConfigR\x08\x63hannels\"\xd0\x02\n\rChannelConfig\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12\x17\n\x04unit\x18\x02 \x01(\tB\x03\xe0\x41\x01R\x04unit\x12%\n\x0b\x64\x65scription\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x0b\x64\x65scription\x12\x46\n\tdata_type\x18\x04 \x01(\x0e\x32$.sift.common.type.v1.ChannelDataTypeB\x03\xe0\x41\x02R\x08\x64\x61taType\x12\x43\n\nenum_types\x18\x05 \x03(\x0b\x32$.sift.common.type.v1.ChannelEnumTypeR\tenumTypes\x12Y\n\x12\x62it_field_elements\x18\x06 \x03(\x0b\x32+.sift.common.type.v1.ChannelBitFieldElementR\x10\x62itFieldElements\"P\n\x19GetIngestionConfigRequest\x12\x33\n\x13ingestion_config_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x11ingestionConfigId\"x\n\x1aGetIngestionConfigResponse\x12Z\n\x10ingestion_config\x18\x01 \x01(\x0b\x32*.sift.ingestion_configs.v2.IngestionConfigB\x03\xe0\x41\x02R\x0fingestionConfig\"\xd1\x01\n\x1c\x43reateIngestionConfigRequest\x12\"\n\nasset_name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\tassetName\x12;\n\x05\x66lows\x18\x02 \x03(\x0b\x32%.sift.ingestion_configs.v2.FlowConfigR\x05\x66lows\x12,\n\x0forganization_id\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\x12\"\n\nclient_key\x18\x04 \x01(\tB\x03\xe0\x41\x01R\tclientKey\"{\n\x1d\x43reateIngestionConfigResponse\x12Z\n\x10ingestion_config\x18\x01 \x01(\x0b\x32*.sift.ingestion_configs.v2.IngestionConfigB\x03\xe0\x41\x02R\x0fingestionConfig\"\x80\x01\n\x1bListIngestionConfigsRequest\x12 \n\tpage_size\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\"\x9f\x01\n\x1cListIngestionConfigsResponse\x12W\n\x11ingestion_configs\x18\x01 \x03(\x0b\x32*.sift.ingestion_configs.v2.IngestionConfigR\x10ingestionConfigs\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken\"\x90\x01\n!CreateIngestionConfigFlowsRequest\x12.\n\x13ingestion_config_id\x18\x01 \x01(\tR\x11ingestionConfigId\x12;\n\x05\x66lows\x18\x02 \x03(\x0b\x32%.sift.ingestion_configs.v2.FlowConfigR\x05\x66lows\"$\n\"CreateIngestionConfigFlowsResponse\"\xb9\x01\n\x1fListIngestionConfigFlowsRequest\x12\x33\n\x13ingestion_config_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x11ingestionConfigId\x12 \n\tpage_size\x18\x02 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x03 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x04 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\"\x87\x01\n ListIngestionConfigFlowsResponse\x12;\n\x05\x66lows\x18\x01 \x03(\x0b\x32%.sift.ingestion_configs.v2.FlowConfigR\x05\x66lows\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken2\x91\x0e\n\x16IngestionConfigService\x12\x9f\x02\n\x12GetIngestionConfig\x12\x34.sift.ingestion_configs.v2.GetIngestionConfigRequest\x1a\x35.sift.ingestion_configs.v2.GetIngestionConfigResponse\"\x9b\x01\x92\x41\x61\x12\x12GetIngestionConfig\x1a\x1eRetrieves an ingestion config.*+IngestionConfigService_GetIngestionConfigV2\x82\xd3\xe4\x93\x02\x31\x12//api/v2/ingestion-configs/{ingestion_config_id}\x12\x98\x02\n\x15\x43reateIngestionConfig\x12\x37.sift.ingestion_configs.v2.CreateIngestionConfigRequest\x1a\x38.sift.ingestion_configs.v2.CreateIngestionConfigResponse\"\x8b\x01\x92\x41\x64\x12\x15\x43reateIngestionConfig\x1a\x1b\x43reate an ingestion config.*.IngestionConfigService_CreateIngestionConfigV2\x82\xd3\xe4\x93\x02\x1e\"\x19/api/v2/ingestion-configs:\x01*\x12\xa5\x02\n\x14ListIngestionConfigs\x12\x36.sift.ingestion_configs.v2.ListIngestionConfigsRequest\x1a\x37.sift.ingestion_configs.v2.ListIngestionConfigsResponse\"\x9b\x01\x92\x41w\x12\x14ListIngestionConfigs\x1a\x30List ingestion configs using an optional filter.*-IngestionConfigService_ListIngestionConfigsV2\x82\xd3\xe4\x93\x02\x1b\x12\x19/api/v2/ingestion-configs\x12\xe3\x02\n\x1a\x43reateIngestionConfigFlows\x12<.sift.ingestion_configs.v2.CreateIngestionConfigFlowsRequest\x1a=.sift.ingestion_configs.v2.CreateIngestionConfigFlowsResponse\"\xc7\x01\x92\x41\x83\x01\x12\x1a\x43reateIngestionConfigFlows\x1a\x30\x43reate ingestion config [flows](/glossary#flow).*3IngestionConfigService_CreateIngestionConfigFlowsV2\x82\xd3\xe4\x93\x02:\"5/api/v2/ingestion-configs/{ingestion_config_id}/flows:\x01*\x12\xed\x02\n\x18ListIngestionConfigFlows\x12:.sift.ingestion_configs.v2.ListIngestionConfigFlowsRequest\x1a;.sift.ingestion_configs.v2.ListIngestionConfigFlowsResponse\"\xd7\x01\x92\x41\x96\x01\x12\x18ListIngestionConfigFlows\x1aGList ingestion config [flows](/glossary#flow) using an optional filter.*1IngestionConfigService_ListIngestionConfigFlowsV2\x82\xd3\xe4\x93\x02\x37\x12\x35/api/v2/ingestion-configs/{ingestion_config_id}/flows\x1a\xbb\x01\x92\x41\xb7\x01\x12ZService to programmatically interact with [ingestion configs](/glossary#ingestion-config).\x1aY\n+Read more about what ingestion configs are.\x12*/ingestion/creating-amend-ingestion-configB\xd7\x01\n\x1d\x63om.sift.ingestion_configs.v2B\x15IngestionConfigsProtoP\x01\xa2\x02\x03SIX\xaa\x02\x18Sift.IngestionConfigs.V2\xca\x02\x18Sift\\IngestionConfigs\\V2\xe2\x02$Sift\\IngestionConfigs\\V2\\GPBMetadata\xea\x02\x1aSift::IngestionConfigs::V2\x92\x41\x1c\x12\x1a\n\x18Ingestion Config Serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.ingestion_configs.v2.ingestion_configs_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\035com.sift.ingestion_configs.v2B\025IngestionConfigsProtoP\001\242\002\003SIX\252\002\030Sift.IngestionConfigs.V2\312\002\030Sift\\IngestionConfigs\\V2\342\002$Sift\\IngestionConfigs\\V2\\GPBMetadata\352\002\032Sift::IngestionConfigs::V2\222A\034\022\032\n\030Ingestion Config Service'
  _globals['_INGESTIONCONFIG'].fields_by_name['ingestion_config_id']._loaded_options = None
  _globals['_INGESTIONCONFIG'].fields_by_name['ingestion_config_id']._serialized_options = b'\340A\002'
  _globals['_INGESTIONCONFIG'].fields_by_name['asset_id']._loaded_options = None
  _globals['_INGESTIONCONFIG'].fields_by_name['asset_id']._serialized_options = b'\340A\002'
  _globals['_INGESTIONCONFIG'].fields_by_name['client_key']._loaded_options = None
  _globals['_INGESTIONCONFIG'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_FLOWCONFIG'].fields_by_name['name']._loaded_options = None
  _globals['_FLOWCONFIG'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_CHANNELCONFIG'].fields_by_name['name']._loaded_options = None
  _globals['_CHANNELCONFIG'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_CHANNELCONFIG'].fields_by_name['unit']._loaded_options = None
  _globals['_CHANNELCONFIG'].fields_by_name['unit']._serialized_options = b'\340A\001'
  _globals['_CHANNELCONFIG'].fields_by_name['description']._loaded_options = None
  _globals['_CHANNELCONFIG'].fields_by_name['description']._serialized_options = b'\340A\001'
  _globals['_CHANNELCONFIG'].fields_by_name['data_type']._loaded_options = None
  _globals['_CHANNELCONFIG'].fields_by_name['data_type']._serialized_options = b'\340A\002'
  _globals['_GETINGESTIONCONFIGREQUEST'].fields_by_name['ingestion_config_id']._loaded_options = None
  _globals['_GETINGESTIONCONFIGREQUEST'].fields_by_name['ingestion_config_id']._serialized_options = b'\340A\002'
  _globals['_GETINGESTIONCONFIGRESPONSE'].fields_by_name['ingestion_config']._loaded_options = None
  _globals['_GETINGESTIONCONFIGRESPONSE'].fields_by_name['ingestion_config']._serialized_options = b'\340A\002'
  _globals['_CREATEINGESTIONCONFIGREQUEST'].fields_by_name['asset_name']._loaded_options = None
  _globals['_CREATEINGESTIONCONFIGREQUEST'].fields_by_name['asset_name']._serialized_options = b'\340A\002'
  _globals['_CREATEINGESTIONCONFIGREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_CREATEINGESTIONCONFIGREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_CREATEINGESTIONCONFIGREQUEST'].fields_by_name['client_key']._loaded_options = None
  _globals['_CREATEINGESTIONCONFIGREQUEST'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_CREATEINGESTIONCONFIGRESPONSE'].fields_by_name['ingestion_config']._loaded_options = None
  _globals['_CREATEINGESTIONCONFIGRESPONSE'].fields_by_name['ingestion_config']._serialized_options = b'\340A\002'
  _globals['_LISTINGESTIONCONFIGSREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTINGESTIONCONFIGSREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTINGESTIONCONFIGSREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTINGESTIONCONFIGSREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTINGESTIONCONFIGSREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTINGESTIONCONFIGSREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['ingestion_config_id']._loaded_options = None
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['ingestion_config_id']._serialized_options = b'\340A\002'
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_INGESTIONCONFIGSERVICE']._loaded_options = None
  _globals['_INGESTIONCONFIGSERVICE']._serialized_options = b'\222A\267\001\022ZService to programmatically interact with [ingestion configs](/glossary#ingestion-config).\032Y\n+Read more about what ingestion configs are.\022*/ingestion/creating-amend-ingestion-config'
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['GetIngestionConfig']._loaded_options = None
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['GetIngestionConfig']._serialized_options = b'\222Aa\022\022GetIngestionConfig\032\036Retrieves an ingestion config.*+IngestionConfigService_GetIngestionConfigV2\202\323\344\223\0021\022//api/v2/ingestion-configs/{ingestion_config_id}'
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['CreateIngestionConfig']._loaded_options = None
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['CreateIngestionConfig']._serialized_options = b'\222Ad\022\025CreateIngestionConfig\032\033Create an ingestion config.*.IngestionConfigService_CreateIngestionConfigV2\202\323\344\223\002\036\"\031/api/v2/ingestion-configs:\001*'
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['ListIngestionConfigs']._loaded_options = None
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['ListIngestionConfigs']._serialized_options = b'\222Aw\022\024ListIngestionConfigs\0320List ingestion configs using an optional filter.*-IngestionConfigService_ListIngestionConfigsV2\202\323\344\223\002\033\022\031/api/v2/ingestion-configs'
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['CreateIngestionConfigFlows']._loaded_options = None
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['CreateIngestionConfigFlows']._serialized_options = b'\222A\203\001\022\032CreateIngestionConfigFlows\0320Create ingestion config [flows](/glossary#flow).*3IngestionConfigService_CreateIngestionConfigFlowsV2\202\323\344\223\002:\"5/api/v2/ingestion-configs/{ingestion_config_id}/flows:\001*'
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['ListIngestionConfigFlows']._loaded_options = None
  _globals['_INGESTIONCONFIGSERVICE'].methods_by_name['ListIngestionConfigFlows']._serialized_options = b'\222A\226\001\022\030ListIngestionConfigFlows\032GList ingestion config [flows](/glossary#flow) using an optional filter.*1IngestionConfigService_ListIngestionConfigFlowsV2\202\323\344\223\0027\0225/api/v2/ingestion-configs/{ingestion_config_id}/flows'
  _globals['_INGESTIONCONFIG']._serialized_start=335
  _globals['_INGESTIONCONFIG']._serialized_end=473
  _globals['_FLOWCONFIG']._serialized_start=475
  _globals['_FLOWCONFIG']._serialized_end=582
  _globals['_CHANNELCONFIG']._serialized_start=585
  _globals['_CHANNELCONFIG']._serialized_end=921
  _globals['_GETINGESTIONCONFIGREQUEST']._serialized_start=923
  _globals['_GETINGESTIONCONFIGREQUEST']._serialized_end=1003
  _globals['_GETINGESTIONCONFIGRESPONSE']._serialized_start=1005
  _globals['_GETINGESTIONCONFIGRESPONSE']._serialized_end=1125
  _globals['_CREATEINGESTIONCONFIGREQUEST']._serialized_start=1128
  _globals['_CREATEINGESTIONCONFIGREQUEST']._serialized_end=1337
  _globals['_CREATEINGESTIONCONFIGRESPONSE']._serialized_start=1339
  _globals['_CREATEINGESTIONCONFIGRESPONSE']._serialized_end=1462
  _globals['_LISTINGESTIONCONFIGSREQUEST']._serialized_start=1465
  _globals['_LISTINGESTIONCONFIGSREQUEST']._serialized_end=1593
  _globals['_LISTINGESTIONCONFIGSRESPONSE']._serialized_start=1596
  _globals['_LISTINGESTIONCONFIGSRESPONSE']._serialized_end=1755
  _globals['_CREATEINGESTIONCONFIGFLOWSREQUEST']._serialized_start=1758
  _globals['_CREATEINGESTIONCONFIGFLOWSREQUEST']._serialized_end=1902
  _globals['_CREATEINGESTIONCONFIGFLOWSRESPONSE']._serialized_start=1904
  _globals['_CREATEINGESTIONCONFIGFLOWSRESPONSE']._serialized_end=1940
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST']._serialized_start=1943
  _globals['_LISTINGESTIONCONFIGFLOWSREQUEST']._serialized_end=2128
  _globals['_LISTINGESTIONCONFIGFLOWSRESPONSE']._serialized_start=2131
  _globals['_LISTINGESTIONCONFIGFLOWSRESPONSE']._serialized_end=2266
  _globals['_INGESTIONCONFIGSERVICE']._serialized_start=2269
  _globals['_INGESTIONCONFIGSERVICE']._serialized_end=4078
# @@protoc_insertion_point(module_scope)
