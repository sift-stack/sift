# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/views/v1/views.proto
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


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x19sift/views/v1/views.proto\x12\rsift.views.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a google/protobuf/field_mask.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\"\xa9\n\n\x04View\x12\x17\n\x07view_id\x18\x01 \x01(\tR\x06viewId\x12\x12\n\x04name\x18\x02 \x01(\tR\x04name\x12?\n\x0b\x61xis_groups\x18\x03 \x01(\x0b\x32\x1e.sift.views.v1.View.AxisGroupsR\naxisGroups\x12\x37\n\x08\x63hannels\x18\x04 \x03(\x0b\x32\x1b.sift.views.v1.View.ChannelR\x08\x63hannels\x12=\n\x0c\x63reated_date\x18\x05 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\x0b\x63reatedDate\x12?\n\rmodified_date\x18\x06 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\x0cmodifiedDate\x12+\n\x12\x63reated_by_user_id\x18\x07 \x01(\tR\x0f\x63reatedByUserId\x12\'\n\x0forganization_id\x18\x08 \x01(\tR\x0eorganizationId\x12-\n\x13modified_by_user_id\x18\t \x01(\tR\x10modifiedByUserId\x12\x1b\n\tis_pinned\x18\n \x01(\x08R\x08isPinned\x1a\x36\n\nAxisGroups\x12\x12\n\x04left\x18\x01 \x03(\tR\x04left\x12\x14\n\x05right\x18\x02 \x03(\tR\x05right\x1a\x9f\x06\n\x07\x43hannel\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12!\n\tcomponent\x18\x02 \x01(\tB\x03\xe0\x41\x02R\tcomponent\x12 \n\tdata_type\x18\x03 \x01(\tB\x03\xe0\x41\x02R\x08\x64\x61taType\x12\x1d\n\naxis_group\x18\x04 \x01(\tR\taxisGroup\x12&\n\x0f\x62it_field_names\x18\x05 \x03(\tR\rbitFieldNames\x12t\n\x19\x63\x61lculated_channel_config\x18\x06 \x01(\x0b\x32\x33.sift.views.v1.View.Channel.CalculatedChannelConfigH\x00R\x17\x63\x61lculatedChannelConfig\x88\x01\x01\x1a\xda\x03\n\x17\x43\x61lculatedChannelConfig\x12$\n\x0b\x63hannel_key\x18\x01 \x01(\tB\x03\xe0\x41\x02R\nchannelKey\x12~\n\x12\x63hannel_references\x18\x02 \x03(\x0b\x32J.sift.views.v1.View.Channel.CalculatedChannelConfig.ChannelReferencesEntryB\x03\xe0\x41\x02R\x11\x63hannelReferences\x12#\n\nexpression\x18\x03 \x01(\tB\x03\xe0\x41\x02R\nexpression\x12\x17\n\x04unit\x18\x04 \x01(\tB\x03\xe0\x41\x02R\x04unit\x1aN\n\x10\x43hannelReference\x12\x17\n\x04name\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x04name\x12!\n\tcomponent\x18\x02 \x01(\tB\x03\xe0\x41\x02R\tcomponent\x1a\x8a\x01\n\x16\x43hannelReferencesEntry\x12\x10\n\x03key\x18\x01 \x01(\tR\x03key\x12Z\n\x05value\x18\x02 \x01(\x0b\x32\x44.sift.views.v1.View.Channel.CalculatedChannelConfig.ChannelReferenceR\x05value:\x02\x38\x01\x42\x1c\n\x1a_calculated_channel_config\".\n\x0eGetViewRequest\x12\x1c\n\x07view_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x06viewId\":\n\x0fGetViewResponse\x12\'\n\x04view\x18\x01 \x01(\x0b\x32\x13.sift.views.v1.ViewR\x04view\"A\n\x11\x43reateViewRequest\x12,\n\x04view\x18\x01 \x01(\x0b\x32\x13.sift.views.v1.ViewB\x03\xe0\x41\x02R\x04view\"=\n\x12\x43reateViewResponse\x12\'\n\x04view\x18\x01 \x01(\x0b\x32\x13.sift.views.v1.ViewR\x04view\"~\n\x11UpdateViewRequest\x12,\n\x04view\x18\x01 \x01(\x0b\x32\x13.sift.views.v1.ViewB\x03\xe0\x41\x02R\x04view\x12;\n\x0bupdate_mask\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.FieldMaskR\nupdateMask\"=\n\x12UpdateViewResponse\x12\'\n\x04view\x18\x01 \x01(\x0b\x32\x13.sift.views.v1.ViewR\x04view\"f\n\x10ListViewsRequest\x12\x1b\n\tpage_size\x18\x01 \x01(\rR\x08pageSize\x12\x1d\n\npage_token\x18\x02 \x01(\tR\tpageToken\x12\x16\n\x06\x66ilter\x18\x03 \x01(\tR\x06\x66ilter\"f\n\x11ListViewsResponse\x12)\n\x05views\x18\x01 \x03(\x0b\x32\x13.sift.views.v1.ViewR\x05views\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken\"\x98\x01\n\x1aListApplicableViewsRequest\x12\x1b\n\tpage_size\x18\x01 \x01(\rR\x08pageSize\x12\x1d\n\npage_token\x18\x02 \x01(\tR\tpageToken\x12 \n\tasset_ids\x18\x03 \x03(\tB\x03\xe0\x41\x01R\x08\x61ssetIds\x12\x1c\n\x07run_ids\x18\x04 \x03(\tB\x03\xe0\x41\x01R\x06runIds\"p\n\x1bListApplicableViewsResponse\x12)\n\x05views\x18\x01 \x03(\x0b\x32\x13.sift.views.v1.ViewR\x05views\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken\"1\n\x11\x44\x65leteViewRequest\x12\x1c\n\x07view_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x06viewId\"\x14\n\x12\x44\x65leteViewResponse\".\n\x0ePinViewRequest\x12\x1c\n\x07view_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x06viewId\"\x11\n\x0fPinViewResponse\"0\n\x10UnpinViewRequest\x12\x1c\n\x07view_id\x18\x01 \x01(\tB\x03\xe0\x41\x02R\x06viewId\"\x13\n\x11UnpinViewResponse2\xa6\r\n\x0bViewService\x12\x88\x01\n\x07GetView\x12\x1d.sift.views.v1.GetViewRequest\x1a\x1e.sift.views.v1.GetViewResponse\">\x92\x41\x1c\x12\x07GetView\x1a\x11Retrieves a view.\x82\xd3\xe4\x93\x02\x19\x12\x17/api/v1/views/{view_id}\x12\x8e\x01\n\nCreateView\x12 .sift.views.v1.CreateViewRequest\x1a!.sift.views.v1.CreateViewResponse\";\x92\x41\x1d\x12\nCreateView\x1a\x0f\x43reates a view.\x82\xd3\xe4\x93\x02\x15\"\r/api/v1/views:\x04view\x12\xdf\x01\n\nUpdateView\x12 .sift.views.v1.UpdateViewRequest\x1a!.sift.views.v1.UpdateViewResponse\"\x8b\x01\x92\x41\x61\x12\nUpdateView\x1aSUpdates an existing view using using the list of fields specified in `update_mask`.\x82\xd3\xe4\x93\x02!2\x1c/api/v1/views/{view.view_id}:\x01*\x12\x9e\x01\n\tListViews\x12\x1f.sift.views.v1.ListViewsRequest\x1a .sift.views.v1.ListViewsResponse\"N\x92\x41\x36\x12\tListViews\x1a)Retrieves views using an optional filter.\x82\xd3\xe4\x93\x02\x0f\x12\r/api/v1/views\x12\x8e\x01\n\x13ListApplicableViews\x12).sift.views.v1.ListApplicableViewsRequest\x1a*.sift.views.v1.ListApplicableViewsResponse\" \x82\xd3\xe4\x93\x02\x1a\x12\x18/api/v1/views:applicable\x12\x92\x01\n\nDeleteView\x12 .sift.views.v1.DeleteViewRequest\x1a!.sift.views.v1.DeleteViewResponse\"?\x92\x41\x1d\x12\nDeleteView\x1a\x0f\x44\x65letes a view.\x82\xd3\xe4\x93\x02\x19*\x17/api/v1/views/{view_id}\x12\x87\x02\n\x07PinView\x12\x1d.sift.views.v1.PinViewRequest\x1a\x1e.sift.views.v1.PinViewResponse\"\xbc\x01\x92\x41\x92\x01\x12\x07PinView\x1a\x0cPins a view.\"y\n\x1fLearn more about pinning views.\x12Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/295436289\x82\xd3\xe4\x93\x02 2\x1b/api/v1/views/{view_id}/pin:\x01*\x12\x93\x02\n\tUnpinView\x12\x1f.sift.views.v1.UnpinViewRequest\x1a .sift.views.v1.UnpinViewResponse\"\xc2\x01\x92\x41\x96\x01\x12\tUnpinView\x1a\x0eUnpins a view.\"y\n\x1fLearn more about pinning views.\x12Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/295436289\x82\xd3\xe4\x93\x02\"2\x1d/api/v1/views/{view_id}/unpin:\x01*\x1a\xb1\x01\x92\x41\xad\x01\x12\x30Service to programmatically interact with views.\x1ay\n\x1fRead more about what views are.\x12Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/298188809B\x88\x01\n\x11\x63om.sift.views.v1B\nViewsProtoP\x01\xa2\x02\x03SVX\xaa\x02\rSift.Views.V1\xca\x02\rSift\\Views\\V1\xe2\x02\x19Sift\\Views\\V1\\GPBMetadata\xea\x02\x0fSift::Views::V1\x92\x41\x10\x12\x0e\n\x0cView Serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.views.v1.views_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\021com.sift.views.v1B\nViewsProtoP\001\242\002\003SVX\252\002\rSift.Views.V1\312\002\rSift\\Views\\V1\342\002\031Sift\\Views\\V1\\GPBMetadata\352\002\017Sift::Views::V1\222A\020\022\016\n\014View Service'
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCE'].fields_by_name['name']._loaded_options = None
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCE'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCE'].fields_by_name['component']._loaded_options = None
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCE'].fields_by_name['component']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCESENTRY']._loaded_options = None
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCESENTRY']._serialized_options = b'8\001'
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['channel_key']._loaded_options = None
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['channel_key']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['channel_references']._loaded_options = None
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['channel_references']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['expression']._loaded_options = None
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['expression']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['unit']._loaded_options = None
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG'].fields_by_name['unit']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL'].fields_by_name['name']._loaded_options = None
  _globals['_VIEW_CHANNEL'].fields_by_name['name']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL'].fields_by_name['component']._loaded_options = None
  _globals['_VIEW_CHANNEL'].fields_by_name['component']._serialized_options = b'\340A\002'
  _globals['_VIEW_CHANNEL'].fields_by_name['data_type']._loaded_options = None
  _globals['_VIEW_CHANNEL'].fields_by_name['data_type']._serialized_options = b'\340A\002'
  _globals['_GETVIEWREQUEST'].fields_by_name['view_id']._loaded_options = None
  _globals['_GETVIEWREQUEST'].fields_by_name['view_id']._serialized_options = b'\340A\002'
  _globals['_CREATEVIEWREQUEST'].fields_by_name['view']._loaded_options = None
  _globals['_CREATEVIEWREQUEST'].fields_by_name['view']._serialized_options = b'\340A\002'
  _globals['_UPDATEVIEWREQUEST'].fields_by_name['view']._loaded_options = None
  _globals['_UPDATEVIEWREQUEST'].fields_by_name['view']._serialized_options = b'\340A\002'
  _globals['_LISTAPPLICABLEVIEWSREQUEST'].fields_by_name['asset_ids']._loaded_options = None
  _globals['_LISTAPPLICABLEVIEWSREQUEST'].fields_by_name['asset_ids']._serialized_options = b'\340A\001'
  _globals['_LISTAPPLICABLEVIEWSREQUEST'].fields_by_name['run_ids']._loaded_options = None
  _globals['_LISTAPPLICABLEVIEWSREQUEST'].fields_by_name['run_ids']._serialized_options = b'\340A\001'
  _globals['_DELETEVIEWREQUEST'].fields_by_name['view_id']._loaded_options = None
  _globals['_DELETEVIEWREQUEST'].fields_by_name['view_id']._serialized_options = b'\340A\002'
  _globals['_PINVIEWREQUEST'].fields_by_name['view_id']._loaded_options = None
  _globals['_PINVIEWREQUEST'].fields_by_name['view_id']._serialized_options = b'\340A\002'
  _globals['_UNPINVIEWREQUEST'].fields_by_name['view_id']._loaded_options = None
  _globals['_UNPINVIEWREQUEST'].fields_by_name['view_id']._serialized_options = b'\340A\002'
  _globals['_VIEWSERVICE']._loaded_options = None
  _globals['_VIEWSERVICE']._serialized_options = b'\222A\255\001\0220Service to programmatically interact with views.\032y\n\037Read more about what views are.\022Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/298188809'
  _globals['_VIEWSERVICE'].methods_by_name['GetView']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['GetView']._serialized_options = b'\222A\034\022\007GetView\032\021Retrieves a view.\202\323\344\223\002\031\022\027/api/v1/views/{view_id}'
  _globals['_VIEWSERVICE'].methods_by_name['CreateView']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['CreateView']._serialized_options = b'\222A\035\022\nCreateView\032\017Creates a view.\202\323\344\223\002\025\"\r/api/v1/views:\004view'
  _globals['_VIEWSERVICE'].methods_by_name['UpdateView']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['UpdateView']._serialized_options = b'\222Aa\022\nUpdateView\032SUpdates an existing view using using the list of fields specified in `update_mask`.\202\323\344\223\002!2\034/api/v1/views/{view.view_id}:\001*'
  _globals['_VIEWSERVICE'].methods_by_name['ListViews']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['ListViews']._serialized_options = b'\222A6\022\tListViews\032)Retrieves views using an optional filter.\202\323\344\223\002\017\022\r/api/v1/views'
  _globals['_VIEWSERVICE'].methods_by_name['ListApplicableViews']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['ListApplicableViews']._serialized_options = b'\202\323\344\223\002\032\022\030/api/v1/views:applicable'
  _globals['_VIEWSERVICE'].methods_by_name['DeleteView']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['DeleteView']._serialized_options = b'\222A\035\022\nDeleteView\032\017Deletes a view.\202\323\344\223\002\031*\027/api/v1/views/{view_id}'
  _globals['_VIEWSERVICE'].methods_by_name['PinView']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['PinView']._serialized_options = b'\222A\222\001\022\007PinView\032\014Pins a view.\"y\n\037Learn more about pinning views.\022Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/295436289\202\323\344\223\002 2\033/api/v1/views/{view_id}/pin:\001*'
  _globals['_VIEWSERVICE'].methods_by_name['UnpinView']._loaded_options = None
  _globals['_VIEWSERVICE'].methods_by_name['UnpinView']._serialized_options = b'\222A\226\001\022\tUnpinView\032\016Unpins a view.\"y\n\037Learn more about pinning views.\022Vhttps://customer.support.siftstack.com/servicedesk/customer/portal/2/article/295436289\202\323\344\223\002\"2\035/api/v1/views/{view_id}/unpin:\001*'
  _globals['_VIEW']._serialized_start=223
  _globals['_VIEW']._serialized_end=1544
  _globals['_VIEW_AXISGROUPS']._serialized_start=688
  _globals['_VIEW_AXISGROUPS']._serialized_end=742
  _globals['_VIEW_CHANNEL']._serialized_start=745
  _globals['_VIEW_CHANNEL']._serialized_end=1544
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG']._serialized_start=1040
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG']._serialized_end=1514
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCE']._serialized_start=1295
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCE']._serialized_end=1373
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCESENTRY']._serialized_start=1376
  _globals['_VIEW_CHANNEL_CALCULATEDCHANNELCONFIG_CHANNELREFERENCESENTRY']._serialized_end=1514
  _globals['_GETVIEWREQUEST']._serialized_start=1546
  _globals['_GETVIEWREQUEST']._serialized_end=1592
  _globals['_GETVIEWRESPONSE']._serialized_start=1594
  _globals['_GETVIEWRESPONSE']._serialized_end=1652
  _globals['_CREATEVIEWREQUEST']._serialized_start=1654
  _globals['_CREATEVIEWREQUEST']._serialized_end=1719
  _globals['_CREATEVIEWRESPONSE']._serialized_start=1721
  _globals['_CREATEVIEWRESPONSE']._serialized_end=1782
  _globals['_UPDATEVIEWREQUEST']._serialized_start=1784
  _globals['_UPDATEVIEWREQUEST']._serialized_end=1910
  _globals['_UPDATEVIEWRESPONSE']._serialized_start=1912
  _globals['_UPDATEVIEWRESPONSE']._serialized_end=1973
  _globals['_LISTVIEWSREQUEST']._serialized_start=1975
  _globals['_LISTVIEWSREQUEST']._serialized_end=2077
  _globals['_LISTVIEWSRESPONSE']._serialized_start=2079
  _globals['_LISTVIEWSRESPONSE']._serialized_end=2181
  _globals['_LISTAPPLICABLEVIEWSREQUEST']._serialized_start=2184
  _globals['_LISTAPPLICABLEVIEWSREQUEST']._serialized_end=2336
  _globals['_LISTAPPLICABLEVIEWSRESPONSE']._serialized_start=2338
  _globals['_LISTAPPLICABLEVIEWSRESPONSE']._serialized_end=2450
  _globals['_DELETEVIEWREQUEST']._serialized_start=2452
  _globals['_DELETEVIEWREQUEST']._serialized_end=2501
  _globals['_DELETEVIEWRESPONSE']._serialized_start=2503
  _globals['_DELETEVIEWRESPONSE']._serialized_end=2523
  _globals['_PINVIEWREQUEST']._serialized_start=2525
  _globals['_PINVIEWREQUEST']._serialized_end=2571
  _globals['_PINVIEWRESPONSE']._serialized_start=2573
  _globals['_PINVIEWRESPONSE']._serialized_end=2590
  _globals['_UNPINVIEWREQUEST']._serialized_start=2592
  _globals['_UNPINVIEWREQUEST']._serialized_end=2640
  _globals['_UNPINVIEWRESPONSE']._serialized_start=2642
  _globals['_UNPINVIEWRESPONSE']._serialized_end=2661
  _globals['_VIEWSERVICE']._serialized_start=2664
  _globals['_VIEWSERVICE']._serialized_end=4366
# @@protoc_insertion_point(module_scope)
