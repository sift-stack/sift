import os
from pathlib import Path

from dotenv import load_dotenv
from sift_py.calculated_channels.service import CalculatedChannelService
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel


def calculated_channel_demo():
    load_dotenv()
    channel_config: SiftChannelConfig = {
        "apikey": os.getenv("SIFT_API_KEY", ""),
        "uri": os.getenv("BASE_URI", ""),
    }
    config_path = Path(__file__).parent / "config_examples" / "calculated_channels.yml"

    with use_sift_channel(channel_config) as channel:
        service = CalculatedChannelService(channel)

        results = service.create_or_update_calculated_channel_from_yaml(
            config_path,
        )
        print(results)


if __name__ == "__main__":
    calculated_channel_demo()
