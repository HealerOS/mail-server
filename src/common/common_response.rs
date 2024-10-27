use crate::exception::biz_exception::BizError;
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
}

impl CommonResponse<String> {
    pub fn error_response(err: BizError) -> CommonResponse<String> {
        CommonResponse {
            code: 500,
            msg: err.to_string(),
            data: None,
        }
    }
    pub fn success_response_without_data() -> CommonResponse<String> {
        Self::success_response(None)
    }
}
