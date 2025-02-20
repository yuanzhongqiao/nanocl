use ntex::web;

use nanocl_error::http::HttpResult;

use nanocl_stubs::{
  generic::{GenericCount, GenericListQuery},
  resource_kind::{ResourceKindPartial, ResourceKindVersion},
};

use crate::{
  utils,
  repositories::generic::*,
  models::{ResourceKindDb, SpecDb, SystemState},
};

/// List resource kinds
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "ResourceKinds",
  path = "/resource/kinds",
  params(
    ("filter" = Option<String>, Query, description = "Generic filter", example = "{ \"filter\": { \"where\": { \"name\": { \"eq\": \"test\" } } } }"),
  ),
  responses(
    (status = 200, description = "List of jobs", body = [ResourceKind]),
  ),
))]
#[web::get("/resource/kinds")]
pub async fn list_resource_kind(
  state: web::types::State<SystemState>,
  _version: web::types::Path<String>,
  qs: web::types::Query<GenericListQuery>,
) -> HttpResult<web::HttpResponse> {
  let filter = utils::query_string::parse_qs_filter(&qs)?;
  let resource_kinds =
    ResourceKindDb::transform_read_by(&filter, &state.inner.pool).await?;
  Ok(web::HttpResponse::Ok().json(&resource_kinds))
}

/// Create a resource kind
#[cfg_attr(feature = "dev", utoipa::path(
  post,
  tag = "ResourceKinds",
  path = "/resource/kinds",
  request_body = ResourceKindPartial,
  responses(
    (status = 201, description = "Job created", body = ResourceKind),
  ),
))]
#[web::post("/resource/kinds")]
pub async fn create_resource_kind(
  state: web::types::State<SystemState>,
  _version: web::types::Path<String>,
  payload: web::types::Json<ResourceKindPartial>,
) -> HttpResult<web::HttpResponse> {
  let item =
    ResourceKindDb::create_from_spec(&payload, &state.inner.pool).await?;
  Ok(web::HttpResponse::Created().json(&item))
}

/// Delete a resource kind
#[cfg_attr(feature = "dev", utoipa::path(
  delete,
  tag = "ResourceKinds",
  path = "/resource/kinds/{domain}/{name}",
  params(
    ("domain" = String, Path, description = "Domain of the resource kind"),
    ("name" = String, Path, description = "Name of the resource kind"),
  ),
  responses(
    (status = 202, description = "Resource kind deleted"),
    (status = 404, description = "Resource kind does not exist"),
  ),
))]
#[web::delete("/resource/kinds/{domain}/{name}")]
pub async fn delete_resource_kind(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String, String)>,
) -> HttpResult<web::HttpResponse> {
  let key = format!("{}/{}", path.1, path.2);
  ResourceKindDb::read_by_pk(&key, &state.inner.pool).await?;
  ResourceKindDb::del_by_pk(&key, &state.inner.pool).await?;
  SpecDb::del_by_kind_key(&key, &state.inner.pool).await?;
  Ok(web::HttpResponse::Accepted().into())
}

/// Inspect a resource kind
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "ResourceKinds",
  path = "/resource/kinds/{domain}/{name}/inspect",
  params(
    ("domain" = String, Path, description = "Domain of the resource kind"),
    ("name" = String, Path, description = "Name of the resource kind"),
  ),
  responses(
    (status = 200, description = "Details about a resource kind", body = ResourceKindInspect),
  ),
))]
#[web::get("/resource/kinds/{domain}/{name}/inspect")]
pub async fn inspect_resource_kind(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String, String)>,
) -> HttpResult<web::HttpResponse> {
  let key: String = format!("{}/{}", path.1, path.2);
  let kind = ResourceKindDb::inspect_by_pk(&key, &state.inner.pool).await?;
  Ok(web::HttpResponse::Ok().json(&kind))
}

/// Inspect a specific version of a resource kind
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "ResourceKinds",
  path = "/resource/kinds/{domain}/{name}/version/{version}",
  params(
    ("domain" = String, Path, description = "Domain of the resource kind"),
    ("name" = String, Path, description = "Name of the resource kind"),
  ),
  responses(
    (status = 200, description = "Details about a resource kind", body = ResourceKindVersion),
  ),
))]
#[web::get("/resource/kinds/{domain}/{name}/version/{version}/inspect")]
pub async fn inspect_resource_kind_version(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String, String, String)>,
) -> HttpResult<web::HttpResponse> {
  let key = format!("{}/{}", path.1, path.2);
  let kind_version =
    SpecDb::get_version(&key, &path.3, &state.inner.pool).await?;
  let kind_version: ResourceKindVersion = kind_version.try_into()?;
  Ok(web::HttpResponse::Ok().json(&kind_version))
}

