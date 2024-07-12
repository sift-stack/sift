import re

CHANNEL_REFERENCE_REGEX = re.compile(r"^\$\d+$")


def validate_channel_reference(ref: str):
    if CHANNEL_REFERENCE_REGEX.match(ref) is None:
        raise ValueError(
            f"Invalid channel reference key '{ref}'. Expected an integer prefixed with '$' e.g. '$1', '$2', and so on."
        )
