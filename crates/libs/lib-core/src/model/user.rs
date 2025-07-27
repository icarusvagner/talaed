use crate::ctx::Ctx;
use crate::model::base::{self, prep_fields_for_update, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use crate::types::user::*;
use lib_auth::pwd::{self, ContentToHash};
use modql::field::{Fields, HasSeaFields, SeaField, SeaFields};
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- UserBmc

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "users";
}

impl UserBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_c: UserForCreate,
	) -> Result<i64> {
		let UserForCreate {
			email_add,
			pwd_clear,
		} = user_c;

		// -- Create the user row
		let user_fi = UserForInsert {
			email_add: email_add.to_string(),
		};

		// Start the transaction
		let mm = mm.new_with_txn()?;

		mm.dbx().begin_txn().await?;

		let user_id = base::create::<Self, _>(ctx, &mm, user_fi).await.map_err(
			|model_error| {
				Error::resolve_unique_violation(
					model_error,
					Some(|table: &str, constraint: &str| {
						if table == "users" && constraint.contains("email_add") {
							Some(Error::UserAlreadyExists { email_add })
						} else {
							None // Error::UniqueViolation will be created by resolve_unique_violation
						}
					}),
				)
			},
		)?;

		// -- Update the database
		Self::update_pwd(ctx, &mm, user_id, &pwd_clear).await?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(user_id)
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_email_add<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		email_add: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::sea_idens())
			.and_where(Expr::col(UserIden::EmailAdd).eq(email_add));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<UserFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<User>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		// -- Prep password
		let user: UserForLogin = Self::get(ctx, mm, id).await?;
		let pwd = pwd::hash_pwd(ContentToHash {
			content: pwd_clear.to_string(),
			salt: user.password_salt,
		})
		.await?;

		// -- Prep the data
		let mut fields =
			SeaFields::new(vec![SeaField::new(UserIden::PasswordHash, pwd)]);
		prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

		// -- Build query
		let fields = fields.for_sea_update();
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(fields)
			.and_where(Expr::col(UserIden::Id).eq(id));

		// -- Exec query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _count = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}

	/// TODO: For User, deletion will require a soft-delete approach:
	///       - Set `deleted: true`.
	///       - Change `email_add` to "DELETED-_user_id_".
	///       - Clear any other UUIDs or PII (Personally Identifiable Information).
	///       - The automatically set `mid`/`mtime` will record who performed the deletion.
	///       - It's likely necessary to record this action in a `um_change_log` (a user management change audit table).
	///       - Remove or clean up any user-specific assets (messages, etc.).
	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

// endregion: --- UserBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>; // For tests.

	use super::*;
	use crate::_dev_utils;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_email_add = "test_create_ok-user-01";
		let fx_pwd_clear = "test_create_ok pwd 01";

		// -- Exec
		let user_id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				email_add: fx_email_add.to_string(),
				pwd_clear: fx_pwd_clear.to_string(),
			},
		)
		.await?;

		// -- Check
		let user: UserForLogin = UserBmc::get(&ctx, &mm, user_id).await?;
		assert_eq!(user.email_add, fx_email_add);

		// -- Clean
		UserBmc::delete(&ctx, &mm, user_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_first_ok_demo1() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_email_add = "demo1";

		// -- Exec
		let user: User = UserBmc::first_by_email_add(&ctx, &mm, fx_email_add)
			.await?
			.ok_or("Should have user 'demo1'")?;

		// -- Check
		assert_eq!(user.email_add, fx_email_add);

		Ok(())
	}
}

// endregion: --- Tests
