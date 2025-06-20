import os

from dotenv import load_dotenv
from sift_client.client import SiftClient


def main():
    sift = SiftClient(
        api_key=apikey,
        rest_url="https://api.development.siftstack.com",
        grpc_url="https://grpc-api.development.siftstack.com",
    )

    # Find an asset
    asset = sift.assets.find(name_contains="Nostromo")
    print(asset.tags)
    asset.update(dict(tags=["simulator", "my_new_tag"]))
    print(asset)


if __name__ == "__main__":
    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "Missing 'SIFT_API_KEY' environment variable."

    main()
