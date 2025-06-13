import asyncio
import os

from dotenv import load_dotenv
from sift_client.client import SiftClient


async def main(client):
    print("Async in async loop:", await sift.ping_async.ping())
    print("Sync in async loop:", sift.ping.ping())


if __name__ == "__main__":
    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "Missing 'SIFT_API_KEY' environment variable."

    sift = SiftClient(
        api_key=apikey,
        rest_url="https://api.development.siftstack.com",
        grpc_url="https://grpc-api.development.siftstack.com",
    )

    asyncio.run(main(sift))

    print("Sync:", sift.ping.ping())
