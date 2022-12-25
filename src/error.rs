use pixels::TextureError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("resizing texture")]
    ResizeError,
}

impl From<TextureError> for Error {
    fn from(_: TextureError) -> Self {
        Self::ResizeError
    }
}
