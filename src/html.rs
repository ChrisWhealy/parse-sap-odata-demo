use crate::{
    business_partner_network_public_api_business_partner_service, public_api_digital_twin_service,
    service_project_partner_service_v2, service_project_service_v2,
};
use parse_sap_atom_feed::atom::feed::Feed;
use parse_sap_odata::entity::ODataEntity;
use std::str::FromStr;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Minimal HTML helpers
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const CSS_BODY: &str = "body { font-family: sans-serif; margin: 2rem; }";
const CSS_H1: &str = "h1 { border-bottom: 1px solid #ccc; padding-bottom: .5rem; }";
const CSS_UL: &str = "ul { list-style: none; padding: 0; }";
const CSS_LI: &str = "li { margin: .4rem 0; }";
const CSS_A: &str = "a { text-decoration: none; color: #0070c0; }";
const CSS_A_HOVER: &str = "a:hover { text-decoration: underline; }";
const CSS_TABLE: &str = "table { border-collapse: collapse; width: 100%; font-size: .9rem; }";
const CSS_TH_TD: &str =
    "th, td { border: 1px solid #ddd; padding: .4rem .6rem; text-align: left; }";
const CSS_TH: &str = "th { background: #f0f0f0; }";
const CSS_TR_NTH: &str = "tr:nth-child(even) { background: #fafafa; }";
const CSS_CLASS_BACK: &str = ".back { margin-bottom: 1rem; display: block; }";
const CSS_CLASS_ERROR: &str = ".error {{ color: #c00; }}";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_page(page_title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>{page_title}</title>
  <style>
    {CSS_BODY}
    {CSS_H1}
    {CSS_UL}
    {CSS_LI}
    {CSS_A}
    {CSS_A_HOVER}
    {CSS_TABLE}
    {CSS_TH_TD}
    {CSS_TH}
    {CSS_TR_NTH}
    {CSS_CLASS_BACK}
    {CSS_CLASS_ERROR}
  </style>
</head>
<body>
  <h1>{page_title}</h1>
  {body}
</body>
</html>"#
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Dispatch (service, collection) → typed Feed parse → HTML table
//
// All types across OData services must be fully qualified to avoid name collisions
pub fn render_collection(svc_id: &str, col_name: &str, xml: &str) -> String {
    let back =
        format!("<a class=\"back\" href=\"/service/{svc_id}\">&#8592; Back to collections</a>");

    let table = match (svc_id, col_name) {
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Project Service V2
        ("project-service-v2", "GroupMemberships") => {
            render_feed::<service_project_service_v2::GroupMemberships>(xml)
        }
        ("project-service-v2", "Groups") => render_feed::<service_project_service_v2::Groups>(xml),
        ("project-service-v2", "Users") => render_feed::<service_project_service_v2::Users>(xml),

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Business Partner Service
        ("business-partner-service", "Companies") => render_feed::<
            business_partner_network_public_api_business_partner_service::Companies,
        >(xml),
        ("business-partner-service", "Users") => {
            render_feed::<business_partner_network_public_api_business_partner_service::Users>(xml)
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Project Partner Service V2
        ("project-partner-service-v2", "Users") => {
            render_feed::<service_project_partner_service_v2::Users>(xml)
        }
        ("project-partner-service-v2", "UserInvitations") => {
            render_feed::<service_project_partner_service_v2::UserInvitations>(xml)
        }
        ("project-partner-service-v2", "Companies") => {
            render_feed::<service_project_partner_service_v2::Companies>(xml)
        }
        ("project-partner-service-v2", "CompanyInvitations") => {
            render_feed::<service_project_partner_service_v2::CompanyInvitations>(xml)
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        // Digital Twin Service
        ("digital-twin-service", "DigitalTwins") => {
            render_feed::<public_api_digital_twin_service::DigitalTwins>(xml)
        }
        ("digital-twin-service", "DigitalTwinSourceDocuments") => {
            render_feed::<public_api_digital_twin_service::DigitalTwinSourceDocuments>(xml)
        }
        ("digital-twin-service", "DigitalTwinObjects") => {
            render_feed::<public_api_digital_twin_service::DigitalTwinObjects>(xml)
        }
        ("digital-twin-service", "DigitalTwinModelVisualizations") => {
            render_feed::<public_api_digital_twin_service::DigitalTwinModelVisualizations>(xml)
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
pub fn render_feed<T>(xml: &str) -> String
where
    T: serde::de::DeserializeOwned + serde::Serialize + ODataEntity,
{
    let columns = T::field_names();
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
