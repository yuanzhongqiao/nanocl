/*
* Endpoints to manipulate resources
*/
use ntex::web;

use nanocl_error::http::HttpResult;

use nanocl_stubs::{
  generic::{GenericClause, GenericCount, GenericFilter, GenericListQuery},
  resource::{ResourcePartial, ResourceSpec, ResourceUpdate},
};

use crate::{
  models::{ResourceDb, SpecDb, SystemState},
  objects::generic::*,
  repositories::generic::*,
  utils,
};

/// List resources
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Resources",
  path = "/resources",
  params(
    ("filter" = Option<String>, Query, description = "Generic filter", example = "{ \"filter\": { \"where\": { \"kind\": { \"eq\": \"ncproxy.io/rule\" } } } }"),
  ),
  responses(
    (status = 200, description = "List of resources", body = [Resource]),
  ),
))]
#[web::get("/resources")]
pub async fn list_resource(
  state: web::types::State<SystemState>,
  qs: web::types::Query<GenericListQuery>,
) -> HttpResult<web::HttpResponse> {
  let filter = utils::query_string::parse_qs_filter(&qs)?;
  let items = ResourceDb::transform_read_by(&filter, &state.inner.pool).await?;
  Ok(web::HttpResponse::Ok().json(&items))
}

/// Get detailed information about a resource
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Resources",
  path = "/resources/{name}/inspect",
  params(
    ("name" = String, Path, description = "The resource name to inspect")
  ),
  responses(
    (status = 200, description = "Detailed information about a resource", body = Resource),
    (status = 404, description = "Resource is not existing", body = ApiError),
  ),
))]
#[web::get("/resources/{name}/inspect")]
pub async fn inspect_resource(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
) -> HttpResult<web::HttpResponse> {
  let resource =
    ResourceDb::transform_read_by_pk(&path.1, &state.inner.pool).await?;
  Ok(web::HttpResponse::Ok().json(&resource))
}

/// Create a resource
#[cfg_attr(feature = "dev", utoipa::path(
  post,
  request_body = ResourcePartial,
  tag = "Resources",
  path = "/resources",
  responses(
    (status = 200, description = "The created resource", body = Resource),
  ),
))]
#[web::post("/resources")]
pub async fn create_resource(
  state: web::types::State<SystemState>,
  payload: web::types::Json<ResourcePartial>,
) -> HttpResult<web::HttpResponse> {
  let resource = ResourceDb::create_obj(&payload, &state).await?;
  Ok(web::HttpResponse::Created().json(&resource))
}

/// Delete a resource
#[cfg_attr(feature = "dev", utoipa::path(
  delete,
  tag = "Resources",
  path = "/resources/{name}",
  params(
    ("name" = String, Path, description = "The resource name to delete")
  ),
  responses(
    (status = 202, description = "The resource and his history has been deleted"),
    (status = 404, description = "Resource is not existing", body = ApiError),
  ),
))]
#[web::delete("/resources/{name}")]
pub async fn delete_resource(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
) -> HttpResult<web::HttpResponse> {
  ResourceDb::del_obj_by_pk(&path.1, &(), &state).await?;
  Ok(web::HttpResponse::Accepted().finish())
}

/// Create a new resource spec and add history entry
#[cfg_attr(feature = "dev", utoipa::path(
  put,
  request_body = ResourceUpdate,
  tag = "Resources",
  path = "/resources/{name}",
  params(
    ("name" = String, Path, description = "Name of the resource")
  ),
  responses(
    (status = 200, description = "Resource updated", body = Resource),
    (status = 404, description = "Resource does not exit", body = ApiError),
  ),
))]
#[web::put("/resources/{name}")]
pub async fn put_resource(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
  payload: web::types::Json<ResourceUpdate>,
) -> HttpResult<web::HttpResponse> {
  let resource =
    ResourceDb::transform_read_by_pk(&path.1, &state.inner.pool).await?;
  let new_resource = ResourcePartial {
    name: path.1.clone(),
    kind: resource.kind,
    data: payload.data.clone(),
    metadata: payload.metadata.clone(),
  };
  let resource =
    ResourceDb::put_obj_by_pk(&path.1, &new_resource, &state).await?;
  Ok(web::HttpResponse::Ok().json(&resource))
}

