use napi::bindgen_prelude::*;
use napi_derive::napi;

/// module registration is done by the runtime, no need to explicitly do it now.
#[napi]
fn fibonacci(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// async fn, require `async` feature enabled.
/// [dependencies]
/// napi = {version="2", features=["async"]}
#[napi]
async fn read_file_async(path: String) -> Result<Buffer> {
    match tokio::fs::read(path).await {
        Ok(content) => Ok(content.into()),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("failed to read file, {}", e),
        )),
    }
}
