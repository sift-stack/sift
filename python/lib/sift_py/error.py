class SiftError(Exception):
    """
    These exceptions are raised when something totally unexpected occurs and is
    meant to indicate that the error is likely not caused by the user, but rather,
    the library itself. These errors should be reported to Sift.
    """

    msg: str

    def __init__(self, msg: str):
        super().__init__(f"{msg}\nPlease notify Sift.")

class SiftAPIDeprecationWarning(FutureWarning):
    """
    Warning used for deprecated API features that may be removed in future updates.
    """
    ...

