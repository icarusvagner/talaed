use serde::{Deserialize, Serialize};

// region:    --- Login Statuses Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "login_status")]
pub enum LoginStatus {
	Online,
	Oncall,
	Offline,
	Busy,
	Pending,
	Inactive,
	Locked,
}

impl From<LoginStatus> for sea_query::Value {
	fn from(val: LoginStatus) -> Self {
		val.to_string().into()
	}
}

// endregoin:    --- Login Statuses Types
