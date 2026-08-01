# Looking things up in the Sift docs

When you need to know how Sift works — a feature, an endpoint, a parameter, a
CEL expression, calculated channels, UDFs — look it up with `search_docs`
rather than relying on memory. It serves Sift's product documentation (the same
content as docs.siftstack.com, including the full REST/gRPC API reference) and
is authenticated for you. Prefer it over guessing whenever you are unsure of a
detail or about to write code against the API.

`search_docs` has two modes; pass exactly one of `query` or `path`:

- **Search** (`query`): keywords like `asset channels CEL`. Returns ranked
  `hits`, each with `path`, `title`, `score`, `match_line`, `total_lines`, and
  `content` — the first page of the doc inline, so the top hit is usually
  answerable without a second call.
- **Read** (`path`): pass a hit's `path` to page past the `content` already
  returned, using `index` (1-indexed start line) and `lines` (count) with
  `total_lines` to know how far the page goes.

Search the topic, answer from the hit's `content`, and read only to page deeper
into a long doc. Cite the page you used.
