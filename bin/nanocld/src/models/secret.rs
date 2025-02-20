use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use nanocl_error::io::IoError;

use nanocl_stubs::secret::{Secret, SecretPartial, SecretUpdate};

use crate::schema::secrets;

/// This structure represent the secret in the database.
/// A secret is a key/value pair that can be used by the user to store
/// sensitive data. It is stored as a json object in the database.
#[derive(
  Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable,
)]
#[serde(rename_all = "PascalCase")]
#[diesel(primary_key(key))]
#[diesel(table_name = secrets)]
pub struct SecretDb {
  /// The key of the secret
  pub key: String,
  /// The creation date
  pub created_at: chrono::NaiveDateTime,
  /// The last update date
  pub updated_at: chrono::NaiveDateTime,
  /// The kind of secret
  pub kind: String,
  /// The secret cannot be updated
  pub immutable: bool,
  /// The secret data
  pub data: serde_json::Value,
  // The metadata (user defined)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option<serde_json::Value>,
}

impl From<&SecretPartial> for SecretDb {
  fn from(secret: &SecretPartial) -> Self {
    Self {
      key: secret.name.clone(),
      created_at: chrono::Utc::now().naive_utc(),
      updated_at: chrono::Utc::now().naive_utc(),
      kind: secret.kind.clone(),
      immutable: secret.immutable,
      data: secret.data.clone(),
      metadata: secret.metadata.clone(),
    }
  }
}

impl TryFrom<SecretDb> for Secret {
  type Error = IoError;

  fn try_from(db: SecretDb) -> Result<Self, Self::Error> {
    Ok(Secret {
      name: db.key,
      created_at: db.created_at,
      updated_at: db.updated_at,
      kind: db.kind,
      immutable: db.immutable,
      data: db.data,
      metadata: db.metadata,
    })
  }
}

/// This structure is used to update a secret in the database.
#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = secrets)]
pub struct SecretUpdateDb {
  /// The secret data
  pub data: Option<serde_json::Value>,
  // The metadata (user defined)
  pub metadata: Option<serde_json::Value>,
}

impl From<&SecretUpdate> for SecretUpdateDb {
  fn from(update: &SecretUpdate) -> Self {
    Self {
      data: Some(update.data.clone()),
      metadata: update.metadata.clone(),
    }
  }
}
