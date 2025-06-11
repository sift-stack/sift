# Sift Client Internal Architecture

This directory contains internal implementation details for the Sift Client library. These components are not part of the public API and may change without notice.

## Event Loop Architecture

The Sift Client uses a dedicated event loop architecture for handling asynchronous gRPC operations. This document explains the design decisions and implications of this approach.

### Overview

The architecture consists of:

1. **Low-level clients**: Pure async implementations that wrap gRPC stubs
2. **High-level APIs**: Both sync and async versions that provide user-friendly interfaces
3. **Dedicated event loop**: A separate event loop in its own thread for gRPC async operations

### Design Decisions

#### Why a Dedicated Event Loop?

The gRPC async client requires that all operations (creating stubs and making calls) happen in the same event loop context. When using the client in different contexts (like inside `asyncio.run()` or in a Jupyter notebook), this can lead to the "Task got Future attached to a different loop" error.

To solve this, we create a dedicated event loop in a separate thread specifically for gRPC operations. This ensures that:

1. All gRPC async stubs are created in the same event loop
2. All gRPC async operations run in that same event loop
3. The client works reliably in any context (sync or async)

#### Low-Level vs High-Level APIs

- **Low-level clients** (`_internal/low_level_wrappers/`):
  - Pure async implementations
  - Direct mapping to gRPC services
  - Not intended for direct use by end users

- **High-level APIs** (`resources/`):
  - Both sync (`PingAPI`) and async (`PingAPIAsync`) versions
  - User-friendly interfaces with proper error handling
  - The sync version internally manages event loops to work with async low-level clients

### Implications

#### Performance Considerations

Using a dedicated event loop in a separate thread has some performance implications:

1. **Thread Overhead**: Additional memory and CPU usage for the extra thread
2. **Synchronization Cost**: Communication between event loops requires thread synchronization
3. **Concurrency Benefit**: gRPC operations can run concurrently with the main application

#### Usage in Different Contexts

1. **In a synchronous context**:
   - The sync API (`PingAPI`) handles all event loop management internally
   - Users don't need to worry about async/await or event loops

2. **In an asynchronous context**:
   - Users can use the async API (`PingAPIAsync`) directly with `await`
   - Operations still execute in the dedicated gRPC event loop, not in the caller's loop

3. **In a Jupyter notebook or interactive environment**:
   - The dedicated event loop approach ensures reliable operation
   - No "loop already running" errors when using the sync API

### Cleanup and Resource Management

The `GrpcClient` class handles proper cleanup of resources:

1. It registers an `atexit` handler to ensure channels are closed
2. It implements context manager protocol for both sync and async usage
3. It properly stops the dedicated event loop and joins the thread on cleanup

### Alternative Approaches Considered

1. **Event Loop Sharing**: Using the caller's event loop if one exists
   - More efficient but requires careful management
   - Difficult to ensure reliability in all contexts

2. **Fully Synchronous Low-Level API**: Using only sync gRPC stubs
   - Simpler but less efficient
   - Would require duplicating code for async versions

3. **Thread-Local Event Loops**: Using thread-local storage for event loops
   - More complex implementation
   - Potential for resource leaks

The dedicated event loop approach was chosen as the best balance between reliability, performance, and code maintainability.
