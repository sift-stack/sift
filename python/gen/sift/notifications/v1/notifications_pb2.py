# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: sift/notifications/v1/notifications.proto
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


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n)sift/notifications/v1/notifications.proto\x12\x15sift.notifications.v1\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/api/field_behavior.proto\x1a google/protobuf/field_mask.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a.protoc-gen-openapiv2/options/annotations.proto\"\xad\x04\n\x0cNotification\x12\'\n\x0fnotification_id\x18\x01 \x01(\tR\x0enotificationId\x12=\n\x0c\x63reated_date\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\x0b\x63reatedDate\x12?\n\rmodified_date\x18\x03 \x01(\x0b\x32\x1a.google.protobuf.TimestampR\x0cmodifiedDate\x12+\n\x12\x63reated_by_user_id\x18\x04 \x01(\tR\x0f\x63reatedByUserId\x12-\n\x13modified_by_user_id\x18\x05 \x01(\tR\x10modifiedByUserId\x12\'\n\x0forganization_id\x18\x06 \x01(\tR\x0eorganizationId\x12*\n\x11recipient_user_id\x18\x07 \x01(\tR\x0frecipientUserId\x12\x17\n\x07is_read\x18\x08 \x01(\x08R\x06isRead\x12\x1b\n\tfull_link\x18\t \x01(\tR\x08\x66ullLink\x12T\n\x11notification_type\x18\n \x01(\x0e\x32\'.sift.notifications.v1.NotificationKindR\x10notificationType\x12\x1a\n\x08\x63ontents\x18\x0b \x01(\tR\x08\x63ontents\x12\x1b\n\tentity_id\x18\x0c \x01(\tR\x08\x65ntityId\"}\n\x18ListNotificationsRequest\x12 \n\tpage_size\x18\x01 \x01(\rB\x03\xe0\x41\x01R\x08pageSize\x12\"\n\npage_token\x18\x02 \x01(\tB\x03\xe0\x41\x01R\tpageToken\x12\x1b\n\x06\x66ilter\x18\x03 \x01(\tB\x03\xe0\x41\x01R\x06\x66ilter\"\x8e\x01\n\x19ListNotificationsResponse\x12I\n\rnotifications\x18\x01 \x03(\x0b\x32#.sift.notifications.v1.NotificationR\rnotifications\x12&\n\x0fnext_page_token\x18\x02 \x01(\tR\rnextPageToken\"t\n\x1f\x42\x61tchUpdateNotificationsRequest\x12Q\n\x08requests\x18\x01 \x03(\x0b\x32\x30.sift.notifications.v1.UpdateNotificationRequestB\x03\xe0\x41\x02R\x08requests\"\xab\x01\n\x19UpdateNotificationRequest\x12L\n\x0cnotification\x18\x01 \x01(\x0b\x32#.sift.notifications.v1.NotificationB\x03\xe0\x41\x02R\x0cnotification\x12@\n\x0bupdate_mask\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.FieldMaskB\x03\xe0\x41\x02R\nupdateMask\"m\n BatchUpdateNotificationsResponse\x12I\n\rnotifications\x18\x01 \x03(\x0b\x32#.sift.notifications.v1.NotificationR\rnotifications*\x8e\x02\n\x10NotificationKind\x12!\n\x1dNOTIFICATION_KIND_UNSPECIFIED\x10\x00\x12\x1a\n\x16NOTIFICATION_KIND_TEXT\x10\x01\x12)\n%NOTIFICATION_KIND_ANNOTATION_ASSIGNED\x10\x02\x12\x35\n1NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT\x10\x03\x12)\n%NOTIFICATION_KIND_CONDITION_TRIGGERED\x10\x04\x12.\n*NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED\x10\x05\x32\xe9\x02\n\x13NotificationService\x12\x95\x01\n\x11ListNotifications\x12/.sift.notifications.v1.ListNotificationsRequest\x1a\x30.sift.notifications.v1.ListNotificationsResponse\"\x1d\x82\xd3\xe4\x93\x02\x17\x12\x15/api/v1/notifications\x12\xb9\x01\n\x18\x42\x61tchUpdateNotifications\x12\x36.sift.notifications.v1.BatchUpdateNotificationsRequest\x1a\x37.sift.notifications.v1.BatchUpdateNotificationsResponse\",\x82\xd3\xe4\x93\x02&\"!/api/v1/notifications:batchUpdate:\x01*B\xff\x01\n\x19\x63om.sift.notifications.v1B\x12NotificationsProtoP\x01Z=azimuth/gen/protos/go/sift/notifications/v1;notificationsv1pb\xa2\x02\x03SNX\xaa\x02\x15Sift.Notifications.V1\xca\x02\x15Sift\\Notifications\\V1\xe2\x02!Sift\\Notifications\\V1\\GPBMetadata\xea\x02\x17Sift::Notifications::V1\x92\x41\x18\x12\x16\n\x14Notification Serviceb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'sift.notifications.v1.notifications_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\031com.sift.notifications.v1B\022NotificationsProtoP\001Z=azimuth/gen/protos/go/sift/notifications/v1;notificationsv1pb\242\002\003SNX\252\002\025Sift.Notifications.V1\312\002\025Sift\\Notifications\\V1\342\002!Sift\\Notifications\\V1\\GPBMetadata\352\002\027Sift::Notifications::V1\222A\030\022\026\n\024Notification Service'
  _globals['_LISTNOTIFICATIONSREQUEST'].fields_by_name['page_size']._loaded_options = None
  _globals['_LISTNOTIFICATIONSREQUEST'].fields_by_name['page_size']._serialized_options = b'\340A\001'
  _globals['_LISTNOTIFICATIONSREQUEST'].fields_by_name['page_token']._loaded_options = None
  _globals['_LISTNOTIFICATIONSREQUEST'].fields_by_name['page_token']._serialized_options = b'\340A\001'
  _globals['_LISTNOTIFICATIONSREQUEST'].fields_by_name['filter']._loaded_options = None
  _globals['_LISTNOTIFICATIONSREQUEST'].fields_by_name['filter']._serialized_options = b'\340A\001'
  _globals['_BATCHUPDATENOTIFICATIONSREQUEST'].fields_by_name['requests']._loaded_options = None
  _globals['_BATCHUPDATENOTIFICATIONSREQUEST'].fields_by_name['requests']._serialized_options = b'\340A\002'
  _globals['_UPDATENOTIFICATIONREQUEST'].fields_by_name['notification']._loaded_options = None
  _globals['_UPDATENOTIFICATIONREQUEST'].fields_by_name['notification']._serialized_options = b'\340A\002'
  _globals['_UPDATENOTIFICATIONREQUEST'].fields_by_name['update_mask']._loaded_options = None
  _globals['_UPDATENOTIFICATIONREQUEST'].fields_by_name['update_mask']._serialized_options = b'\340A\002'
  _globals['_NOTIFICATIONSERVICE'].methods_by_name['ListNotifications']._loaded_options = None
  _globals['_NOTIFICATIONSERVICE'].methods_by_name['ListNotifications']._serialized_options = b'\202\323\344\223\002\027\022\025/api/v1/notifications'
  _globals['_NOTIFICATIONSERVICE'].methods_by_name['BatchUpdateNotifications']._loaded_options = None
  _globals['_NOTIFICATIONSERVICE'].methods_by_name['BatchUpdateNotifications']._serialized_options = b'\202\323\344\223\002&\"!/api/v1/notifications:batchUpdate:\001*'
  _globals['_NOTIFICATIONKIND']._serialized_start=1482
  _globals['_NOTIFICATIONKIND']._serialized_end=1752
  _globals['_NOTIFICATION']._serialized_start=247
  _globals['_NOTIFICATION']._serialized_end=804
  _globals['_LISTNOTIFICATIONSREQUEST']._serialized_start=806
  _globals['_LISTNOTIFICATIONSREQUEST']._serialized_end=931
  _globals['_LISTNOTIFICATIONSRESPONSE']._serialized_start=934
  _globals['_LISTNOTIFICATIONSRESPONSE']._serialized_end=1076
  _globals['_BATCHUPDATENOTIFICATIONSREQUEST']._serialized_start=1078
  _globals['_BATCHUPDATENOTIFICATIONSREQUEST']._serialized_end=1194
  _globals['_UPDATENOTIFICATIONREQUEST']._serialized_start=1197
  _globals['_UPDATENOTIFICATIONREQUEST']._serialized_end=1368
  _globals['_BATCHUPDATENOTIFICATIONSRESPONSE']._serialized_start=1370
  _globals['_BATCHUPDATENOTIFICATIONSRESPONSE']._serialized_end=1479
  _globals['_NOTIFICATIONSERVICE']._serialized_start=1755
  _globals['_NOTIFICATIONSERVICE']._serialized_end=2116
# @@protoc_insertion_point(module_scope)
