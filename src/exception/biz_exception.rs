use anyhow::Error;
use thiserror::Error;

/// 错误类型
#[derive(Debug, Error)]
pub enum BizError {
    /// 其他错误
    #[error(transparent)]
    Other(Error),

    /// 未知错误
    #[error("未知错误")]
    UnknownError,

    #[error("参数异常")]
    ParamInvalid(String),
}

pub type BizResult<T> = Result<T, BizError>;