/// Count resource kinds
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "ResourceKinds",
  path = "/resource/kinds/count",
  params(
    ("filter" = Option<String>, Query, description = "Generic filter", example = "{ \"filter\": { \"where\": { \"name\": { \"eq\": \"global\" } } } }"),
  ),
  responses(
    (status = 200, description = "Count result", body = GenericCount),
  ),
))]
#[web::get("/resource/kinds/count")]
pub async fn count_resource_kind(
  state: web::types::State<SystemState>,
  qs: web::types::Query<GenericListQuery>,
) -> HttpResult<web::HttpResponse> {
  let filter = utils::query_string::parse_qs_filter(&qs)?;
  let count = ResourceKindDb::count_by(&filter, &state.inner.pool).await?;
  Ok(web::HttpResponse::Ok().json(&GenericCount { count }))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config
    .service(list_resource_kind)
    .service(create_resource_kind)
    .service(delete_resource_kind)
    .service(inspect_resource_kind)
    .service(count_resource_kind)
    .service(inspect_resource_kind_version);
}

#[cfg(test)]
mod tests {
  use ntex::http;

  const ENDPOINT: &str = "/resource/kinds";

  use crate::utils::tests::*;

  use nanocl_stubs::resource_kind::{
    ResourceKind, ResourceKindPartial, ResourceKindSpec, ResourceKindInspect,
    ResourceKindVersion,
  };

  #[ntex::test]
  async fn test_inspect_version_not_found() {
    let system = gen_default_test_system().await;
    let client = system.client;
    let res = client
      .send_get(
        &format!("{}/test.io/api-test/version/v12/inspect", ENDPOINT),
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::NOT_FOUND,
      "resource kind inspect version"
    );
  }

  #[ntex::test]
  async fn test_wrong_name() {
    let system = gen_default_test_system().await;
    let client = system.client;
    let payload = ResourceKindPartial {
      name: "api-test".to_owned(),
      version: "v1".to_owned(),
      metadata: None,
      data: ResourceKindSpec {
        schema: None,
        url: Some("unix:///run/nanocl/proxy.sock".to_owned()),
      },
    };
    let res = client
      .send_post(ENDPOINT, Some(&payload), None::<String>)
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::BAD_REQUEST,
      "resource kind create"
    );
  }

  #[ntex::test]
  async fn test_wrong_spec() {
    let system = gen_default_test_system().await;
    let client = system.client;
    let payload = ResourceKindPartial {
      name: "test.io/api-test".to_owned(),
      version: "v1".to_owned(),
      metadata: None,
      data: ResourceKindSpec {
        schema: None,
        url: None,
      },
    };
    let res = client
      .send_post(ENDPOINT, Some(&payload), None::<String>)
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::BAD_REQUEST,
      "resource kind create"
    );
  }
  #[ntex::test]
  async fn basic_list() {
    let system = gen_default_test_system().await;
    let client = system.client;
    // Create
    let payload = ResourceKindPartial {
      name: "test.io/api-test".to_owned(),
      version: "v1".to_owned(),
      metadata: None,
      data: ResourceKindSpec {
        schema: None,
        url: Some("unix:///run/nanocl/proxy.sock".to_owned()),
      },
    };
    let mut res = client
      .send_post(ENDPOINT, Some(&payload), None::<String>)
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::CREATED,
      "resource kind create"
    );
    let kind = res.json::<ResourceKind>().await.unwrap();
    assert_eq!(kind.name, payload.name);
    assert_eq!(kind.version, payload.version);
    // List
    let mut res = client.send_get(ENDPOINT, None::<String>).await;
    test_status_code!(res.status(), http::StatusCode::OK, "resource kind list");
    let items = res.json::<Vec<ResourceKind>>().await.unwrap();
    assert!(items.iter().any(|i| i.name == payload.name));
    // Inspect
    let mut res = client
      .send_get(
        &format!("{}/{}/inspect", ENDPOINT, payload.name),
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "resource kind inspect"
    );
    let kind = res.json::<ResourceKindInspect>().await.unwrap();
    assert_eq!(kind.name, payload.name);
    // Inspect version
    let mut res = client
      .send_get(
        &format!(
          "{}/{}/version/{}/inspect",
          ENDPOINT, payload.name, payload.version
        ),
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "resource kind inspect version"
    );
    let _ = res.json::<ResourceKindVersion>().await.unwrap();
    // Delete
    let res = client
      .send_delete(&format!("{}/{}", ENDPOINT, payload.name), None::<String>)
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::ACCEPTED,
      "resource kind delete"
    );
  }
}
