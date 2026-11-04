Fixed file uploads from an `io.BytesIO` that is not at position 0, so they send the bytes from the current position, as uploads from an open file do.
