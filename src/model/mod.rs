#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONResponse<T> {
    success: bool,
    payload: T,
}

pub mod public;
pub mod private;
