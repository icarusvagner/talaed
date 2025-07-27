use std::sync::OnceLock;

use lib_utils::envs::get_env;

pub fn frontend_config() -> &'static FrontendConfig {
	static INSTANCE: OnceLock<FrontendConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		FrontendConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FrontendConfig FATAL - WHILE LOADING CONF - cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct FrontendConfig {
	pub DEV_REQ_URL: String,
	pub PROD_REQ_URL: String,
	pub TEST_REQ_URL: String,
}

impl FrontendConfig {
	fn load_from_env() -> lib_utils::envs::Result<FrontendConfig> {
		Ok(FrontendConfig {
			DEV_REQ_URL: get_env("REQUEST_DEV_URL")?,
			PROD_REQ_URL: get_env("REQUEST_PROD_URL")?,
			TEST_REQ_URL: get_env("REQUEST_TEST_URL")?,
		})
	}
}
