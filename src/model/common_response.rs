use serde::Serialize;

#[derive(Serialize)]
pub struct CommonResponse<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> CommonResponse<T> {
    pub fn success_response(data: Option<T>) -> CommonResponse<T> {
        CommonResponse {
            code: 0,
            msg: "Success".to_string(),
            data,
        }
    }
    pub fn success_response_without_data() -> CommonResponse<T> {
        Self::success_response(None)
    }
}
