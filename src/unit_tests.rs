use rust_decimal::Decimal;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
    string::FromUtf8Error,
};

use parse_sap_atom_feed::{atom::feed::Feed, xml::sanitise_xml};

static FEED_XML_BASE: &str =
    "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/";

static ATOM_XML_NAMESPACE: &str = "http://www.w3.org/2005/Atom";

parse_sap_odata::include_mod!("gwsample_basic");
parse_sap_odata::include_mod!("gwsample_basic_metadata");

use gwsample_basic::*;
use gwsample_basic_metadata::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Asserts equality, returning a descriptive Err if the values differ.
/// Expands to a `return Err(...)` so it must be used in functions returning `Result<(), String>`.
/// Optional first integer argument labels the failure with a test number.
macro_rules! check_eq {
    ($test_num:literal, $left:expr, $right:expr $(,)?) => {
        if $left != $right {
            return Err(format!(
                "Test {}: assertion failed: expected {:?} but got {:?}",
                $test_num, $right, $left
            ));
        }
    };
    ($left:expr, $right:expr $(,)?) => {
        if $left != $right {
            return Err(format!(
                "assertion failed: expected {:?} but got {:?}",
                $right, $left
            ));
        }
    };
}

/// Asserts that a value is None, returning a descriptive Err if it is Some.
/// Expands to a `return Err(...)` so it must be used in functions returning `Result<(), String>`.
/// Optional first integer argument labels the failure with a test number.
macro_rules! check_none {
    ($test_num:literal, $val:expr $(,)?) => {
        if let Some(ref inner) = $val {
            return Err(format!(
                "Test {}: expected None but got Some({:?})",
                $test_num, inner
            ));
        }
    };
    ($val:expr $(,)?) => {
        if let Some(ref inner) = $val {
            return Err(format!("expected None but got Some({:?})", inner));
        }
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn fetch_xml_as_string(filename: &str) -> Result<String, FromUtf8Error> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(&format!("./test_data/{}", filename))).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    String::from_utf8(xml_buffer)
}

fn check_starts_with(value: &str, prefix: &str) -> Result<(), String> {
    if !value.starts_with(prefix) {
        Err(format!(
            "expected value starting with {:?}, but got {:?}",
            prefix, value
        ))
    } else {
        Ok(())
    }
}

