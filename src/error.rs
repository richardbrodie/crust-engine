use pixels::TextureError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("resizing texture")]
    ResizeError,
}

impl From<TextureError> for Error {
    fn from(value: TextureError) -> Self {
        Self::ResizeError
    }
}
