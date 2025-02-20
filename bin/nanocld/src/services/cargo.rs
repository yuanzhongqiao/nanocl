use ntex::web;

use nanocl_error::{http::HttpResult, io::IoResult};

use nanocl_stubs::{
  cargo::CargoDeleteQuery,
  cargo_spec::{CargoSpecPartial, CargoSpecUpdate},
  generic::{
    GenericClause, GenericCount, GenericListQueryNsp, GenericNspQuery,
  },
};

use crate::{
  utils,
  objects::generic::*,
  repositories::generic::*,
  models::{
    SystemState, SpecDb, CargoObjCreateIn, CargoDb, CargoObjPutIn,
    CargoObjPatchIn,
  },
};

/// List cargoes
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Cargoes",
  path = "/cargoes",
  params(
    ("filter" = Option<String>, Query, description = "Generic filter", example = "{ \"filter\": { \"where\": { \"name\": { \"eq\": \"test\" } } } }"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargoes are"),
  ),
  responses(
    (status = 200, description = "List of cargoes", body = [CargoSummary]),
  ),
))]
#[web::get("/cargoes")]
pub async fn list_cargo(
  state: web::types::State<SystemState>,
  qs: web::types::Query<GenericListQueryNsp>,
) -> HttpResult<web::HttpResponse> {
  let query = utils::query_string::parse_qs_nsp_filter(&qs)?;
  log::debug!("got query {query:#?}");
  let cargoes = CargoDb::list(&query, &state).await?;
  Ok(web::HttpResponse::Ok().json(&cargoes))
}

/// Get detailed information about a cargo
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Cargoes",
  path = "/cargoes/{name}/inspect",
  params(
    ("name" = String, Path, description = "Name of the cargo"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargo belongs"),
  ),
  responses(
    (status = 200, description = "Cargo details", body = CargoInspect),
  ),
))]
#[web::get("/cargoes/{name}/inspect")]
pub async fn inspect_cargo(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
  qs: web::types::Query<GenericNspQuery>,
) -> HttpResult<web::HttpResponse> {
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let key = utils::key::gen_key(&namespace, &path.1);
  let cargo = CargoDb::inspect_obj_by_pk(&key, &state).await?;
  Ok(web::HttpResponse::Ok().json(&cargo))
}

/// Create a new cargo
#[cfg_attr(feature = "dev", utoipa::path(
  post,
  tag = "Cargoes",
  path = "/cargoes",
  request_body = CargoSpecPartial,
  params(
    ("namespace" = Option<String>, Query, description = "Namespace where the cargo belongs"),
  ),
  responses(
    (status = 201, description = "Cargo created", body = Cargo),
  ),
))]
#[web::post("/cargoes")]
pub async fn create_cargo(
  state: web::types::State<SystemState>,
  path: web::types::Path<String>,
  payload: web::types::Json<CargoSpecPartial>,
  qs: web::types::Query<GenericNspQuery>,
) -> HttpResult<web::HttpResponse> {
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let obj = CargoObjCreateIn {
    namespace: namespace.clone(),
    spec: payload.into_inner(),
    version: path.into_inner(),
  };
  let cargo = CargoDb::create_obj(&obj, &state).await?;
  Ok(web::HttpResponse::Created().json(&cargo))
}

/// Delete a cargo
#[cfg_attr(feature = "dev", utoipa::path(
  delete,
  tag = "Cargoes",
  path = "/cargoes/{name}",
  params(
    ("name" = String, Path, description = "Name of the cargo"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargo belongs"),
    ("force" = bool, Query, description = "If true forces the delete operation"),
  ),
  responses(
    (status = 202, description = "Cargo deleted"),
    (status = 404, description = "Cargo does not exist"),
  ),
))]
#[web::delete("/cargoes/{name}")]
pub async fn delete_cargo(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
  qs: web::types::Query<CargoDeleteQuery>,
) -> HttpResult<web::HttpResponse> {
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let key = utils::key::gen_key(&namespace, &path.1);
  log::debug!("service::delete_cargo: {key}");
  CargoDb::del_obj_by_pk(&key, &qs, &state).await?;
  Ok(web::HttpResponse::Accepted().finish())
}

/// Create a new cargo spec and add history entry
#[cfg_attr(feature = "dev", utoipa::path(
  put,
  tag = "Cargoes",
  request_body = CargoSpecPartial,
  path = "/cargoes/{name}",
  params(
    ("name" = String, Path, description = "Name of the cargo"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargo belongs"),
  ),
  responses(
    (status = 200, description = "Cargo updated", body = Cargo),
    (status = 404, description = "Cargo does not exist"),
  ),
))]
#[web::put("/cargoes/{name}")]
pub async fn put_cargo(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
  payload: web::types::Json<CargoSpecPartial>,
  qs: web::types::Query<GenericNspQuery>,
) -> HttpResult<web::HttpResponse> {
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let key = utils::key::gen_key(&namespace, &path.1);
  let obj = &CargoObjPutIn {
    spec: payload.into_inner(),
    version: path.0.clone(),
  };
  let cargo = CargoDb::put_obj_by_pk(&key, obj, &state).await?;
  Ok(web::HttpResponse::Ok().json(&cargo))
}

