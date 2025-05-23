# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/me/v2/me.proto
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
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
from protoc_gen_openapiv2.options import annotations_pb2 as protoc__gen__openapiv2_dot_options_dot_annotations__pb2
from sift.common.type.v1 import organization_pb2 as sift_dot_common_dot_type_dot_v1_dot_organization__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x13sift/me/v2/me.proto\x12\nsift.me.v2\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\x1a&sift/common/type/v1/organization.proto\"Q\n\tResources\x12 \n\tasset_ids\x18\x01 \x03(\tB\x03\xe0\x41\x01R\x08\x61ssetIds\x12\"\n\nall_assets\x18\x02 \x01(\x08\x42\x03\xe0\x41\x01R\tallAssets\"\xe6\x01\n\x13PermissionResources\x12p\n\x14permission_resources\x18\x01 \x03(\x0b\x32\x38.sift.me.v2.PermissionResources.PermissionResourcesEntryB\x03\xe0\x41\x02R\x13permissionResources\x1a]\n\x18PermissionResourcesEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12+\n\x05value\x18\x02 \x01(\x0b\x32\x15.sift.me.v2.ResourcesR\x05value:\x02\x38\x01\"\x92\x02\n\x0bPermissions\x12\x8d\x01\n!organization_permission_resources\x18\x01 \x03(\x0b\x32<.sift.me.v2.Permissions.OrganizationPermissionResourcesEntryB\x03\xe0\x41\x02R\x1forganizationPermissionResources\x1as\n$OrganizationPermissionResourcesEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12\x35\n\x05value\x18\x02 \x01(\x0b\x32\x1f.sift.me.v2.PermissionResourcesR\x05value:\x02\x38\x01\"\x0e\n\x0cGetMeRequest\"\x9b\x03\n\rGetMeResponse\x12\x1c\n\x07user_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x06userId\x12\"\n\nuser_email\x18\x02 \x01(\tB\x03\xe0\x41\x02R\tuserEmail\x12L\n\rorganizations\x18\x03 \x03(\x0b\x32!.sift.common.type.v1.OrganizationB\x03\xe0\x41\x02R\rorganizations\x12\x1e\n\x08is_admin\x18\x04 \x01(\x08\x42\x03\xe0\x41\x02R\x07isAdmin\x12>\n\x0bpermissions\x18\x05 \x01(\x0b\x32\x17.sift.me.v2.PermissionsB\x03\xe0\x41\x02R\x0bpermissions\x12\x42\n\x0c\x63reated_date\x18\x06 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0b\x63reatedDate\x12V\n&hash_based_message_authentication_code\x18\x07 \x01(\tB\x02\x18\x01R\"hashBasedMessageAuthenticationCode2\xad\x01\n\tMeService\x12\x9f\x01\n\x05GetMe\x12\x18.sift.me.v2.GetMeRequest\x1a\x19.sift.me.v2.GetMeResponse\"a\x92\x41L\x12\x17Get My User Information\x1a\x1eRetrieve a user\'s information.*\x11MeService_GetMeV2\x82\xd3\xe4\x93\x02\x0c\x12\n/api/v2/meBt\n\x0e\x63om.sift.me.v2B\x07MeProtoP\x01\xa2\x02\x03SMX\xaa\x02\nSift.Me.V2\xca\x02\nSift\\Me\\V2\xe2\x02\x16Sift\\Me\\V2\\GPBMetadata\xea\x02\x0cSift::Me::V2\x92\x41\x0e\x12\x0c\n\nMe Serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.me.v2.me_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\016com.sift.me.v2B\007MeProtoP\001\242\002\003SMX\252\002\nSift.Me.V2\312\002\nSift\\Me\\V2\342\002\026Sift\\Me\\V2\\GPBMetadata\352\002\014Sift::Me::V2\222A\016\022\014\n\nMe Service'
  _globals['_RESOURCES'].fields_by_name['asset_ids']._loaded_options = None
  _globals['_RESOURCES'].fields_by_name['asset_ids']._serialized_options = b'\340A\001'
  _globals['_RESOURCES'].fields_by_name['all_assets']._loaded_options = None
  _globals['_RESOURCES'].fields_by_name['all_assets']._serialized_options = b'\340A\001'
  _globals['_PERMISSIONRESOURCES_PERMISSIONRESOURCESENTRY']._loaded_options = None
  _globals['_PERMISSIONRESOURCES_PERMISSIONRESOURCESENTRY']._serialized_options = b'8\001'
  _globals['_PERMISSIONRESOURCES'].fields_by_name['permission_resources']._loaded_options = None
  _globals['_PERMISSIONRESOURCES'].fields_by_name['permission_resources']._serialized_options = b'\340A\002'
  _globals['_PERMISSIONS_ORGANIZATIONPERMISSIONRESOURCESENTRY']._loaded_options = None
  _globals['_PERMISSIONS_ORGANIZATIONPERMISSIONRESOURCESENTRY']._serialized_options = b'8\001'
  _globals['_PERMISSIONS'].fields_by_name['organization_permission_resources']._loaded_options = None
  _globals['_PERMISSIONS'].fields_by_name['organization_permission_resources']._serialized_options = b'\340A\002'
  _globals['_GETMERESPONSE'].fields_by_name['user_id']._loaded_options = None
  _globals['_GETMERESPONSE'].fields_by_name['user_id']._serialized_options = b'\340A\002'
  _globals['_GETMERESPONSE'].fields_by_name['user_email']._loaded_options = None
  _globals['_GETMERESPONSE'].fields_by_name['user_email']._serialized_options = b'\340A\002'
  _globals['_GETMERESPONSE'].fields_by_name['organizations']._loaded_options = None
  _globals['_GETMERESPONSE'].fields_by_name['organizations']._serialized_options = b'\340A\002'
  _globals['_GETMERESPONSE'].fields_by_name['is_admin']._loaded_options = None
  _globals['_GETMERESPONSE'].fields_by_name['is_admin']._serialized_options = b'\340A\002'
  _globals['_GETMERESPONSE'].fields_by_name['permissions']._loaded_options = None
  _globals['_GETMERESPONSE'].fields_by_name['permissions']._serialized_options = b'\340A\002'
  _globals['_GETMERESPONSE'].fields_by_name['created_date']._loaded_options = None
  _globals['_GETMERESPONSE'].fields_by_name['created_date']._serialized_options = b'\340A\002'
  _globals['_GETMERESPONSE'].fields_by_name['hash_based_message_authentication_code']._loaded_options = None
  _globals['_GETMERESPONSE'].fields_by_name['hash_based_message_authentication_code']._serialized_options = b'\030\001'
  _globals['_MESERVICE'].methods_by_name['GetMe']._loaded_options = None
  _globals['_MESERVICE'].methods_by_name['GetMe']._serialized_options = b'\222AL\022\027Get My User Information\032\036Retrieve a user\'s information.*\021MeService_GetMeV2\202\323\344\223\002\014\022\n/api/v2/me'
  _globals['_RESOURCES']._serialized_start=219
  _globals['_RESOURCES']._serialized_end=300
  _globals['_PERMISSIONRESOURCES']._serialized_start=303
  _globals['_PERMISSIONRESOURCES']._serialized_end=533
  _globals['_PERMISSIONRESOURCES_PERMISSIONRESOURCESENTRY']._serialized_start=440
  _globals['_PERMISSIONRESOURCES_PERMISSIONRESOURCESENTRY']._serialized_end=533
  _globals['_PERMISSIONS']._serialized_start=536
  _globals['_PERMISSIONS']._serialized_end=810
  _globals['_PERMISSIONS_ORGANIZATIONPERMISSIONRESOURCESENTRY']._serialized_start=695
  _globals['_PERMISSIONS_ORGANIZATIONPERMISSIONRESOURCESENTRY']._serialized_end=810
  _globals['_GETMEREQUEST']._serialized_start=812
  _globals['_GETMEREQUEST']._serialized_end=826
  _globals['_GETMERESPONSE']._serialized_start=829
  _globals['_GETMERESPONSE']._serialized_end=1240
  _globals['_MESERVICE']._serialized_start=1243
  _globals['_MESERVICE']._serialized_end=1416
# @@protoc_insertion_point(module_scope)
