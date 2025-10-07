use diesel::{AsChangeset, Insertable, Queryable};
use super::schema::members;

#[derive(serde::Serialize, serde::Deserialize, Queryable, AsChangeset)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, serde::Deserialize)]
#[table_name="members"]
pub struct NewMember {
    pub name: String,
    pub email: String,
}