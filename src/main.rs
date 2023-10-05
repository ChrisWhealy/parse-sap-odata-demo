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
use std::{
    collections::HashMap,
    str::{self, FromStr},
};
use tinytemplate::TinyTemplate;

include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

use gwsample_basic::*;

pub mod gwsample_basic_metadata {
    pub enum SAPSemanticsProperty {
        TelephoneNumber,
        WorkCellphoneNumber,
        FaxNumber,
        EmailAddress,
        PreferredEmailAddress,
        URL,
        Fullname,
        FirstOrGivenName,
        MiddleName,
        LastName,
        Nickname,
        Title,
        NameSuffix,
        VCardNotes,
        PhotoURL,
        City,
        Street,
        Country,
        Region,
        PostalCode,
        PostOfficeBox,
        OrganizationName,
        OrganizationalUnit,
        OrganizationalRole,
        JobTitle,
        DateOfBirth,
        CalendarComponentSummary,
        CalendarComponentDescription,
        CalendarComponentCategories,
        CalendarComponentStartDateTime,
        CalendarComponentEndDateTime,
        CalendarComponentDuration,
        ToDoDueDateTime,
        ToDoCompletedDateTime,
        CalendarComponentPriority,
        CalendarComponentAccessClassification,
        CalendarComponentStatus,
        ToDoPercentComplete,
        CalendarComponentContact,
        CalendarComponentVenue,
        TransparentEvent,
        CalendarComponentFreeBusyTime,
        CalendarComponentOccupiesWholeDay,
        CalendarComponentYear,
        CalendarComponentYearMonth,
        CalendarComponentYearMonthDay,
        EmailFrom,
        EmailSender,
        EmailToList,
        EmailCCList,
        EmailBCCList,
        EmailSubject,
        EmailBody,
        EmailKeywordList,
        EmailDateTimeReceived,
        GeolocationLongitude,
        GeolocationLatitude,
        CurrencyCode,
        UnitOfMeasure,
        Count,
    }

    pub enum SAPFieldControlProperty {
        Hidden,
        ReadOnly,
        Optional,
        Mandatory,
    }

    pub enum SAPDisplayFormatProperty {
        Date,
        NonNegative,
        UpperCase,
    }

    pub enum SAPFilterRestrictionProperty {
        SingleValue,
        MultiValue,
        Interval,
    }

    pub enum SAPAggregationRoleProperty {
        Dimension,
        Measure,
        TotalPropertiesList,
    }

    pub enum SAPParameterProperty {
        Mandatory,
        Optional,
    }

    pub struct SAPAnnotationsProperty {
        pub label: Option<String>,
        pub heading: Option<String>,
        pub quick_info: Option<String>,
        pub is_unicode: bool,
        pub semantics: Option<SAPSemanticsProperty>,
        pub is_creatable: bool,
        pub is_updatable: bool,
        pub is_sortable: bool,
        pub is_filterable: bool,
        pub is_addressable: bool,
        pub is_required_in_filter: bool,
        pub filter_restriction: Option<SAPFilterRestrictionProperty>,
        pub filter_for: Option<String>,
        pub text: Option<String>,
        pub text_for: Option<String>,
        pub unit: Option<String>,
        pub precision: Option<String>,
        pub is_visible: bool,
        pub field_control: Option<SAPFieldControlProperty>,
        pub validation_regexp: Option<String>,
        pub display_format: Option<SAPDisplayFormatProperty>,
        pub value_list: Option<String>,
        pub lower_boundary: Option<String>,
        pub upper_boundary: Option<String>,
        pub aggregation_role: Option<SAPAggregationRoleProperty>,
        pub super_ordinate: Option<String>,
        pub attribute_for: Option<String>,
        pub hierarchy_node_for: Option<String>,
        pub hierarchy_node_external_key_for: Option<String>,
        pub hierarchy_level_for: Option<String>,
        pub hierarchy_parent_node_for: Option<String>,
        pub hierarchy_parent_navigation_for: Option<String>,
        pub hierarchy_drill_state_for: Option<String>,
        pub hierarchy_node_descendant_count_for: Option<String>,
        pub hierarchy_preorder_rank_for: Option<String>,
        pub hierarchy_sibling_rank_for: Option<String>,
        pub parameter: Option<SAPParameterProperty>,
        pub is_annotation: bool,
        pub updatable_path: Option<String>,
        pub preserve_flag_for: Option<String>,
        pub has_variable_scale: bool,
    }

    pub struct SAPAnnotationsNavigationProperty {
        pub is_creatable: bool,
        pub creatable_path: Option<String>,
        pub is_filterable: bool,
    }

    pub struct SAPAnnotationsSchema {
        pub schema_version: String,
    }

    pub struct PropertyRef {
        pub name: String,
    }

    pub struct Key {
        pub property_refs: Vec<PropertyRef>,
    }

    pub struct NavigationProperty {
        pub name: String,
        pub relationship: String,
        pub to_role: String,
        pub from_role: String,
        pub sap_annotations: SAPAnnotationsNavigationProperty,
    }

    pub enum EntityTypeSAPSemantics {
        VCard,
        VEvent,
        VToDo,
        Paramaters,
        Aggregate,
        Variant,
    }

