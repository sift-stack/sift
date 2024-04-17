import sys
from dotenv import load_dotenv
import os
from sift.annotations.v1.annotations_pb2_grpc import AnnotationServiceStub
from sift.annotations.v1.annotations_pb2 import ListAnnotationsRequest
from util.grpc import create_authenticated_connection

if __name__ == "__main__":
    load_dotenv()
    API_KEY = os.getenv("SIFT_API_KEY")
    BASE_URI = os.getenv("BASE_URI")
    authorization_header = "authorization"
    authorization_value = f"Bearer {API_KEY}"

    if len(sys.argv) < 2:
        print("Please provide a name.")
        sys.exit(1)

    name = sys.argv[1]
    channel = create_authenticated_connection(BASE_URI, authorization_header, authorization_value)
    annotation_service = AnnotationServiceStub(channel)
    request = ListAnnotationsRequest(filter=f"name.matches(\"(?i){name}\")")
    response = annotation_service.ListAnnotations(request)
    print(response)
    channel.close()
