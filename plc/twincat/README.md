# Beckhoff TwinCAT3 Sift Client Library

This TwinCAT 3 library provides a framework for streaming data from the [Beckhoff](https://www.beckhoff.com/en-us/) TwinCAT3 environment to [Sift](https://siftstack.com) in real-time from a PLC. It offers functionality for configuring flows, defining data channels, and sending structured messages efficiently.

## Requirements

This library depends on the following TwinCAT modules:

- `Tc2_Standard`
- `Tc2_System`
- `Tc2_TcpIp`
- `Tc2_Utilities`
- `Tc3_JsonXml`
- `Tc3_Module`

Ensure these libraries are installed and referenced in your project before using this package.

## Examples

Example usages are provided in the `examples` directory. These include:

- `Ping` - To verify everything is installed correctly and credentials are valid.
- `Ingestion` - Shows how to stream data from a PLC application. 
