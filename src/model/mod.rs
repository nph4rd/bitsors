#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONResponse<T> {
    success: bool,
    payload: T,
}

pub mod private;
pub mod public;
