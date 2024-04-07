# Sift Protobufs Examples

Currently there are recommended installation instructions and example usages for the following languages:
- [Go](go/)
- [Rust](rust/)

Each example will cover how to manually compile the protobufs into the user's target language such that it's ready for immediate use in the user's project.
Users are by no means obligated to follow these instructions exactly and are instead encouraged to do what makes the most sense for their project.

In addition to covering installation the examples will also contain code that demonstrates how to use the compiled protobufs by building a basic CLI
that allows users to query annotations by name via a case-insensitive partial substring match:


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
