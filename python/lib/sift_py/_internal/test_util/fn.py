from types import ModuleType
from typing import Callable


def _mock_path(subject_module: ModuleType) -> Callable[[Callable], str]:
    """
    Returns a function that can be used to conveniently generate the mock path
    for a function which could then be passed to `pytest_mock.MockFixture.patch`
    """

    def mock_fn(fn: Callable) -> str:
        return f"{subject_module.__name__}.{fn.__name__}"

    return mock_fn
