use std::fs::File;
use std::path::PathBuf;
use futures::TryStreamExt;
use reqwest::{Client, Response};
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tempfile::NamedTempFile;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use crate::progress::ProgressReadAdapter;

mod progress;

#[tokio::main()]
async fn main() {
    let client = Client::new();
    let response = client.get("http://distribution.bbb3d.renderfarming.net/video/mp4/bbb_sunflower_native_60fps_stereo_abl.mp4").send().await.unwrap();
    match response_to_file(response, "/Users/bes/Downloads/bbb.mp4".into()).await {
        Ok(_) => println!("Everything OK"),
        Err(err) => eprintln!("{err}"),
    }
}

async fn response_to_file(response: Response, path: PathBuf) -> Result<(), ApiError> {
    let download = response.bytes_stream();

    let download = download
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read();

    let download = download.compat();

    // Wrap download to be able to get progress in terminal
    let mut download = ProgressReadAdapter::new(download);

    let temp_file = tokio::task::spawn_blocking(NamedTempFile::new)
        .await
        .wrap_api_err()?
        .wrap_api_err()?;

    let mut outfile = tokio::fs::File::create(temp_file.path())
        .await
        .wrap_api_err()?;

    // Code hangs here forever after a network switch
    tokio::io::copy(&mut download, &mut outfile)
        .await
        .wrap_api_err()?;

    outfile.flush().await.wrap_api_err()?;

    let _persisted_file: File = tokio::task::spawn_blocking(move || temp_file.persist(path))
        .await
        .wrap_api_err()?
        .wrap_api_err()?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Wrapped Error {0}")]
    WrappedError(Box<dyn Error + Send + Sync>),
}

impl ApiError {
    pub fn wrap<E>(e: E) -> ApiError
        where
            E: Error + Send + Sync + 'static,
    {
        ApiError::WrappedError(Box::new(e))
    }
}

pub trait WrapApiError<T> {
    fn wrap_api_err(self) -> Result<T, ApiError>;
}

impl<T, E> WrapApiError<T> for Result<T, E>
    where
        E: Error + Send + Sync + 'static,
{
    fn wrap_api_err(self) -> Result<T, ApiError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(ApiError::WrappedError(Box::new(e))),
        }
    }
}