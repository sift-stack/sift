import os
from datetime import datetime

from dotenv import load_dotenv
from sift_py.asset.service import AssetService
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel


def update_asset_metadata():
    """
    Updates the NostromoLV426 asset with test metadata.
    """
    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    if apikey is None:
        raise Exception("Missing 'SIFT_API_KEY' environment variable.")

    base_uri = os.getenv("BASE_URI")
    if base_uri is None:
        raise Exception("Missing 'BASE_URI' environment variable.")

    # Use the asset name from an ingestion example (here from ingestion_with_python_config).
    # Run that example to populate a run w/ data assigned to the example asset.
    asset_name = "NostromoLV426"

    # Create a gRPC transport channel configured specifically for the Sift API
    sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)

    with use_sift_channel(sift_channel_config) as channel:
        # Create asset service
        asset_service = AssetService(channel)

        # Get the asset by name
        assets = asset_service.list_assets(names=[asset_name])
        if not assets:
            raise Exception(f"Asset '{asset_name}' not found")

        asset = assets[0]
        print(f"Found asset {asset.name} with ID {asset.asset_id}")

        # Update a specific metadata field.
        asset.metadata["test_boolean"] = True

        # You can update existing metadata with a dict.
        timestamp = datetime.now()
        test_metadata = {
            "test_string": f"updated at {timestamp.isoformat()}",
            "test_number": timestamp.timestamp(),
        }
        # Note: If you assign the metadata to a dict vs updating it, all existing metadata not in the new dict is lost.
        asset.metadata.update(test_metadata)

        updated_asset = asset_service.update_asset(asset=asset, update_metadata=True)

        print(f"Successfully updated asset {updated_asset.name} with test metadata")


if __name__ == "__main__":
    update_asset_metadata()
