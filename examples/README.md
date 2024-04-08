# Sift Protobufs Examples

This directory contains examples on how to use code generated from Sift protobufs for various programming languages.

Each example is a small CLI that allows users to query annotations by doing a case-insensitive substring match against the name of each annotation which
will behave like so:

```
$ sift_cli voltage
Annotation ID: c6810746-6f3a-47bd-8f03-bf008e7ca983
        Name: Voltage threshold
        Description: Generated from an automated rule
        State: ANNOTATION_STATE_OPEN
        Type: ANNOTATION_TYPE_DATA_REVIEW
        Created at: 2023-08-18 18:25:42.754271 +0000 UTC
        Modified at: 2023-08-25 23:02:04.692864 +0000 UTC
        Created by rule condition ID:

Annotation ID: 3279649b-158c-4cc9-b6a4-731726f39047
        Name: Voltage threshold
        Description: Generated from an automated rule
        State: ANNOTATION_STATE_OPEN
        Type: ANNOTATION_TYPE_DATA_REVIEW
        Created at: 2023-08-18 18:25:43.699856 +0000 UTC
        Modified at: 2023-08-25 23:02:04.692864 +0000 UTC
        Created by rule condition ID:

...
```
