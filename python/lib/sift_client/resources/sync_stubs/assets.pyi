# Generated stubs for sift_client.resources.assets

from sift_client.resources.assets import *

class AssetsAPI:
    """Sync counterpart to `AssetsAPI`.

    High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, client: 'GrpcClient') -> None: ...
    
    def list_assets(self, limit: int = 100, offset: int = 0) -> 'List[Asset]':
        """List all assets.
        
        Args:
            limit: Maximum number of assets to return.
            offset: Number of assets to skip.
            
        Returns:
            List of assets.
        """
        ...
    
    # Add other methods as needed based on the original AssetsAPI class
