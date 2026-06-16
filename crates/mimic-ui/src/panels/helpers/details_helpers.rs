// ── HTTP Methods ───────────────────────────────────────────────────────────────
pub const HTTP_METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"];

// ── Common HTTP Status Codes ───────────────────────────────────────────────────
pub const HTTP_STATUS_CODES: &[(u16, &str)] = &[
    (200, "200 OK"),
    (201, "201 Created"),
    (204, "204 No Content"),
    (301, "301 Moved Permanently"),
    (302, "302 Found"),
    (304, "304 Not Modified"),
    (400, "400 Bad Request"),
    (401, "401 Unauthorized"),
    (403, "403 Forbidden"),
    (404, "404 Not Found"),
    (405, "405 Method Not Allowed"),
    (409, "409 Conflict"),
    (422, "422 Unprocessable Entity"),
    (429, "429 Too Many Requests"),
    (500, "500 Internal Server Error"),
    (502, "502 Bad Gateway"),
    (503, "503 Service Unavailable"),
];

/// Validates that path is a relative URL path (no scheme, no host)
pub fn validate_path(path: &str) -> Option<String> {
    if path.is_empty() {
        return Some("Path cannot be empty".to_string());
    }

    if !path.starts_with('/') {
        return Some("Path must start with /".to_string());
    }

    if path.contains("://") {
        return Some("Path must not include a host or scheme".to_string());
    }

    // Validate segment by segment
    // Split on '/' — first element will be empty string (before leading slash)
    for segment in path.split('/').skip(1) {
        // Empty segment — only allowed as the last char (trailing slash)
        // e.g. "/api/" splits into ["", "api", ""] — the trailing "" is fine
        if segment.is_empty() {
            continue;
        }

        if segment.starts_with(':') {
            // ── Param segment e.g. ":orderId" ───────────────────────
            let param_name = &segment[1..]; // everything after ':'

            // Bare colon with no name — reject
            if param_name.is_empty() {
                return Some(format!(
                    "Invalid param segment ':{param_name}' — param name cannot be empty"
                ));
            }

            // Param name must be alphanumeric + underscore only
            // e.g. ":orderId" ✅  ":order-id" ❌  ":order.id" ❌
            let invalid = param_name
                .chars()
                .find(|c| !c.is_alphanumeric() && *c != '_');

            if let Some(ch) = invalid {
                return Some(format!(
                    "Invalid character '{ch}' in param name '{segment}' — only letters, digits, and _ allowed"
                ));
            }
        } else {
            // ── Static segment e.g. "api", "orders", "v1" ───────────
            // Allow: letters, digits, -, _, .
            // Reject: :, {, }, and everything else
            let invalid = segment
                .chars()
                .find(|c| !c.is_alphanumeric() && !matches!(c, '-' | '_' | '.'));

            if let Some(ch) = invalid {
                return Some(format!("Invalid character '{ch}' in segment '{segment}'"));
            }
        }
    }

    None
}
