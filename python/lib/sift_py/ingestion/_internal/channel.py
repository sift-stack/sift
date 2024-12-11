from sift.rules.v1.rules_pb2 import ChannelReference


def channel_reference_from_fqn(fqn: str) -> ChannelReference:
    parts = fqn.split(".")

    if len(parts) == 1:
        return ChannelReference(name=parts[0])

    component_parts = parts[: len(parts) - 1]

    return ChannelReference(name=parts[-1], component=".".join(component_parts))
