use serde::{Deserialize, Serialize};

// region:    --- Login Statuses Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "gender")]
pub enum Gender {
	Male,
	Female,
	Other,
	PreferNotToSay,
}

impl From<Gender> for sea_query::Value {
	fn from(val: Gender) -> Self {
		val.to_string().into()
	}
}

// endregoin:    --- Login Statuses Types
