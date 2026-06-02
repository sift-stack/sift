"""Placeholder for a future ``MetadataMixin`` (not yet implemented).

TODO(metadata-mixin): metadata updates REPLACE the whole map.
``entity.update({"metadata": {...}})`` builds a field mask over ``metadata``
(see ``ModelUpdate.to_proto_with_mask`` in ``sift_types/_base.py``) and replaces
it server-side — callers must spread the current ``.metadata`` first or silently
drop existing keys (config defaults, git fields, ``pytest_command``).

Planned shape: a ``MetadataMixin`` exposing a read-merge-write helper such as
``add_metadata(**kv)`` / ``merge_metadata(dict)``, implemented as
``self.update({"metadata": {**self.metadata, **kv}})``. Mix into every read
entity that carries a ``metadata`` field — ``Asset``, ``Run``, ``Report``,
``TestReport``, ``TestStep``, ``TestMeasurement`` — alongside
``FileAttachmentsMixin`` and ``SimulatedMixin``. It stays a mixin (not a
``BaseType`` method) because it relies on the ``metadata`` field, which not
every ``BaseType`` subclass has (e.g. ``CalculatedChannel`` exposes metadata
only on its Create/Update models, so it is out of scope). Until it exists,
merge at the call site.
"""
