# Changelog

## [2.3.0] - 2026-10-03

## [2.2.0] - 2026-09-30

## [2.0.0] - 2026-09-30

### Breaking Changes
- **Transport rewritten from gRPC to REST** — requests now go over HTTP/JSON via `reqwest` instead of `tonic`/`prost`, matching the Python, TypeScript, Go, and Java SDKs.
- **Protobuf/gRPC types removed** — the `anduril.entitymanager.v1`, `anduril.taskmanager.v1`, `anduril.tasks.*`, and `anduril.ontology.v1` tonic/prost modules are gone. Use the new `Lattice` client and its `entities`, `tasks`, `objects`, and `video` resource clients instead.
- **Client construction changed** — build a client with `Lattice::new(ClientConfig { .. })`, configured via `Environment` and an OAuth `client_id`/`client_secret` pair or a bearer token, instead of a manual `tonic::transport::Channel`.

### Added
- **`Lattice` client** — typed resource clients for Entities, Tasks, the Object Store, and Video Manager.
- **OAuth client-credentials authentication** — built-in token exchange via `client_id`/`client_secret` on `ClientConfig`.
- **`entities.long_poll_entity_events`** — long-polling entity event stream support.
- **Object Store client** (`client.objects`) — upload, download, and manage Lattice objects over REST.
- **Video Manager client** (`client.video`) — manage ingress and egress video streams (RTSP, SRT, MPEG-TS).
- **Configurable HTTP behavior** — retries, timeouts, custom headers, and a pluggable `reqwest::Client`.
- **Server-sent events (SSE) support** — opt-in via the `sse` feature flag.

### Removed
- **Protobuf/gRPC dependencies** — `tonic`, `prost`, `pbjson`, and `pbjson-types` are gone. The crate now depends on `reqwest`, `serde`, and `serde_json`.
