# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/saved_searches/v1/saved_searches.proto
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


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n+sift/saved_searches/v1/saved_searches.proto\x12\x16sift.saved_searches.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a google/protobuf/field_mask.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\"\xc5\x03\n\x0bSavedSearch\x12+\n\x0fsaved_search_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\rsavedSearchId\x12,\n\x0forganization_id\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x0eorganizationId\x12\x17\n\x04name\x18\x03 \x01(\tB\x03\xe0\x41\x02R\x04name\x12R\n\nproperties\x18\x04 \x01(\x0b\x32-.sift.saved_searches.v1.SavedSearchPropertiesB\x03\xe0\x41\x02R\nproperties\x12\x30\n\x12\x63reated_by_user_id\x18\x05 \x01(\tB\x03\xe0\x41\x02R\x0f\x63reatedByUserId\x12\x32\n\x13modified_by_user_id\x18\x06 \x01(\tB\x03\xe0\x41\x02R\x10modifiedByUserId\x12\x42\n\x0c\x63reated_date\x18\x07 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0b\x63reatedDate\x12\x44\n\rmodified_date\x18\x08 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x02R\x0cmodifiedDate\"\xdd\x05\n\x15SavedSearchProperties\x12(\n\roverview_mode\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x0coverviewMode\x12)\n\x0bsearch_term\x18\x02 \x01(\tB\x03\xe0\x41\x01H\x00R\nsearchTerm\x88\x01\x01\x12J\n\x0e\x66rom_date_time\x18\x03 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01H\x01R\x0c\x66romDateTime\x88\x01\x01\x12\x46\n\x0cto_date_time\x18\x04 \x01(\x0b\x32\x1a.google.protobuf.TimestampB\x03\xe0\x41\x01H\x02R\ntoDateTime\x88\x01\x01\x12S\n\x0b\x61sset_items\x18\x05 \x03(\x0b\x32-.sift.saved_searches.v1.SavedSearchFilterItemB\x03\xe0\x41\x01R\nassetItems\x12Q\n\nuser_items\x18\x06 \x03(\x0b\x32-.sift.saved_searches.v1.SavedSearchFilterItemB\x03\xe0\x41\x01R\tuserItems\x12O\n\ttag_items\x18\x07 \x03(\x0b\x32-.sift.saved_searches.v1.SavedSearchFilterItemB\x03\xe0\x41\x01R\x08tagItems\x12]\n\x10\x61nnotation_items\x18\x08 \x03(\x0b\x32-.sift.saved_searches.v1.SavedSearchFilterItemB\x03\xe0\x41\x01R\x0f\x61nnotationItems\x12O\n\trun_items\x18\t \x03(\x0b\x32-.sift.saved_searches.v1.SavedSearchFilterItemB\x03\xe0\x41\x01R\x08runItemsB\x0e\n\x0c_search_termB\x11\n\x0f_from_date_timeB\x0f\n\r_to_date_time\"E\n\x15SavedSearchFilterItem\x12\x13\n\x02id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x02id\x12\x17\n\x04name\x18\x02 \x01(\tB\x03\xe0\x41\x02R\x04name\"D\n\x15GetSavedSearchRequest\x12+\n\x0fsaved_search_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\rsavedSearchId\"e\n\x16GetSavedSearchResponse\x12K\n\x0csaved_search\x18\x01 \x01(\x0b\x32#.sift.saved_searches.v1.SavedSearchB\x03\xe0\x41\x02R\x0bsavedSearch\"\xab\x01\n\x18ListSavedSearchesRequest\x12 \n\tpage_size\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\x12,\n\x0forganization_id\x18\x04 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\"\x8f\x01\n\x19ListSavedSearchesResponse\x12J\n\x0esaved_searches\x18\x01 \x03(\x0b\x32#.sift.saved_searches.v1.SavedSearchR\rsavedSearches\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken\"\xb5\x01\n\x18\x43reateSavedSearchRequest\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12R\n\nproperties\x18\x02 \x01(\x0b\x32-.sift.saved_searches.v1.SavedSearchPropertiesB\x03\xe0\x41\x02R\nproperties\x12,\n\x0forganization_id\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x0eorganizationId\"h\n\x19\x43reateSavedSearchResponse\x12K\n\x0csaved_search\x18\x01 \x01(\x0b\x32#.sift.saved_searches.v1.SavedSearchB\x03\xe0\x41\x02R\x0bsavedSearch\"G\n\x18\x44\x65leteSavedSearchRequest\x12+\n\x0fsaved_search_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\rsavedSearchId\"\x1b\n\x19\x44\x65leteSavedSearchResponse\"K\n\x1f\x42\x61tchDeleteSavedSearchesRequest\x12(\n\x10saved_search_ids\x18\x01 \x03(\tR\x0esavedSearchIds\"\"\n BatchDeleteSavedSearchesResponse\"\xa9\x01\n\x18UpdateSavedSearchRequest\x12K\n\x0csaved_search\x18\x01 \x01(\x0b\x32#.sift.saved_searches.v1.SavedSearchB\x03\xe0\x41\x02R\x0bsavedSearch\x12@\n\x0bupdate_mask\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.FieldMaskB\x03\xe0\x41\x02R\nupdateMask\"h\n\x19UpdateSavedSearchResponse\x12K\n\x0csaved_search\x18\x01 \x01(\x0b\x32#.sift.saved_searches.v1.SavedSearchB\x03\xe0\x41\x02R\x0bsavedSearch2\xdf\n\n\x12SavedSearchService\x12\xce\x01\n\x0eGetSavedSearch\x12-.sift.saved_searches.v1.GetSavedSearchRequest\x1a..sift.saved_searches.v1.GetSavedSearchResponse\"]\x92\x41*\x12\x0eGetSavedSearch\x1a\x18Retrieve a saved search.\x82\xd3\xe4\x93\x02*\x12(/api/v1/saved_searches/{saved_search_id}\x12\xc9\x01\n\x11\x43reateSavedSearch\x12\x30.sift.saved_searches.v1.CreateSavedSearchRequest\x1a\x31.sift.saved_searches.v1.CreateSavedSearchResponse\"O\x92\x41+\x12\x11\x43reateSavedSearch\x1a\x16\x43reate a saved search.\x82\xd3\xe4\x93\x02\x1b\"\x16/api/v1/saved_searches:\x01*\x12\xc4\x01\n\x11ListSavedSearches\x12\x30.sift.saved_searches.v1.ListSavedSearchesRequest\x1a\x31.sift.saved_searches.v1.ListSavedSearchesResponse\"J\x92\x41)\x12\x11ListSavedSearches\x1a\x14List saved searches.\x82\xd3\xe4\x93\x02\x18\x12\x16/api/v1/saved_searches\x12\x8f\x02\n\x11UpdateSavedSearch\x12\x30.sift.saved_searches.v1.UpdateSavedSearchRequest\x1a\x31.sift.saved_searches.v1.UpdateSavedSearchResponse\"\x94\x01\x92\x41p\x12\x11UpdateSavedSearch\x1a[Updates an existing saved search using using the list of fields specified in `update_mask`.\x82\xd3\xe4\x93\x02\x1b\x32\x16/api/v1/saved_searches:\x01*\x12\xd8\x01\n\x11\x44\x65leteSavedSearch\x12\x30.sift.saved_searches.v1.DeleteSavedSearchRequest\x1a\x31.sift.saved_searches.v1.DeleteSavedSearchResponse\"^\x92\x41+\x12\x11\x44\x65leteSavedSearch\x1a\x16\x44\x65lete a saved search.\x82\xd3\xe4\x93\x02**(/api/v1/saved_searches/{saved_search_id}\x12\xf7\x01\n\x18\x42\x61tchDeleteSavedSearches\x12\x37.sift.saved_searches.v1.BatchDeleteSavedSearchesRequest\x1a\x38.sift.saved_searches.v1.BatchDeleteSavedSearchesResponse\"h\x92\x41\x38\x12\x18\x42\x61tchDeleteSavedSearches\x1a\x1c\x42\x61tch delete saved searches.\x82\xd3\xe4\x93\x02\'\"\"/api/v1/saved_searches:batchDelete:\x01*B\xc0\x01\n\x1a\x63om.sift.saved_searches.v1B\x12SavedSearchesProtoP\x01\xa2\x02\x03SSX\xaa\x02\x15Sift.SavedSearches.V1\xca\x02\x15Sift\\SavedSearches\\V1\xe2\x02!Sift\\SavedSearches\\V1\\GPBMetadata\xea\x02\x17Sift::SavedSearches::V1\x92\x41\x17\x12\x15\n\x13SavedSearch serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.saved_searches.v1.saved_searches_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\032com.sift.saved_searches.v1B\022SavedSearchesProtoP\001\242\002\003SSX\252\002\025Sift.SavedSearches.V1\312\002\025Sift\\SavedSearches\\V1\342\002!Sift\\SavedSearches\\V1\\GPBMetadata\352\002\027Sift::SavedSearches::V1\222A\027\022\025\n\023SavedSearch service'
  _globals['_SAVEDSEARCH'].fields_by_name['saved_search_id']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['saved_search_id']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCH'].fields_by_name['organization_id']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['organization_id']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCH'].fields_by_name['name']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCH'].fields_by_name['properties']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['properties']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCH'].fields_by_name['created_by_user_id']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['created_by_user_id']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCH'].fields_by_name['modified_by_user_id']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['modified_by_user_id']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCH'].fields_by_name['created_date']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['created_date']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCH'].fields_by_name['modified_date']._loaded_options = None
  _globals['_SAVEDSEARCH'].fields_by_name['modified_date']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['overview_mode']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['overview_mode']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['search_term']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['search_term']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['from_date_time']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['from_date_time']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['to_date_time']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['to_date_time']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['asset_items']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['asset_items']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['user_items']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['user_items']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['tag_items']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['tag_items']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['annotation_items']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['annotation_items']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['run_items']._loaded_options = None
  _globals['_SAVEDSEARCHPROPERTIES'].fields_by_name['run_items']._serialized_options = b'\340A\001'
  _globals['_SAVEDSEARCHFILTERITEM'].fields_by_name['id']._loaded_options = None
  _globals['_SAVEDSEARCHFILTERITEM'].fields_by_name['id']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCHFILTERITEM'].fields_by_name['name']._loaded_options = None
  _globals['_SAVEDSEARCHFILTERITEM'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_GETSAVEDSEARCHREQUEST'].fields_by_name['saved_search_id']._loaded_options = None
  _globals['_GETSAVEDSEARCHREQUEST'].fields_by_name['saved_search_id']._serialized_options = b'\340A\002'
  _globals['_GETSAVEDSEARCHRESPONSE'].fields_by_name['saved_search']._loaded_options = None
  _globals['_GETSAVEDSEARCHRESPONSE'].fields_by_name['saved_search']._serialized_options = b'\340A\002'
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_LISTSAVEDSEARCHESREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_CREATESAVEDSEARCHREQUEST'].fields_by_name['name']._loaded_options = None
  _globals['_CREATESAVEDSEARCHREQUEST'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_CREATESAVEDSEARCHREQUEST'].fields_by_name['properties']._loaded_options = None
  _globals['_CREATESAVEDSEARCHREQUEST'].fields_by_name['properties']._serialized_options = b'\340A\002'
  _globals['_CREATESAVEDSEARCHREQUEST'].fields_by_name['organization_id']._loaded_options = None
  _globals['_CREATESAVEDSEARCHREQUEST'].fields_by_name['organization_id']._serialized_options = b'\340A\001'
  _globals['_CREATESAVEDSEARCHRESPONSE'].fields_by_name['saved_search']._loaded_options = None
  _globals['_CREATESAVEDSEARCHRESPONSE'].fields_by_name['saved_search']._serialized_options = b'\340A\002'
  _globals['_DELETESAVEDSEARCHREQUEST'].fields_by_name['saved_search_id']._loaded_options = None
  _globals['_DELETESAVEDSEARCHREQUEST'].fields_by_name['saved_search_id']._serialized_options = b'\340A\002'
  _globals['_UPDATESAVEDSEARCHREQUEST'].fields_by_name['saved_search']._loaded_options = None
  _globals['_UPDATESAVEDSEARCHREQUEST'].fields_by_name['saved_search']._serialized_options = b'\340A\002'
  _globals['_UPDATESAVEDSEARCHREQUEST'].fields_by_name['update_mask']._loaded_options = None
  _globals['_UPDATESAVEDSEARCHREQUEST'].fields_by_name['update_mask']._serialized_options = b'\340A\002'
  _globals['_UPDATESAVEDSEARCHRESPONSE'].fields_by_name['saved_search']._loaded_options = None
  _globals['_UPDATESAVEDSEARCHRESPONSE'].fields_by_name['saved_search']._serialized_options = b'\340A\002'
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['GetSavedSearch']._loaded_options = None
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['GetSavedSearch']._serialized_options = b'\222A*\022\016GetSavedSearch\032\030Retrieve a saved search.\202\323\344\223\002*\022(/api/v1/saved_searches/{saved_search_id}'
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['CreateSavedSearch']._loaded_options = None
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['CreateSavedSearch']._serialized_options = b'\222A+\022\021CreateSavedSearch\032\026Create a saved search.\202\323\344\223\002\033\"\026/api/v1/saved_searches:\001*'
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['ListSavedSearches']._loaded_options = None
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['ListSavedSearches']._serialized_options = b'\222A)\022\021ListSavedSearches\032\024List saved searches.\202\323\344\223\002\030\022\026/api/v1/saved_searches'
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['UpdateSavedSearch']._loaded_options = None
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['UpdateSavedSearch']._serialized_options = b'\222Ap\022\021UpdateSavedSearch\032[Updates an existing saved search using using the list of fields specified in `update_mask`.\202\323\344\223\002\0332\026/api/v1/saved_searches:\001*'
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['DeleteSavedSearch']._loaded_options = None
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['DeleteSavedSearch']._serialized_options = b'\222A+\022\021DeleteSavedSearch\032\026Delete a saved search.\202\323\344\223\002**(/api/v1/saved_searches/{saved_search_id}'
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['BatchDeleteSavedSearches']._loaded_options = None
  _globals['_SAVEDSEARCHSERVICE'].methods_by_name['BatchDeleteSavedSearches']._serialized_options = b'\222A8\022\030BatchDeleteSavedSearches\032\034Batch delete saved searches.\202\323\344\223\002\'\"\"/api/v1/saved_searches:batchDelete:\001*'
  _globals['_SAVEDSEARCH']._serialized_start=250
  _globals['_SAVEDSEARCH']._serialized_end=703
  _globals['_SAVEDSEARCHPROPERTIES']._serialized_start=706
  _globals['_SAVEDSEARCHPROPERTIES']._serialized_end=1439
  _globals['_SAVEDSEARCHFILTERITEM']._serialized_start=1441
  _globals['_SAVEDSEARCHFILTERITEM']._serialized_end=1510
  _globals['_GETSAVEDSEARCHREQUEST']._serialized_start=1512
  _globals['_GETSAVEDSEARCHREQUEST']._serialized_end=1580
  _globals['_GETSAVEDSEARCHRESPONSE']._serialized_start=1582
  _globals['_GETSAVEDSEARCHRESPONSE']._serialized_end=1683
  _globals['_LISTSAVEDSEARCHESREQUEST']._serialized_start=1686
  _globals['_LISTSAVEDSEARCHESREQUEST']._serialized_end=1857
  _globals['_LISTSAVEDSEARCHESRESPONSE']._serialized_start=1860
  _globals['_LISTSAVEDSEARCHESRESPONSE']._serialized_end=2003
  _globals['_CREATESAVEDSEARCHREQUEST']._serialized_start=2006
  _globals['_CREATESAVEDSEARCHREQUEST']._serialized_end=2187
  _globals['_CREATESAVEDSEARCHRESPONSE']._serialized_start=2189
  _globals['_CREATESAVEDSEARCHRESPONSE']._serialized_end=2293
  _globals['_DELETESAVEDSEARCHREQUEST']._serialized_start=2295
  _globals['_DELETESAVEDSEARCHREQUEST']._serialized_end=2366
  _globals['_DELETESAVEDSEARCHRESPONSE']._serialized_start=2368
  _globals['_DELETESAVEDSEARCHRESPONSE']._serialized_end=2395
  _globals['_BATCHDELETESAVEDSEARCHESREQUEST']._serialized_start=2397
  _globals['_BATCHDELETESAVEDSEARCHESREQUEST']._serialized_end=2472
  _globals['_BATCHDELETESAVEDSEARCHESRESPONSE']._serialized_start=2474
  _globals['_BATCHDELETESAVEDSEARCHESRESPONSE']._serialized_end=2508
  _globals['_UPDATESAVEDSEARCHREQUEST']._serialized_start=2511
  _globals['_UPDATESAVEDSEARCHREQUEST']._serialized_end=2680
  _globals['_UPDATESAVEDSEARCHRESPONSE']._serialized_start=2682
  _globals['_UPDATESAVEDSEARCHRESPONSE']._serialized_end=2786
  _globals['_SAVEDSEARCHSERVICE']._serialized_start=2789
  _globals['_SAVEDSEARCHSERVICE']._serialized_end=4164
# @@protoc_insertion_point(module_scope)
