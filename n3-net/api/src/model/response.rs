use crate::error::Result;

#[derive(Serialize, Deserialize)]
pub struct BoolResult {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ObjResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error_msg: Option<String>,
}

impl From<bool> for BoolResult {
    fn from(success: bool) -> Self {
        Self { success }
    }
}

impl<T> From<Option<T>> for ObjResult<T> {
    fn from(data: Option<T>) -> Self {
        Self {
            success: data.is_some(),
            data,
            error_msg: None,
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
