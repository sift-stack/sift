# PR 789 — proposed review comments

One review, `event: COMMENT`, on head SHA `2baa0a690945f535b667943742e2916e0ab915ff`.
Six inline comments plus a body. Every anchor verified to fall inside a diff hunk on the
RIGHT side. Confidence scores stripped, as phase 2 requires.

Not posted. Nothing goes to GitHub until you say so.

---

## Review body

> Audit against `sift_client/_internal/CONTRIBUTING.md`, consistency with the existing
> resources, and the public SDK surface. One blocking item: the `RuleAction.tags`
> semantics change. Separately, the description advertises `RuleAction.webhook(...)` —
> the method is `for_webhook(...)`, since `webhook` is now a field on the class.

The PR-description point has no line to anchor to, so it rides in the body.

---

## 1. `python/lib/sift_client/sift_types/rule.py:288` — RIGHT

> [!CAUTION]
> `tags` was a property returning a fetched `list[Tag]`; it is now a field of ID strings,
> with the fetching moved to `resolved_tags`. Callers keep running and break only on
> element access: `[t.name for t in action.tags]` raises `AttributeError: 'str' object has
> no attribute 'name'` for any action read back from the API. No shim, and no CHANGELOG
> entry.
>
> Matching `Asset.tags` (already `list[str | Tag]` holding strings) is the right shape.
> Reusing the old name with a new meaning is the problem.

**Fix:** Name the field `tag_refs` and keep `tags` as the fetching property with a
`DeprecationWarning` for one release.

## 2. `python/CHANGELOG.md:12` — RIGHT

> [!WARNING]
> Three public renames ship unannounced: `RuleAction.annotation` → `for_annotation`,
> `tags_ids` → `tags`, `default_assignee_user` → `assignee`. Every caller of
> `RuleAction.annotation(...)` breaks with `AttributeError` on upgrade. v0.21.0 records
> this kind of change ("Breaking change: `client.reports.create_from_template` now takes
> …"), and the file has a `#### Breaking Changes` heading further down.

**Fix:** Add a `#### Breaking Changes` block to Unreleased giving the before and after
call for all four renames, and keep `annotation` as a deprecated alias.

## 3. `python/lib/sift_client/resources/webhooks.py:189-194` — RIGHT

> [!WARNING]
> `webhook` is optional but positional, which locks its position into the public
> signature. CONTRIBUTING requires keyword-only optional arguments on user-facing methods,
> and all three existing either/or methods put both arguments behind `*`: `assets.get`,
> `runs.get`, `reports.wait_until_complete`.

```suggestion
    async def test(
        self,
        *,
        webhook: str | Webhook | None = None,
        create: WebhookCreate | dict | None = None,
    ) -> WebhookTestResult:
```

Safe to one-click: the only in-repo caller, `Webhook.test()` at `sift_types/webhook.py:137`,
already passes `webhook=self`.

## 4. `python/lib/sift_client/sift_types/rule.py:306` — RIGHT

> [!WARNING]
> Same as the note on `webhooks.py` `test()`: `assignee` is optional but positional, which
> locks its position into the public signature. CONTRIBUTING requires keyword-only
> optional arguments on user-facing methods.

```suggestion
        *,
        assignee: Ref[User] | None = None,
```

Single-line range. Lines 303-304 are unchanged, so a wider range would be rejected.

## 5. `python/lib/sift_client/sift_types/_refs.py:24` — RIGHT

> [!TIP]
> `Ref[T]` is `Union[T, str]` under a new name — the same type to both Pydantic and the
> type checker — and it lives in a private module while appearing in public signatures, so
> the generated API reference renders `for_webhook(webhook: Ref[Webhook])`, which does not
> tell a caller what to pass. The house spelling is inline: `list[str | Tag]` in asset.py,
> report.py and report_template.py, `CalculatedChannel | str` in channel.py, and
> CONTRIBUTING's own example is `asset: Asset | str`.

**Fix:** Spell the fields and arguments `Webhook | str` and `list[str | Tag]`, drop the
alias, and keep `resolve_id` and `resolve_ids`.

## 6. `python/lib/sift_client/sift_types/webhook.py:166` — RIGHT

> [!TIP]
> `_validate_target_url` reads its own field through `getattr` because `target_url` is
> declared on the subclasses rather than on `WebhookBase`, so a typo in a subclass field
> name would silently disable the validator instead of failing. CONTRIBUTING puts all
> shared field definitions on the domain base class.

**Fix:** Declare `name` and `target_url` on `WebhookBase` as optional, narrow them to
required on `WebhookCreate`, and read `self.target_url` directly.

## 7. `python/lib/sift_client/sift_types/webhook.py:144` — RIGHT

> [!TIP]
> `payload` is a public create and update field with no documentation on the model, in the
> `create` docstring, or in the proto. A caller cannot tell whether it is a template, a
> static body, or JSON, or what happens when it is omitted.

**Fix:** Document `payload` with one sentence and an example value.

---

## Notes before posting

- **Comments 4 and 5 conflict as patches.** If the author takes 5 and drops `Ref`, the
  suggestion in 4 no longer applies as written. 4 is anchored to a line 5 would rewrite.
  Posting both is fine — they read as one direction — but do not expect both one-click
  buttons to work in sequence.
- **Cut candidates if seven feels long.** 7 (`payload`) was the weakest finding in the
  audit; 6 is a small internal tidy on a file nobody outside the package reads. Cutting
  both leaves the two breaking-change comments, the two keyword-only ones, and `Ref`.
- **Nothing from the Uncertain list is proposed.** Both items depend on service behaviour
  I could not check, and a comment that opens with a guess costs more than it returns.
- **The prior thread at rule.py:277 is not re-raised.** Commit `2baa0a6` addresses it.
