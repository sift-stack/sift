# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/tags/v1/tags.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.api import field_behavior_pb2 as google_dot_api_dot_field__behavior__pb2
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x17sift/tags/v1/tags.proto\x12\x0csift.tags.v1\x1a\x1fgoogle/api/field_behavior.proto\x1a\x1fgoogle/protobuf/timestamp.proto\"\xd8\x02\n\x03Tag\x12\x1a\n\x06tag_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x05tagId\x12\x17\n\x04name\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x04name\x12,\n\x0forganization_id\x18\x03 \x01(\tB\x03\xe0\x41\x02R\x0eorganizationId\x12\x30\n\x12\x63reated_by_user_id\x18\x04 \x01(\tB\x03\xe0\x41\x02R\x0f\x63reatedByUserId\x12\x32\n\x13modified_by_user_id\x18\x05 \x01(\tB\x03\xe0\x41\x02R\x10modifiedByUserId\x12\x42\n\x0c\x63reated_date\x18\x06 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0b\x63reatedDate\x12\x44\n\rmodified_date\x18\x07 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0cmodifiedDate\"=\n\x06TagRef\x12\x1a\n\x06tag_id\x18\x01 \x01(\tB\x03\xe0\x41\x01R\x05tagId\x12\x17\n\x04name\x18\x02 \x01(\tB\x03\xe0\x41\x01R\x04nameBo\n\x10\x63om.sift.tags.v1B\tTagsProtoP\x01\xa2\x02\x03STX\xaa\x02\x0cSift.Tags.V1\xca\x02\x0cSift\\Tags\\V1\xe2\x02\x18Sift\\Tags\\V1\\GPBMetadata\xea\x02\x0eSift::Tags::V1b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.tags.v1.tags_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\020com.sift.tags.v1B\tTagsProtoP\001\242\002\003STX\252\002\014Sift.Tags.V1\312\002\014Sift\\Tags\\V1\342\002\030Sift\\Tags\\V1\\GPBMetadata\352\002\016Sift::Tags::V1'
  _globals['_TAG'].fields_by_name['tag_id']._loaded_options = None
  _globals['_TAG'].fields_by_name['tag_id']._serialized_options = b'\340A\002'
  _globals['_TAG'].fields_by_name['name']._loaded_options = None
  _globals['_TAG'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_TAG'].fields_by_name['organization_id']._loaded_options = None
  _globals['_TAG'].fields_by_name['organization_id']._serialized_options = b'\340A\002'
  _globals['_TAG'].fields_by_name['created_by_user_id']._loaded_options = None
  _globals['_TAG'].fields_by_name['created_by_user_id']._serialized_options = b'\340A\002'
  _globals['_TAG'].fields_by_name['modified_by_user_id']._loaded_options = None
  _globals['_TAG'].fields_by_name['modified_by_user_id']._serialized_options = b'\340A\002'
  _globals['_TAG'].fields_by_name['created_date']._loaded_options = None
  _globals['_TAG'].fields_by_name['created_date']._serialized_options = b'\340A\002'
  _globals['_TAG'].fields_by_name['modified_date']._loaded_options = None
  _globals['_TAG'].fields_by_name['modified_date']._serialized_options = b'\340A\002'
  _globals['_TAGREF'].fields_by_name['tag_id']._loaded_options = None
  _globals['_TAGREF'].fields_by_name['tag_id']._serialized_options = b'\340A\001'
  _globals['_TAGREF'].fields_by_name['name']._loaded_options = None
  _globals['_TAGREF'].fields_by_name['name']._serialized_options = b'\340A\001'
  _globals['_TAG']._serialized_start=108
  _globals['_TAG']._serialized_end=452
  _globals['_TAGREF']._serialized_start=454
  _globals['_TAGREF']._serialized_end=515
# @@protoc_insertion_point(module_scope)
