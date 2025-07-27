use serde::{Deserialize, Serialize};

// region:    --- Login Statuses Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "device_type")]
pub enum DeviceType {
	Mobile,
	Desktop,
	Tablet,
	Other,
}

impl From<DeviceType> for sea_query::Value {
	fn from(val: DeviceType) -> Self {
		val.to_string().into()
	}
}

// endregoin:    --- Login Statuses Types