/// List resource history
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Resources",
  path = "/resources/{name}/histories",
  params(
    ("name" = String, Path, description = "The resource name to list history")
  ),
  responses(
    (status = 200, description = "The resource history", body = [ResourceSpec]),
    (status = 404, description = "Resource is not existing", body = ApiError),
  ),
))]
#[web::get("/resources/{name}/histories")]
pub async fn list_resource_history(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String)>,
) -> HttpResult<web::HttpResponse> {
  let filter =
    GenericFilter::new().r#where("kind_key", GenericClause::Eq(path.1.clone()));
  let items = SpecDb::read_by(&filter, &state.inner.pool)
    .await?
    .into_iter()
    .map(ResourceSpec::from)
    .collect::<Vec<_>>();
  Ok(web::HttpResponse::Ok().json(&items))
}

/// Revert a resource to a specific history
#[cfg_attr(feature = "dev", utoipa::path(
  patch,
  tag = "Resources",
  path = "/resources/{name}/histories/{id}/revert",
  params(
    ("name" = String, Path, description = "The resource name to revert"),
    ("id" = String, Path, description = "The resource history id to revert to")
  ),
  responses(
    (status = 200, description = "The resource has been revert", body = Resource),
    (status = 404, description = "Resource is not existing", body = ApiError),
  ),
))]
#[web::patch("/resources/{name}/histories/{id}/revert")]
pub async fn revert_resource(
  state: web::types::State<SystemState>,
  path: web::types::Path<(String, String, uuid::Uuid)>,
) -> HttpResult<web::HttpResponse> {
  let history = SpecDb::read_by_pk(&path.2, &state.inner.pool).await?;
  let resource =
    ResourceDb::transform_read_by_pk(&path.1, &state.inner.pool).await?;
  let new_resource = ResourcePartial {
    name: resource.spec.resource_key,
    kind: resource.kind,
    data: history.data,
    metadata: history.metadata,
  };
  let resource =
    ResourceDb::put_obj_by_pk(&path.1, &new_resource, &state).await?;
  Ok(web::HttpResponse::Ok().json(&resource))
}

/// Count resources
#[cfg_attr(feature = "dev", utoipa::path(
  get,
  tag = "Resources",
  path = "/resources/count",
  params(
    ("filter" = Option<String>, Query, description = "Generic filter", example = "{ \"filter\": { \"where\": { \"name\": { \"eq\": \"global\" } } } }"),
  ),
  responses(
    (status = 200, description = "Count result", body = GenericCount),
  ),
))]
#[web::get("/resources/count")]
pub async fn count_resource(
  state: web::types::State<SystemState>,
  qs: web::types::Query<GenericListQuery>,
) -> HttpResult<web::HttpResponse> {
  let filter = utils::query_string::parse_qs_filter(&qs)?;
  let count = ResourceDb::count_by(&filter, &state.inner.pool).await?;
  Ok(web::HttpResponse::Ok().json(&GenericCount { count }))
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(create_resource);
  config.service(delete_resource);
  config.service(list_resource);
  config.service(inspect_resource);
  config.service(put_resource);
  config.service(count_resource);
  config.service(list_resource_history);
  config.service(revert_resource);
}

#[cfg(test)]
mod tests {
  use ntex::http;
  use nanocl_stubs::{
    resource::{Resource, ResourcePartial, ResourceUpdate},
    generic::{GenericFilter, GenericClause, GenericListQuery},
    resource_kind::{ResourceKindPartial, ResourceKindSpec},
  };

  use crate::utils::tests::*;

  const ENDPOINT: &str = "/resources";

