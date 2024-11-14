# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/runs/v2/runs.proto
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


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x17sift/runs/v2/runs.proto\x12\x0csift.runs.v2\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a google/protobuf/field_mask.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\"\xc8\x05\n\x03Run\x12\x1a\n\x06run_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x05runId\x12\x42\n\x0c\x63reated_date\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0b\x63reatedDate\x12\x44\n\rmodified_date\x18\x03 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0cmodifiedDate\x12\x30\n\x12\x63reated_by_user_id\x18\x04 \x01(\tB\x03\xe0\x41\x02R\x0f\x63reatedByUserId\x12\x32\n\x13modified_by_user_id\x18\x05 \x01(\tB\x03\xe0\x41\x02R\x10modifiedByUserId\x12,\n\x0forganization_id\x18\x06 \x01(\tB\x03\xe0\x41\x02R\x0eorganizationId\x12\x43\n\nstart_time\x18\x07 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01H\x00R\tstartTime\x88\x01\x01\x12\x41\n\tstop_time\x18\x08 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01H\x01R\x08stopTime\x88\x01\x01\x12 \n\tis_pinned\x18\t \x01(\x08\x42\x03\xe0\x41\x02R\x08isPinned\x12\x17\n\x04name\x18\n \x01(\tB\x03\xe0\x41\x02R\x04name\x12%\n\x0b\x64\x65scription\x18\x0b \x01(\tB\x03\xe0\x41\x02R\x0b\x64\x65scription\x12\x17\n\x04tags\x18\x0c \x03(\tB\x03\xe0\x41\x02R\x04tags\x12/\n\x11\x64\x65\x66\x61ult_report_id\x18\r \x01(\tB\x03\xe0\x41\x01R\x0f\x64\x65\x66\x61ultReportId\x12\'\n\nclient_key\x18\x0e \x01(\tB\x03\xe0\x41\x01H\x02R\tclientKey\x88\x01\x01\x42\r\n\x0b_start_timeB\x0c\n\n_stop_timeB\r\n\x0b_client_key\"+\n\rGetRunRequest\x12\x1a\n\x06run_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x05runId\":\n\x0eGetRunResponse\x12(\n\x03run\x18\x01 \x01(\x0b\x32\x11.sift.runs.v2.RunB\x03\xe0\x41\x02R\x03run\"\x94\x01\n\x0fListRunsRequest\x12 \n\tpage_size\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\x12\x1e\n\x08order_by\x18\x04 \x01(\tB\x03\xe0\x41\x01R\x07orderBy\"f\n\x10ListRunsResponse\x12*\n\x04runs\x18\x01 \x03(\x0b\x32\x11.sift.runs.v2.RunB\x03\xe0\x41\x02R\x04runs\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken\"\xcf\x02\n\x10\x43reateRunRequest\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12%\n\x0b\x64\x65scription\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x0b\x64\x65scription\x12\x17\n\x04tags\x18\x03 \x03(\tB\x03\xe0\x41\x01R\x04tags\x12>\n\nstart_time\x18\x04 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01R\tstartTime\x12<\n\tstop_time\x18\x05 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01R\x08stopTime\x12,\n\x0forganization_id\x18\x07 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\x12\'\n\nclient_key\x18\x08 \x01(\tB\x03\xe0\x41\x01H\x00R\tclientKey\x88\x01\x01\x42\r\n\x0b_client_key\"=\n\x11\x43reateRunResponse\x12(\n\x03run\x18\x01 \x01(\x0b\x32\x11.sift.runs.v2.RunB\x03\xe0\x41\x02R\x03run\"~\n\x10UpdateRunRequest\x12(\n\x03run\x18\x01 \x01(\x0b\x32\x11.sift.runs.v2.RunB\x03\xe0\x41\x02R\x03run\x12@\n\x0bupdate_mask\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.FieldMaskB\x03\xe0\x41\x02R\nupdateMask\"=\n\x11UpdateRunResponse\x12(\n\x03run\x18\x01 \x01(\x0b\x32\x11.sift.runs.v2.RunB\x03\xe0\x41\x02R\x03run\"q\n-CreateAutomaticRunAssociationForAssetsRequest\x12\x1a\n\x06run_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x05runId\x12$\n\x0b\x61sset_names\x18\x02 \x03(\tB\x03\xe0\x41\x02R\nassetNames\"0\n.CreateAutomaticRunAssociationForAssetsResponse\".\n\x10\x44\x65leteRunRequest\x12\x1a\n\x06run_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x05runId\"\x13\n\x11\x44\x65leteRunResponse\",\n\x0eStopRunRequest\x12\x1a\n\x06run_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x05runId\"\x11\n\x0fStopRunResponse2\xcd\x0c\n\nRunService\x12~\n\x06GetRun\x12\x1b.sift.runs.v2.GetRunRequest\x1a\x1c.sift.runs.v2.GetRunResponse\"9\x92\x41\x19\x12\x06GetRun\x1a\x0fRetrieve a run.\x82\xd3\xe4\x93\x02\x17\x12\x15/api/v2/runs/{run_id}\x12\x95\x01\n\x08ListRuns\x12\x1d.sift.runs.v2.ListRunsRequest\x1a\x1e.sift.runs.v2.ListRunsResponse\"J\x92\x41\x33\x12\x08ListRuns\x1a\'Retrieve runs using an optional filter.\x82\xd3\xe4\x93\x02\x0e\x12\x0c/api/v2/runs\x12\x82\x01\n\tCreateRun\x12\x1e.sift.runs.v2.CreateRunRequest\x1a\x1f.sift.runs.v2.CreateRunResponse\"4\x92\x41\x1a\x12\tCreateRun\x1a\rCreate a run.\x82\xd3\xe4\x93\x02\x11\"\x0c/api/v2/runs:\x01*\x12\xc7\x01\n\tUpdateRun\x12\x1e.sift.runs.v2.UpdateRunRequest\x1a\x1f.sift.runs.v2.UpdateRunResponse\"y\x92\x41_\x12\tUpdateRun\x1aRUpdates an existing run using using the list of fields specified in `update_mask`.\x82\xd3\xe4\x93\x02\x11\x32\x0c/api/v2/runs:\x01*\x12\xdc\x01\n\tDeleteRun\x12\x1e.sift.runs.v2.DeleteRunRequest\x1a\x1f.sift.runs.v2.DeleteRunResponse\"\x8d\x01\x92\x41m\x12\tDeleteRun\x1a`Permanently delete a given run. In order for a run to be deleted it must have a set `stop_time`.\x82\xd3\xe4\x93\x02\x17*\x15/api/v2/runs/{run_id}\x12\xe6\x01\n\x07StopRun\x12\x1c.sift.runs.v2.StopRunRequest\x1a\x1d.sift.runs.v2.StopRunResponse\"\x9d\x01\x92\x41~\x12\x07StopRun\x1asSet the stop time of a run to the current time. To set the stop time of a run to an arbitrary time see `UpdateRun`.\x82\xd3\xe4\x93\x02\x16\x32\x11/api/v2/runs:stop:\x01*\x12\xcc\x02\n&CreateAutomaticRunAssociationForAssets\x12;.sift.runs.v2.CreateAutomaticRunAssociationForAssetsRequest\x1a<.sift.runs.v2.CreateAutomaticRunAssociationForAssetsResponse\"\xa6\x01\x92\x41W\x12&CreateAutomaticRunAssociationForAssets\x1a-Associates a list of assets with a given run.\x82\xd3\xe4\x93\x02\x46\"A/api/v2/runs/{run_id}:create-automatic-run-association-for-assets:\x01*\x1a\xc0\x01\x92\x41\xbc\x01\x12@Service to programmatically interact with [runs](/glossary#run).\x1ax\n\x1eRead more about what runs are.\x12Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/265454053B\x81\x01\n\x10\x63om.sift.runs.v2B\tRunsProtoP\x01\xa2\x02\x03SRX\xaa\x02\x0cSift.Runs.V2\xca\x02\x0cSift\\Runs\\V2\xe2\x02\x18Sift\\Runs\\V2\\GPBMetadata\xea\x02\x0eSift::Runs::V2\x92\x41\x0f\x12\r\n\x0bRun serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.runs.v2.runs_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\020com.sift.runs.v2B\tRunsProtoP\001\242\002\003SRX\252\002\014Sift.Runs.V2\312\002\014Sift\\Runs\\V2\342\002\030Sift\\Runs\\V2\\GPBMetadata\352\002\016Sift::Runs::V2\222A\017\022\r\n\013Run service'
  _globals['_RUN'].fields_by_name['run_id']._loaded_options = None
  _globals['_RUN'].fields_by_name['run_id']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['created_date']._loaded_options = None
  _globals['_RUN'].fields_by_name['created_date']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['modified_date']._loaded_options = None
  _globals['_RUN'].fields_by_name['modified_date']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['created_by_user_id']._loaded_options = None
  _globals['_RUN'].fields_by_name['created_by_user_id']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['modified_by_user_id']._loaded_options = None
  _globals['_RUN'].fields_by_name['modified_by_user_id']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['organization_id']._loaded_options = None
  _globals['_RUN'].fields_by_name['organization_id']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['start_time']._loaded_options = None
  _globals['_RUN'].fields_by_name['start_time']._serialized_options = b'\340A\001'
  _globals['_RUN'].fields_by_name['stop_time']._loaded_options = None
  _globals['_RUN'].fields_by_name['stop_time']._serialized_options = b'\340A\001'
  _globals['_RUN'].fields_by_name['is_pinned']._loaded_options = None
  _globals['_RUN'].fields_by_name['is_pinned']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['name']._loaded_options = None
  _globals['_RUN'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['description']._loaded_options = None
  _globals['_RUN'].fields_by_name['description']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['tags']._loaded_options = None
  _globals['_RUN'].fields_by_name['tags']._serialized_options = b'\340A\002'
  _globals['_RUN'].fields_by_name['default_report_id']._loaded_options = None
  _globals['_RUN'].fields_by_name['default_report_id']._serialized_options = b'\340A\001'
  _globals['_RUN'].fields_by_name['client_key']._loaded_options = None
  _globals['_RUN'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_GETRUNREQUEST'].fields_by_name['run_id']._loaded_options = None
  _globals['_GETRUNREQUEST'].fields_by_name['run_id']._serialized_options = b'\340A\002'
  _globals['_GETRUNRESPONSE'].fields_by_name['run']._loaded_options = None
  _globals['_GETRUNRESPONSE'].fields_by_name['run']._serialized_options = b'\340A\002'
  _globals['_LISTRUNSREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTRUNSREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTRUNSREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTRUNSREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTRUNSREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTRUNSREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_LISTRUNSREQUEST'].fields_by_name['order_by']._loaded_options = None
  _globals['_LISTRUNSREQUEST'].fields_by_name['order_by']._serialized_options = b'\340A\001'
  _globals['_LISTRUNSRESPONSE'].fields_by_name['runs']._loaded_options = None
  _globals['_LISTRUNSRESPONSE'].fields_by_name['runs']._serialized_options = b'\340A\002'
  _globals['_CREATERUNREQUEST'].fields_by_name['name']._loaded_options = None
  _globals['_CREATERUNREQUEST'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_CREATERUNREQUEST'].fields_by_name['description']._loaded_options = None
  _globals['_CREATERUNREQUEST'].fields_by_name['description']._serialized_options = b'\340A\002'
  _globals['_CREATERUNREQUEST'].fields_by_name['tags']._loaded_options = None
  _globals['_CREATERUNREQUEST'].fields_by_name['tags']._serialized_options = b'\340A\001'
  _globals['_CREATERUNREQUEST'].fields_by_name['start_time']._loaded_options = None
  _globals['_CREATERUNREQUEST'].fields_by_name['start_time']._serialized_options = b'\340A\001'
  _globals['_CREATERUNREQUEST'].fields_by_name['stop_time']._loaded_options = None
  _globals['_CREATERUNREQUEST'].fields_by_name['stop_time']._serialized_options = b'\340A\001'
  _globals['_CREATERUNREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_CREATERUNREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_CREATERUNREQUEST'].fields_by_name['client_key']._loaded_options = None
  _globals['_CREATERUNREQUEST'].fields_by_name['client_key']._serialized_options = b'\340A\001'
  _globals['_CREATERUNRESPONSE'].fields_by_name['run']._loaded_options = None
  _globals['_CREATERUNRESPONSE'].fields_by_name['run']._serialized_options = b'\340A\002'
  _globals['_UPDATERUNREQUEST'].fields_by_name['run']._loaded_options = None
  _globals['_UPDATERUNREQUEST'].fields_by_name['run']._serialized_options = b'\340A\002'
  _globals['_UPDATERUNREQUEST'].fields_by_name['update_mask']._loaded_options = None
  _globals['_UPDATERUNREQUEST'].fields_by_name['update_mask']._serialized_options = b'\340A\002'
  _globals['_UPDATERUNRESPONSE'].fields_by_name['run']._loaded_options = None
  _globals['_UPDATERUNRESPONSE'].fields_by_name['run']._serialized_options = b'\340A\002'
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSREQUEST'].fields_by_name['run_id']._loaded_options = None
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSREQUEST'].fields_by_name['run_id']._serialized_options = b'\340A\002'
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSREQUEST'].fields_by_name['asset_names']._loaded_options = None
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSREQUEST'].fields_by_name['asset_names']._serialized_options = b'\340A\002'
  _globals['_DELETERUNREQUEST'].fields_by_name['run_id']._loaded_options = None
  _globals['_DELETERUNREQUEST'].fields_by_name['run_id']._serialized_options = b'\340A\002'
  _globals['_STOPRUNREQUEST'].fields_by_name['run_id']._loaded_options = None
  _globals['_STOPRUNREQUEST'].fields_by_name['run_id']._serialized_options = b'\340A\002'
  _globals['_RUNSERVICE']._loaded_options = None
  _globals['_RUNSERVICE']._serialized_options = b'\222A\274\001\022@Service to programmatically interact with [runs](/glossary#run).\032x\n\036Read more about what runs are.\022Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/265454053'
  _globals['_RUNSERVICE'].methods_by_name['GetRun']._loaded_options = None
  _globals['_RUNSERVICE'].methods_by_name['GetRun']._serialized_options = b'\222A\031\022\006GetRun\032\017Retrieve a run.\202\323\344\223\002\027\022\025/api/v2/runs/{run_id}'
  _globals['_RUNSERVICE'].methods_by_name['ListRuns']._loaded_options = None
  _globals['_RUNSERVICE'].methods_by_name['ListRuns']._serialized_options = b'\222A3\022\010ListRuns\032\'Retrieve runs using an optional filter.\202\323\344\223\002\016\022\014/api/v2/runs'
  _globals['_RUNSERVICE'].methods_by_name['CreateRun']._loaded_options = None
  _globals['_RUNSERVICE'].methods_by_name['CreateRun']._serialized_options = b'\222A\032\022\tCreateRun\032\rCreate a run.\202\323\344\223\002\021\"\014/api/v2/runs:\001*'
  _globals['_RUNSERVICE'].methods_by_name['UpdateRun']._loaded_options = None
  _globals['_RUNSERVICE'].methods_by_name['UpdateRun']._serialized_options = b'\222A_\022\tUpdateRun\032RUpdates an existing run using using the list of fields specified in `update_mask`.\202\323\344\223\002\0212\014/api/v2/runs:\001*'
  _globals['_RUNSERVICE'].methods_by_name['DeleteRun']._loaded_options = None
  _globals['_RUNSERVICE'].methods_by_name['DeleteRun']._serialized_options = b'\222Am\022\tDeleteRun\032`Permanently delete a given run. In order for a run to be deleted it must have a set `stop_time`.\202\323\344\223\002\027*\025/api/v2/runs/{run_id}'
  _globals['_RUNSERVICE'].methods_by_name['StopRun']._loaded_options = None
  _globals['_RUNSERVICE'].methods_by_name['StopRun']._serialized_options = b'\222A~\022\007StopRun\032sSet the stop time of a run to the current time. To set the stop time of a run to an arbitrary time see `UpdateRun`.\202\323\344\223\002\0262\021/api/v2/runs:stop:\001*'
  _globals['_RUNSERVICE'].methods_by_name['CreateAutomaticRunAssociationForAssets']._loaded_options = None
  _globals['_RUNSERVICE'].methods_by_name['CreateAutomaticRunAssociationForAssets']._serialized_options = b'\222AW\022&CreateAutomaticRunAssociationForAssets\032-Associates a list of assets with a given run.\202\323\344\223\002F\"A/api/v2/runs/{run_id}:create-automatic-run-association-for-assets:\001*'
  _globals['_RUN']._serialized_start=220
  _globals['_RUN']._serialized_end=932
  _globals['_GETRUNREQUEST']._serialized_start=934
  _globals['_GETRUNREQUEST']._serialized_end=977
  _globals['_GETRUNRESPONSE']._serialized_start=979
  _globals['_GETRUNRESPONSE']._serialized_end=1037
  _globals['_LISTRUNSREQUEST']._serialized_start=1040
  _globals['_LISTRUNSREQUEST']._serialized_end=1188
  _globals['_LISTRUNSRESPONSE']._serialized_start=1190
  _globals['_LISTRUNSRESPONSE']._serialized_end=1292
  _globals['_CREATERUNREQUEST']._serialized_start=1295
  _globals['_CREATERUNREQUEST']._serialized_end=1630
  _globals['_CREATERUNRESPONSE']._serialized_start=1632
  _globals['_CREATERUNRESPONSE']._serialized_end=1693
  _globals['_UPDATERUNREQUEST']._serialized_start=1695
  _globals['_UPDATERUNREQUEST']._serialized_end=1821
  _globals['_UPDATERUNRESPONSE']._serialized_start=1823
  _globals['_UPDATERUNRESPONSE']._serialized_end=1884
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSREQUEST']._serialized_start=1886
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSREQUEST']._serialized_end=1999
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSRESPONSE']._serialized_start=2001
  _globals['_CREATEAUTOMATICRUNASSOCIATIONFORASSETSRESPONSE']._serialized_end=2049
  _globals['_DELETERUNREQUEST']._serialized_start=2051
  _globals['_DELETERUNREQUEST']._serialized_end=2097
  _globals['_DELETERUNRESPONSE']._serialized_start=2099
  _globals['_DELETERUNRESPONSE']._serialized_end=2118
  _globals['_STOPRUNREQUEST']._serialized_start=2120
  _globals['_STOPRUNREQUEST']._serialized_end=2164
  _globals['_STOPRUNRESPONSE']._serialized_start=2166
  _globals['_STOPRUNRESPONSE']._serialized_end=2183
  _globals['_RUNSERVICE']._serialized_start=2186
  _globals['_RUNSERVICE']._serialized_end=3799
# @@protoc_insertion_point(module_scope)
