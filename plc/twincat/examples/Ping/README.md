# TwinCAT3 Sift Ping Example

This example demonstrates how to use the Sift Client within a TwinCAT3 PLC application to run a ping test. This will verify that `sift_proxy` is installed correctly and that the user's aPI credentials are valid for communicating with `Sift`.

## Overview

The application walks through the ping process using the Sift API:
1. **Initialize the `FB_SiftClient` function block** with a gRPC URI, API key, and string buffer for JSON messages.
```
fbSiftClient(sGrpcUri := sGrpcUri, 
             sApiKey := sApiKey,
             pJsonBuffer := ADR(aJsonBuffer),
             pJbLength := SIZEOF(aJsonBuffer),
             bExecute := TRUE);
```

2. **Ping Sift** using the `FB_PingService`. `bExecute` will trigger the ping on a rising edge.

```
fbPing(bExecute:=TRUE, hSiftClient:=fbSiftClient.hSiftClient);
...
fbPing(bExecute:=FALSE);
```

3. **Verify response** using `FB_PingService`'s return value.
```
IF fbPing.bSuccess THEN
     eSiftState := PING_SUCCESS;
ELSE
     eSiftState := PING_FAIL;
END_IF
```

4. **Gracefully stop or error-handle** the session using `bStop`.
```
fbSiftClient(bStop := TRUE);
fbSiftClient(bStop := FALSE);
```