#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONResponse<T> {
    pub success: bool,
    pub payload: T,
}

pub mod private;
pub mod public;
