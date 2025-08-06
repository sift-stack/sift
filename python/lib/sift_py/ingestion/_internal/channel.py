from sift.rules.v1.rules_pb2 import ChannelReference


def channel_reference_from_fqn(fqn: str) -> ChannelReference:
    # Components are depreciated, so use full channel name in name
    return ChannelReference(name=fqn)
