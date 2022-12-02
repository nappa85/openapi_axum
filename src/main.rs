use std::sync::Arc;

use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    openapi,
    redoc::Redoc,
};

use axum::{routing::get, Extension, Json, Server};

use once_cell::sync::OnceCell;

use schemars::JsonSchema;

use serde::{Deserialize, Serialize};

static OPENAPI_JSON: OnceCell<String> = OnceCell::new();
static OPENAPI_YAML: OnceCell<String> = OnceCell::new();

async fn serve_json(Extension(api): Extension<Arc<openapi::OpenApi>>) -> impl IntoApiResponse {
    OPENAPI_JSON
        .get_or_init(|| serde_json::to_string(api.as_ref()).unwrap())
        .as_str()
}
async fn serve_yaml(Extension(api): Extension<Arc<openapi::OpenApi>>) -> impl IntoApiResponse {
    OPENAPI_YAML
        .get_or_init(|| serde_yaml::to_string(api.as_ref()).unwrap())
        .as_str()
}

fn get_v1_router() -> ApiRouter {
    ApiRouter::new().api_route(
        "/foo",
        get_with(foo, |o| {
            o.id("foo")
                .description("Example method")
                .tag("bar")
                .response_with::<200, Json<Foo>, _>(|res| {
                    res.description("successful operation")
                        .example(Foo::get_example())
                })
        }),
    )
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct Foo {
    a: u8,
    b: String,
    c: Vec<f64>,
}

impl Foo {
    fn get_example() -> Self {
        Foo {
            a: 1,
            b: "foo".to_string(),
            c: vec![0.0, 0.1, 0.2],
        }
    }
}

async fn foo(Json(foo): Json<Foo>) -> Json<Foo> {
    Json(foo)
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();

    let mut api = openapi::OpenApi {
        info: openapi::Info {
            title: "Example API".to_string(),
            description: Some("Really cool description".to_string()),
            version: "1.0.0".to_string(),
            license: Some(openapi::License {
                name: "Apache-2.0".to_string(),
                url: Some("https://www.apache.org/licenses/LICENSE-2.0.html".to_string()),
                ..openapi::License::default()
            }),
            ..openapi::Info::default()
        },
        servers: vec![openapi::Server {
            url: "https://api.example.com".to_string(),
            ..openapi::Server::default()
        }],
        components: Some(openapi::Components {
            security_schemes: [(
                "my_auth_key".to_string(),
                openapi::ReferenceOr::Item(openapi::SecurityScheme::ApiKey {
                    name: "X-MY-KEY".to_string(),
                    location: openapi::ApiKeyLocation::Header,
                    description: Some("The **Api key**.".to_string()),
                    extensions: [].into(),
                }),
            )]
            .into(),
            ..openapi::Components::default()
        }),
        ..openapi::OpenApi::default()
    };

    let routes = ApiRouter::new()
        .nest("/v1", get_v1_router())
        .route("/openapi.json", get(serve_json))
        .route("/openapi.yaml", get(serve_yaml))
        .route("/redoc", Redoc::new("/openapi.json").axum_route())
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)));

    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
