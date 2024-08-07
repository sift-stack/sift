# Sift API Status Codes and Timeouts

This document covers all the various status codes Sift may return and under what conditions, as well as what users can expect with regard to timeouts.

- [Sift gRPC API Status Codes](#sift-grpc-api-status-codes)
    * [Status Codes Explicitly Set by Sift](#status-codes-explicitly-set-by-sift)
    * [Status Codes Set by gRPC Libraries](#status-codes-set-by-grpc-libraries)
    * [Recommendations](#recommendations)
- [Sift API Timeouts](sift-api-timeouts)
    * [Keepalive Pings During Streaming](#keepalive-pings-during-streaming)

## Sift gRPC API Status Codes

The gRPC status codes that are ultimately returned to the client may fall into one of these two categories:

- Status codes explicitly set by Sift
- Status codes set by underlying gRPC libraries used by the Sift API or the client

Below provides further details to elucidate situations where a known status code might be expected to occur and is subject to change in the future. The meaning behind each error code can be found at this [link](https://grpc.github.io/grpc/core/md_doc_statuscodes.html).

### Status Codes Explicitly Set by Sift

At the unary interceptor (i.e. gRPC middleware) level Sift controls what status codes are returned to the client. The status codes currently being utilized by Sift are limited to what is shown in the following table. Any status code that falls outside of this list can be assumed to be set at the level of gRPC libraries with the exception of `INTERNAL` and `UNAUTHENTICATED` which could be set either by Sift or the gRPC libraries used by Sift or the client.

| Code | Number |
| -------- | ------- |
|OK|0|
|INVALID_ARGUMENT|3|
|NOT_FOUND|5|
|ALREADY_EXISTS|6|
|PERMISSION_DENIED|7|
|INTERNAL|13|
|UNAUTHENTICATED|16|

### Status Codes Set by gRPC Libraries

The following status codes may be generated by the gRPC libraries used by the server or client, meaning that they can be set either at the server or client level. These are not explicitly set by Sift. The following table details where each status code is generated and some common example situations demonstrating when they may be set.

|Example Scenarios|Code|Number|Generated at Client or Server|
| -------- | ------- | ------- | ------- |
|Indicates the operation was canceled either by server or client. | CANCELLED | 1 | Both |
|Server side application doesn’t return a valid gRPC status|UNKNOWN|2|Server|
|Error parsing a returned status|UNKNOWN|2|Client|
|No response received before Deadline expires. This may occur either when the client is unable to send the request to the server or when the server fails to respond in time.|DEADLINE_EXCEEDED|4|Both|
|Server temporarily out of resources|RESOURCE_EXHAUSTED|8|Server|
|Client does not have enough memory to hold the server response|RESOURCE_EXHAUSTED|8|Client|
|Sent or received message was larger than configured limit|RESOURCE_EXHAUSTED|8|Both|
|Error parsing request proto|INTERNAL|13|Server|
|Error parsing response proto|INTERNAL|13||Client|
|Client tries to send a request to a closing connection|INTERNAL|13|Server|
|Server shutting down|UNAVAILABLE|14|Server|
|Occurs during abrupt shutdown of application server or network connection|UNAVAILABLE|14|Both|
|Incorrect Auth metadata ( Credentials failed to get metadata, Incompatible credentials set on channel and call, Invalid host set in :authority metadata, etc.)|UNAUTHENTICATED|16|Both|

### Recommendations

Generally speaking knowing where the status code gets set is incredibly useful for debugging. In common situations such as user error the Sift API will provide a message in the response proto to indicate what needs to be rectified, e.g. the request is missing a required parameter. In situations where a gRPC status code is INTERNAL it is crucial to inspect the response beyond just the status code. INTERNAL errors set by Sift often come with a trace-ID which you can provide to Sift in order to help us debug your particular issue. If there is no trace-ID, it’s usually indicative that the errors are occurring library-level. Regardless, if there is an issue that you are experiencing with the Sift API that you are unable to resolve please provide us with as much information about the request and the response as possible and we will do our best to assist you.

## Sift API Timeouts

Regardless of which API a client is connecting to—whether it be gRPC or REST—the maximum duration a connection is allowed to stay alive without any network traffic is 60 seconds. This is what is referred to as an idle timeout. When an idle timeout occurs, one of two things may occur:

Either the Sift API gracefully shuts down the connection by sending an HTTP/2 GOAWAY frame, or

the reverse proxy automatically terminates the connection.

Depending on the language and gRPC library used by the client, and idle timeout could produce either a DEADLINE_EXCEEDED gRPC status code or a UNAVAILABLE.

### Keepalive Pings During Streaming

When clients are streaming data to Sift, it is common for there to be periods of inactivity that might go beyond Sift’s configured idle timeout. In these situations it is important to properly configure the client to leverage gRPC’s keepalive mechanism to ensure that a connection isn’t prematurely terminated.

How keepalive pings are configured is largely dependent on the language and library, but regardless of which the following channel arguments are what is used to configure periodic keep-alive pings:

- `GRPC_ARG_KEEPALIVE_TIME_MS`
    * This channel argument controls the period (in milliseconds) after which a keep-alive ping is sent on the transport.

- `GRPC_ARG_KEEPALIVE_TIMEOUT_MS`
    * This channel argument controls the amount of time (in milliseconds) the sender of the keep-alive ping waits for an acknowledgement. If it does not receive an acknowledgment within this time, it will close the connection.

As an example, consider the following channel configurations in pseudo-code:

```python
channel_options = [
    ("grpc.keepalive_time_ms", 8000),
    ("grpc.keepalive_timeout_ms", 5000),
]
```

With respect to this configuration:
- A keepalive ping will be sent every 8 seconds (8000 ms) during a period of inactivity
- The client will wait for 5 seconds (5000 ms) for the server to acknowledge a keepalive ping before deciding to take a particular action:
    * The client could decide to completely shut down the connection, or
    * The client could try to manually re-establish a connection, or
    * An automated retry mechanism could be initiated.

For a full-guide on available channel options to configure keepalive pings, please refer to this [link](https://github.com/grpc/grpc/blob/master/doc/keepalive.md).
