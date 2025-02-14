import os

from dotenv import load_dotenv
from sift_py.calculated_channels.config import CalculatedChannelConfig
from sift_py.calculated_channels.service import CalculatedChannelService
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel


def calculated_channel_demo():
    load_dotenv()
    channel_config: SiftChannelConfig = {
        "apikey": os.getenv("SIFT_API_KEY", ""),
        "uri": os.getenv("BASE_URI", ""),
    }

    with use_sift_channel(channel_config) as channel:
        service = CalculatedChannelService(channel)

        config = CalculatedChannelConfig(
            name="My-Calculation",
            description="A description",
            expression="$1 * 50",
            channel_references=[
                dict(channel_reference="$1", channel_identifier="mainmotor.velocity")
            ],
            asset_ids=["c8c724b9-6993-4423-9cc4-4c4ecdbc5751"],
        )
        chan, validation = service.create_calculated_channel(
            config,
        )
        print(chan, validation)


if __name__ == "__main__":
    calculated_channel_demo()
