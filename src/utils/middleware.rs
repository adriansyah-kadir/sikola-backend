pub fn make_trace_http_layer() -> tower_http::trace::TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    impl tower_http::trace::MakeSpan<axum::body::Body> + Clone,
> {
    tower_http::trace::TraceLayer::new_for_http().make_span_with(|req: &axum::http::Request<_>| {
        let request_id = req
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("missing");

        tracing::info_span!(
            "request",
            method     = %req.method(),
            uri        = %req.uri(),
            version    = ?req.version(),
            request_id = %request_id,
        )
    })
}
