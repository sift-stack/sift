# sift_stream

## Overview

SiftStream is a Rust-based telemetry streaming system that provides reliable, high-throughput data ingestion to the
Sift platform. The architecture is built around a task-based system with multiple async tasks communicating through
channels and control messages to ensure data reliability and fault tolerance.

Features highlights:
- Builtin retries with default or custom retry policies in the case of a Sift outage or a
  client-side network outage.
- Periodic checkpointing to confirm that all data within a particular period has been received
  by Sift.
- Optional automated backups to mitigate data-loss in the case of misc. outages.
- Optional tracing/logging to monitor the health of your stream and view various ingestion
  performance metrics.

See the [examples](https://github.com/sift-stack/sift/tree/main/rust/crates/sift_stream/examples/) directory for demonstrations on how to stream data to Sift using
`sift_stream`.


## Core Architecture Components

### Main Entry Points

- **SiftStreamBuilder**: Configures and builds SiftStream instances with various options
- **SiftStream<IngestionConfigMode>**: Main streaming interface that users interact with
- **IngestionConfigMode**: Core streaming implementation that manages the task system

### Task System

The SiftStream architecture consists of three main async tasks that work together to provide reliable data streaming:

1. **Backup Manager Task** - Handles backup file creation and management
2. **Ingestion Task** - Manages gRPC streaming to Sift
3. **Re-ingestion Task** - Handles re-ingestion of backup files when failures occur

## Control Messages

Control messages are low-frequency messages sent between tasks via broadcast channels to coordinate checkpointing,
error handling, and shutdown processes.

### ControlMessage Enum

```rust
pub enum ControlMessage {
    /// Signal that the backup is full and a new checkpoint should be started
    BackupFull,
    
    /// Request to re-ingest backup files
    ReingestBackups { backup_files: Vec<PathBuf> },
    
    /// Signal to complete the checkpoint
    CheckpointComplete,
    
    /// Signal the checkpoint needs re-ingestion
    CheckpointNeedsReingestion,
    
    /// Error signal
    ErrorSignal { error: String },
    
    /// Shutdown signal for all tasks
    Shutdown,
}
```

## Task Responsibilities and Control Message Flow

### 1. Backup Manager Task

**Responsibilities:**
- Receives data messages via `backup_tx` channel
- Writes messages to backup files in PBFS format
- Manages backup file rotation based on size/count limits
- Handles checkpoint completion and cleanup

**Control Messages Sent:**
- `BackupFull` - When backup files reach maximum count limit
- `ReingestBackups` - When checkpoint fails and backup files need re-ingestion

**Control Messages Received:**
- `CheckpointComplete` - Signals checkpoint completion
- `CheckpointNeedsReingestion` - Signals the current checkpoint will need re-ingestion
- `Shutdown` - Initiates graceful shutdown

**Conditions for Sending Messages:**
- `BackupFull`: Triggered when the number of backup files being tracked reaches the configured maximum
- `ReingestBackups`: Triggered when a message (control or data) has indicated the current checkpoint should be re-ingested

### 2. Ingestion Task

**Responsibilities:**
- Receives data messages via `ingestion_tx` channel
- Creates and manages gRPC streams to Sift
- Implements retry logic with exponential backoff
- Handles checkpoint timing and completion

**Control Messages Sent:**
- `CheckpointComplete` - When checkpoint timer expires or backup is full
- `CheckpointNeedsReingestion` - When gRPC stream fails

**Control Messages Received:**
- `BackupFull` - Triggers immediate checkpoint completion, reseting the normal checkpoint interval
- `Shutdown` - Initiates graceful shutdown with final stream completion

**Conditions for Sending Messages:**
- `CheckpointComplete`: 
  - Timer expires (checkpoint_interval reached)
  - Backup full signal received
  - During shutdown process
- `CheckpointNeedsReingestion`: When gRPC stream fails with error

### 3. Re-ingestion Task

**Responsibilities:**
- Receives backup file paths via `ReingestBackups` control message
- Reads backup files and re-ingests data to Sift
- Implements retry logic for failed re-ingestion attempts
- Manages backup file cleanup after successful re-ingestion

**Control Messages Received:**
- `ReingestBackups` - Contains list of backup files to re-ingest
- `Shutdown` - Initiates graceful shutdown

## Channel Architecture

### Data Channels
- **ingestion_tx/ingestion_rx**: Bounded channel for high-frequency data messages
- **backup_tx/backup_rx**: Bounded channel for backup data messages

**IMPORTANT**:
Data reliability is among the most important requirements of sift-stream, thus if the data channel used for backups
becomes full, an error is returned to the caller. This is in contrast to the data channel used for the primary
ingestion into Sift -- if this channel becomes full, the oldest data will be removed in favor of streaming newer
data. The data removed during this process will have been backed up to disk and will be re-ingested at the next
checkpoint.

### Control Channel
- **control_tx/control_rx**: Broadcast channel (1,024 capacity) for low-frequency control messages

### Channel Capacities

The default capacities are as follows:
- **data**: 10,240
- **control**: 1024

These can be configured however, based on individual streaming needs.

## Checkpoint System

The checkpoint system ensures data reliability by periodically creating checkpoints that can be used for recovery.

### Checkpoint Triggers

1. **Timer-based**: Regular intervals (configurable via `checkpoint_interval`)
2. **Backup full**: When backup files reach maximum count limit
3. **Manual**: During shutdown process

### Checkpoint Process

1. **Checkpoint Signal**: Timer expires or backup full signal received
2. **Stream Completion**: Current gRPC stream completes sending all buffered data
3. **Checkpoint Complete**: `CheckpointComplete` control message sent
4. **Backup Cleanup**: Backup files either deleted (success) or queued for re-ingestion (failure)

Backup files can also be retained regardless of successful checkpoints and re-ingestion processes.

### Backup File Management

The backup system manages files through a rotation policy that balances data reliability with storage efficiency. Understanding how backup files are handled is crucial for optimizing checkpoint behavior and minimizing unnecessary re-ingestion.

#### Backup File Lifecycle

1. **File Creation**: New backup files are created at the start of each checkpoint
2. **Data Writing**: Messages are written to the current backup file during the checkpoint
3. **File Rotation**: Files are rotated when they exceed size limits
4. **Checkpoint Completion**: Files are either deleted (success) or queued for re-ingestion (failure)
5. **Re-ingestion**: Failed checkpoints trigger re-ingestion of all files from that checkpoint

#### Backup File Configuration

The backup system is configured through the `DiskBackupPolicy` with two key parameters:

- **`max_file_size`**: Maximum size per backup file before rotation
- **`max_file_count`**: Maximum number of backup files to maintain before forcing a checkpoint

#### Selecting Optimal Backup File Parameters

**Key Principle**: Smaller, more frequent checkpoints reduce the amount of data that needs to be re-ingested when failures occur, but increase the overhead of checkpoint management and creating new gRPC streams to Sift.

##### Max File Size Considerations

**Smaller Files (Recommended for most use cases)**:
- **Pros**: 
  - Faster re-ingestion when failures occur
  - Lower memory usage during backup operations
  - More granular recovery options
- **Cons**: 
  - More file I/O operations
  - Higher checkpoint frequency
- **Use Case**: Lower throughput applications where checkpoint overhead is not a primary concern

**Larger Files**:
- **Pros**: 
  - Fewer file operations
  - Lower checkpoint overhead
- **Cons**: 
  - More data re-ingested on failures
  - Higher memory usage
  - Longer recovery times
- **Use Case**: High-throughput, low-latency applications where checkpoint overhead is a concern

##### Max File Count Considerations

**Lower Count (Recommended)**:
- **Pros**: 
  - Forces more frequent checkpoints
  - Limits storage usage
  - Reduces re-ingestion volume on failures
- **Cons**: 
  - More frequent checkpoint operations

**Higher Count**:
- **Pros**: 
  - Fewer checkpoint operations
  - Better throughput for high-volume applications
- **Cons**: 
  - More data at risk during failures
  - Higher storage requirements
  - Longer recovery times

#### Re-ingestion Impact Analysis

The backup file configuration directly impacts the cost of failures:

**Example Scenarios**:

1. **Conservative Configuration** (2 files, 1MB each):
   - Failure impact: ~2MB of data re-ingested
   - Recovery time: Fast
   - Storage overhead: Low

2. **Aggressive Configuration** (10 files, 100MB each):
   - Failure impact: Up to 1GB of data re-ingested
   - Recovery time: Slow
   - Storage overhead: High

3. **Balanced Configuration** (5 files, 10MB each):
   - Failure impact: ~50MB of data re-ingested
   - Recovery time: Moderate
   - Storage overhead: Moderate

#### Configuration Guidelines

**For High-Reliability Applications**:
```rust
DiskBackupPolicy {
    max_file_size: 5 * 1024 * 1024,  // 5MB
    max_file_count: Some(3),          // 3 files max
    retain_backups: false,            // Clean up on success
}
```

**For High-Throughput Applications**:
```rust
DiskBackupPolicy {
    max_file_size: 50 * 1024 * 1024, // 50MB
    max_file_count: Some(10),         // 10 files max
    retain_backups: false,            // Clean up on success
}
```

**For Development/Testing**:
```rust
DiskBackupPolicy {
    max_file_size: 1024 * 1024,      // 1MB
    max_file_count: Some(2),          // 2 files max
    retain_backups: true,             // Keep files for debugging
}
```

#### Monitoring Backup File Behavior

Key metrics to monitor for backup file optimization:

- **Checkpoint frequency**: How often checkpoints are triggered
- **Re-ingestion volume**: Amount of data re-ingested on failures
- **Storage usage**: Total disk space used by backup files
- **Recovery time**: Time to recover from failures

#### Best Practices

1. **Start Conservative**: Begin with smaller files and lower counts, then optimize based on observed behavior
2. **Monitor Re-ingestion**: Track how much data is being re-ingested to optimize file sizes
3. **Consider Network Conditions**: In unreliable network environments, prefer smaller files for faster recovery
4. **Balance Storage vs. Recovery**: Larger files reduce storage overhead but increase recovery time
5. **Test Failure Scenarios**: Validate that your configuration provides acceptable recovery times

## Error Handling and Recovery

### Error Scenarios

1. **gRPC Stream Failure**: 
   - Ingestion task sends `CheckpointNeedsReingestion`
   - Backup manager queues the current checkpoint's backup files for re-ingestion
   - Re-ingestion task processes backup files

2. **gRPC Re-Ingest Failure**:
   - Backoff retries for automated recovery
   - Data remains persisted to disk if all retries fail

3. **Channel Overflow**:
   - Data channels are bounded to prevent unbounded memory growth
   - Messages may be dropped if channels are full

### Retry Logic

- **Exponential Backoff**: Configurable retry policy with exponential backoff
- **Retry Counters**: Metrics track retry attempts and failures
- **Backup Recovery**: Failed data is automatically re-ingested from backup files

## Shutdown Process

The shutdown process ensures graceful termination with data preservation:

### Shutdown Sequence

1. **Shutdown Signal**: `Shutdown` control message sent to all tasks
2. **Channel Closure**: Data channels (`ingestion_tx`, `backup_tx`) are dropped
3. **Final Stream**: Ingestion task completes final gRPC stream
4. **Checkpoint Complete**: Final `CheckpointComplete` message sent
5. **Task Completion**: All tasks complete and return results
6. **Cleanup**: Backup files are cleaned up based on success/failure status

## Data Flow Architecture

### Normal Operation Flow

1. **User sends data** → `SiftStream::send()`
2. **Data validation** → Flow cache lookup
3. **Dual routing** → Both `ingestion_tx` and `backup_tx` channels
4. **Parallel processing**:
   - Ingestion task → gRPC stream → Sift
   - Backup task → Backup files
5. **Checkpoint completion** → Cleanup or re-ingestion

### Failure Recovery Flow

1. **gRPC failure** → `CheckpointNeedsReingestion` signal
2. **Backup manager** → Queues backup files for re-ingestion
3. **Re-ingestion task** → Reads backup files and re-streams to Sift
4. **Success** → Backup files deleted
5. **Failure** → Retry with exponential backoff

## Metrics and Monitoring

The system provides comprehensive metrics for monitoring:

- **Message counts**: Messages sent, received, dropped
- **Byte counts**: Bytes sent, received
- **Checkpoint metrics**: Checkpoint success/failure counts, timing
- **Retry metrics**: Retry counts, backoff timing
- **Backup metrics**: Backup file counts, sizes, rotation events

## Key Design Principles

1. **Fault Tolerance**: Multiple layers of error handling and recovery
2. **Data Reliability**: Backup system ensures no data loss
3. **High Throughput**: Bounded channels prevent memory issues
4. **Graceful Shutdown**: Clean termination with data preservation
5. **Observability**: Comprehensive metrics and logging
6. **Configurability**: Flexible configuration for different use cases

## Conclusion

The SiftStream architecture provides a robust, fault-tolerant system for streaming telemetry data to Sift.
The task-based design with control message coordination ensures reliable data delivery while maintaining high
performance and providing comprehensive error recovery mechanisms.
