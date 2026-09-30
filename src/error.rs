use crate::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("BadRequestError: Bad request - {message}")]
    BadRequestError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("UnauthorizedError: Authentication failed - {message}")]
    UnauthorizedError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("ForbiddenError: Access forbidden - {message}")]
    ForbiddenError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("NotFoundError: Resource not found - {message}")]
    NotFoundError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("ServiceUnavailableError: {message}")]
    ServiceUnavailableError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("ConflictError: Conflict - {message}")]
    ConflictError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("TooManyRequestsError: Rate limit exceeded - {message}")]
    TooManyRequestsError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("InternalServerError: Internal server error - {message}")]
    InternalServerError {
        message: String,
        code: Option<i64>,
        details: Option<Vec<GoogleProtobufAny>>,
    },
    #[error("RequestTimeoutError: {message}")]
    RequestTimeoutError {
        message: String,
        error_code: Option<String>,
    },
    #[error("ContentTooLargeError: {message}")]
    ContentTooLargeError {
        message: String,
        code: Option<String>,
    },
    #[error("InsufficientStorageError: {message}")]
    InsufficientStorageError {
        message: String,
        code: Option<String>,
    },
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Network error: {0}")]
    Network(reqwest::Error),
    #[error("Request executor error: {0}")]
    Executor(Box<dyn std::error::Error + Send + Sync>),
    #[error("Serialization error: {0}")]
    Serialization(serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
    #[error("SSE stream terminated")]
    StreamTerminated,
    #[error("SSE stream timed out waiting for next event")]
    StreamTimeout,
    #[error("SSE parse error: {0}")]
    SseParseError(String),
}

impl ApiError {
    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        match status_code {
            400 => {
                // Parse error body for BadRequestError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadRequestError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::BadRequestError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            401 => {
                // Parse error body for UnauthorizedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnauthorizedError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::UnauthorizedError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            403 => {
                // Parse error body for ForbiddenError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ForbiddenError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::ForbiddenError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            404 => {
                // Parse error body for NotFoundError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NotFoundError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::NotFoundError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            503 => {
                // Parse error body for ServiceUnavailableError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ServiceUnavailableError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::ServiceUnavailableError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            409 => {
                // Parse error body for ConflictError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ConflictError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::ConflictError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            429 => {
                // Parse error body for TooManyRequestsError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::TooManyRequestsError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::TooManyRequestsError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            500 => {
                // Parse error body for InternalServerError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::InternalServerError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            details: parsed.get("details").and_then(|v| {
                                serde_json::from_value::<Vec<GoogleProtobufAny>>(v.clone()).ok()
                            }),
                        };
                    }
                }
                return Self::InternalServerError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    details: None,
                };
            }
            408 => {
                // Parse error body for RequestTimeoutError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::RequestTimeoutError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            error_code: parsed
                                .get("errorCode")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::RequestTimeoutError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    error_code: None,
                };
            }
            413 => {
                // Parse error body for ContentTooLargeError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ContentTooLargeError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ContentTooLargeError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                };
            }
            507 => {
                // Parse error body for InsufficientStorageError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::InsufficientStorageError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::InsufficientStorageError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                };
            }
            _ => Self::Http {
                status: status_code,
                message: body.unwrap_or("Unknown error").to_string(),
            },
        }
    }
}

/// Error returned when a required field was not set on a builder.
#[derive(Debug)]
pub struct BuildError {
    field: &'static str,
}

impl BuildError {
    pub fn missing_field(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` was not set but is required", self.field)
    }
}

impl std::error::Error for BuildError {}
