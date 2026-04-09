# sift_stream

[![Crates.io](https://img.shields.io/crates/v/sift_stream.svg)](https://crates.io/crates/sift_stream)
[![docs.rs](https://img.shields.io/docsrs/sift_stream)](https://docs.rs/sift_stream/latest/sift_stream/)

## Overview

`sift_stream` is a Rust telemetry streaming library for the Sift platform. It is built around a
task-based async architecture with multiple cooperating tasks communicating through bounded channels
and control messages to provide reliable, high-throughput data ingestion.

Feature highlights (availability depends on mode — see [Transport Modes](#transport-modes)):
- Builtin retries with configurable exponential-backoff retry policies.
- Periodic checkpointing to obtain delivery confirmation from Sift.
- Optional disk backups and automatic re-ingestion on failure.
- Optional tracing/logging for stream health and ingestion performance metrics.

See the [examples](https://github.com/sift-stack/sift/tree/main/rust/crates/sift_stream/examples/)
directory for end-to-end usage.


## Core Architecture Components

### Main Entry Points

- **`SiftStreamBuilder`**: Configures and builds `SiftStream` instances via a fluent builder chain.
- **`SiftStream<E, T>`**: Generic streaming interface where `E: Encoder` encodes data and
  `T: Transport` transmits it.
- **`SiftStream<IngestionConfigEncoder, LiveStreamingOnly>`**: Real-time gRPC streaming, single
  ingestion channel, direct backpressure. No checkpointing or disk backups.
- **`SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>`**: Real-time gRPC streaming with
  periodic checkpointing, retry, and disk backups. Dual-channel architecture.
- **`SiftStream<IngestionConfigEncoder, FileBackup>`**: Writes data to rolling disk files without
  live network streaming.

### Architecture Overview

`SiftStream` separates **encoding** (how data is structured) from **transport** (how data is
transmitted):

- **`Encoder` trait**: Converts user-provided data (e.g. `Flow` messages) into the wire format,
  manages flow descriptors and ingestion configuration, and exposes metrics snapshots. Implemented
  by `IngestionConfigEncoder`.

- **`Transport` trait**: Delivers encoded messages to their destination, manages internal channels
  and background tasks, and handles shutdown. Implemented by `LiveStreamingOnly`,
  `LiveStreamingWithBackups`, and `FileBackup`.

- **`Encodeable` trait**: Implemented by types that can be passed to `SiftStream::send` (e.g.
  `Flow`, `FlowBuilder`).

This separation allows encoding schemes and transport mechanisms to evolve independently.

### Transport Modes

Three transport modes are available, selected via the builder chain:

- **`LiveStreamingOnly`**: Streams data to Sift in real-time over a single bounded ingestion
  channel. `send` awaits when the ingestion channel is full — the caller blocks until the
  ingestion task drains capacity. No checkpointing, no disk backups. Retry is available.
  Selected with `.live_only().build()`.

- **`LiveStreamingWithBackups`**: Streams data to Sift in real-time using a dual-channel
  architecture (backup channel + ingestion channel) with periodic checkpointing, retry, and
  disk backups. `send` awaits on the **backup channel** for data durability. Selected with
  `.live_with_backups().build()`.

- **`FileBackup`**: Writes data to rolling disk files without live network streaming. `send`
  awaits when the write channel is full. Useful for offline recording or CI/CD workflows where
  data is only uploaded to Sift conditionally. Selected with `.file_backup().build()`.

### Task System

The task system is **specific to `LiveStreamingWithBackups`**. That mode spawns three async tasks:

1. **Backup Manager Task** — Manages backup file creation, rotation, and cleanup.
2. **Ingestion Task** — Manages gRPC streaming to Sift, checkpoint timing, and retry.
3. **Re-ingestion Task** — Reads backup files and re-streams them to Sift after failures.

`LiveStreamingOnly` spawns an ingestion task.
`FileBackup` spawns a file-writer task.


## Control Messages

Control messages are low-frequency signals sent between tasks via a broadcast channel to coordinate
checkpointing, failure recovery, and shutdown.

## Task Responsibilities and Control Message Flow

### 1. Backup Manager Task

**Responsibilities:**
- Receives data messages via the `backup_tx` channel.
- Writes messages to backup files in PBFS format.
- Manages backup file rotation based on size/count limits.
- Handles checkpoint completion and cleanup.
- Tracks message IDs to determine which messages have been successfully committed to Sift.
- Skips messages that have already been confirmed as successfully streamed (when backups lag ingestion).

**Control Messages Sent:**
- `BackupFull` — When backup files reach the maximum count limit.
- `ReingestBackups` — When a checkpoint fails and backup files need re-ingestion.

**Control Messages Received:**
- `CheckpointComplete { first_message_id, last_message_id }` — Checkpoint succeeded; range of confirmed message IDs.
- `CheckpointNeedsReingestion { first_message_id, last_message_id }` — Checkpoint failed; range of message IDs to re-ingest.
- `Shutdown` — Initiates graceful shutdown.

### 2. Ingestion Task

**Responsibilities:**
- Receives data messages via the `ingestion_tx` channel.
- Creates and manages gRPC streams to Sift.
- Implements retry logic with exponential backoff.
- Manages checkpoint timing.

**Control Messages Sent:**
- `SignalNextCheckpoint` — When a new stream is desired to confirm messages have been received.
- `CheckpointComplete { first_message_id, last_message_id }` — Stream concluded at checkpoint boundary; includes confirmed message ID range.
- `CheckpointNeedsReingestion { first_message_id, last_message_id }` — gRPC stream failed; includes message ID range that needs re-ingestion.

**Control Messages Received:**
- `BackupFull` — Triggers an immediate checkpoint, resetting the normal interval.
- `Shutdown` — Initiates graceful shutdown with a final stream completion.

### 3. Re-ingestion Task

**Responsibilities:**
- Receives backup file paths via `ReingestBackups` control messages.
- Reads backup files and re-ingests the data to Sift.
- Implements retry logic for failed re-ingestion attempts.
- Cleans up backup files after successful re-ingestion.

**Control Messages Received:**
- `ReingestBackups` — List of backup files to re-ingest.
- `Shutdown` — Initiates graceful shutdown.


## Channel Architecture

### Data Channels (per mode)

| Mode | Channels | Backpressure source |
|---|---|---|
| `LiveStreamingOnly` | `ingestion_tx` only | Ingestion channel full → caller awaits |
| `LiveStreamingWithBackups` | `backup_tx` + `ingestion_tx` | Backup channel full → caller awaits; ingestion channel full → oldest message evicted |
| `FileBackup` | `write_tx` only | Write channel full → caller awaits |

**`LiveStreamingWithBackups` channel notes:**

The backup channel is the primary durability path. All backpressure is applied there. The ingestion
channel uses force-send: when full, the oldest buffered message is evicted and redirected to the
backup channel (awaiting capacity there). This means `send` errors may return an older displaced
message rather than the one that was just passed in.


### Control Channel

- **`control_tx/control_rx`**: Broadcast channel for low-frequency control messages.

### Channel Capacities

Default capacities:
- **Data channels**: 102,400
- **Control channel**: 1,024

These are configurable via the mode builders (`ingestion_data_channel_capacity`,
`backup_data_channel_capacity`, `control_channel_capacity`).


## Checkpoint System

Only applicable to `LiveStreamingWithBackups`.

The checkpoint system provides periodic delivery confirmation by closing and re-opening the gRPC
stream at configured intervals. Message IDs are used to precisely track which messages have been
confirmed, enabling efficient backup management even when ingestion and backup are out of sync.

### Message ID-Based Tracking

Each message is assigned a monotonically increasing ID. Checkpoints track the range
(`first_message_id` to `last_message_id`) included in each checkpoint, enabling the backup manager
to:

- **Drain committed messages efficiently**: Messages with IDs ≤ the confirmed ID are skipped during
  backup, allowing the backup system to catch up faster.
- **Match files to checkpoints precisely**: Each backup file tracks its message ID range for exact
  checkpoint matching.
- **Handle out-of-order scenarios**: The system correctly handles cases where ingestion completes
  before backups catch up, and vice versa.

### Checkpoint Triggers

1. **Timer-based**: Regular intervals (configurable via `checkpoint_interval`, default 60 s).
2. **Backup full**: When backup files reach the maximum count limit.
3. **Shutdown**: Final checkpoint during graceful shutdown.

### Checkpoint Process

1. Timer expires or backup-full signal received.
2. Current gRPC stream completes, flushing all buffered data.
3. `CheckpointComplete { first_message_id, last_message_id }` sent with the confirmed message ID range.
4. Backup files either deleted (success) or queued for re-ingestion (failure), matched by message ID range.

An "ok" gRPC status from Sift after stream close indicates all messages for that stream were
received. Backup files can be retained indefinitely regardless of checkpoint outcome via
`DiskBackupPolicy::retain_backups`.

### Backup File Management

#### Backup File Lifecycle

1. **Creation**: New file created when the first message requiring backup arrives.
2. **Writing**: Messages are written with the file tracking its `first_message_id`/`last_message_id` range.
3. **Rotation**: File rotated when it exceeds `max_backup_file_size`.
4. **Checkpoint completion**: Files processed based on message ID range matching — deleted on success or queued for re-ingestion on failure.
5. **Re-ingestion**: All files whose message ID ranges overlap a failed checkpoint range are re-ingested.

#### Message ID-Based File Processing

When a checkpoint completes:

- Files fully within a successful checkpoint range → deleted.
- Files overlapping a failed checkpoint range → queued for re-ingestion.
- Files beyond the current checkpoint range → retained until their checkpoint completes.

#### Backup File Configuration

Configured via `DiskBackupPolicy`:

- **`max_backup_file_size`**: Maximum raw byte length of a single backup file before rotation. Defaults to 500 MiB.
- **`max_file_count`** (via `RollingFilePolicy`): Maximum number of files before a checkpoint is forced. Defaults to unlimited.

#### Selecting Optimal Backup File Parameters

Smaller, more frequent checkpoints reduce the data volume that must be re-ingested on failure but
increase checkpoint management overhead (each checkpoint re-opens the gRPC stream). The default
configuration is appropriate for most use cases.


## Error Handling and Recovery

### Error Scenarios

1. **gRPC Stream Failure** (`LiveStreamingWithBackups` only):
   - Ingestion task sends `CheckpointNeedsReingestion`.
   - Backup manager matches backup files to the failed checkpoint range and queues them for re-ingestion.
   - Re-ingestion task processes files asynchronously alongside regular ingestion.

2. **gRPC Re-ingestion Failure** (`LiveStreamingWithBackups` only):
   - Exponential-backoff retries for automated recovery.
   - Data remains on disk if all retries fail.

3. **Ingestion Channel Full** (`LiveStreamingWithBackups` only):
   - Oldest buffered message in the ingestion channel is evicted via force-send.
   - Evicted message is redirected to the backup channel.
   - No data is permanently lost provided the backup channel has capacity.

4. **Backup/Write Channel Full** (all live modes):
   - `send` awaits until the channel drains — backpressure is applied to the caller.
   - `try_send` returns `TrySendError::Full` immediately with the undelivered message.

### Retry Logic

- **Exponential backoff**: Configurable via `RetryPolicy` (`max_attempts`, `initial_backoff`,
  `max_backoff`, `backoff_multiplier`).
- **Backup recovery**: Failed data is automatically re-ingested from backup files
  (`LiveStreamingWithBackups` only).


## Shutdown Process

The shutdown process ensures graceful termination with data preservation.

### Shutdown Sequence (`LiveStreamingWithBackups`)

1. Both data channels (`backup_tx`, `ingestion_tx`) are dropped.
2. `Shutdown` control message sent to all tasks.
3. Ingestion task drains queued messages and completes a final gRPC stream.
4. Final `CheckpointComplete` sent.
5. All three tasks complete.
6. Backup files cleaned up based on the final checkpoint outcome.

In order to return quickly, re-ingestion is halted on shutdown. Any backup files not yet
re-ingested remain on disk for manual ingestion.

**Important**: Always call `SiftStream::finish()` before dropping the stream. Dropping without
calling `finish` may result in tail-end data not reaching Sift.

### Shutdown Sequence (`LiveStreamingOnly`)

1. Ingestion channel is closed.
2. `Shutdown` control message sent.
3. Ingestion task drains all queued messages then exits.

### Shutdown Sequence (`FileBackup`)

1. Write channel is closed.
2. File-writer task drains all queued messages, flushes and syncs the current file, then exits.


## Data Flow Architecture

### Normal Operation Flow

1. **User calls `SiftStream::send()`** with an `Encodeable` type (e.g. `Flow`).
2. **Encoding**: `IngestionConfigEncoder` converts it to `IngestWithConfigDataStreamRequest`.
3. **Transport dispatch**:
   - `LiveStreamingOnly`: Message assigned an ID → sent to `ingestion_tx` channel.
   - `LiveStreamingWithBackups`: Message assigned an ID → sent to `backup_tx`, then force-sent to `ingestion_tx` (evicting the oldest if full to `backup_tx`).
   - `FileBackup`: Message sent to `write_tx` channel → written to disk.
4. **Background tasks** (`LiveStreamingWithBackups`):
   - Ingestion task → gRPC stream → Sift.
   - Backup manager task → backup files on disk.
5. **Checkpoint completion** (`LiveStreamingWithBackups`): Files deleted or queued for re-ingestion based on checkpoint outcome.

### Failure Recovery Flow (`LiveStreamingWithBackups`)

1. gRPC failure → `CheckpointNeedsReingestion { first_message_id, last_message_id }`.
2. Backup manager matches backup files to the failed checkpoint's message ID range and queues them.
3. Re-ingestion task reads backup files and re-streams to Sift.
4. Success → backup files deleted.
5. Failure → exponential-backoff retry.


## Metrics and Monitoring

The system records per-instance metrics:

- **Message counts**: messages received, sent, dropped.
- **Byte counts**: bytes sent and received.
- **Checkpoint metrics**: success/failure counts, timing.
- **Retry metrics**: retry counts, backoff timing.
- **Backup metrics**: file counts, sizes, rotation events, committed message ID, queued checkpoints and file contexts.

Metrics are accessible via `SiftStream::get_metrics_snapshot` (requires `metrics-unstable` feature)
or via the optional lightweight HTTP metrics server.


## Key Design Principles

1. **Fault tolerance**: Multiple layers of error handling and recovery.
2. **Data reliability**: Backup system protects against data loss between checkpoints.
3. **Backpressure**: Bounded channels prevent unbounded memory growth; each mode has a clearly defined backpressure source.
4. **Graceful shutdown**: Clean termination with data preservation across all modes.
5. **Observability**: Comprehensive metrics and structured tracing.
6. **Configurability**: Channel capacities, checkpoint intervals, retry policies, and backup policies are all tunable.
