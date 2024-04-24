// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        (unknown)
// source: sift/notifications/v1/notifications.proto

package notificationsv1

import (
	_ "github.com/sift-stack/sift/go/gen/protos/go/google/api"
	_ "github.com/sift-stack/sift/go/gen/protos/go/protoc-gen-openapiv2/options"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	fieldmaskpb "google.golang.org/protobuf/types/known/fieldmaskpb"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type NotificationKind int32

const (
	NotificationKind_NOTIFICATION_KIND_UNSPECIFIED                     NotificationKind = 0
	NotificationKind_NOTIFICATION_KIND_TEXT                            NotificationKind = 1
	NotificationKind_NOTIFICATION_KIND_ANNOTATION_ASSIGNED             NotificationKind = 2
	NotificationKind_NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT NotificationKind = 3
	NotificationKind_NOTIFICATION_KIND_CONDITION_TRIGGERED             NotificationKind = 4
	NotificationKind_NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED        NotificationKind = 5
)

// Enum value maps for NotificationKind.
var (
	NotificationKind_name = map[int32]string{
		0: "NOTIFICATION_KIND_UNSPECIFIED",
		1: "NOTIFICATION_KIND_TEXT",
		2: "NOTIFICATION_KIND_ANNOTATION_ASSIGNED",
		3: "NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT",
		4: "NOTIFICATION_KIND_CONDITION_TRIGGERED",
		5: "NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED",
	}
	NotificationKind_value = map[string]int32{
		"NOTIFICATION_KIND_UNSPECIFIED":                     0,
		"NOTIFICATION_KIND_TEXT":                            1,
		"NOTIFICATION_KIND_ANNOTATION_ASSIGNED":             2,
		"NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT": 3,
		"NOTIFICATION_KIND_CONDITION_TRIGGERED":             4,
		"NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED":        5,
	}
)

func (x NotificationKind) Enum() *NotificationKind {
	p := new(NotificationKind)
	*p = x
	return p
}

func (x NotificationKind) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (NotificationKind) Descriptor() protoreflect.EnumDescriptor {
	return file_sift_notifications_v1_notifications_proto_enumTypes[0].Descriptor()
}

func (NotificationKind) Type() protoreflect.EnumType {
	return &file_sift_notifications_v1_notifications_proto_enumTypes[0]
}

func (x NotificationKind) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use NotificationKind.Descriptor instead.
func (NotificationKind) EnumDescriptor() ([]byte, []int) {
	return file_sift_notifications_v1_notifications_proto_rawDescGZIP(), []int{0}
}

type Notification struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	NotificationId   string                 `protobuf:"bytes,1,opt,name=notification_id,json=notificationId,proto3" json:"notification_id,omitempty"`
	CreatedDate      *timestamppb.Timestamp `protobuf:"bytes,2,opt,name=created_date,json=createdDate,proto3" json:"created_date,omitempty"`
	ModifiedDate     *timestamppb.Timestamp `protobuf:"bytes,3,opt,name=modified_date,json=modifiedDate,proto3" json:"modified_date,omitempty"`
	CreatedByUserId  string                 `protobuf:"bytes,4,opt,name=created_by_user_id,json=createdByUserId,proto3" json:"created_by_user_id,omitempty"`
	ModifiedByUserId string                 `protobuf:"bytes,5,opt,name=modified_by_user_id,json=modifiedByUserId,proto3" json:"modified_by_user_id,omitempty"`
	OrganizationId   string                 `protobuf:"bytes,6,opt,name=organization_id,json=organizationId,proto3" json:"organization_id,omitempty"`
	RecipientUserId  string                 `protobuf:"bytes,7,opt,name=recipient_user_id,json=recipientUserId,proto3" json:"recipient_user_id,omitempty"`
	IsRead           bool                   `protobuf:"varint,8,opt,name=is_read,json=isRead,proto3" json:"is_read,omitempty"`
	FullLink         string                 `protobuf:"bytes,9,opt,name=full_link,json=fullLink,proto3" json:"full_link,omitempty"`
	NotificationType NotificationKind       `protobuf:"varint,10,opt,name=notification_type,json=notificationType,proto3,enum=sift.notifications.v1.NotificationKind" json:"notification_type,omitempty"`
	Contents         string                 `protobuf:"bytes,11,opt,name=contents,proto3" json:"contents,omitempty"`
	EntityId         string                 `protobuf:"bytes,12,opt,name=entity_id,json=entityId,proto3" json:"entity_id,omitempty"`
}

