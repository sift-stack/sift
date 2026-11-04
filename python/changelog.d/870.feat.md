Added `ArtifactVersion.file_sha256`. Artifact uploads now send the SHA-256 of the file, and each version returns it, so a caller can skip uploading an unchanged file.