  #[ntex::test]
  async fn basic() {
    const TEST_RESOURCE: &str = "test_resource";
    const TEST_RESOURCE_KIND: &str = "test.io/test-resource";
    const TEST_RESOURCE_KIND_VERSION: &str = "v1";
    let system = gen_default_test_system().await;
    let client = system.client;
    let spec = serde_json::json!({
      "Schema": {
        "title": "VpnUser",
        "description": "Create a new vpn user",
        "type": "object",
        "required": [
          "Username"
        ],
        "properties": {
          "Username": {
            "description": "Username for the vpn user",
            "type": "string"
          },
          "Password": {
            "description": "Password for the vpn user",
            "type": "string"
          }
        }
      }
    });
    let payload = ResourceKindPartial {
      name: TEST_RESOURCE_KIND.to_owned(),
      version: TEST_RESOURCE_KIND_VERSION.to_owned(),
      metadata: None,
      data: ResourceKindSpec {
        schema: Some(spec),
        url: None,
      },
    };
    let res = client
      .send_post("/resource/kinds", Some(&payload), None::<String>)
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::CREATED,
      "create resource kind"
    );
    let data = serde_json::json!({
      "Username": "test",
    });
    let resource = ResourcePartial {
      name: TEST_RESOURCE.to_owned(),
      kind: TEST_RESOURCE_KIND.to_owned(),
      data: data.clone(),
      metadata: Some(serde_json::json!({
        "Test": "gg",
      })),
    };
    let mut res = client
      .send_post(ENDPOINT, Some(&resource), None::<String>)
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::CREATED,
      "create resource"
    );
    let resource = res.json::<Resource>().await.unwrap();
    assert_eq!(resource.spec.resource_key, TEST_RESOURCE);
    assert_eq!(resource.kind, TEST_RESOURCE_KIND);
    // Basic list
    let mut res = client.send_get(ENDPOINT, None::<String>).await;
    test_status_code!(res.status(), http::StatusCode::OK, "list resource");
    let _ = res.json::<Vec<Resource>>().await.unwrap();
    // Using filter exists
    let filter = GenericFilter::new()
      .r#where("data", GenericClause::HasKey("Username".to_owned()));
    let query = GenericListQuery::try_from(filter).unwrap();
    let mut res = client.send_get(ENDPOINT, Some(&query)).await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "filter resource by data HasKey"
    );
    let resources = res.json::<Vec<Resource>>().await.unwrap();
    assert!(
      resources.len() == 1,
      "Expect 1 resource when filter by data HasKey"
    );
    let filter = GenericFilter::new().r#where(
      "data",
      GenericClause::Contains(serde_json::json!({
        "Username": "test"
      })),
    );
    let query = GenericListQuery::try_from(filter).unwrap();
    let mut res = client.send_get(ENDPOINT, Some(&query)).await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "filter resource by data contains"
    );
    let resources = res.json::<Vec<Resource>>().await.unwrap();
    assert!(
      resources.len() == 1,
      "Expect 1 resource when filter by data contains"
    );
    let filter = GenericFilter::new()
      .r#where("metadata", GenericClause::HasKey("Test".to_owned()));
    let query = GenericListQuery::try_from(filter).unwrap();
    let mut res = client.send_get(ENDPOINT, Some(&query)).await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "filter resource by meta exists"
    );
    let resources = res.json::<Vec<Resource>>().await.unwrap();
    assert!(
      resources.len() == 1,
      "Expect 1 resource when filter by metadata HasKey"
    );
    let filter = GenericFilter::new().r#where(
      "metadata",
      GenericClause::Contains(serde_json::json!({
        "Test": "gg",
      })),
    );
    let query = GenericListQuery::try_from(filter).unwrap();
    let mut res = client.send_get(ENDPOINT, Some(&query)).await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "filter resource by meta contains"
    );
    let resources = res.json::<Vec<Resource>>().await.unwrap();
    assert!(
      resources.len() == 1,
      "Expect 1 resource when filter by meta contains"
    );
    // Inspect
    let mut res = client
      .send_get(
        &format!("{ENDPOINT}/{TEST_RESOURCE}/inspect"),
        None::<String>,
      )
      .await;
    test_status_code!(res.status(), http::StatusCode::OK, "inspect resource");
    let resource = res.json::<Resource>().await.unwrap();
    assert_eq!(resource.spec.resource_key, TEST_RESOURCE);
    assert_eq!(&resource.kind, TEST_RESOURCE_KIND);
    assert_eq!(&resource.spec.data, &data);
    // History
    let _ = client
      .send_get(
        &format!("{ENDPOINT}/{TEST_RESOURCE}/histories"),
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::OK,
      "list resource history"
    );
    let data = serde_json::json!({
      "Username": "test_update",
    });
    let new_resource = ResourceUpdate {
      data: data.clone(),
      metadata: None,
    };
    let mut res = client
      .send_put(
        &format!("{ENDPOINT}/{TEST_RESOURCE}"),
        Some(&new_resource),
        None::<String>,
      )
      .await;
    test_status_code!(res.status(), http::StatusCode::OK, "patch resource");
    let resource = res.json::<Resource>().await.unwrap();
    assert_eq!(resource.spec.resource_key, TEST_RESOURCE);
    assert_eq!(&resource.kind, TEST_RESOURCE_KIND);
    // Delete
    let resp = client
      .send_delete(&format!("{ENDPOINT}/{TEST_RESOURCE}"), None::<String>)
      .await;
    test_status_code!(
      resp.status(),
      http::StatusCode::ACCEPTED,
      "delete resource"
    );
    let res = client
      .send_delete(
        &format!("/resource/kinds/{TEST_RESOURCE_KIND}"),
        None::<String>,
      )
      .await;
    test_status_code!(
      res.status(),
      http::StatusCode::ACCEPTED,
      "delete resource kind"
    );
  }
}
