## grpclib vs. grpc
Betterproto uses grpclib instead of grpc for handling grpc messages.

|                      | grpclib                  | grpc      |
|----------------------|--------------------------|-----------|
| **async support**    | only async/await | only sync |
| **retry/keep alive** | built-in                 | custom    |