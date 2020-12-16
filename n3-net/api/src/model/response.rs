use crate::error::Result;

#[derive(Serialize, Deserialize)]
pub struct BoolResult {
    pub success: bool,
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ObjResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error_msg: Option<String>,
}

impl From<bool> for BoolResult {
    fn from(success: bool) -> Self {
        Self {
            success,
            error_msg: None,
        }
    }
}

impl From<Result<()>> for BoolResult {
    fn from(error: Result<()>) -> Self {
        match error {
            Ok(()) => Self {
                success: true,
                error_msg: None,
            },
            Err(error) => Self {
                success: false,
                error_msg: Some(format!("{:?}", error)),
            },
        }
    }
}

impl<T> From<Result<T>> for ObjResult<T> {
    fn from(error: Result<T>) -> Self {
        match error {
            Ok(data) => Self {
                success: true,
                data: Some(data),
                error_msg: None,
            },
            Err(error) => Self {
                success: false,
                data: None,
                error_msg: Some(format!("{:?}", error)),
            },
        }
    }
}
