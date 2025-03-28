# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/remote_files/v1/remote_files.proto
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


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\'sift/remote_files/v1/remote_files.proto\x12\x14sift.remote_files.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a google/protobuf/field_mask.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\"\xce\x07\n\nRemoteFile\x12)\n\x0eremote_file_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x0cremoteFileId\x12,\n\x0forganization_id\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x0eorganizationId\x12 \n\tentity_id\x18\x03 \x01(\tB\x03\xe0\x41\x02R\x08\x65ntityId\x12\x46\n\x0b\x65ntity_type\x18\x04 \x01(\x0e\x32 .sift.remote_files.v1.EntityTypeB\x03\xe0\x41\x02R\nentityType\x12 \n\tfile_name\x18\x05 \x01(\tB\x03\xe0\x41\x02R\x08\x66ileName\x12)\n\x0e\x66ile_mime_type\x18\x06 \x01(\tB\x03\xe0\x41\x02R\x0c\x66ileMimeType\x12\x37\n\x15\x66ile_content_encoding\x18\x07 \x01(\tB\x03\xe0\x41\x02R\x13\x66ileContentEncoding\x12$\n\x0bstorage_key\x18\x08 \x01(\tB\x03\xe0\x41\x02R\nstorageKey\x12 \n\tfile_size\x18\t \x01(\x04\x42\x03\xe0\x41\x02R\x08\x66ileSize\x12*\n\x0b\x64\x65scription\x18\n \x01(\tB\x03\xe0\x41\x01H\x01R\x0b\x64\x65scription\x88\x01\x01\x12Q\n\x0evideo_metadata\x18\x0b \x01(\x0b\x32#.sift.remote_files.v1.VideoMetadataB\x03\xe0\x41\x01H\x00R\rvideoMetadata\x12Q\n\x0eimage_metadata\x18\x0c \x01(\x0b\x32#.sift.remote_files.v1.ImageMetadataB\x03\xe0\x41\x01H\x00R\rimageMetadata\x12Q\n\x0e\x61udio_metadata\x18\x11 \x01(\x0b\x32#.sift.remote_files.v1.AudioMetadataB\x03\xe0\x41\x01H\x00R\raudioMetadata\x12\x30\n\x12\x63reated_by_user_id\x18\r \x01(\tB\x03\xe0\x41\x02R\x0f\x63reatedByUserId\x12\x32\n\x13modified_by_user_id\x18\x0e \x01(\tB\x03\xe0\x41\x02R\x10modifiedByUserId\x12\x42\n\x0c\x63reated_date\x18\x0f \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0b\x63reatedDate\x12\x44\n\rmodified_date\x18\x10 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0cmodifiedDateB\n\n\x08metadataB\x0e\n\x0c_description\"\xb6\x01\n\rVideoMetadata\x12\x1b\n\x06height\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x06height\x12\x19\n\x05width\x18\x02 \x01(\rB\x03\xe0\x41\x01R\x05width\x12.\n\x10\x64uration_seconds\x18\x03 \x01(\x02\x42\x03\xe0\x41\x01R\x0f\x64urationSeconds\x12=\n\ttimestamp\x18\x04 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01R\ttimestamp\"G\n\rImageMetadata\x12\x1b\n\x06height\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x06height\x12\x19\n\x05width\x18\x02 \x01(\rB\x03\xe0\x41\x01R\x05width\"~\n\rAudioMetadata\x12.\n\x10\x64uration_seconds\x18\x01 \x01(\x02\x42\x03\xe0\x41\x01R\x0f\x64urationSeconds\x12=\n\ttimestamp\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01R\ttimestamp\"A\n\x14GetRemoteFileRequest\x12)\n\x0eremote_file_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x0cremoteFileId\"_\n\x15GetRemoteFileResponse\x12\x46\n\x0bremote_file\x18\x01 \x01(\x0b\x32 .sift.remote_files.v1.RemoteFileB\x03\xe0\x41\x02R\nremoteFile\"\xa9\x01\n\x16ListRemoteFilesRequest\x12 \n\tpage_size\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\x12,\n\x0forganization_id\x18\x04 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\"\x86\x01\n\x17ListRemoteFilesResponse\x12\x43\n\x0cremote_files\x18\x01 \x03(\x0b\x32 .sift.remote_files.v1.RemoteFileR\x0bremoteFiles\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken\"\xd5\x05\n\x17\x43reateRemoteFileRequest\x12 \n\tfile_name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x08\x66ileName\x12 \n\tentity_id\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x08\x65ntityId\x12\x46\n\x0b\x65ntity_type\x18\x03 \x01(\x0e\x32 .sift.remote_files.v1.EntityTypeB\x03\xe0\x41\x02R\nentityType\x12)\n\x0e\x66ile_mime_type\x18\x04 \x01(\tB\x03\xe0\x41\x02R\x0c\x66ileMimeType\x12\x37\n\x15\x66ile_content_encoding\x18\x05 \x01(\tB\x03\xe0\x41\x02R\x13\x66ileContentEncoding\x12 \n\tfile_size\x18\x06 \x01(\x04\x42\x03\xe0\x41\x02R\x08\x66ileSize\x12*\n\x0b\x64\x65scription\x18\x07 \x01(\tB\x03\xe0\x41\x01H\x01R\x0b\x64\x65scription\x88\x01\x01\x12,\n\x0forganization_id\x18\x08 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\x12Q\n\x0evideo_metadata\x18\t \x01(\x0b\x32#.sift.remote_files.v1.VideoMetadataB\x03\xe0\x41\x01H\x00R\rvideoMetadata\x12Q\n\x0eimage_metadata\x18\n \x01(\x0b\x32#.sift.remote_files.v1.ImageMetadataB\x03\xe0\x41\x01H\x00R\rimageMetadata\x12Q\n\x0e\x61udio_metadata\x18\x0c \x01(\x0b\x32#.sift.remote_files.v1.AudioMetadataB\x03\xe0\x41\x01H\x00R\raudioMetadata\x12)\n\x0b\x63ustom_uuid\x18\x0b \x01(\tB\x03\xe0\x41\x01H\x02R\ncustomUuid\x88\x01\x01\x42\n\n\x08metadataB\x0e\n\x0c_descriptionB\x0e\n\x0c_custom_uuid\"b\n\x18\x43reateRemoteFileResponse\x12\x46\n\x0bremote_file\x18\x01 \x01(\x0b\x32 .sift.remote_files.v1.RemoteFileB\x03\xe0\x41\x02R\nremoteFile\"D\n\x17\x44\x65leteRemoteFileRequest\x12)\n\x0eremote_file_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x0cremoteFileId\"\x1a\n\x18\x44\x65leteRemoteFileResponse\"G\n\x1d\x42\x61tchDeleteRemoteFilesRequest\x12&\n\x0fremote_file_ids\x18\x01 \x03(\tR\rremoteFileIds\" \n\x1e\x42\x61tchDeleteRemoteFilesResponse\"\xa3\x01\n\x17UpdateRemoteFileRequest\x12\x46\n\x0bremote_file\x18\x01 \x01(\x0b\x32 .sift.remote_files.v1.RemoteFileB\x03\xe0\x41\x02R\nremoteFile\x12@\n\x0bupdate_mask\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.FieldMaskB\x03\xe0\x41\x02R\nupdateMask\"b\n\x18UpdateRemoteFileResponse\x12\x46\n\x0bremote_file\x18\x01 \x01(\x0b\x32 .sift.remote_files.v1.RemoteFileB\x03\xe0\x41\x02R\nremoteFile\"L\n\x1fGetRemoteFileDownloadUrlRequest\x12)\n\x0eremote_file_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x0cremoteFileId\"J\n GetRemoteFileDownloadUrlResponse\x12&\n\x0c\x64ownload_url\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x0b\x64ownloadUrl*\x91\x01\n\nEntityType\x12\x1b\n\x17\x45NTITY_TYPE_UNSPECIFIED\x10\x00\x12\x13\n\x0f\x45NTITY_TYPE_RUN\x10\x01\x12\x1a\n\x16\x45NTITY_TYPE_ANNOTATION\x10\x02\x12\x15\n\x11\x45NTITY_TYPE_ASSET\x10\x03\x12\x1e\n\x1a\x45NTITY_TYPE_ANNOTATION_LOG\x10\x04\x32\xbf\x0c\n\x11RemoteFileService\x12\xc2\x01\n\rGetRemoteFile\x12*.sift.remote_files.v1.GetRemoteFileRequest\x1a+.sift.remote_files.v1.GetRemoteFileResponse\"X\x92\x41(\x12\rGetRemoteFile\x1a\x17Retrieve a remote file.\x82\xd3\xe4\x93\x02\'\x12%/api/v1/remote-files/{remote_file_id}\x12\xbe\x01\n\x10\x43reateRemoteFile\x12-.sift.remote_files.v1.CreateRemoteFileRequest\x1a..sift.remote_files.v1.CreateRemoteFileResponse\"K\x92\x41)\x12\x10\x43reateRemoteFile\x1a\x15\x43reate a remote file.\x82\xd3\xe4\x93\x02\x19\"\x14/api/v1/remote-files:\x01*\x12\xb4\x01\n\x0fListRemoteFiles\x12,.sift.remote_files.v1.ListRemoteFilesRequest\x1a-.sift.remote_files.v1.ListRemoteFilesResponse\"D\x92\x41%\x12\x0fListRemoteFiles\x1a\x12List remote files.\x82\xd3\xe4\x93\x02\x16\x12\x14/api/v1/remote-files\x12\x84\x02\n\x10UpdateRemoteFile\x12-.sift.remote_files.v1.UpdateRemoteFileRequest\x1a..sift.remote_files.v1.UpdateRemoteFileResponse\"\x90\x01\x92\x41n\x12\x10UpdateRemoteFile\x1aZUpdates an existing remote file using using the list of fields specified in `update_mask`.\x82\xd3\xe4\x93\x02\x19\x32\x14/api/v1/remote-files:\x01*\x12\xcc\x01\n\x10\x44\x65leteRemoteFile\x12-.sift.remote_files.v1.DeleteRemoteFileRequest\x1a..sift.remote_files.v1.DeleteRemoteFileResponse\"Y\x92\x41)\x12\x10\x44\x65leteRemoteFile\x1a\x15\x44\x65lete a remote file.\x82\xd3\xe4\x93\x02\'*%/api/v1/remote-files/{remote_file_id}\x12\x8f\x02\n\x16\x42\x61tchDeleteRemoteFiles\x12\x33.sift.remote_files.v1.BatchDeleteRemoteFilesRequest\x1a\x34.sift.remote_files.v1.BatchDeleteRemoteFilesResponse\"\x89\x01\x92\x41[\x12\x16\x42\x61tchDeleteRemoteFiles\x1a\x41\x42\x61tch delete remote files. Each batch is limited to 1000 records.\x82\xd3\xe4\x93\x02%\" /api/v1/remote-files:batchDelete:\x01*\x12\x84\x02\n\x18GetRemoteFileDownloadUrl\x12\x35.sift.remote_files.v1.GetRemoteFileDownloadUrlRequest\x1a\x36.sift.remote_files.v1.GetRemoteFileDownloadUrlResponse\"y\x92\x41<\x12\x10GetRemoteFileUrl\x1a(Gets a download URL for the remote file.\x82\xd3\xe4\x93\x02\x34\x12\x32/api/v1/remote-files/{remote_file_id}/download-urlB\xb3\x01\n\x18\x63om.sift.remote_files.v1B\x10RemoteFilesProtoP\x01\xa2\x02\x03SRX\xaa\x02\x13Sift.RemoteFiles.V1\xca\x02\x13Sift\\RemoteFiles\\V1\xe2\x02\x1fSift\\RemoteFiles\\V1\\GPBMetadata\xea\x02\x15Sift::RemoteFiles::V1\x92\x41\x16\x12\x14\n\x12RemoteFile serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.remote_files.v1.remote_files_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\030com.sift.remote_files.v1B\020RemoteFilesProtoP\001\242\002\003SRX\252\002\023Sift.RemoteFiles.V1\312\002\023Sift\\RemoteFiles\\V1\342\002\037Sift\\RemoteFiles\\V1\\GPBMetadata\352\002\025Sift::RemoteFiles::V1\222A\026\022\024\n\022RemoteFile service'
  _globals['_REMOTEFILE'].fields_by_name['remote_file_id']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['remote_file_id']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['organization_id']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['organization_id']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['entity_id']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['entity_id']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['entity_type']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['entity_type']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['file_name']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['file_name']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['file_mime_type']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['file_mime_type']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['file_content_encoding']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['file_content_encoding']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['storage_key']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['storage_key']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['file_size']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['file_size']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['description']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['description']._serialized_options = b'\340A\001'
  _globals['_REMOTEFILE'].fields_by_name['video_metadata']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['video_metadata']._serialized_options = b'\340A\001'
  _globals['_REMOTEFILE'].fields_by_name['image_metadata']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['image_metadata']._serialized_options = b'\340A\001'
  _globals['_REMOTEFILE'].fields_by_name['audio_metadata']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['audio_metadata']._serialized_options = b'\340A\001'
  _globals['_REMOTEFILE'].fields_by_name['created_by_user_id']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['created_by_user_id']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['modified_by_user_id']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['modified_by_user_id']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['created_date']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['created_date']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILE'].fields_by_name['modified_date']._loaded_options = None
  _globals['_REMOTEFILE'].fields_by_name['modified_date']._serialized_options = b'\340A\002'
  _globals['_VIDEOMETADATA'].fields_by_name['height']._loaded_options = None
  _globals['_VIDEOMETADATA'].fields_by_name['height']._serialized_options = b'\340A\001'
  _globals['_VIDEOMETADATA'].fields_by_name['width']._loaded_options = None
  _globals['_VIDEOMETADATA'].fields_by_name['width']._serialized_options = b'\340A\001'
  _globals['_VIDEOMETADATA'].fields_by_name['duration_seconds']._loaded_options = None
  _globals['_VIDEOMETADATA'].fields_by_name['duration_seconds']._serialized_options = b'\340A\001'
  _globals['_VIDEOMETADATA'].fields_by_name['timestamp']._loaded_options = None
  _globals['_VIDEOMETADATA'].fields_by_name['timestamp']._serialized_options = b'\340A\001'
  _globals['_IMAGEMETADATA'].fields_by_name['height']._loaded_options = None
  _globals['_IMAGEMETADATA'].fields_by_name['height']._serialized_options = b'\340A\001'
  _globals['_IMAGEMETADATA'].fields_by_name['width']._loaded_options = None
  _globals['_IMAGEMETADATA'].fields_by_name['width']._serialized_options = b'\340A\001'
  _globals['_AUDIOMETADATA'].fields_by_name['duration_seconds']._loaded_options = None
  _globals['_AUDIOMETADATA'].fields_by_name['duration_seconds']._serialized_options = b'\340A\001'
  _globals['_AUDIOMETADATA'].fields_by_name['timestamp']._loaded_options = None
  _globals['_AUDIOMETADATA'].fields_by_name['timestamp']._serialized_options = b'\340A\001'
  _globals['_GETREMOTEFILEREQUEST'].fields_by_name['remote_file_id']._loaded_options = None
  _globals['_GETREMOTEFILEREQUEST'].fields_by_name['remote_file_id']._serialized_options = b'\340A\002'
  _globals['_GETREMOTEFILERESPONSE'].fields_by_name['remote_file']._loaded_options = None
  _globals['_GETREMOTEFILERESPONSE'].fields_by_name['remote_file']._serialized_options = b'\340A\002'
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_LISTREMOTEFILESREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_name']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_name']._serialized_options = b'\340A\002'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['entity_id']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['entity_id']._serialized_options = b'\340A\002'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['entity_type']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['entity_type']._serialized_options = b'\340A\002'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_mime_type']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_mime_type']._serialized_options = b'\340A\002'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_content_encoding']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_content_encoding']._serialized_options = b'\340A\002'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_size']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['file_size']._serialized_options = b'\340A\002'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['description']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['description']._serialized_options = b'\340A\001'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['video_metadata']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['video_metadata']._serialized_options = b'\340A\001'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['image_metadata']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['image_metadata']._serialized_options = b'\340A\001'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['audio_metadata']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['audio_metadata']._serialized_options = b'\340A\001'
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['custom_uuid']._loaded_options = None
  _globals['_CREATEREMOTEFILEREQUEST'].fields_by_name['custom_uuid']._serialized_options = b'\340A\001'
  _globals['_CREATEREMOTEFILERESPONSE'].fields_by_name['remote_file']._loaded_options = None
  _globals['_CREATEREMOTEFILERESPONSE'].fields_by_name['remote_file']._serialized_options = b'\340A\002'
  _globals['_DELETEREMOTEFILEREQUEST'].fields_by_name['remote_file_id']._loaded_options = None
  _globals['_DELETEREMOTEFILEREQUEST'].fields_by_name['remote_file_id']._serialized_options = b'\340A\002'
  _globals['_UPDATEREMOTEFILEREQUEST'].fields_by_name['remote_file']._loaded_options = None
  _globals['_UPDATEREMOTEFILEREQUEST'].fields_by_name['remote_file']._serialized_options = b'\340A\002'
  _globals['_UPDATEREMOTEFILEREQUEST'].fields_by_name['update_mask']._loaded_options = None
  _globals['_UPDATEREMOTEFILEREQUEST'].fields_by_name['update_mask']._serialized_options = b'\340A\002'
  _globals['_UPDATEREMOTEFILERESPONSE'].fields_by_name['remote_file']._loaded_options = None
  _globals['_UPDATEREMOTEFILERESPONSE'].fields_by_name['remote_file']._serialized_options = b'\340A\002'
  _globals['_GETREMOTEFILEDOWNLOADURLREQUEST'].fields_by_name['remote_file_id']._loaded_options = None
  _globals['_GETREMOTEFILEDOWNLOADURLREQUEST'].fields_by_name['remote_file_id']._serialized_options = b'\340A\002'
  _globals['_GETREMOTEFILEDOWNLOADURLRESPONSE'].fields_by_name['download_url']._loaded_options = None
  _globals['_GETREMOTEFILEDOWNLOADURLRESPONSE'].fields_by_name['download_url']._serialized_options = b'\340A\002'
  _globals['_REMOTEFILESERVICE'].methods_by_name['GetRemoteFile']._loaded_options = None
  _globals['_REMOTEFILESERVICE'].methods_by_name['GetRemoteFile']._serialized_options = b'\222A(\022\rGetRemoteFile\032\027Retrieve a remote file.\202\323\344\223\002\'\022%/api/v1/remote-files/{remote_file_id}'
  _globals['_REMOTEFILESERVICE'].methods_by_name['CreateRemoteFile']._loaded_options = None
  _globals['_REMOTEFILESERVICE'].methods_by_name['CreateRemoteFile']._serialized_options = b'\222A)\022\020CreateRemoteFile\032\025Create a remote file.\202\323\344\223\002\031\"\024/api/v1/remote-files:\001*'
  _globals['_REMOTEFILESERVICE'].methods_by_name['ListRemoteFiles']._loaded_options = None
  _globals['_REMOTEFILESERVICE'].methods_by_name['ListRemoteFiles']._serialized_options = b'\222A%\022\017ListRemoteFiles\032\022List remote files.\202\323\344\223\002\026\022\024/api/v1/remote-files'
  _globals['_REMOTEFILESERVICE'].methods_by_name['UpdateRemoteFile']._loaded_options = None
  _globals['_REMOTEFILESERVICE'].methods_by_name['UpdateRemoteFile']._serialized_options = b'\222An\022\020UpdateRemoteFile\032ZUpdates an existing remote file using using the list of fields specified in `update_mask`.\202\323\344\223\002\0312\024/api/v1/remote-files:\001*'
  _globals['_REMOTEFILESERVICE'].methods_by_name['DeleteRemoteFile']._loaded_options = None
  _globals['_REMOTEFILESERVICE'].methods_by_name['DeleteRemoteFile']._serialized_options = b'\222A)\022\020DeleteRemoteFile\032\025Delete a remote file.\202\323\344\223\002\'*%/api/v1/remote-files/{remote_file_id}'
  _globals['_REMOTEFILESERVICE'].methods_by_name['BatchDeleteRemoteFiles']._loaded_options = None
  _globals['_REMOTEFILESERVICE'].methods_by_name['BatchDeleteRemoteFiles']._serialized_options = b'\222A[\022\026BatchDeleteRemoteFiles\032ABatch delete remote files. Each batch is limited to 1000 records.\202\323\344\223\002%\" /api/v1/remote-files:batchDelete:\001*'
  _globals['_REMOTEFILESERVICE'].methods_by_name['GetRemoteFileDownloadUrl']._loaded_options = None
  _globals['_REMOTEFILESERVICE'].methods_by_name['GetRemoteFileDownloadUrl']._serialized_options = b'\222A<\022\020GetRemoteFileUrl\032(Gets a download URL for the remote file.\202\323\344\223\0024\0222/api/v1/remote-files/{remote_file_id}/download-url'
  _globals['_ENTITYTYPE']._serialized_start=3533
  _globals['_ENTITYTYPE']._serialized_end=3678
  _globals['_REMOTEFILE']._serialized_start=244
  _globals['_REMOTEFILE']._serialized_end=1218
  _globals['_VIDEOMETADATA']._serialized_start=1221
  _globals['_VIDEOMETADATA']._serialized_end=1403
  _globals['_IMAGEMETADATA']._serialized_start=1405
  _globals['_IMAGEMETADATA']._serialized_end=1476
  _globals['_AUDIOMETADATA']._serialized_start=1478
  _globals['_AUDIOMETADATA']._serialized_end=1604
  _globals['_GETREMOTEFILEREQUEST']._serialized_start=1606
  _globals['_GETREMOTEFILEREQUEST']._serialized_end=1671
  _globals['_GETREMOTEFILERESPONSE']._serialized_start=1673
  _globals['_GETREMOTEFILERESPONSE']._serialized_end=1768
  _globals['_LISTREMOTEFILESREQUEST']._serialized_start=1771
  _globals['_LISTREMOTEFILESREQUEST']._serialized_end=1940
  _globals['_LISTREMOTEFILESRESPONSE']._serialized_start=1943
  _globals['_LISTREMOTEFILESRESPONSE']._serialized_end=2077
  _globals['_CREATEREMOTEFILEREQUEST']._serialized_start=2080
  _globals['_CREATEREMOTEFILEREQUEST']._serialized_end=2805
  _globals['_CREATEREMOTEFILERESPONSE']._serialized_start=2807
  _globals['_CREATEREMOTEFILERESPONSE']._serialized_end=2905
  _globals['_DELETEREMOTEFILEREQUEST']._serialized_start=2907
  _globals['_DELETEREMOTEFILEREQUEST']._serialized_end=2975
  _globals['_DELETEREMOTEFILERESPONSE']._serialized_start=2977
  _globals['_DELETEREMOTEFILERESPONSE']._serialized_end=3003
  _globals['_BATCHDELETEREMOTEFILESREQUEST']._serialized_start=3005
  _globals['_BATCHDELETEREMOTEFILESREQUEST']._serialized_end=3076
  _globals['_BATCHDELETEREMOTEFILESRESPONSE']._serialized_start=3078
  _globals['_BATCHDELETEREMOTEFILESRESPONSE']._serialized_end=3110
  _globals['_UPDATEREMOTEFILEREQUEST']._serialized_start=3113
  _globals['_UPDATEREMOTEFILEREQUEST']._serialized_end=3276
  _globals['_UPDATEREMOTEFILERESPONSE']._serialized_start=3278
  _globals['_UPDATEREMOTEFILERESPONSE']._serialized_end=3376
  _globals['_GETREMOTEFILEDOWNLOADURLREQUEST']._serialized_start=3378
  _globals['_GETREMOTEFILEDOWNLOADURLREQUEST']._serialized_end=3454
  _globals['_GETREMOTEFILEDOWNLOADURLRESPONSE']._serialized_start=3456
  _globals['_GETREMOTEFILEDOWNLOADURLRESPONSE']._serialized_end=3530
  _globals['_REMOTEFILESERVICE']._serialized_start=3681
  _globals['_REMOTEFILESERVICE']._serialized_end=5280
# @@protoc_insertion_point(module_scope)
