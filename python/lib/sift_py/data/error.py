class DataError(Exception):
    msg: str

    def __init__(self, msg: str):
        super().__init__(msg)
