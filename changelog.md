# Changelog

## [2.7.0] - 2026-11-14
### Added
- **`VideoClient::query_ingress_streams`** — returns ingress streams that match an optional filter, newest first, with API-created streams listed before mesh-discovered ones.
- **`QueryIngressStreamsRequest`** and **`QueryIngressStreamsResponse`** — paginated request and response types with `page_size` (default 50, max 100), `page_token`, `filter`, and `next_page_token`, plus matching builders.
- **`IngressStreamFilter`** — narrows query results by `creator`, `ingress_id`, `title`, and `statuses`, where a stream must match every field that is set.
- **`CreatorFilter`**, **`StringFilter`**, and **`IngressStreamFilterStatusesItem`** — supporting filter types: `CreatorFilter` matches on ownership through `owned`, `StringFilter` does case-sensitive `exact`, `prefix`, or `contains` matching, and `IngressStreamFilterStatusesItem` is a non-exhaustive status enum.

## [2.6.0] - 2026-11-09
### Added
- **`CameraConfiguration`** and **`CameraConfigurationBuilder`** — a new type that reports a camera sensor's zoom state through an optional `zoom_percentage`, where 0 is fully zoomed out and 100 is fully zoomed in.
- **`Sensor.camera_configuration`** — a new optional field that carries the camera zoom state for camera sensors, with a matching `SensorBuilder::camera_configuration` setter.

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
