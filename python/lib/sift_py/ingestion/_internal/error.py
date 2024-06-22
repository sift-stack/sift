class IngestionValidationError(Exception):
    """
    Errors that can occur while initializing the ingestion service
    or when creating ingestion requests.
    """

    message: str

    def __init__(self, message: str):
        super().__init__(message)