func (x *Notification) Reset() {
	*x = Notification{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_notifications_v1_notifications_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Notification) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Notification) ProtoMessage() {}

func (x *Notification) ProtoReflect() protoreflect.Message {
	mi := &file_sift_notifications_v1_notifications_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Notification.ProtoReflect.Descriptor instead.
func (*Notification) Descriptor() ([]byte, []int) {
	return file_sift_notifications_v1_notifications_proto_rawDescGZIP(), []int{0}
}

func (x *Notification) GetNotificationId() string {
	if x != nil {
		return x.NotificationId
	}
	return ""
}

func (x *Notification) GetCreatedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.CreatedDate
	}
	return nil
}

func (x *Notification) GetModifiedDate() *timestamppb.Timestamp {
	if x != nil {
		return x.ModifiedDate
	}
	return nil
}

func (x *Notification) GetCreatedByUserId() string {
	if x != nil {
		return x.CreatedByUserId
	}
	return ""
}

func (x *Notification) GetModifiedByUserId() string {
	if x != nil {
		return x.ModifiedByUserId
	}
	return ""
}

func (x *Notification) GetOrganizationId() string {
	if x != nil {
		return x.OrganizationId
	}
	return ""
}

func (x *Notification) GetRecipientUserId() string {
	if x != nil {
		return x.RecipientUserId
	}
	return ""
}

func (x *Notification) GetIsRead() bool {
	if x != nil {
		return x.IsRead
	}
	return false
}

func (x *Notification) GetFullLink() string {
	if x != nil {
		return x.FullLink
	}
	return ""
}

func (x *Notification) GetNotificationType() NotificationKind {
	if x != nil {
		return x.NotificationType
	}
	return NotificationKind_NOTIFICATION_KIND_UNSPECIFIED
}

func (x *Notification) GetContents() string {
	if x != nil {
		return x.Contents
	}
	return ""
}

func (x *Notification) GetEntityId() string {
	if x != nil {
		return x.EntityId
	}
	return ""
}

// The request for a call to `NotificationService_ListNotifications` to retrieve notifications.
type ListNotificationsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The maximum number of notifications to return.
	// The service may return fewer than this value.
	// If unspecified, at most 50 notifications will be returned.
	// The maximum value is 1000; values above 1000 will be coerced to 1000.
	PageSize uint32 `protobuf:"varint,1,opt,name=page_size,json=pageSize,proto3" json:"page_size,omitempty"`
	// A page token, received from a previous `ListNotifications` call.
	// Provide this to retrieve the subsequent page.
	// When paginating, all other parameters provided to `ListNotifications` must match
	// the call that provided the page token.
	PageToken string `protobuf:"bytes,2,opt,name=page_token,json=pageToken,proto3" json:"page_token,omitempty"`
	// A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
	// Available fields to filter by are `notification_id`, `created_by_user_id`, `recipient_user_id`,
	// `created_date`, `notification_type`, and `is_read`.
	// For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
	// For more information about the fields used for filtering, please refer to [this definition](/ingestion/api#sift_notifications_v1_notifications-proto). Optional.
	Filter string `protobuf:"bytes,3,opt,name=filter,proto3" json:"filter,omitempty"`
}

func (x *ListNotificationsRequest) Reset() {
	*x = ListNotificationsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_notifications_v1_notifications_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListNotificationsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListNotificationsRequest) ProtoMessage() {}

