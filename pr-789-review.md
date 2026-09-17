# PR 789: python(feat): webhooks resource

Adds a `webhooks` resource to `sift_client` (low-level wrapper, Pydantic types, async
API, sync stubs) and reworks `RuleAction` so a rule can call a webhook on violation.

Head SHA: `2baa0a690945f535b667943742e2916e0ab915ff`

## Conventions applied

- `python/lib/sift_client/_internal/CONTRIBUTING.md` (registered) — keyword-only optional
  arguments, standard `get`/`list_`/`find`/`create`/`update` signatures, Create/Update
  model inheritance, reference arguments accepting object or ID.
- `python/lib/sift_client/_internal/README.md` (registered) — transport / low-level /
  types / resources layering. The new code sits correctly in all four layers.
- `python/CHANGELOG.md` (discovered) — Keep-a-Changelog format with an established
  "Breaking change:" / `#### Breaking Changes` convention (see lines 181 and 400).

## Findings

### 1. critical - python/lib/sift_client/sift_types/rule.py:288, 384-387
`RuleAction.tags` was a property that fetched and returned `list[Tag]`. It is now a field
holding tag ID strings, and the fetching property is renamed `resolved_tags`. Existing
code keeps type-checking and keeps running — `action.tags` is still a list — and fails
only when it touches an element: `[t.name for t in action.tags]` raises
`AttributeError: 'str' object has no attribute 'name'` on any action read back from the
API. Nothing in the PR shims it and the CHANGELOG does not mention it.

The new shape is right: `Asset.tags` is already `list[str | Tag]` populated with strings
from the proto, so this makes `RuleAction` match. The break is that the old name kept
working with a different meaning.

**Fix:** Keep `tags` as the deprecated fetching property for one release and name the new
field `tag_refs` (or land the rename with a `DeprecationWarning` shim on the old
property), then record it under `#### Breaking Changes` in the Unreleased section.

Source: surface - public SDK type, rendered in the published API reference
Source: exemplar - sift_types/asset.py:29,105 (field holds strings, not Tag objects)
Confidence: 95

### 2. important - python/CHANGELOG.md:8-12
Three more public renames ship unannounced: `RuleAction.annotation` → `for_annotation`,
`tags_ids` → `tags`, `default_assignee_user` → `assignee`. Every caller of
`RuleAction.annotation(...)` — the documented way to build a rule action — breaks with
`AttributeError` on upgrade, and the Unreleased entry describes only the additions. The
CHANGELOG already has a convention for this: v0.21.0 opens with "Breaking change:
`client.reports.create_from_template` now takes …" (line 181) and there is a
`#### Breaking Changes` heading at line 400.

**Fix:** Add a `#### Breaking Changes` block to Unreleased listing all four renames
(including `tags` from finding 1) with the before/after call, and keep `annotation` as a
deprecated alias delegating to `for_annotation`.

Source: convention - python/CHANGELOG.md, "Breaking change" entries at lines 181 and 400
Source: surface - public SDK classmethod
Confidence: 90

### 3. important - python/lib/sift_client/resources/webhooks.py:189-194
`test(webhook=None, *, create=None)` makes one of two mutually exclusive optional
arguments positional. CONTRIBUTING: "user-facing methods that use optional arguments
should use keyword-only arguments … This allows us to evolve method signatures without
breaking backwards compatibility." Every other method in the resource obeys it, and all
three existing either/or methods put both arguments behind `*`: `assets.get` (both
keyword-only, `ValueError` when neither is given), `runs.get`, and
`reports.wait_until_complete`. `RuleAction.for_annotation` (rule.py:302-306) has the same
problem with its optional `assignee`.

**Fix:** Move `*` ahead of `webhook` in `test`, and ahead of `assignee` in
`for_annotation`. Both are new surfaces, so nothing else needs updating.

```suggestion
    async def test(
        self,
        *,
        webhook: str | Webhook | None = None,
        create: WebhookCreate | dict | None = None,
    ) -> WebhookTestResult:
```
Source: convention - _internal/CONTRIBUTING.md, "Keyword Only Arguments"
Source: exemplar - resources/assets.py:38, runs.py:37, reports.py:391 (3 of 3)
Confidence: 85

### 4. nice-to-have - PR description
The description advertises `RuleAction.webhook(...)`. The method is `for_webhook(...)` —
`webhook` is now a field on the class, so the classmethod could not keep that name. A
reviewer reading the description tries a call that does not exist.

**Fix:** Update the description to `RuleAction.for_webhook(...)` and add a line recording
the `Ref` decision from finding 5.

Source: surface - PR description is the reviewer's entry point
Confidence: 90

