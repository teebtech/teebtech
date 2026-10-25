use shared::AppResult;

/// Initializes the observability / tracing subsystem.
pub fn init_telemetry(service_name: &str) -> AppResult<()> {
    #[cfg(feature = "tracing")]
    {
        tracing::info!("Telemetry initialized for service: {}", service_name);
    }
    let _ = service_name;
    Ok(())
}
