use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    openapi,
    redoc::Redoc,
};

use axum::{routing::get, Json, Server};

use once_cell::sync::OnceCell;

use schemars::JsonSchema;

use serde::{Deserialize, Serialize};

static OPENAPI_JSON: OnceCell<String> = OnceCell::new();
static OPENAPI_YAML: OnceCell<String> = OnceCell::new();
fn store_openapi(api: openapi::OpenApi) {
    OPENAPI_JSON
        .set(serde_json::to_string(&api).unwrap())
        .unwrap();
    OPENAPI_YAML
        .set(serde_yaml::to_string(&api).unwrap())
        .unwrap();
}
async fn serve_json() -> impl IntoApiResponse {
    OPENAPI_JSON.get().unwrap().as_str()
}
async fn serve_yaml() -> impl IntoApiResponse {
    OPENAPI_YAML.get().unwrap().as_str()
}

fn get_v1_router() -> ApiRouter {
    ApiRouter::new().api_route(
        "/foo",
        get_with(foo, |o| {
            o.id("foo")
                .description("Example method")
                .tag("bar")
                .response_with::<200, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<201, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<202, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<203, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<204, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<205, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<206, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<207, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<208, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<209, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<210, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<211, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<212, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<213, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<214, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<215, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<216, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<217, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<218, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<219, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<220, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<221, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<222, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<223, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<224, Json<Foo>, _>(|res| res.description("successful operation"))
                .response_with::<225, Json<Foo>, _>(|res| res.description("successful operation"))
        }),
    )
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[schemars(example = "Foo::get_example")]
struct Foo {
    a: u8,
    b: Option<String>,
    c: Vec<f64>,
}

impl Foo {
    fn get_example() -> Self {
        Foo {
            a: 1,
            b: Some("foo".to_string()),
            c: vec![0.0, 0.1, 0.2],
        }
    }
}

async fn foo(Json(json): Json<Foo>) -> Json<Foo> {
    Json(json)
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
            schemas: {
                let mut gen = schemars::gen::SchemaGenerator::default();
                [(
                    Foo::schema_name(),
                    openapi::SchemaObject {
                        json_schema: Foo::json_schema(&mut gen),
                        external_docs: None,
                        example: None,
                    },
                )]
                .into()
            },
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
        .finish_api(&mut api);
    store_openapi(api);
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
