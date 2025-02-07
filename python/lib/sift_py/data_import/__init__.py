"""
This module contains services to facilitate importing data.
It also provides utilities to easily query the import status.

The fundamental components of this module are the following:
- `sift_py.data_import.config.CsvConfig`
- `sift_py.data_import.csv.CsvUploadService`
- `sift_py.data_import.status.DataImportService`


## Simple CSV Upload

A simple CSV upload without needing to craft a custom CSV config can be done like so:
```python
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.rest import SiftRestConfig

rest_config: SiftRestConfig = {
    "uri": sift_uri,
    "apikey": apikey,
}

asset_name = "Your Asset Name"
csv_upload_service = CsvUploadService(rest_config)
import_service: DataImportService  = csv_upload_service.simple_upload(asset_name, "sample_data.csv")

# Blocks until the import is completed.
import_service.wait_until_complete()
```

This example assumes several things about how the data is formatted. For example, that first column
contains datetime formatted time stamps. See docstring for `simple_upload` to see what can be overridden.

## TDMS Upload

Specify `sift-stack-py[tdms]` in your dependencies to use the TDMS upload service.
TDMS files can be uploaded like so:
```python
from sift_py.data_import.csv import TdmsUploadService
from sift_py.data_import.status import DataImportService
from sift_py.rest import SiftRestConfig

rest_config: SiftRestConfig = {
    "uri": sift_uri,
    "apikey": apikey,
}

asset_name = "Your Asset Name"
csv_upload_service = CsvUploadService(rest_config)
import_service: DataImportService  = csv_upload_service.simple_upload(asset_name, "sample_data.tdms")

# Blocks until the import is completed.
import_service.wait_until_complete()
```
If you want to upload TDMS groups prefixes to channel names set `prefix_channel_with_group` to True:
```python
csv_upload_service.simple_upload(asset_name, "sample_data.tdms", prefix_channel_with_group=True)
```

Some times there are TDMS channels without valid data or timing information, you can skip these channels by
setting `ignore_errors` to True:
```python
csv_upload_service.simple_upload(asset_name, "sample_data.tdms", ignore_errors=True)
```
The channels being skipped will be printed out to stdout.

## CSV Upload with custom CSV config

If your data is formatted a specific way you can create a CsvConfig that will be used to properly
parse your data:
```python
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.rest import SiftRestConfig
from sift_py.data_import.config import CsvConfig

rest_config: SiftRestConfig = {
    "uri": sift_uri,
    "apikey": apikey,
}

csv_upload_service = CsvUploadService(rest_config)

# Create CSV config.
input_csv = "sample_data.csv"

# Parse CSV to get channel names.
data_config = {}
with open(input_csv, "r") as f:
    reader = csv.DictReader(f)
    headers = next(reader)
    for i, channel in enumerate(headers):
        if channel == "timestamp":
            continue
        data_config[i + 1] = {
            "name": channel,
            # This example assumes all channels are doubles.
            # Can also use `ChannelDoubleType.DOUBLE` or `double`
            "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
            "description": f"Example channel {channel}",
        }

csv_config = CsvConfig(
    {
        "asset_name": asset_name,
        "first_data_row": 2,
        "time_column": {
            "format": "TIME_FORMAT_ABSOLUTE_DATETIME",
            # Can also use `TimeFormatType.ABSOLUTE_DATETIME`
            "column_number": 1,
        },
        "data_columns": data_config,
    }
)

import_service: DataImportService = csv_upload_service.upload(input_csv, csv_config)
import_service.wait_until_complete()
```

In this example the CSV can be created programmatically. You can also import use a json file directly:
```python
import json
from sift_py.data_import.config import CsvConfig

with open("config.json") as f:
    csv_config = CsvConfig(json.load(f))
```
"""