func (x *ListNotificationsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_notifications_v1_notifications_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListNotificationsRequest.ProtoReflect.Descriptor instead.
func (*ListNotificationsRequest) Descriptor() ([]byte, []int) {
	return file_sift_notifications_v1_notifications_proto_rawDescGZIP(), []int{1}
}

func (x *ListNotificationsRequest) GetPageSize() uint32 {
	if x != nil {
		return x.PageSize
	}
	return 0
}

func (x *ListNotificationsRequest) GetPageToken() string {
	if x != nil {
		return x.PageToken
	}
	return ""
}

func (x *ListNotificationsRequest) GetFilter() string {
	if x != nil {
		return x.Filter
	}
	return ""
}

// The response of a call to `NotificationService_ListNotifications`.
type ListNotificationsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Notifications []*Notification `protobuf:"bytes,1,rep,name=notifications,proto3" json:"notifications,omitempty"`
	NextPageToken string          `protobuf:"bytes,2,opt,name=next_page_token,json=nextPageToken,proto3" json:"next_page_token,omitempty"`
}

func (x *ListNotificationsResponse) Reset() {
	*x = ListNotificationsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_notifications_v1_notifications_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListNotificationsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListNotificationsResponse) ProtoMessage() {}

func (x *ListNotificationsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_notifications_v1_notifications_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListNotificationsResponse.ProtoReflect.Descriptor instead.
func (*ListNotificationsResponse) Descriptor() ([]byte, []int) {
	return file_sift_notifications_v1_notifications_proto_rawDescGZIP(), []int{2}
}

func (x *ListNotificationsResponse) GetNotifications() []*Notification {
	if x != nil {
		return x.Notifications
	}
	return nil
}

func (x *ListNotificationsResponse) GetNextPageToken() string {
	if x != nil {
		return x.NextPageToken
	}
	return ""
}

// The request for a call to `NotificationService_BatchUpdateNotifications` to update notifications.
// A maximum of 1000 notifications can be modified in a batch.
type BatchUpdateNotificationsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Requests []*UpdateNotificationRequest `protobuf:"bytes,1,rep,name=requests,proto3" json:"requests,omitempty"`
}

func (x *BatchUpdateNotificationsRequest) Reset() {
	*x = BatchUpdateNotificationsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_notifications_v1_notifications_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BatchUpdateNotificationsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BatchUpdateNotificationsRequest) ProtoMessage() {}

func (x *BatchUpdateNotificationsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_notifications_v1_notifications_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BatchUpdateNotificationsRequest.ProtoReflect.Descriptor instead.
func (*BatchUpdateNotificationsRequest) Descriptor() ([]byte, []int) {
	return file_sift_notifications_v1_notifications_proto_rawDescGZIP(), []int{3}
}

func (x *BatchUpdateNotificationsRequest) GetRequests() []*UpdateNotificationRequest {
	if x != nil {
		return x.Requests
	}
	return nil
}

type UpdateNotificationRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The notification to update. The notification's `notification_id` field is used to identify the notification to update
	// and must be provided.
	Notification *Notification `protobuf:"bytes,1,opt,name=notification,proto3" json:"notification,omitempty"`
	// The list of fields to be updated. Currently, the only field that can be updated is `is_read`.
	UpdateMask *fieldmaskpb.FieldMask `protobuf:"bytes,2,opt,name=update_mask,json=updateMask,proto3" json:"update_mask,omitempty"`
}

func (x *UpdateNotificationRequest) Reset() {
	*x = UpdateNotificationRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_notifications_v1_notifications_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UpdateNotificationRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UpdateNotificationRequest) ProtoMessage() {}

func (x *UpdateNotificationRequest) ProtoReflect() protoreflect.Message {
	mi := &file_sift_notifications_v1_notifications_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UpdateNotificationRequest.ProtoReflect.Descriptor instead.
func (*UpdateNotificationRequest) Descriptor() ([]byte, []int) {
	return file_sift_notifications_v1_notifications_proto_rawDescGZIP(), []int{4}
}

func (x *UpdateNotificationRequest) GetNotification() *Notification {
	if x != nil {
		return x.Notification
	}
	return nil
}

func (x *UpdateNotificationRequest) GetUpdateMask() *fieldmaskpb.FieldMask {
	if x != nil {
		return x.UpdateMask
	}
	return nil
}

// The response of a call to `NotificationService_BatchUpdateNotifications` containing the updated notifications.
type BatchUpdateNotificationsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The updated notifications.
	Notifications []*Notification `protobuf:"bytes,1,rep,name=notifications,proto3" json:"notifications,omitempty"`
}

