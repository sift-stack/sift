import sys
from dotenv import load_dotenv
import grpc
import os
from sift.annotations.v1.annotations_pb2_grpc import AnnotationServiceStub
from sift.annotations.v1.annotations_pb2 import ListAnnotationsRequest

if __name__ == "__main__":
    load_dotenv()
    API_KEY = os.getenv("SIFT_API_KEY")
    BASE_URI = os.getenv("BASE_URI")

    if len(sys.argv) < 2:
        print("Please provide a name.")
        sys.exit(1)

    name = sys.argv[1]

    credentials = grpc.ssl_channel_credentials()
    call_credentials = grpc.access_token_call_credentials(API_KEY)
    composite_credentials = grpc.composite_channel_credentials(
        credentials, call_credentials
    )

    with grpc.secure_channel(BASE_URI, composite_credentials) as channel:
        annotation_service = AnnotationServiceStub(channel)
        request = ListAnnotationsRequest(filter=f'name.matches("(?i){name}")')
        response = annotation_service.ListAnnotations(request)
        print(response)
        channel.close()
