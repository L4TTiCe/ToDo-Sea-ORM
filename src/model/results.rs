use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct RowsAffected {
    pub rows_affected: u64,
}

impl From<sea_orm::DeleteResult> for RowsAffected {
    fn from(result: sea_orm::DeleteResult) -> Self {
        Self {
            rows_affected: result.rows_affected,
        }
    }
}