/// Patch a cargo spec meaning merging current spec with the new one and add history entry
#[cfg_attr(feature = "dev", utoipa::path(
  patch,
  tag = "Cargoes",
  request_body = CargoSpecUpdate,
  path = "/cargoes/{name}",
  params(
    ("name" = String, Path, description = "Name of the cargo"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargo belongs"),
  ),
  responses(
    (status = 200, description = "Cargo updated", body = Cargo),
    (status = 404, description = "Cargo does not exist"),
  ),
))]
#[web::patch("/cargoes/{name}")]
pub async fn patch_cargo(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
  payload: web::types::Json<CargoSpecUpdate>,
  qs: web::types::Query<GenericNspQuery>,
) -> HttpResult<web::HttpResponse> {
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let key = utils::key::gen_key(&namespace, &path.1);
  let obj = &CargoObjPatchIn {
    spec: payload.into_inner(),
    version: path.0.clone(),
  };
  let cargo = CargoDb::patch_obj_by_pk(&key, obj, &state).await?;
  Ok(web::HttpResponse::Ok().json(&cargo))
}

/// List cargo histories
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Cargoes",
  path = "/cargoes/{name}/histories",
  params(
    ("name" = String, Path, description = "Name of the cargo"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargo belongs"),
  ),
  responses(
    (status = 200, description = "List of cargo histories", body = Vec<CargoSpec>),
    (status = 404, description = "Cargo does not exist"),
  ),
))]
#[web::get("/cargoes/{name}/histories")]
pub async fn list_cargo_history(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
  qs: web::types::Query<GenericNspQuery>,
) -> HttpResult<web::HttpResponse> {
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let key = utils::key::gen_key(&namespace, &path.1);
  let histories = SpecDb::read_by_kind_key(&key, &state.inner.pool)
    .await?
    .into_iter()
    .map(|e| e.try_to_cargo_spec())
    .collect::<IoResult<Vec<_>>>()?;
  Ok(web::HttpResponse::Ok().json(&histories))
}

/// Revert a cargo to a specific history
#[cfg_attr(feature = "dev", utoipa::path(
  patch,
  tag = "Cargoes",
  path = "/cargoes/{name}/histories/{id}/revert",
  params(
    ("name" = String, Path, description = "Name of the cargo"),
    ("id" = String, Path, description = "Id of the cargo history"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargo belongs"),
  ),
  responses(
    (status = 200, description = "Cargo revert", body = Cargo),
    (status = 404, description = "Cargo does not exist", body = ApiError),
  ),
))]
#[web::patch("/cargoes/{name}/histories/{id}/revert")]
pub async fn revert_cargo(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String, uuid::Uuid)>,
  qs: web::types::Query<GenericNspQuery>,
) -> HttpResult<web::HttpResponse> {
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let cargo_key = utils::key::gen_key(&namespace, &path.1);
  let spec = SpecDb::read_by_pk(&path.2, &state.inner.pool)
    .await?
    .try_to_cargo_spec()?;
  let obj = &CargoObjPutIn {
    spec: spec.into(),
    version: path.0.clone(),
  };
  let cargo = CargoDb::put_obj_by_pk(&cargo_key, obj, &state).await?;
  Ok(web::HttpResponse::Ok().json(&cargo))
}

/// Count cargoes
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Cargoes",
  path = "/cargoes/count",
  params(
    ("filter" = Option<String>, Query, description = "Generic filter", example = "{ \"filter\": { \"where\": { \"kind\": { \"eq\": \"CPU\" } } } }"),
    ("namespace" = Option<String>, Query, description = "Namespace where the cargoes are"),
  ),
  responses(
    (status = 200, description = "Count result", body = GenericCount),
  ),
))]
#[web::get("/cargoes/count")]
pub async fn count_cargo(
  state: web::types::State<SystemState>,
  qs: web::types::Query<GenericListQueryNsp>,
) -> HttpResult<web::HttpResponse> {
  let filter = utils::query_string::parse_qs_nsp_filter(&qs)?;
  let namespace = utils::key::resolve_nsp(&qs.namespace);
  let filter = filter
    .filter
    .clone()
    .unwrap_or_default()
    .r#where("namespace_name", GenericClause::Eq(namespace));
  let count = CargoDb::count_by(&filter, &state.inner.pool).await?;
  Ok(web::HttpResponse::Ok().json(&GenericCount { count }))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(create_cargo);
  config.service(delete_cargo);
  config.service(patch_cargo);
  config.service(put_cargo);
  config.service(list_cargo);
  config.service(inspect_cargo);
  config.service(list_cargo_history);
  config.service(revert_cargo);
  config.service(count_cargo);
}

#[cfg(test)]
mod tests {
  use ntex::http;

  use nanocl_stubs::cargo_spec::{CargoSpec, CargoSpecPartial};
  use nanocl_stubs::cargo::{
    Cargo, CargoSummary, CargoInspect, CargoDeleteQuery, CargoKillOptions,
  };

  use crate::utils::tests::*;

