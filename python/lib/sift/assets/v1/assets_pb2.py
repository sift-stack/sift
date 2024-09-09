# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/assets/v1/assets.proto
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


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1bsift/assets/v1/assets.proto\x12\x0esift.assets.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a google/protobuf/field_mask.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\"\xf7\x02\n\x05\x41sset\x12\x1e\n\x08\x61sset_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x07\x61ssetId\x12\x17\n\x04name\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x04name\x12,\n\x0forganization_id\x18\x04 \x01(\tB\x03\xe0\x41\x02R\x0eorganizationId\x12\x42\n\x0c\x63reated_date\x18\x05 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0b\x63reatedDate\x12\x30\n\x12\x63reated_by_user_id\x18\x06 \x01(\tB\x03\xe0\x41\x02R\x0f\x63reatedByUserId\x12\x44\n\rmodified_date\x18\x07 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0cmodifiedDate\x12\x32\n\x13modified_by_user_id\x18\x08 \x01(\tB\x03\xe0\x41\x02R\x10modifiedByUserId\x12\x17\n\x04tags\x18\t \x03(\tB\x03\xe0\x41\x02R\x04tags\"v\n\x11ListAssetsRequest\x12 \n\tpage_size\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\"k\n\x12ListAssetsResponse\x12-\n\x06\x61ssets\x18\x01 \x03(\x0b\x32\x15.sift.assets.v1.AssetR\x06\x61ssets\x12&\n\x0fnext_page_token\x18\x05 \x01(\tR\rnextPageToken\"4\n\x12\x44\x65leteAssetRequest\x12\x1e\n\x08\x61sset_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x07\x61ssetId\"\x15\n\x13\x44\x65leteAssetResponse\"1\n\x0fGetAssetRequest\x12\x1e\n\x08\x61sset_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x07\x61ssetId\"?\n\x10GetAssetResponse\x12+\n\x05\x61sset\x18\x01 \x01(\x0b\x32\x15.sift.assets.v1.AssetR\x05\x61sset\"\x83\x01\n\x12UpdateAssetRequest\x12+\n\x05\x61sset\x18\x01 \x01(\x0b\x32\x15.sift.assets.v1.AssetR\x05\x61sset\x12@\n\x0bupdate_mask\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.FieldMaskB\x03\xe0\x41\x02R\nupdateMask\"B\n\x13UpdateAssetResponse\x12+\n\x05\x61sset\x18\x01 \x01(\x0b\x32\x15.sift.assets.v1.AssetR\x05\x61sset2\x87\x06\n\x0c\x41ssetService\x12\x9c\x01\n\x0b\x44\x65leteAsset\x12\".sift.assets.v1.DeleteAssetRequest\x1a#.sift.assets.v1.DeleteAssetResponse\"D\x92\x41 \x12\x0b\x44\x65leteAsset\x1a\x11\x44\x65letes an asset.\x82\xd3\xe4\x93\x02\x1b*\x19/api/v1/assets/{asset_id}\x12\x92\x01\n\x08GetAsset\x12\x1f.sift.assets.v1.GetAssetRequest\x1a .sift.assets.v1.GetAssetResponse\"C\x92\x41\x1f\x12\x08GetAsset\x1a\x13Retrieves an asset.\x82\xd3\xe4\x93\x02\x1b\x12\x19/api/v1/assets/{asset_id}\x12\xa6\x01\n\nListAssets\x12!.sift.assets.v1.ListAssetsRequest\x1a\".sift.assets.v1.ListAssetsResponse\"Q\x92\x41\x38\x12\nListAssets\x1a*Retrieves assets using an optional filter.\x82\xd3\xe4\x93\x02\x10\x12\x0e/api/v1/assets\x12\x9d\x01\n\x0bUpdateAsset\x12\".sift.assets.v1.UpdateAssetRequest\x1a#.sift.assets.v1.UpdateAssetResponse\"E\x92\x41)\x12\x0bUpdateAsset\x1a\x1aUpdate fields on an asset.\x82\xd3\xe4\x93\x02\x13\x32\x0e/api/v1/assets:\x01*\x1az\x92\x41w\x12\x44Service to programmatically interact with [assets](/glossary#asset).\x1a/\n Read more about what assets are.\x12\x0b/data-modelB\x8f\x01\n\x12\x63om.sift.assets.v1B\x0b\x41ssetsProtoP\x01\xa2\x02\x03SAX\xaa\x02\x0eSift.Assets.V1\xca\x02\x0eSift\\Assets\\V1\xe2\x02\x1aSift\\Assets\\V1\\GPBMetadata\xea\x02\x10Sift::Assets::V1\x92\x41\x11\x12\x0f\n\rAsset Serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.assets.v1.assets_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\022com.sift.assets.v1B\013AssetsProtoP\001\242\002\003SAX\252\002\016Sift.Assets.V1\312\002\016Sift\\Assets\\V1\342\002\032Sift\\Assets\\V1\\GPBMetadata\352\002\020Sift::Assets::V1\222A\021\022\017\n\rAsset Service'
  _globals['_ASSET'].fields_by_name['asset_id']._loaded_options = None
  _globals['_ASSET'].fields_by_name['asset_id']._serialized_options = b'\340A\002'
  _globals['_ASSET'].fields_by_name['name']._loaded_options = None
  _globals['_ASSET'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_ASSET'].fields_by_name['organization_id']._loaded_options = None
  _globals['_ASSET'].fields_by_name['organization_id']._serialized_options = b'\340A\002'
  _globals['_ASSET'].fields_by_name['created_date']._loaded_options = None
  _globals['_ASSET'].fields_by_name['created_date']._serialized_options = b'\340A\002'
  _globals['_ASSET'].fields_by_name['created_by_user_id']._loaded_options = None
  _globals['_ASSET'].fields_by_name['created_by_user_id']._serialized_options = b'\340A\002'
  _globals['_ASSET'].fields_by_name['modified_date']._loaded_options = None
  _globals['_ASSET'].fields_by_name['modified_date']._serialized_options = b'\340A\002'
  _globals['_ASSET'].fields_by_name['modified_by_user_id']._loaded_options = None
  _globals['_ASSET'].fields_by_name['modified_by_user_id']._serialized_options = b'\340A\002'
  _globals['_ASSET'].fields_by_name['tags']._loaded_options = None
  _globals['_ASSET'].fields_by_name['tags']._serialized_options = b'\340A\002'
  _globals['_LISTASSETSREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTASSETSREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTASSETSREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTASSETSREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTASSETSREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTASSETSREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_DELETEASSETREQUEST'].fields_by_name['asset_id']._loaded_options = None
  _globals['_DELETEASSETREQUEST'].fields_by_name['asset_id']._serialized_options = b'\340A\002'
  _globals['_GETASSETREQUEST'].fields_by_name['asset_id']._loaded_options = None
  _globals['_GETASSETREQUEST'].fields_by_name['asset_id']._serialized_options = b'\340A\002'
  _globals['_UPDATEASSETREQUEST'].fields_by_name['update_mask']._loaded_options = None
  _globals['_UPDATEASSETREQUEST'].fields_by_name['update_mask']._serialized_options = b'\340A\002'
  _globals['_ASSETSERVICE']._loaded_options = None
  _globals['_ASSETSERVICE']._serialized_options = b'\222Aw\022DService to programmatically interact with [assets](/glossary#asset).\032/\n Read more about what assets are.\022\013/data-model'
  _globals['_ASSETSERVICE'].methods_by_name['DeleteAsset']._loaded_options = None
  _globals['_ASSETSERVICE'].methods_by_name['DeleteAsset']._serialized_options = b'\222A \022\013DeleteAsset\032\021Deletes an asset.\202\323\344\223\002\033*\031/api/v1/assets/{asset_id}'
  _globals['_ASSETSERVICE'].methods_by_name['GetAsset']._loaded_options = None
  _globals['_ASSETSERVICE'].methods_by_name['GetAsset']._serialized_options = b'\222A\037\022\010GetAsset\032\023Retrieves an asset.\202\323\344\223\002\033\022\031/api/v1/assets/{asset_id}'
  _globals['_ASSETSERVICE'].methods_by_name['ListAssets']._loaded_options = None
  _globals['_ASSETSERVICE'].methods_by_name['ListAssets']._serialized_options = b'\222A8\022\nListAssets\032*Retrieves assets using an optional filter.\202\323\344\223\002\020\022\016/api/v1/assets'
  _globals['_ASSETSERVICE'].methods_by_name['UpdateAsset']._loaded_options = None
  _globals['_ASSETSERVICE'].methods_by_name['UpdateAsset']._serialized_options = b'\222A)\022\013UpdateAsset\032\032Update fields on an asset.\202\323\344\223\002\0232\016/api/v1/assets:\001*'
  _globals['_ASSET']._serialized_start=226
  _globals['_ASSET']._serialized_end=601
  _globals['_LISTASSETSREQUEST']._serialized_start=603
  _globals['_LISTASSETSREQUEST']._serialized_end=721
  _globals['_LISTASSETSRESPONSE']._serialized_start=723
  _globals['_LISTASSETSRESPONSE']._serialized_end=830
  _globals['_DELETEASSETREQUEST']._serialized_start=832
  _globals['_DELETEASSETREQUEST']._serialized_end=884
  _globals['_DELETEASSETRESPONSE']._serialized_start=886
  _globals['_DELETEASSETRESPONSE']._serialized_end=907
  _globals['_GETASSETREQUEST']._serialized_start=909
  _globals['_GETASSETREQUEST']._serialized_end=958
  _globals['_GETASSETRESPONSE']._serialized_start=960
  _globals['_GETASSETRESPONSE']._serialized_end=1023
  _globals['_UPDATEASSETREQUEST']._serialized_start=1026
  _globals['_UPDATEASSETREQUEST']._serialized_end=1157
  _globals['_UPDATEASSETRESPONSE']._serialized_start=1159
  _globals['_UPDATEASSETRESPONSE']._serialized_end=1225
  _globals['_ASSETSERVICE']._serialized_start=1228
  _globals['_ASSETSERVICE']._serialized_end=2003
# @@protoc_insertion_point(module_scope)
