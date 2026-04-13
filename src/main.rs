mod auth;
mod html;
mod odata_services;

use crate::{
    html::{gen_page, render_collection},
    odata_services::{find_service, SERVICES},
};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use parse_sap_atom_feed::atom::service::AtomService;
use reqwest::Client;
use std::{process::exit, str::FromStr};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
parse_sap_odata::include_mod!("service_project_service_v2");
parse_sap_odata::include_mod!("service_project_service_v2_metadata");
parse_sap_odata::include_mod!("service_project_partner_service_v2");
parse_sap_odata::include_mod!("service_project_partner_service_v2_metadata");
parse_sap_odata::include_mod!("public_api_digital_twin_service");
parse_sap_odata::include_mod!("public_api_digital_twin_service_metadata");
parse_sap_odata::include_mod!("business_partner_network_public_api_business_partner_service");
parse_sap_odata::include_mod!(
    "business_partner_network_public_api_business_partner_service_metadata"
);

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Shared application state
struct AppState {
    api_key: String,
    client: Client,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// GET /  – Display OData service catalogue
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[get("/")]
async fn index() -> impl Responder {
    let mut items = String::new();
    for svc in SERVICES {
        items.push_str(&format!(
            "<li><a href=\"/service/{id}\">{name}</a></li>\n",
            id = svc.id,
            name = svc.display_name,
        ));
    }
    let body = format!("<p>Select a service to browse its collections:</p><ul>{items}</ul>");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(gen_page("OData Services", &body))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// GET /service/{svc} – Fetch service document for selected OData service
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[get("/service/{svc}")]
async fn service_index(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let svc_id = path.into_inner();
    let Some(svc) = find_service(&svc_id) else {
        return HttpResponse::NotFound()
            .content_type("text/html; charset=utf-8")
            .body(gen_page(
                "Not Found",
                "<p class=\"error\">Unknown service.</p>",
            ));
    };

    match fetch_service_doc(&state.client, svc.base_url, &state.api_key).await {
        Err(e) => {
            let body = format!("<p class=\"error\">Failed to fetch service document: {e}</p>");
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(gen_page(svc.display_name, &body))
        }
        Ok(doc) => {
            let base = &doc.base_url;
            let mut items = String::new();
            for col in &doc.workspace.collections {
                items.push_str(&format!(
                    "<li><a href=\"/service/{svc_id}/col/{href}\">{title}</a> \
                     &nbsp;<small>({href})</small></li>\n",
                    href = col.href,
                    title = col.title,
                ));
            }
            let body = format!(
                "<a class=\"back\" href=\"/\">&#8592; Back to services</a>\
                 <p>Base URL: <code>{base}</code></p>\
                 <p>Collections:</p><ul>{items}</ul>"
            );
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(gen_page(svc.display_name, &body))
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// GET /service/{svc}/col/{name} – Fetch an OData collection from the selected service
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[get("/service/{svc}/col/{name}")]
async fn collection(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (svc_id, col_name) = path.into_inner();
    let Some(svc) = find_service(&svc_id) else {
        return HttpResponse::NotFound()
            .content_type("text/html; charset=utf-8")
            .body(gen_page(
                "Not Found",
                "<p class=\"error\">Unknown service.</p>",
            ));
    };

    let url = format!("{}/{col_name}", svc.base_url);
    match fetch_raw(&state.client, &url, &state.api_key, "application/atom+xml").await {
        Err(e) => {
            let body = format!("<p class=\"error\">Request failed: {e}</p>");
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(gen_page(&col_name, &body))
        }
        Ok(xml) => {
            let body = render_collection(&svc_id, &col_name, &xml);
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(gen_page(
                    &format!("{} – {col_name}", svc.display_name),
                    &body,
                ))
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// HTTP helpers
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
async fn fetch_service_doc(
    client: &Client,
    base_url: &str,
    api_key: &str,
) -> Result<AtomService, String> {
    let xml = fetch_raw(client, base_url, api_key, "application/atomsvc+xml").await?;
    AtomService::from_str(&xml).map_err(|e| {
        log::error!("Service document parse error: {e}\nRaw XML:\n{xml}");
        e.to_string()
    })
}

async fn fetch_raw(
    client: &Client,
    url: &str,
    api_key: &str,
    accept: &str,
) -> Result<String, String> {
    let resp = client
        .get(url)
        .header("apikey", api_key)
        .header("Accept", accept)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    resp.text().await.map_err(|e| e.to_string())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Entry point
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let api_key = auth::fetch_auth().unwrap_or_else(|err_txt| {
        eprintln!("{err_txt}");
        exit(1);
    });

    let client = Client::builder()
        .build()
        .expect("failed to build HTTP client");

    let state = web::Data::new(AppState { api_key, client });

    let addr = "127.0.0.1:8080";
    log::info!("Starting HTTP server at http://{addr}");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(service_index)
            .service(collection)
    })
    .bind(addr)?
    .run()
    .await
}
