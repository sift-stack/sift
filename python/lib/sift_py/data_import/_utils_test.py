import pytest

from sift_py.data_import import _utils


def test_convert_keys_to_snake_case_simple():
    obj = {"camelCase": 1, "PascalCase": 2}
    result = _utils.convert_keys_to_snake_case(obj)
    assert "camel_case" in result
    assert "pascal_case" in result


def test_convert_keys_to_snake_case_nested():
    obj = {"outerKey": {"innerKey": [{"deepKey": 1}, {"already_snake_case": 13}]}}
    result = _utils.convert_keys_to_snake_case(obj)
    assert "outer_key" in result
    assert "inner_key" in result["outer_key"]
    assert "deep_key" in result["outer_key"]["inner_key"][0]
    assert "already_snake_case" in result["outer_key"]["inner_key"][1]


def test_progress_file_zero_bytes(tmp_path):
    file = tmp_path / "empty.txt"
    file.write_text("")
    with pytest.raises(Exception) as excinfo:
        _utils.ProgressFile(file)
    assert "is 0 bytes" in str(excinfo.value)
