use crate::model::modql_utils::time_to_sea_value;
use modql::{
	field::{Fields, HasSeaFields},
	filter::{FilterNodes, OpValsInt64, OpValsString, OpValsValue},
};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

// region:    --- User Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "user_typ")]
pub enum UserTyp {
	Admin,
	Teacher,
	Student,
}

impl From<UserTyp> for sea_query::Value {
	fn from(val: UserTyp) -> Self {
		val.to_string().into()
	}
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub email_add: String,
	pub user_type: UserTyp,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub email_add: String,
	pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
	pub email_add: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub email_add: String,

	// -- pwd and token info
	pub password_hash: Option<String>, // encrypted, #_scheme_id_#....
	pub password_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub email_add: String,

	// -- token info
	pub token_salt: Uuid,
}

/// Marker trait
pub trait UserBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// Note: Since the entity properties Iden will be given by modql
//       UserIden does not have to be exhaustive, but just have the columns
//       we use in our specific code.
#[derive(Iden)]
pub enum UserIden {
	Id,
	EmailAdd,
	PasswordHash,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
	pub id: Option<OpValsInt64>,

	pub email_add: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

// endregion: --- User Types
