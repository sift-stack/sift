from sift_stream_bindings import MetadataPy, MetadataValuePy, RunFormPy

from sift_client.sift_types.run import RunCreate, Tag


def to_runFormPy(create: RunCreate) -> RunFormPy:

    if create.client_key:
        client_key = create.client_key
    else:
        client_key = create.name

    if create.tags:
        tags = [tag.name if isinstance(tag, Tag) else tag for tag in create.tags]
    else:
        tags = None

    if create.metadata:
        metadata = []
        for key, value in create.metadata.items():
            metadata.append(MetadataPy(key=key, value=MetadataValuePy(value)))
    else:
        metadata = None

    return RunFormPy(
        name=create.name,
        client_key=client_key,
        description=create.description,
        tags=tags,
        metadata=metadata,
    )