  const ENDPOINT: &str = "/cargoes";

  /// Test to create start patch stop and delete a cargo with valid data
  #[ntex::test]
  async fn basic() {
    let system = gen_default_test_system().await;
    let client = system.client;
    let test_cargoes = [
      "1daemon-test-cargo",
      "2another-test-cargo",
      "2daemon-test-cargo",
    ];
    let main_test_cargo = test_cargoes[0];
    for test_cargo in test_cargoes.iter() {
      let test_cargo = test_cargo.to_owned();
      let res = client
        .send_post(
          ENDPOINT,
          Some(&CargoSpecPartial {
            name: test_cargo.to_owned(),
            container: bollard_next::container::Config {
              image: Some(
                "ghcr.io/next-hat/nanocl-get-started:latest".to_owned(),
              ),
              ..Default::default()
            },
            ..Default::default()
          }),
          None::<String>,
        )
        .await;
      test_status_code!(
        res.status(),
        http::StatusCode::CREATED,
        "basic cargo create"
      );
      let cargo = TestClient::res_json::<Cargo>(res).await;
      assert_eq!(cargo.spec.name, test_cargo, "Invalid cargo name");
      assert_eq!(cargo.namespace_name, "global", "Invalid cargo namespace");
      assert_eq!(
        cargo.spec.container.image,
        Some("ghcr.io/next-hat/nanocl-get-started:latest".to_owned())
      );
    }
    let mut res = client
      .send_get(
        &format!("{ENDPOINT}/{main_test_cargo}/inspect"),
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "basic cargo inspect"
    );
    let response = res.json::<CargoInspect>().await.unwrap();
    assert_eq!(
      response.spec.name, main_test_cargo,
      "Expected to find cargo with name {main_test_cargo} got {}",
      response.spec.name
    );
    let mut res = client.send_get(ENDPOINT, None::<String>).await;
    test_status_code!(res.status(), http::StatusCode::OK, "basic cargo list");
    let cargoes = res.json::<Vec<CargoSummary>>().await.unwrap();
    assert!(!cargoes.is_empty(), "Expected to find cargoes");
    let res = client
      .send_post(
        &format!("/processes/cargo/{main_test_cargo}/start"),
        None::<String>,
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::ACCEPTED,
      "basic cargo start"
    );
    let res = client
      .send_post(
        &format!("/processes/cargo/{main_test_cargo}/kill"),
        Some(&CargoKillOptions {
          signal: "SIGINT".to_owned(),
        }),
        None::<String>,
      )
      .await;
    test_status_code!(res.status(), http::StatusCode::OK, "basic cargo kill");
    let res = client
      .send_post(
        &format!("/processes/cargo/{main_test_cargo}/restart"),
        None::<String>,
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::ACCEPTED,
      "basic cargo restart"
    );
    let mut res = client
      .send_put(
        &format!("{ENDPOINT}/{main_test_cargo}"),
        Some(&CargoSpecPartial {
          name: main_test_cargo.to_owned(),
          container: bollard_next::container::Config {
            image: Some(
              "ghcr.io/next-hat/nanocl-get-started:latest".to_owned(),
            ),
            env: Some(vec!["TEST=1".to_owned()]),
            ..Default::default()
          },
          ..Default::default()
        }),
        None::<String>,
      )
      .await;
    test_status_code!(res.status(), http::StatusCode::OK, "basic cargo patch");
    let patch_response = res.json::<Cargo>().await.unwrap();
    assert_eq!(patch_response.spec.name, main_test_cargo);
    assert_eq!(patch_response.namespace_name, "global");
    assert_eq!(
      patch_response.spec.container.image,
      Some("ghcr.io/next-hat/nanocl-get-started:latest".to_owned())
    );
    assert_eq!(
      patch_response.spec.container.env,
      Some(vec!["TEST=1".to_owned()])
    );
    let mut res = client
      .send_get(
        &format!("{ENDPOINT}/{main_test_cargo}/histories"),
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "basic cargo history"
    );
    let histories = res.json::<Vec<CargoSpec>>().await.unwrap();
    assert!(histories.len() > 1, "Expected to find cargo histories");
    let id = histories[0].key;
    let res = client
      .send_patch(
        &format!("{ENDPOINT}/{main_test_cargo}/histories/{id}/revert"),
        None::<String>,
        None::<String>,
      )
      .await;
    test_status_code!(res.status(), http::StatusCode::OK, "basic cargo revert");
    let res = client
      .send_post(
        &format!("/processes/cargo/{main_test_cargo}/stop"),
        None::<String>,
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::ACCEPTED,
      "basic cargo stop"
    );
    for test_cargo in test_cargoes.iter() {
      let res = client
        .send_delete(
          &format!("{ENDPOINT}/{test_cargo}"),
          Some(CargoDeleteQuery {
            force: Some(true),
            ..Default::default()
          }),
        )
        .await;
      test_status_code!(
        res.status(),
        http::StatusCode::ACCEPTED,
        "basic cargo delete"
      );
    }
    ntex::time::sleep(std::time::Duration::from_secs(1)).await;
    system.state.wait_event_loop().await;
  }
}
