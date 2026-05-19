# sift_test_util — guidance for Claude

This crate exposes `mockall`-generated mocks for `sift_rs` gRPC service traits. When the user asks you to "create a mock for `sift_rs::<path>::<ServiceTrait>`", follow the recipe below.

## Reference example

`src/mock/assets/v1.rs` is the canonical example. It mocks `sift_rs::assets::v1::asset_service_server::AssetService`. Mirror its structure exactly when adding a new mock.

## Recipe: adding a mock for a `sift_rs` service trait

Given a request like *"create me a mock for `sift_rs::assets::v1::asset_service_server::AssetService`"*:

### 1. Locate the generated trait

The trait lives in `rust/crates/sift_rs/src/gen/sift/<service>/<version>/sift.<service>.<version>.tonic.rs`. For example:

- `sift_rs::assets::v1::asset_service_server::AssetService` → `rust/crates/sift_rs/src/gen/sift/assets/v1/sift.assets.v1.tonic.rs`
- `sift_rs::ingest::v1::ingest_service_server::IngestService` → `rust/crates/sift_rs/src/gen/sift/ingest/v1/sift.ingest.v1.tonic.rs`

Inside that file, find the `pub trait <Name>: ... { ... }` block. Every `async fn` in it must appear in the mock — copy the signatures verbatim.

### 2. Pick the file path

The mock path mirrors the `sift_rs` path:

| `sift_rs` trait | mock file |
| --- | --- |
| `sift_rs::<service>::<version>::<svc>_service_server::<Svc>Service` | `src/mock/<service>/<version>.rs` |

For example `sift_rs::ingest::v1::ingest_service_server::IngestService` → `src/mock/ingest/v1.rs`. Create intermediate `mod.rs` files (`src/mock/<service>/mod.rs`) if the service directory doesn't exist yet, and add `pub mod <service>;` to `src/mock/mod.rs`.

### 3. Write the mock file

Pattern (taken from `src/mock/assets/v1.rs`):

```rust
use async_trait::async_trait;
use sift_rs::<service>::<version>::{
    <svc>_service_server::<Svc>Service,
    // ...every request/response type referenced by the trait methods...
};
use mockall::mock;
use tonic::{Request, Response, Status};

mock! {
    pub <Svc>ServiceImpl {}

    #[async_trait]
    impl <Svc>Service for <Svc>ServiceImpl {
        async fn <method_name>(
            &self,
            request: Request<<MethodRequest>>,
        ) -> std::result::Result<
            Response<<MethodResponse>>,
            Status,
        >;
        // ...one entry per method on the trait...
    }
}
```

Rules:

- **Struct name is `Mock<Svc>ServiceImpl`** — `mockall` prepends `Mock` automatically, so write `pub <Svc>ServiceImpl {}` in the source. For `AssetService` the user-facing type is `MockAssetServiceImpl`.
- **Include every method** from the generated trait, in any order. Missing methods make the `impl` incomplete and the trait won't be satisfied.
- **Match the tonic signatures.** Use `Request<T>` / `Response<T>` (not `tonic::Request` / `tonic::Response`) because `Request` and `Status` are already in scope. Use the fully-qualified `std::result::Result<...>` form — this is what `mockall` expects.
- **Streaming methods**: if the trait has associated stream types (e.g. `type FooStream = ...`) or returns `Response<Self::FooStream>`, the `mock!` macro needs `type FooStream = <concrete stream type>;` inside the `impl` block. Look at the trait's associated-type defaults in the generated file and pick a concrete type like `tonic::Streaming<T>` or a boxed stream — verify against any existing streaming mocks before guessing.
- **Don't add doc comments to the methods.** They aren't load-bearing and clutter the diff.

### 4. Wire it up

After creating `src/mock/<service>/v1.rs`:

1. Ensure `src/mock/<service>/mod.rs` exists and contains `pub mod v1;` (and any other versions present).
2. Ensure `src/mock/mod.rs` contains `pub mod <service>;`.

### 5. Verify it compiles

Run from the workspace root:

```sh
cargo check -p sift_test_util
```

If the user asks for an accompanying test, add it to `src/mock/test.rs` following the existing patterns there (canned response, `withf` request matching, `times(N)` call counts, request-driven responses, pagination). Do not invent new patterns — match what's already in the file.

## Things to avoid

- Don't add the mock to `[dependencies]` of other workspace crates as a runtime dep — `sift_test_util` should appear under `[dev-dependencies]` only.
- Don't re-export `sift_rs` types from the mock module. Callers import them directly from `sift_rs`.
- Don't implement methods inside the `mock!` block — `mockall` generates them. Bodies are configured per-test via `mock.expect_<method>().returning(...)`.
- Don't use `with(eq(...))` to match on `tonic::Request` — `Request` does not implement `PartialEq`. Use `.withf(|req| ...)` instead, as shown in `src/mock/test.rs`.
