# Contributing

## Style Guidelines

### Docstrings

Google style docstrings: https://google.github.io/styleguide/pyguide.html

### Type Hinting

Use Python 3.10+ style type hinting, but maintain backwards compatibility with Python 3.8. This
is possible by using `eval_type_backport` which converts 3.10+ type hints to 3.8+ type hints by
overriding the behavior of `typine._val_type`.

* Always use `from __future__ import annotations` at the top of the file to help with Python 3.8 backwards
  compatibility.
* Use 3.10+ type hints style for built-in types. That is use `list` instead of `List` and `dict` instead of `Dict`, use
  `|` instead of `Union` etc.
* Instead of `Optional[str]` use `str | None`. PEP 484 now discourages implicit `None` in type hints.

### Keyword Only Arguments

To improve continued compatibility with user-code, user-facing methods should use keyword-only arguments. This allows
us to evolve method signatures without breaking backwards compatibility.

This can be done by adding a `*` to the argument list, e.g.:

```python
def foo(self, *, b):
    pass
```

Users will then be required to do `obj.foo(b=1)` instead of `obj.foo(1)`.