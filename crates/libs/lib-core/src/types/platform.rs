use serde::{Deserialize, Serialize};

// region:    --- Login Statuses Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "platform")]
pub enum Platform {
	Windows,
	MacOS,
	Linux,
	Android,
	IOs,
	Web,
	Other,
}

impl From<Platform> for sea_query::Value {
	fn from(val: Platform) -> Self {
		val.to_string().into()
	}
}

// endregoin:    --- Login Statuses Types
