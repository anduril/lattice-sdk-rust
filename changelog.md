# Changelog

## [2.6.0] - 2026-11-06
### Added
- **`VideoClient::query_ingress_streams`** — returns ingress streams matching a filter, newest first, with API-created streams listed before mesh-discovered ones and paginated via `page_size` and `page_token`.
- **`QueryIngressStreamsRequest`** and **`QueryIngressStreamsResponse`** — new request and response types (with builders) for the query endpoint, including `next_page_token` for fetching subsequent pages.
- **`IngressStreamFilter`**, **`CreatorFilter`**, **`StringFilter`**, and **`IngressStreamFilterStatusesItem`** — new filter types that match streams by creator ownership, ingress ID, title (exact, prefix, or contains), and status.
- **`Sensor.camera_configuration`** — new optional field carrying a `CameraConfiguration` with the camera's `zoom_percentage` (0-100), plus a matching `SensorBuilder` setter.

## [2.5.0] - 2026-10-30
### Added
- **`TasksClient::send_manual_control_frame`** — sends a single manual control frame (for example, joystick input) for an executing manual control task, which Lattice forwards to the executing agent.
- **`ManualControlFrameRequest`** and **`ManualControlFrameRequestBuilder`** — the request payload carries a required `session_epoch_micros`, where a higher epoch takes over the session and a lower one is rejected as stale, plus an optional `sequence` and a task-specific `specification`.
- **`ManualControlFrameResponse`** — new response type returned by the send manual control frame endpoint.

## [2.4.0] - 2026-10-27
### Added
- **`RequestOptions::additional_body_param`** — merges extra top-level properties into JSON and form-encoded request bodies, overriding generated fields with the same key, which lets callers send undocumented or beta fields.
- **`TaskStatus.last_update_time`** and **`TaskStatus.end_time`** — new optional timestamps that report when a task's status last changed (to guard against out-of-order updates) and when the task finished execution, with matching `TaskStatusBuilder` setters.

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
