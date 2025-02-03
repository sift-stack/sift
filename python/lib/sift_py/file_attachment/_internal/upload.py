import mimetypes
from pathlib import Path
from typing import Any, Dict, Optional, Tuple, Union
from urllib.parse import urljoin

from requests_toolbelt import MultipartEncoder

from sift_py._internal.convert.json import to_json
from sift_py.file_attachment.entity import Entity
from sift_py.file_attachment.metadata import Metadata
from sift_py.rest import SiftRestConfig, _RestService


class UploadService(_RestService):
    """
    Service used to upload attachments.
    """

    UPLOAD_PATH = "/api/v0/remote-files/upload"
    UPLOAD_BULK_PATH = "/api/v0/remote-files/upload:bulk"

    _upload_uri: str
    _upload_bulk_uri: str
    _apikey: str

    def __init__(self, rest_conf: SiftRestConfig):
        super().__init__(rest_conf=rest_conf)
        self._upload_uri = urljoin(self._base_uri, self.UPLOAD_PATH)
        self._upload_bulk_uri = urljoin(self._base_uri, self.UPLOAD_BULK_PATH)

    def upload_attachment(
        self,
        path: Union[str, Path],
        entity: Entity,
        metadata: Optional[Metadata] = None,
        description: Optional[str] = None,
        organization_id: Optional[str] = None,
    ) -> str:
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        file_name, mimetype, content_encoding = self.__class__._mime_and_content_type_from_path(
            posix_path
        )

        if not mimetype:
            raise Exception(f"The MIME-type of '{posix_path}' could not be computed.")

        with open(path, "rb") as file:
            form_fields: Dict[str, Any] = {
                "entityId": entity.entity_id,
                "entityType": entity.entity_type.value,
            }

            if content_encoding:
                form_fields["file"] = (
                    file_name,
                    file,
                    mimetype,
                    {
                        "Content-Encoding": content_encoding,
                    },
                )
            else:
                form_fields["file"] = (file_name, file, mimetype)

            if metadata:
                form_fields["metadata"] = to_json(metadata)

            if organization_id:
                form_fields["organizationId"] = organization_id

            if description:
                form_fields["description"] = description

            form_data = MultipartEncoder(fields=form_fields)

            # https://github.com/requests/toolbelt/issues/312
            # Issue above is reason for the type ignoring
            response = self._session.post(
                url=self._upload_uri,
                data=form_data,  # type: ignore
                headers={
                    "Content-Type": form_data.content_type,
                },
            )

            if response.status_code != 200:
                raise Exception(
                    f"Request failed with status code {response.status_code} ({response.reason})."
                )

            return response.json().get("remoteFile").get("remoteFileId")

    @staticmethod
    def _mime_and_content_type_from_path(path: Path) -> Tuple[str, Optional[str], Optional[str]]:
        file_name = path.name
        mime, encoding = mimetypes.guess_type(path)
        return file_name, mime, encoding
