use tracing_subscriber::filter::{EnvFilter, LevelFilter};
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_line_number(true)
        .with_file(true)
        .init();
}
