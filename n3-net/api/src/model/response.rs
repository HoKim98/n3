#[derive(Serialize, Deserialize)]
pub struct BoolResult {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ObjResult<T> {
    pub success: bool,
    pub data: Option<T>,
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
        }
    }
}
