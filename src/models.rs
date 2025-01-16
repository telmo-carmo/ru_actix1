use serde::{Deserialize, Serialize};

use crate::schema::bonus;
use crate::schema::dept;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset, utoipa::ToSchema)]
#[diesel(table_name = bonus)]
pub struct Bonus {
    pub ename: String,
    pub job: String,
    pub sal: i32,
    pub comm: i32,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset, utoipa::ToSchema)]
#[diesel(table_name = dept)]
pub struct Dept {
    pub deptno: i32,
    pub dname: String,
    pub loc: String,
}
