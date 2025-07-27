use serde::{Deserialize, Serialize};

// region:    --- Education Level Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "education_level")]
pub enum EducationLevel {
	Primary,
	Secondary,
	Tertiary,
	Adult,
	Special,
}

impl From<EducationLevel> for sea_query::Value {
	fn from(val: EducationLevel) -> Self {
		val.to_string().into()
	}
}

// endregoin:    --- Education Level Types
