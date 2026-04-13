mod auth;
mod html;
mod odata_services;

use crate::{
    html::gen_page,
    odata_services::{find_service, SERVICES},
};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use parse_sap_atom_feed::atom::{feed::Feed, service::AtomService};
use reqwest::Client;
use std::{process::exit, str::FromStr};

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
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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
// Dispatch (service, collection) → typed Feed parse → HTML table
//
// All types across OData services must be fully qualified to avoid name collisions
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn render_collection(svc_id: &str, col_name: &str, xml: &str) -> String {
    let back =
        format!("<a class=\"back\" href=\"/service/{svc_id}\">&#8592; Back to collections</a>");

    let table = match (svc_id, col_name) {
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Project Service V2
        ("project-service-v2", "GroupMemberships") => {
            render_feed::<service_project_service_v2::GroupMemberships>(
                xml,
                service_project_service_v2_metadata::GroupMembershipsMetadata::field_names()
            )
        }
        ("project-service-v2", "Groups") => render_feed::<service_project_service_v2::Groups>(
            xml,
            &["groupId", "type", "projectId", "name", "description"],
        ),
        ("project-service-v2", "Users") => render_feed::<service_project_service_v2::Users>(
            xml,
            service_project_service_v2_metadata::UsersMetadata::field_names()
        ),

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Business Partner Service
        ("business-partner-service", "Companies") => {
            render_feed::<business_partner_network_public_api_business_partner_service::Companies>(
                xml,
                business_partner_network_public_api_business_partner_service_metadata::CompaniesMetadata::field_names()
            )
        }
        ("business-partner-service", "Users") => {
            render_feed::<business_partner_network_public_api_business_partner_service::Users>(
                xml,
                business_partner_network_public_api_business_partner_service_metadata::UsersMetadata::field_names(),
            )
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Project Partner Service V2
        ("project-partner-service-v2", "Users") => {
            render_feed::<service_project_partner_service_v2::Users>(
                xml,
                service_project_partner_service_v2_metadata::UsersMetadata::field_names(),
            )
        }
        ("project-partner-service-v2", "UserInvitations") => {
            render_feed::<service_project_partner_service_v2::UserInvitations>(
                xml,
                service_project_partner_service_v2_metadata::UserInvitationsMetadata::field_names(),
            )
        }
        ("project-partner-service-v2", "Companies") => {
            render_feed::<service_project_partner_service_v2::Companies>(
                xml,
                service_project_partner_service_v2_metadata::CompaniesMetadata::field_names(),
            )
        }
        ("project-partner-service-v2", "CompanyInvitations") => {
            render_feed::<service_project_partner_service_v2::CompanyInvitations>(
                xml,
                service_project_partner_service_v2_metadata::CompanyInvitationsMetadata::field_names(),
            )
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Digital Twin Service
        ("digital-twin-service", "DigitalTwins") => {
            render_feed::<public_api_digital_twin_service::DigitalTwins>(
                xml,
                public_api_digital_twin_service_metadata::DigitalTwinsMetadata::field_names(),
            )
        }
        ("digital-twin-service", "DigitalTwinSourceDocuments") => {
            render_feed::<public_api_digital_twin_service::DigitalTwinSourceDocuments>(
                xml,
                public_api_digital_twin_service_metadata::DigitalTwinSourceDocumentsMetadata::field_names(),
            )
        }
        ("digital-twin-service", "DigitalTwinObjects") => {
            render_feed::<public_api_digital_twin_service::DigitalTwinObjects>(
                xml,
                public_api_digital_twin_service_metadata::DigitalTwinObjectsMetadata::field_names(),
            )
        }
        ("digital-twin-service", "DigitalTwinModelVisualizations") => {
            render_feed::<public_api_digital_twin_service::DigitalTwinModelVisualizations>(
                xml,
                public_api_digital_twin_service_metadata::DigitalTwinModelVisualizationsMetadata::field_names(),
            )
        }

        _ => format!(
            "<p class=\"error\">No type mapping for collection \
             <em>{col_name}</em> in service <em>{svc_id}</em>.</p>"
        ),
    };

    format!("{back}{table}")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generic helper: parse Feed<T>, serialise properties to JSON for table output
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn render_feed<T>(xml: &str, columns: &[&str]) -> String
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let feed = match Feed::<T>::from_str(xml) {
        Ok(f) => f,
        Err(e) => return format!("<p class=\"error\">Parse error: {e}</p>"),
    };

    let entries = match feed.entries {
        Some(e) if !e.is_empty() => e,
        _ => return "<p>No entries found.</p>".to_string(),
    };

    let mut header = String::from("<tr>");
    for col in columns {
        header.push_str(&format!("<th>{col}</th>"));
    }
    header.push_str("</tr>");

    let mut rows = String::new();
    for entry in &entries {
        let props_ref = entry
            .content
            .properties
            .as_ref()
            .or(entry.properties.as_ref());

        let Some(props) = props_ref else {
            rows.push_str("<tr><td colspan=\"100\"><em>(no properties)</em></td></tr>");
            continue;
        };

        let map = match serde_json::to_value(props) {
            Ok(serde_json::Value::Object(m)) => m,
            _ => {
                rows.push_str("<tr><td colspan=\"100\"><em>(serialisation error)</em></td></tr>");
                continue;
            }
        };

        rows.push_str("<tr>");
        for col in columns {
            let cell = map
                .get(*col)
                .map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Null => String::new(),
                    other => other.to_string(),
                })
                .unwrap_or_default();
            rows.push_str(&format!("<td>{cell}</td>"));
        }
        rows.push_str("</tr>");
    }

    format!("<table>{header}{rows}</table>")
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
