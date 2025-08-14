import logging
import sys

from sift_client.client import SiftClient
from sift_client.transport import SiftConnectionConfig

__all__ = [
    "SiftClient",
    "SiftConnectionConfig",
]

logger = logging.getLogger(__name__)
logging.basicConfig(
    level=logging.ERROR, format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)


handler = logging.StreamHandler(sys.stdout)
logger.addHandler(handler)

# TODO: Remove
logger.setLevel(logging.DEBUG)