    pub struct Property {
        pub odata_name: String,
        pub edm_type: String,
        pub nullable: bool,
        pub max_length: Option<u16>,
        pub precision: Option<u16>,
        pub scale: Option<u16>,
        pub concurrency_mode: Option<String>,
        pub fc_keep_in_content: bool,
        pub fc_target_path: Option<String>,
        pub sap_annotations: Option<SAPAnnotationsProperty>,
        pub deserializer_fn: &'static str,
    }

    pub struct EntityType {
        pub name: String,
        pub sap_label: Option<String>,
        pub sap_semantics: Option<EntityTypeSAPSemantics>,
        pub sap_content_version: String,
        pub has_stream: bool,
        pub key: Key,
        pub properties: Vec<Property>,
        pub navigations: Vec<NavigationProperty>,
    }

    pub fn get_entity_type_metadata(entity_type_name: &str) -> EntityType {
        match entity_type_name {
            _ => {
                let key = Key {
                    property_refs: vec![PropertyRef {
                        name: String::from("BusinessPartnerID"),
                    }],
                };

                let prop001 = Property {
                    odata_name: String::from("Address"),
                    edm_type: String::from("CT_Address"),
                    nullable: false,
                    max_length: None,
                    precision: None,
                    scale: None,
                    concurrency_mode: None,
                    fc_keep_in_content: true,
                    fc_target_path: None,
                    sap_annotations: None,
                    deserializer_fn: "",
                };

                let nav001 = NavigationProperty {
                    name: String::from("ToContacts"),
                    relationship: String::from("GWSAMPLE_BASIC.Assoc_BusinessPartner_Contacts"),
                    from_role: String::from("ToRole_Assoc_BusinessPartner_Contacts"),
                    to_role: String::from("FromRole_Assoc_BusinessPartner_Contacts"),
                    sap_annotations: SAPAnnotationsNavigationProperty {
                        is_creatable: false,
                        creatable_path: None,
                        is_filterable: true,
                    },
                };

                let nav002 = NavigationProperty {
                    name: String::from("ToProducts"),
                    relationship: String::from("GWSAMPLE_BASIC.Assoc_BusinessPartner_Products"),
                    from_role: String::from("ToRole_Assoc_BusinessPartner_Products"),
                    to_role: String::from("FromRole_Assoc_BusinessPartner_Products"),
                    sap_annotations: SAPAnnotationsNavigationProperty {
                        is_creatable: false,
                        creatable_path: None,
                        is_filterable: true,
                    },
                };

                EntityType {
                    name: String::from(entity_type_name),
                    sap_label: None,
                    sap_semantics: None,
                    sap_content_version: String::from("1"),
                    has_stream: true,
                    key,
                    properties: vec![prop001],
                    navigations: vec![nav001, nav002],
                }
            }
        }
    }
}

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

fn parse_xml(es_name: &str, xml: &str) -> String {
    match es_name {
        "BusinessPartnerSet" => match Feed::<BusinessPartner>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "ProductSet" => match Feed::<Product>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "SalesOrderSet" => match Feed::<SalesOrder>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "SalesOrderLineItemSet" => match Feed::<SalesOrderLineItem>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "ContactSet" => match Feed::<Contact>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_SexSet" => match Feed::<VhSex>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_CountrySet" => match Feed::<VhCountry>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_AddressTypeSet" => match Feed::<VhAddressType>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_CategorySet" => match Feed::<VhCategory>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_CurrencySet" => match Feed::<VhCurrency>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_UnitQuantitySet" => match Feed::<VhUnitQuantity>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_UnitWeightSet" => match Feed::<VhUnitWeight>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_UnitLengthSet" => match Feed::<VhUnitLength>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_ProductTypeCodeSet" => match Feed::<VhProductTypeCode>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_BPRoleSet" => match Feed::<VhBpRole>::from_str(&xml) {
            Ok(parsed_feed) => format!("{parsed_feed:#?}"),
            Err(e) => format!("Error: {e:?}"),
        },
        "VH_LanguageSet" => match Feed::<VhLanguage>::from_str(&xml) {
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

    if !gwsample_basic::GwsampleBasicEntities::as_list().contains(&&entity_set_name[..]) {
        return Ok(HttpResponse::NotFound().finish());
    }

    let http_response = match fetch_auth() {
        Ok(auth_chars) => {
            let url = format!(
                "{}/{}/{}?$top=100",
                str::from_utf8(HOST_PATH).unwrap(),
                str::from_utf8(SERVICE_NAME).unwrap(),
                entity_set_name
            );
            log::info!("Fetching URL {}", url);

            match client
                .get(url)
                .header("Authorization", format!("Basic {}", auth_chars))
                .send()
                .await
            {
                Ok(response) => {
                    let http_status_code = response.status();
                    log::info!("HTTP Status code = {}", http_status_code);

                    let raw_xml = response.text().await.unwrap();
                    log::info!("Raw XML response\n{}", raw_xml);

                    let response_body = match http_status_code {
                        StatusCode::OK => {
                            parse_xml(&entity_set_name, &sanitise_xml(String::from(raw_xml)))
                        }
                        _ => parse_odata_error(&raw_xml),
                    };

                    HttpResponse::Ok()
                        .content_type(ContentType::plaintext())
                        .body(response_body)
                }
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
