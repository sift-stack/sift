## grpclib vs. grpc
Betterproto uses grpclib instead of grpc for handling grpc messages.

|                      | grpclib                  | grpc      |
|----------------------|--------------------------|-----------|
| **async support**    | only async/await | only sync |
| **retry/keep alive** | built-in                 | custom    |


## Painpoints with betterproto
- pydantic breaks handling/serialization of non-provided values


## Betterproto2?
- python 3.10 and up
- compiler==0.4.0, 0.5.0 doesn't work
- Supports both sync and async
  - sync seems to use the original grpc library while async uses grpclib