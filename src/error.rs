use pixels::TextureError;

#[derive(Debug)]
pub enum Error {
    ResizeError,
}

impl From<TextureError> for Error {
    fn from(_: TextureError) -> Self {
        Self::ResizeError
    }
}
