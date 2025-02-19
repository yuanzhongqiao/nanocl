use bollard_next::service::ContainerSummary;

use nanocl_error::http_client::HttpClientResult;

use nanocl_stubs::{
  generic::{GenericFilterNsp, GenericNspQuery},
  cargo::{Cargo, CargoDeleteQuery, CargoInspect, CargoSummary},
  cargo_spec::{CargoSpec, CargoSpecPartial, CargoSpecUpdate},
};

use super::http_client::NanocldClient;

impl NanocldClient {
  /// ## Default path for cargoes
  const CARGO_PATH: &'static str = "/cargoes";

  /// Create a new cargo in the system
  /// Note that the cargo is not started by default
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let new_cargo = CargoSpecPartial {
  ///  name: String::from("my-cargo"),
  ///  container: bollard_next::container::Config {
  ///    image: Some(String::from("alpine"))
  ///    ..Default::default()
  ///   }
  /// };
  /// let res = client.create_cargo(new_cargo, None).await;
  /// ```
  pub async fn create_cargo(
    &self,
    item: &CargoSpecPartial,
    namespace: Option<&str>,
  ) -> HttpClientResult<Cargo> {
    let res = self
      .send_post(
        Self::CARGO_PATH,
        Some(item),
        Some(&GenericNspQuery::new(namespace)),
      )
      .await?;
    Self::res_json(res).await
  }

  /// Delete a cargo by it's name and namespace
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let res = client.delete_cargo("my-cargo", None).await;
  /// ```
  pub async fn delete_cargo(
    &self,
    name: &str,
    query: Option<&CargoDeleteQuery>,
  ) -> HttpClientResult<()> {
    self
      .send_delete(&format!("{}/{name}", Self::CARGO_PATH), query)
      .await?;
    Ok(())
  }

  /// Inspect a cargo by it's name to get more information about it
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let res = client.inspect_cargo("my-cargo", None).await;
  /// ```
  pub async fn inspect_cargo(
    &self,
    name: &str,
    namespace: Option<&str>,
  ) -> HttpClientResult<CargoInspect> {
    let res = self
      .send_get(
        &format!("{}/{name}/inspect", Self::CARGO_PATH),
        Some(GenericNspQuery::new(namespace)),
      )
      .await?;
    Self::res_json(res).await
  }

  /// List all cargoes in a namespace
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let cargoes = client.list_cargoes(None).await.unwrap();
  /// ```
  pub async fn list_cargo(
    &self,
    query: Option<&GenericFilterNsp>,
  ) -> HttpClientResult<Vec<CargoSummary>> {
    let query = Self::convert_query(query)?;
    let res = self.send_get(Self::CARGO_PATH, Some(query)).await?;
    Self::res_json(res).await
  }

  /// Patch a cargo by it's name
  /// This will update the cargo's spec by merging current spec with new spec and creating an history entry
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let cargo_spec = CargoSpecPatch {
  ///   name: "my-cargo-renamed".into(),
  /// };
  /// client.patch_cargo("my-cargo", cargo, None).await.unwrap();
  /// ```
  pub async fn patch_cargo(
    &self,
    name: &str,
    spec: &CargoSpecUpdate,
    namespace: Option<&str>,
  ) -> HttpClientResult<()> {
    self
      .send_patch(
        &format!("{}/{name}", Self::CARGO_PATH),
        Some(spec),
        Some(GenericNspQuery::new(namespace)),
      )
      .await?;
    Ok(())
  }

  /// Put a cargo by it's name
  /// It will create a new cargo spec and store old one in history
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let cargo_spec = CargoSpecPartial {
  ///   name: "my-cargo-renamed".into(),
  /// };
  /// client.put_cargo("my-cargo", &cargo, None).await.unwrap();
  /// ```
  pub async fn put_cargo(
    &self,
    name: &str,
    spec: &CargoSpecPartial,
    namespace: Option<&str>,
  ) -> HttpClientResult<()> {
    self
      .send_put(
        &format!("{}/{name}", Self::CARGO_PATH),
        Some(spec),
        Some(GenericNspQuery::new(namespace)),
      )
      .await?;
    Ok(())
  }

  /// List cargo histories
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let histories = client.list_history("my-cargo", None).await.unwrap();
  /// ```
  pub async fn list_history_cargo(
    &self,
    name: &str,
    namespace: Option<&str>,
  ) -> HttpClientResult<Vec<CargoSpec>> {
    let res = self
      .send_get(
        &format!("{}/{name}/histories", Self::CARGO_PATH),
        Some(GenericNspQuery::new(namespace)),
      )
      .await?;
    Self::res_json(res).await
  }

  /// Revert a cargo to a specific history
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let cargo = client.revert_cargo("my-cargo", "my-history-id", None).await.unwrap();
  /// ```
  pub async fn revert_cargo(
    &self,
    name: &str,
    id: &str,
    namespace: Option<&str>,
  ) -> HttpClientResult<Cargo> {
    let res = self
      .send_patch(
        &format!("{}/{name}/histories/{id}/revert", Self::CARGO_PATH),
        None::<String>,
        Some(GenericNspQuery::new(namespace)),
      )
      .await?;
    Self::res_json(res).await
  }

  /// List all the instances of a cargo by it's name and namespace
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_to("http://localhost:8585", None);
  /// let res = client.list_cargo_instance("my-cargo", None).await;
  /// ```
  pub async fn list_cargo_instance(
    &self,
    name: &str,
    namespace: Option<&str>,
  ) -> HttpClientResult<Vec<ContainerSummary>> {
    let res = self
      .send_get(
        &format!("{}/{name}/instances", Self::CARGO_PATH),
        Some(GenericNspQuery::new(namespace)),
      )
      .await?;
    Self::res_json(res).await
  }
}

#[cfg(test)]
mod tests {
  use crate::ConnectOpts;

  use super::*;

  use nanocl_error::http_client::HttpClientError;
  use nanocl_stubs::cargo_spec::CargoSpecPartial;

  #[ntex::test]
  async fn basic() {
    const CARGO_NAME: &str = "client-test-cargo";
    let client = NanocldClient::connect_to(&ConnectOpts {
      url: "http://nanocl.internal:8585".into(),
      ..Default::default()
    })
    .expect("Failed to create a nanocl client");
    client.list_cargo(None).await.unwrap();
    let new_cargo = CargoSpecPartial {
      name: CARGO_NAME.into(),
      container: bollard_next::container::Config {
        image: Some("ghcr.io/next-hat/nanocl-get-started:latest".into()),
        ..Default::default()
      },
      ..Default::default()
    };
    client.create_cargo(&new_cargo, None).await.unwrap();
    client
      .start_process("cargo", CARGO_NAME, None)
      .await
      .unwrap();
    client.inspect_cargo(CARGO_NAME, None).await.unwrap();
    let cargo_update = CargoSpecUpdate {
      container: Some(bollard_next::container::Config {
        image: Some("ghcr.io/next-hat/nanocl-get-started:latest".into()),
        env: Some(vec!["TEST=1".into()]),
        ..Default::default()
      }),
      ..Default::default()
    };
    client
      .patch_cargo(CARGO_NAME, &cargo_update, None)
      .await
      .unwrap();
    client
      .put_cargo(CARGO_NAME, &new_cargo, None)
      .await
      .unwrap();
    let histories = client.list_history_cargo(CARGO_NAME, None).await.unwrap();
    assert!(histories.len() > 1);
    let history = histories.first().unwrap();
    client
      .revert_cargo(CARGO_NAME, &history.key.to_string(), None)
      .await
      .unwrap();
    client
      .stop_process("cargo", CARGO_NAME, None)
      .await
      .unwrap();
    client
      .delete_cargo(
        CARGO_NAME,
        Some(&CargoDeleteQuery {
          force: Some(true),
          ..Default::default()
        }),
      )
      .await
      .unwrap();
  }

  #[ntex::test]
  async fn create_cargo_duplicate_name() {
    let client = NanocldClient::connect_to(&ConnectOpts {
      url: "http://nanocl.internal:8585".into(),
      ..Default::default()
    })
    .expect("Failed to create a nanocl client");
    let new_cargo = CargoSpecPartial {
      name: "client-test-cargodup".into(),
      container: bollard_next::container::Config {
        image: Some("ghcr.io/next-hat/nanocl-get-started:latest".into()),
        ..Default::default()
      },
      ..Default::default()
    };
    client.create_cargo(&new_cargo, None).await.unwrap();
    let err = client.create_cargo(&new_cargo, None).await.unwrap_err();
    match err {
      HttpClientError::HttpError(err) => {
        assert_eq!(err.status, 409);
      }
      _ => panic!("Wrong error type"),
    }
    client
      .delete_cargo("client-test-cargodup", None)
      .await
      .unwrap();
  }
}
