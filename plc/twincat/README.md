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

## Installation

Download the release zip which includes the `Sift Client.library` library file, `sift_proxy` binary, and various scripts. After extracting the contents on your PLC run `install_sift_proxy.bat` to install `sift_proxy` then install `Sift Client.library`.

Note: While this library is in beta it depends on a special version of `sift_proxy`. Reach out to the Sift team for a copy and installation instructions.

## Examples

Example usages are provided in the `examples` directory. These include:

- `Ping` - To verify everything is installed correctly and credentials are valid.
- `Ingestion` - Shows how to stream data from a PLC application. 