fn check_feed_header<T>(feed: &Feed<T>, entity_set_name: &str) -> Result<(), String> {
    let expected_id = format!("{}{}", FEED_XML_BASE, entity_set_name);
    check_eq!(1, feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
    check_eq!(2, feed.xml_base, Some(String::from(FEED_XML_BASE)));
    check_eq!(3, feed.id, expected_id);
    check_eq!(4, feed.title, entity_set_name);
    check_eq!(5, feed.links.len(), 1);
    check_eq!(6, feed.links[0].href, entity_set_name);
    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_get_entity_type_key_names() -> Result<(), String> {
    check_eq!(
        1,
        BusinessPartnerMetadata::key()[0].name,
        "business_partner_id"
    );
    check_eq!(2, ProductMetadata::key()[0].name, "product_id");
    check_eq!(3, SalesOrderMetadata::key()[0].name, "sales_order_id");
    check_eq!(
        4,
        SalesOrderLineItemMetadata::key()[0].name,
        "sales_order_id"
    );
    check_eq!(
        5,
        SalesOrderLineItemMetadata::key()[1].name,
        "item_position"
    );
    check_eq!(6, ContactMetadata::key()[0].name, "contact_guid");
    check_eq!(7, VhSexMetadata::key()[0].name, "sex");
    check_eq!(8, VhCountryMetadata::key()[0].name, "land_1");
    check_eq!(9, VhAddressTypeMetadata::key()[0].name, "address_type");
    check_eq!(10, VhCategoryMetadata::key()[0].name, "category");
    check_eq!(11, VhCurrencyMetadata::key()[0].name, "waers");
    check_eq!(12, VhUnitQuantityMetadata::key()[0].name, "msehi");
    check_eq!(13, VhUnitWeightMetadata::key()[0].name, "msehi");
    check_eq!(14, VhUnitLengthMetadata::key()[0].name, "msehi");
    check_eq!(15, VhProductTypeCodeMetadata::key()[0].name, "type_code");
    check_eq!(16, VhBpRoleMetadata::key()[0].name, "bp_role");
    check_eq!(17, VhLanguageMetadata::key()[0].name, "spras");
    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_business_partner_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "BusinessPartnerSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<BusinessPartner>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    check_eq!(1, entries.len(), 5);
    check_eq!(
        2,
        entries[0].id,
        format!("{}{}('0100000000')", FEED_XML_BASE, ENTITY_SET_NAME)
    );

    let etag = String::from(
        entries[0]
            .etag
            .as_ref()
            .ok_or_else(|| "Expected etag to be present on entries[0]".to_string())?,
    );
    check_starts_with(&etag, "datetime")?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(3, props.address.city, Some(String::from("Walldorf")));
    check_eq!(4, props.company_name, "SAP");
    check_eq!(5, props.currency_code, "EUR");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_contact_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "ContactSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let feed = Feed::<Contact>::from_str(&xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 5);
    check_eq!(
        2,
        props.address.street,
        Some(String::from("Robert-Koch-Straße"))
    );
    check_eq!(3, props.first_name, "Karl");
    check_eq!(4, props.last_name, Some(String::from("Müller")));
    check_none!(5, props.date_of_birth);

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_product_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "ProductSet";
    let expected_date = "2023-08-31T01:48:52.9972620";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<Product>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    check_eq!(1, entries.len(), 5);

    let etag = String::from(
        entries[0]
            .etag
            .as_ref()
            .ok_or_else(|| "Expected etag to be present on entries[0]".to_string())?,
    );
    check_starts_with(&etag, "datetime")?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(2, props.product_id, "2GOYBEBFLB");
    check_eq!(3, props.category, "Notebooks");
    check_eq!(4, props.weight_measure, Some(Decimal::new(4200000, 3)));
    check_eq!(
        5,
        props.weight_measure,
        Some(Decimal::from_str("4200.0").unwrap())
    );
    check_eq!(
        6,
        props.created_at,
        Some(chrono::NaiveDateTime::from_str(expected_date).unwrap())
    );

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_sales_order_line_item_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "SalesOrderLineItemSet";
    let expected_date = "2018-01-07T23:00:00.0000000";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<SalesOrderLineItem>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 9);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.product_id, "HT-1000");
    check_eq!(4, props.currency_code, Some(String::from("USD")));
    check_eq!(
        5,
        props.delivery_date,
        chrono::NaiveDateTime::from_str(expected_date).unwrap()
    );

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_sales_order_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "SalesOrderSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<SalesOrder>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 5);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.sales_order_id, "0500000000");
    check_eq!(4, props.currency_code, Some(String::from("USD")));
    check_eq!(
        5,
        props.gross_amount,
        Some(Decimal::from_str("14385.85").unwrap())
    );
    check_eq!(
        6,
        props.delivery_status_description,
        Some(String::from("Delivered"))
    );

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_address_type_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_AddressTypeSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhAddressType>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 2);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.address_type, "01");
    check_eq!(4, props.shorttext, "Private");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_bp_role_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_BPRoleSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhBpRole>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 2);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.bp_role, "01");
    check_eq!(4, props.shorttext, "Customer");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_category_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_CategorySet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhCategory>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[16].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 26);
    check_none!(2, entries[16].etag.as_ref());
    check_eq!(3, props.category, "PDAs & Organizers");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_country_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_CountrySet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhCountry>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[119].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 250);
    check_none!(2, entries[119].etag.as_ref());
    check_eq!(3, props.land_1, "KN");
    check_eq!(4, props.landx, "St Kitts&Nevis");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_currency_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_CurrencySet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhCurrency>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 209);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.waers, "ADP");
    check_eq!(4, props.ltext, "Andorran Peseta --&gt; (Old --&gt; EUR)");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_language_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_LanguageSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhLanguage>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 44);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.spras, "SR");
    check_eq!(4, props.sptxt, "Serbian");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_product_type_code_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_ProductTypeCodeSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhProductTypeCode>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 2);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.type_code, "AD");
    check_eq!(4, props.shorttext, "Advertisement");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_sex_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_SexSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhSex>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 2);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.sex, "F");
    check_eq!(4, props.shorttext, "Female");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_unit_length_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_UnitLengthSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhUnitLength>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 11);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.msehi, "\"");
    check_eq!(4, props.msehl, "Inch");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_unit_quantity_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_UnitQuantitySet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhUnitQuantity>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 28);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.msehi, "AU");
    check_eq!(4, props.msehl, "Activity unit");

    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_unit_weight_set() -> Result<(), String> {
    static ENTITY_SET_NAME: &str = "VH_UnitWeightSet";

    let xml = fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME))
        .map_err(|e| format!("XML test data was not in UTF8 format: {}", e))?;
    let clean_xml = sanitise_xml(xml);
    let feed = Feed::<VhUnitWeight>::from_str(&clean_xml)
        .map_err(|e| format!("Failed to parse {}: {:?}", ENTITY_SET_NAME, e))?;

    check_feed_header(&feed, ENTITY_SET_NAME)?;

    let entries = feed
        .entries
        .ok_or_else(|| format!("Entity set {} should not be empty!", ENTITY_SET_NAME))?;

    let props = entries[0].content.properties.clone().unwrap();
    check_eq!(1, entries.len(), 8);
    check_none!(2, entries[0].etag.as_ref());
    check_eq!(3, props.msehi, "G");
    check_eq!(4, props.msehl, "Gram");

    Ok(())
}
