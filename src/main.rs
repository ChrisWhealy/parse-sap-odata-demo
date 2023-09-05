pub mod auth;
pub mod err_handlers;

use crate::{auth::fetch_auth, err_handlers::error_handlers};
use parse_sap_atom_feed::{atom::feed::Feed, odata_error::ODataError, xml::sanitise_xml};

use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    middleware, web, App, Error, HttpResponse, HttpServer, Result,
};
use serde_json::json;
use std::{collections::HashMap, str};
use tinytemplate::TinyTemplate;

include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

static INDEX: &str = include_str!("../html/index.html");
static HOST_PATH: &[u8] = "https://sapes5.sapdevcenter.com/sap/opu/odata/iwbep".as_bytes();
static SERVICE_NAME: &[u8] = "GWSAMPLE_BASIC".as_bytes();

// ---------------------------------------------------------------------------------------------------------------------
// Serve document root
// ---------------------------------------------------------------------------------------------------------------------
async fn doc_root(
    tmpl: web::Data<TinyTemplate<'_>>,
    _query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let ctx = json!({
      "service_name": str::from_utf8(SERVICE_NAME).unwrap(),
      "option_list": GwsampleBasicEntities::as_list()
    });

    let body = tmpl
        .render("index.html", &ctx)
        .map_err(|err| error::ErrorInternalServerError(format!("Template error\n{}", err)))?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body))
}

fn parse_odata_error(raw_xml: &str) -> String {
    match ODataError::from_str(&raw_xml) {
        Ok(odata_error) => format!("{:#?}", odata_error),
        Err(e) => format!("{:#?}", e),
    }
}

fn parse_raw_xml(es_name: &str, raw_xml: &str) -> String {
    match es_name {
        "BusinessPartnerSet" => match Feed::<BusinessPartner>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "ProductSet" => match Feed::<Product>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "SalesOrderSet" => match Feed::<SalesOrder>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "SalesOrderLineItemSet" => match Feed::<SalesOrderLineItem>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "ContactSet" => match Feed::<Contact>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_SexSet" => match Feed::<VhSex>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_CountrySet" => match Feed::<VhCountry>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_AddressTypeSet" => match Feed::<VhAddressType>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_CategorySet" => match Feed::<VhCategory>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_CurrencySet" => match Feed::<VhCurrency>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_UnitQuantitySet" => match Feed::<VhUnitQuantity>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_UnitWeightSet" => match Feed::<VhUnitWeight>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_UnitLengthSet" => match Feed::<VhUnitLength>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_ProductTypeCodeSet" => match Feed::<VhProductTypeCode>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_BPRoleSet" => match Feed::<VhBpRole>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_LanguageSet" => match Feed::<VhLanguage>::from_str(&raw_xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        _ => format!("Error: invalid entity set name '{}'", es_name),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Serve entity set contents
// ---------------------------------------------------------------------------------------------------------------------
#[get("/{entity_set_name}")]
async fn entity_set(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let client = reqwest::Client::new();
    let entity_set_name = path.into_inner();

    println!("GET: /{}", entity_set_name);

    let http_response = match fetch_auth() {
        Ok(auth_chars) => {
            match client
                .get(format!(
                    "{}/{}/{}?$top=100",
                    str::from_utf8(HOST_PATH).unwrap(),
                    str::from_utf8(SERVICE_NAME).unwrap(),
                    entity_set_name
                ))
                .header("Authorization", format!("Basic {}", auth_chars))
                .send()
                .await
                .expect("Here's an SAP-style error message: Ein Fehler ist aufgetreten. Viel Glück 🤣🤣🤣")
                .text()
                .await
            {
                Ok(raw_xml) => {
                  // Not the best way to handle errors, but it works
                  let response = if raw_xml.contains("<error ") {
                    parse_odata_error(&raw_xml)
                  } else {
                    let clean_xml = sanitise_xml(String::from(raw_xml));
                    parse_raw_xml(&entity_set_name, &clean_xml)
                  };

                  HttpResponse::Ok()
                      .content_type(ContentType::plaintext())
                      .body(response)
                },
                Err(err) => HttpResponse::BadRequest().body(format!("{:#?}", err)),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(format!("{:#?}", err)),
    };

    Ok(http_response)
}

// ---------------------------------------------------------------------------------------------------------------------
// Start web server
// ---------------------------------------------------------------------------------------------------------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("index.html", INDEX).unwrap();

        App::new()
            .app_data(web::Data::new(tt))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(doc_root)))
            .service(entity_set)
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
