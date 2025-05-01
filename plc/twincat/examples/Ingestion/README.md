# TwinCAT3 Sift Ingestion Example

This example demonstrates how to use the Sift Client within a TwinCAT3 PLC application to ingest telemetry data to the Sift platform. The state machine used here walks through all the calls required to initialize the Sift Client, create an ingestion config, create a run, and start streaming data.

## Overview

The application walks through the full ingestion process using the Sift API:

1. **Initialize the `FB_SiftClient` function block** with a gRPC URI, API key, and string buffer for JSON messages.
```
fbSiftClient(sGrpcUri := sGrpcUri, 
             sApiKey := sApiKey,
             pJsonBuffer := ADR(aJsonBuffer),
             pJbLength := SIZEOF(aJsonBuffer),
             bExecute := TRUE);
```

2. **Create an Ingestion Config with the `FB_IngestionConfigService` function block**. This function block will initialize pointers to `ST_FlowConfig` and `ST_ChannelConfig` that can be used later to save and write telemetry values.
```
pFlow1 : POINTER TO ST_FlowConfig;
pIntChannel1 : POINTER TO ST_ChannelConfig;
...
pFlow1 := fbIngestionConfig.CreateFlowConfig(sName := 'flow_1');
pIntChannel1 := fbIngestionConfig.CreateChannelConfig(pFlowConfig := pFlow1, 
                                                      sName:='plc.int_channel1', 
                                                      eDataType:=E_ChannelDataType.Int32, 
                                                      sDescription:='An integer channel', 
                                                      sUnit:='count');

```
Validate the config. After all Flows and Channels have been defined.
```
fbIngestionConfig.Validate();
```

3. **Create a Run with the `FB_RunService` function block** to track a specific data collection session.
```
fbRun(bExecute := TRUE, 
     hSiftClient := fbSiftClient.hSiftClient,
     sName:=sRunName, 
     sDescription := sRunDescription);
```
Runs are optional.

4. **Stream Data** continuously from the PLC to Sift using the `SetChannelValue` and `FB_IngestService`.

```
SetChannelValue(pIntChannel1, nData1);
fbIngest.WriteData(pFlow1, GetCurrentTime());
fbIngest(hSiftClient := fbSiftClient.hSiftClient, sRunId:=fbRun.sId)
```

5. **Handle Telemetering** of internal metrics like buffer utilization and message count. `fbIngest.fBufferUtilization` reports the total utilization of the internal buffer. If there isn't enough margin, increase `SiftClientParameters.MAX_PACKET_SIZE` until there is sufficient margin.

6. **Gracefully stop or error-handle** the session using `bStop`.
```
fbSiftClient(bStop := TRUE);
fbSiftClient(bStop := FALSE);
```