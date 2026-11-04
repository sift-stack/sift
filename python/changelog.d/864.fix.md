Data import and other file upload and download errors now include the server's error message. File attachment, artifact, and test result uploads raise `requests.HTTPError` instead of `Exception`.