### 5. nice-to-have - python/lib/sift_client/sift_types/_refs.py:23-25, rule.py:288-306
`Ref[T]` is `Union[T, str]` under a new name. It is identical to the inline union for both
Pydantic and the type checker, so it buys nothing at the type level, and it lives in a
private module while appearing in public signatures — the generated API reference renders
`for_webhook(webhook: Ref[Webhook])`, which tells a reader nothing about what to pass.
The house spelling is inline: `list[str | Tag]` (asset.py:29 and 116, report.py:185,
report_template.py:213), `list[str | Rule]` (report_template.py:190), `CalculatedChannel |
str` (channel.py:445), and CONTRIBUTING's own example is `async def update(self, asset:
Asset | str, ...)`. Six fields across four files, none using an alias.

`resolve_id` / `resolve_ids` are the part that earns its place — keep them.

**Fix:** Spell the fields and arguments `Webhook | str`, `list[str | Tag]`, `User | str`,
drop the `Ref` alias, and keep `_refs.py` for the resolvers.

Source: convention - _internal/CONTRIBUTING.md, "Resource Method Patterns" example
Source: exemplar - 6 fields across asset.py, report.py, report_template.py, channel.py
Confidence: 75

### 6. nice-to-have - python/lib/sift_client/sift_types/webhook.py:163-169
`WebhookBase._validate_target_url` reads its own field through
`getattr(self, "target_url", None)` because `target_url` is declared on `WebhookCreate`
and `WebhookUpdate` instead of on the base. CONTRIBUTING puts "all shared field
definitions" on the domain base class; the escape hatch means a future typo in a subclass
field name silently disables the validator rather than failing.

**Fix:** Declare `name: str | None = None` and `target_url: str | None = None` on
`WebhookBase` and narrow them to required on `WebhookCreate`, then read `self.target_url`
directly.

Source: convention - _internal/CONTRIBUTING.md, "Domain-Specific Base Classes"
Confidence: 70

### 7. nice-to-have - python/lib/sift_client/sift_types/webhook.py:80, 144
`payload` is a public create/update field with no documentation anywhere — not on the
model, not in the `create` docstring, and not in the proto. A caller cannot tell whether
it is a template, a static body, or a JSON blob, or what happens when it is omitted.
`http_headers`, by contrast, is documented at both the field and the method.

**Fix:** Document `payload` on `WebhookBase` with one sentence and an example value.

Source: surface - public SDK field with no docstring
Confidence: 60

## Uncertain

- `_internal/low_level_wrappers/webhooks.py:143-161` — `update_webhook` fetches the
  webhook and merges the patch into it. If the service redacts header values on read
  (a common treatment for `Authorization`-style headers), any update that does not set
  `http_headers` writes the redacted values back. I could not check what the service
  returns.
- `_internal/low_level_wrappers/webhooks.py:156` — the same read-modify-write loses a
  concurrent update to an unmasked field. Probably acceptable given the service validates
  the whole message, but it is a behaviour the docstring does not mention.

## Not raised

- `sift_types/rule.py:277` (alexluck-sift, unresolved/outdated) asked for the tags type to
  follow the `to_proto` pattern rather than carrying the debt forward. Commit `2baa0a6`
  does that; findings 1 and 2 are about the compatibility cost of the fix, not the fix.
- No unit test for `update_webhook`'s fetch-and-merge, the one piece of non-trivial logic
  in the PR. `_tests/resources/test_webhooks.py::test_update` covers it, but only against
  a live stack. Ten resources have low-level unit tests and eight do not, so this is not a
  convention break.
- `WebhookService` also exposes signature-key and webhook-log RPCs that the resource does
  not wrap. Out of scope for this PR; worth a follow-up, since signature keys are how a
  receiver verifies a delivery came from Sift.

## Undocumented conventions

1. **Repo, established** — `_internal/CONTRIBUTING.md`, under "Resource Method Patterns".
   The doc requires object-or-ID only for *method arguments*. Six model fields across four
   files already do the same thing (asset.py:29,116; report.py:185;
   report_template.py:190,213; channel.py:445), which is what finding 5 rests on, and no
   document says so.
   > Fields on domain, create, and update models that reference another Sift type accept
   > the object or its ID, spelled inline as `Thing | str` or `list[str | Thing]`. Do not
   > introduce an alias for the union.
2. **Repo, established** — same file, under "Keyword Only Arguments". Three methods
   (`assets.get`, `runs.get`, `reports.wait_until_complete`) implement mutually exclusive
   arguments the same way and none is named as the pattern.
   > Where two arguments are mutually exclusive, make both keyword-only and raise
   > `ValueError` when neither or both are given.
3. **Repo, established** — `python/CHANGELOG.md` has no stated rule for breaking changes,
   but two releases use one (lines 181 and 400). Add a line to the file's header, or to
   the Python CONTRIBUTING, requiring a `#### Breaking Changes` entry naming the old and
   new call for any public rename.
4. **Skill registry** — the registry's `Exemplars` cell names `resources/_base.py` and
   `sift_types/_base.py`. Add `resources/assets.py` as the exemplar for a full CRUD
   resource: the new `webhooks.py` mirrors it line for line (filter assembly, the
   `page_size` kwarg splat, `find`, `update`, `unarchive` via update), and it is what a
   next audit would need to measure against.