func (x *BatchUpdateNotificationsResponse) Reset() {
	*x = BatchUpdateNotificationsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_sift_notifications_v1_notifications_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BatchUpdateNotificationsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BatchUpdateNotificationsResponse) ProtoMessage() {}

func (x *BatchUpdateNotificationsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_sift_notifications_v1_notifications_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BatchUpdateNotificationsResponse.ProtoReflect.Descriptor instead.
func (*BatchUpdateNotificationsResponse) Descriptor() ([]byte, []int) {
	return file_sift_notifications_v1_notifications_proto_rawDescGZIP(), []int{5}
}

func (x *BatchUpdateNotificationsResponse) GetNotifications() []*Notification {
	if x != nil {
		return x.Notifications
	}
	return nil
}

var File_sift_notifications_v1_notifications_proto protoreflect.FileDescriptor

var file_sift_notifications_v1_notifications_proto_rawDesc = []byte{
	0x0a, 0x29, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x31, 0x2f, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x15, 0x73, 0x69, 0x66,
	0x74, 0x2e, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
	0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61,
	0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x66, 0x69, 0x65,
	0x6c, 0x64, 0x5f, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
	0x75, 0x66, 0x2f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x2d, 0x67, 0x65, 0x6e,
	0x2d, 0x6f, 0x70, 0x65, 0x6e, 0x61, 0x70, 0x69, 0x76, 0x32, 0x2f, 0x6f, 0x70, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x22, 0xad, 0x04, 0x0a, 0x0c, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x27, 0x0a, 0x0f, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e,
	0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x3d,
	0x0a, 0x0c, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
	0x52, 0x0b, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x44, 0x61, 0x74, 0x65, 0x12, 0x3f, 0x0a,
	0x0d, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
	0x52, 0x0c, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x44, 0x61, 0x74, 0x65, 0x12, 0x2b,
	0x0a, 0x12, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x75, 0x73, 0x65,
	0x72, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x63, 0x72, 0x65, 0x61,
	0x74, 0x65, 0x64, 0x42, 0x79, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x2d, 0x0a, 0x13, 0x6d,
	0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f,
	0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69,
	0x65, 0x64, 0x42, 0x79, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x6f, 0x72,
	0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x0e, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x49, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74,
	0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f,
	0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12,
	0x17, 0x0a, 0x07, 0x69, 0x73, 0x5f, 0x72, 0x65, 0x61, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08,
	0x52, 0x06, 0x69, 0x73, 0x52, 0x65, 0x61, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x66, 0x75, 0x6c, 0x6c,
	0x5f, 0x6c, 0x69, 0x6e, 0x6b, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x66, 0x75, 0x6c,
	0x6c, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x54, 0x0a, 0x11, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0e,
	0x32, 0x27, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4b, 0x69, 0x6e, 0x64, 0x52, 0x10, 0x6e, 0x6f, 0x74, 0x69, 0x66,
	0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63,
	0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63,
	0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x1b, 0x0a, 0x09, 0x65, 0x6e, 0x74, 0x69, 0x74,
	0x79, 0x5f, 0x69, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x65, 0x6e, 0x74, 0x69,
	0x74, 0x79, 0x49, 0x64, 0x22, 0x7d, 0x0a, 0x18, 0x4c, 0x69, 0x73, 0x74, 0x4e, 0x6f, 0x74, 0x69,
	0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x12, 0x20, 0x0a, 0x09, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x0d, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x08, 0x70, 0x61, 0x67, 0x65, 0x53, 0x69,
	0x7a, 0x65, 0x12, 0x22, 0x0a, 0x0a, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x09, 0x70, 0x61, 0x67,
	0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x1b, 0x0a, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
	0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x42, 0x03, 0xe0, 0x41, 0x01, 0x52, 0x06, 0x66, 0x69, 0x6c,
	0x74, 0x65, 0x72, 0x22, 0x8e, 0x01, 0x0a, 0x19, 0x4c, 0x69, 0x73, 0x74, 0x4e, 0x6f, 0x74, 0x69,
	0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x12, 0x49, 0x0a, 0x0d, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e,
	0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x31,
	0x2e, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x6e,
	0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x26, 0x0a, 0x0f,
	0x6e, 0x65, 0x78, 0x74, 0x5f, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x6e, 0x65, 0x78, 0x74, 0x50, 0x61, 0x67, 0x65, 0x54,
	0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x74, 0x0a, 0x1f, 0x42, 0x61, 0x74, 0x63, 0x68, 0x55, 0x70, 0x64,
	0x61, 0x74, 0x65, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x51, 0x0a, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65,
	0x73, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x73, 0x69, 0x66, 0x74,
	0x2e, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76,
	0x31, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x42, 0x03, 0xe0, 0x41, 0x02,
	0x52, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x22, 0xab, 0x01, 0x0a, 0x19, 0x55,
	0x70, 0x64, 0x61, 0x74, 0x65, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x4c, 0x0a, 0x0c, 0x6e, 0x6f, 0x74, 0x69,
	0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23,
	0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0c, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69,
	0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x40, 0x0a, 0x0b, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
	0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f,
	0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x46, 0x69,
	0x65, 0x6c, 0x64, 0x4d, 0x61, 0x73, 0x6b, 0x42, 0x03, 0xe0, 0x41, 0x02, 0x52, 0x0a, 0x75, 0x70,
	0x64, 0x61, 0x74, 0x65, 0x4d, 0x61, 0x73, 0x6b, 0x22, 0x6d, 0x0a, 0x20, 0x42, 0x61, 0x74, 0x63,
	0x68, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x49, 0x0a, 0x0d,
	0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20,
	0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x6e, 0x6f, 0x74, 0x69, 0x66,
	0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4e, 0x6f, 0x74, 0x69,
	0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69,
	0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2a, 0x8e, 0x02, 0x0a, 0x10, 0x4e, 0x6f, 0x74, 0x69,
	0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x21, 0x0a, 0x1d,
	0x4e, 0x4f, 0x54, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4b, 0x49, 0x4e,
	0x44, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12,
	0x1a, 0x0a, 0x16, 0x4e, 0x4f, 0x54, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f,
	0x4b, 0x49, 0x4e, 0x44, 0x5f, 0x54, 0x45, 0x58, 0x54, 0x10, 0x01, 0x12, 0x29, 0x0a, 0x25, 0x4e,
	0x4f, 0x54, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4b, 0x49, 0x4e, 0x44,
	0x5f, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x53, 0x53, 0x49,
	0x47, 0x4e, 0x45, 0x44, 0x10, 0x02, 0x12, 0x35, 0x0a, 0x31, 0x4e, 0x4f, 0x54, 0x49, 0x46, 0x49,
	0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4b, 0x49, 0x4e, 0x44, 0x5f, 0x4d, 0x45, 0x4e, 0x54,
	0x49, 0x4f, 0x4e, 0x45, 0x44, 0x5f, 0x49, 0x4e, 0x5f, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x41, 0x54,
	0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x4f, 0x4d, 0x4d, 0x45, 0x4e, 0x54, 0x10, 0x03, 0x12, 0x29, 0x0a,
	0x25, 0x4e, 0x4f, 0x54, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4b, 0x49,
	0x4e, 0x44, 0x5f, 0x43, 0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x54, 0x52, 0x49,
	0x47, 0x47, 0x45, 0x52, 0x45, 0x44, 0x10, 0x04, 0x12, 0x2e, 0x0a, 0x2a, 0x4e, 0x4f, 0x54, 0x49,
	0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4b, 0x49, 0x4e, 0x44, 0x5f, 0x41, 0x4e,
	0x4e, 0x4f, 0x54, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x43,
	0x48, 0x41, 0x4e, 0x47, 0x45, 0x44, 0x10, 0x05, 0x32, 0xe9, 0x02, 0x0a, 0x13, 0x4e, 0x6f, 0x74,
	0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
	0x12, 0x95, 0x01, 0x0a, 0x11, 0x4c, 0x69, 0x73, 0x74, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x2f, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x6e, 0x6f,
	0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4c,
	0x69, 0x73, 0x74, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x30, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x6e,
	0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x31, 0x2e,
	0x4c, 0x69, 0x73, 0x74, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x1d, 0x82, 0xd3, 0xe4, 0x93, 0x02,
	0x17, 0x12, 0x15, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x6e, 0x6f, 0x74, 0x69, 0x66,
	0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0xb9, 0x01, 0x0a, 0x18, 0x42, 0x61, 0x74,
	0x63, 0x68, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x36, 0x2e, 0x73, 0x69, 0x66, 0x74, 0x2e, 0x6e, 0x6f, 0x74,
	0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x61,
	0x74, 0x63, 0x68, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x37, 0x2e,
	0x73, 0x69, 0x66, 0x74, 0x2e, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x61, 0x74, 0x63, 0x68, 0x55, 0x70, 0x64, 0x61, 0x74,
	0x65, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x2c, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x26, 0x3a, 0x01,
	0x2a, 0x22, 0x21, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x6e, 0x6f, 0x74, 0x69, 0x66,
	0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3a, 0x62, 0x61, 0x74, 0x63, 0x68, 0x55, 0x70,
	0x64, 0x61, 0x74, 0x65, 0x42, 0x93, 0x02, 0x0a, 0x19, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x69, 0x66,
	0x74, 0x2e, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
	0x76, 0x31, 0x42, 0x12, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x51, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2d, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2f,
	0x73, 0x69, 0x66, 0x74, 0x2f, 0x67, 0x6f, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x73, 0x2f, 0x67, 0x6f, 0x2f, 0x73, 0x69, 0x66, 0x74, 0x2f, 0x6e, 0x6f, 0x74, 0x69, 0x66,
	0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x31, 0x3b, 0x6e, 0x6f, 0x74, 0x69,
	0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x76, 0x31, 0xa2, 0x02, 0x03, 0x53, 0x4e,
	0x58, 0xaa, 0x02, 0x15, 0x53, 0x69, 0x66, 0x74, 0x2e, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x15, 0x53, 0x69, 0x66, 0x74,
	0x5c, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5c, 0x56,
	0x31, 0xe2, 0x02, 0x21, 0x53, 0x69, 0x66, 0x74, 0x5c, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74,
	0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x17, 0x53, 0x69, 0x66, 0x74, 0x3a, 0x3a, 0x4e, 0x6f,
	0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x92,
	0x41, 0x18, 0x12, 0x16, 0x0a, 0x14, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x33,
}

var (
	file_sift_notifications_v1_notifications_proto_rawDescOnce sync.Once
	file_sift_notifications_v1_notifications_proto_rawDescData = file_sift_notifications_v1_notifications_proto_rawDesc
)

func file_sift_notifications_v1_notifications_proto_rawDescGZIP() []byte {
	file_sift_notifications_v1_notifications_proto_rawDescOnce.Do(func() {
		file_sift_notifications_v1_notifications_proto_rawDescData = protoimpl.X.CompressGZIP(file_sift_notifications_v1_notifications_proto_rawDescData)
	})
	return file_sift_notifications_v1_notifications_proto_rawDescData
}

var file_sift_notifications_v1_notifications_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_sift_notifications_v1_notifications_proto_msgTypes = make([]protoimpl.MessageInfo, 6)
var file_sift_notifications_v1_notifications_proto_goTypes = []interface{}{
	(NotificationKind)(0),                    // 0: sift.notifications.v1.NotificationKind
	(*Notification)(nil),                     // 1: sift.notifications.v1.Notification
	(*ListNotificationsRequest)(nil),         // 2: sift.notifications.v1.ListNotificationsRequest
	(*ListNotificationsResponse)(nil),        // 3: sift.notifications.v1.ListNotificationsResponse
	(*BatchUpdateNotificationsRequest)(nil),  // 4: sift.notifications.v1.BatchUpdateNotificationsRequest
	(*UpdateNotificationRequest)(nil),        // 5: sift.notifications.v1.UpdateNotificationRequest
	(*BatchUpdateNotificationsResponse)(nil), // 6: sift.notifications.v1.BatchUpdateNotificationsResponse
	(*timestamppb.Timestamp)(nil),            // 7: google.protobuf.Timestamp
	(*fieldmaskpb.FieldMask)(nil),            // 8: google.protobuf.FieldMask
}
var file_sift_notifications_v1_notifications_proto_depIdxs = []int32{
	7,  // 0: sift.notifications.v1.Notification.created_date:type_name -> google.protobuf.Timestamp
	7,  // 1: sift.notifications.v1.Notification.modified_date:type_name -> google.protobuf.Timestamp
	0,  // 2: sift.notifications.v1.Notification.notification_type:type_name -> sift.notifications.v1.NotificationKind
	1,  // 3: sift.notifications.v1.ListNotificationsResponse.notifications:type_name -> sift.notifications.v1.Notification
	5,  // 4: sift.notifications.v1.BatchUpdateNotificationsRequest.requests:type_name -> sift.notifications.v1.UpdateNotificationRequest
	1,  // 5: sift.notifications.v1.UpdateNotificationRequest.notification:type_name -> sift.notifications.v1.Notification
	8,  // 6: sift.notifications.v1.UpdateNotificationRequest.update_mask:type_name -> google.protobuf.FieldMask
	1,  // 7: sift.notifications.v1.BatchUpdateNotificationsResponse.notifications:type_name -> sift.notifications.v1.Notification
	2,  // 8: sift.notifications.v1.NotificationService.ListNotifications:input_type -> sift.notifications.v1.ListNotificationsRequest
	4,  // 9: sift.notifications.v1.NotificationService.BatchUpdateNotifications:input_type -> sift.notifications.v1.BatchUpdateNotificationsRequest
	3,  // 10: sift.notifications.v1.NotificationService.ListNotifications:output_type -> sift.notifications.v1.ListNotificationsResponse
	6,  // 11: sift.notifications.v1.NotificationService.BatchUpdateNotifications:output_type -> sift.notifications.v1.BatchUpdateNotificationsResponse
	10, // [10:12] is the sub-list for method output_type
	8,  // [8:10] is the sub-list for method input_type
	8,  // [8:8] is the sub-list for extension type_name
	8,  // [8:8] is the sub-list for extension extendee
	0,  // [0:8] is the sub-list for field type_name
}

func init() { file_sift_notifications_v1_notifications_proto_init() }
func file_sift_notifications_v1_notifications_proto_init() {
	if File_sift_notifications_v1_notifications_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_sift_notifications_v1_notifications_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Notification); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sift_notifications_v1_notifications_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListNotificationsRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sift_notifications_v1_notifications_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListNotificationsResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sift_notifications_v1_notifications_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BatchUpdateNotificationsRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sift_notifications_v1_notifications_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UpdateNotificationRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_sift_notifications_v1_notifications_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BatchUpdateNotificationsResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_sift_notifications_v1_notifications_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   6,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_sift_notifications_v1_notifications_proto_goTypes,
		DependencyIndexes: file_sift_notifications_v1_notifications_proto_depIdxs,
		EnumInfos:         file_sift_notifications_v1_notifications_proto_enumTypes,
		MessageInfos:      file_sift_notifications_v1_notifications_proto_msgTypes,
	}.Build()
	File_sift_notifications_v1_notifications_proto = out.File
	file_sift_notifications_v1_notifications_proto_rawDesc = nil
	file_sift_notifications_v1_notifications_proto_goTypes = nil
	file_sift_notifications_v1_notifications_proto_depIdxs = nil
}
