# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/ingest/v1/ingest.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1bsift/ingest/v1/ingest.proto\x12\x0esift.ingest.v1\x1a\x1bgoogle/protobuf/empty.proto\x1a\x1fgoogle/protobuf/timestamp.proto\"\xfe\x02\n!IngestWithConfigDataStreamRequest\x12.\n\x13ingestion_config_id\x18\x01 \x01(\tR\x11ingestionConfigId\x12\x12\n\x04\x66low\x18\x02 \x01(\tR\x04\x66low\x12\x38\n\ttimestamp\x18\x03 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\ttimestamp\x12W\n\x0e\x63hannel_values\x18\x04 \x03(\x0b\x32\x30.sift.ingest.v1.IngestWithConfigDataChannelValueR\rchannelValues\x12\x15\n\x06run_id\x18\x05 \x01(\tR\x05runId\x12\x42\n\x1e\x65nd_stream_on_validation_error\x18\x06 \x01(\x08R\x1a\x65ndStreamOnValidationError\x12\'\n\x0forganization_id\x18\x07 \x01(\tR\x0eorganizationId\"$\n\"IngestWithConfigDataStreamResponse\"\xd5\x02\n IngestWithConfigDataChannelValue\x12\x18\n\x06string\x18\x01 \x01(\tH\x00R\x06string\x12\x18\n\x06\x64ouble\x18\x02 \x01(\x01H\x00R\x06\x64ouble\x12\x16\n\x05\x66loat\x18\x03 \x01(\x02H\x00R\x05\x66loat\x12\x14\n\x04\x62ool\x18\x04 \x01(\x08H\x00R\x04\x62ool\x12\x16\n\x05int32\x18\x05 \x01(\x05H\x00R\x05int32\x12\x18\n\x06uint32\x18\x06 \x01(\rH\x00R\x06uint32\x12\x16\n\x05int64\x18\x07 \x01(\x03H\x00R\x05int64\x12\x18\n\x06uint64\x18\x08 \x01(\x04H\x00R\x06uint64\x12\x1d\n\tbit_field\x18\t \x01(\x0cH\x00R\x08\x62itField\x12\x14\n\x04\x65num\x18\n \x01(\rH\x00R\x04\x65num\x12.\n\x05\x65mpty\x18\x0b \x01(\x0b\x32\x16.google.protobuf.EmptyH\x00R\x05\x65mptyB\x06\n\x04type2\x97\x01\n\rIngestService\x12\x85\x01\n\x1aIngestWithConfigDataStream\x12\x31.sift.ingest.v1.IngestWithConfigDataStreamRequest\x1a\x32.sift.ingest.v1.IngestWithConfigDataStreamResponse(\x01\x42\xac\x01\n\x12\x63om.sift.ingest.v1B\x0bIngestProtoP\x01Z/azimuth/gen/protos/go/sift/ingest/v1;ingestv1pb\xa2\x02\x03SIX\xaa\x02\x0eSift.Ingest.V1\xca\x02\x0eSift\\Ingest\\V1\xe2\x02\x1aSift\\Ingest\\V1\\GPBMetadata\xea\x02\x10Sift::Ingest::V1b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.ingest.v1.ingest_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\022com.sift.ingest.v1B\013IngestProtoP\001Z/azimuth/gen/protos/go/sift/ingest/v1;ingestv1pb\242\002\003SIX\252\002\016Sift.Ingest.V1\312\002\016Sift\\Ingest\\V1\342\002\032Sift\\Ingest\\V1\\GPBMetadata\352\002\020Sift::Ingest::V1'
  _globals['_INGESTWITHCONFIGDATASTREAMREQUEST']._serialized_start=110
  _globals['_INGESTWITHCONFIGDATASTREAMREQUEST']._serialized_end=492
  _globals['_INGESTWITHCONFIGDATASTREAMRESPONSE']._serialized_start=494
  _globals['_INGESTWITHCONFIGDATASTREAMRESPONSE']._serialized_end=530
  _globals['_INGESTWITHCONFIGDATACHANNELVALUE']._serialized_start=533
  _globals['_INGESTWITHCONFIGDATACHANNELVALUE']._serialized_end=874
  _globals['_INGESTSERVICE']._serialized_start=877
  _globals['_INGESTSERVICE']._serialized_end=1028
# @@protoc_insertion_point(module_scope)