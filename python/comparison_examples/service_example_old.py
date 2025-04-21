import os

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig

load_dotenv()
apikey = os.getenv("SIFT_API_KEY")
uri = os.getenv("BASE_URI")
channel_config: SiftChannelConfig = {
    "apikey": apikey,
    "uri": uri,
}

from sift.views.v2.views_pb2 import CreateViewRequest, View
from sift.views.v2.views_pb2_grpc import ViewServiceStub
from sift_py.grpc.transport import use_sift_channel

with use_sift_channel(channel_config) as channel:
    view_service = ViewServiceStub(channel)
    response = view_service.CreateView(
        CreateViewRequest(
            view=View(
                name="test view2",
                channels=[View.Channel(name="voltage", data_type="double", axis_group="left1")],
                axis_groups=View.AxisGroups(left=["left1"], right=[]),
            ),
        )
    )
    print(response)
