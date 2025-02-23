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
// include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));
// include!(concat!(env!("OUT_DIR"), "/gwsample_basic_metadata.rs"));

use gwsample_basic::*;
use gwsample_basic_metadata::*;

fn fetch_xml_as_string(filename: &str) -> Result<String, FromUtf8Error> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(&format!("./test_data/{}", filename))).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    String::from_utf8(xml_buffer)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_get_entity_type_key_names() {
    assert_eq!("business_partner_id", BusinessPartnerMetadata::key()[0].name);
    assert_eq!("product_id", ProductMetadata::key()[0].name);
    assert_eq!("sales_order_id", SalesOrderMetadata::key()[0].name);
    assert_eq!("sales_order_id", SalesOrderLineItemMetadata::key()[0].name);
    assert_eq!("item_position", SalesOrderLineItemMetadata::key()[1].name);
    assert_eq!("contact_guid", ContactMetadata::key()[0].name);
    assert_eq!("sex", VhSexMetadata::key()[0].name);
    assert_eq!("land_1", VhCountryMetadata::key()[0].name);
    assert_eq!("address_type", VhAddressTypeMetadata::key()[0].name);
    assert_eq!("category", VhCategoryMetadata::key()[0].name);
    assert_eq!("waers", VhCurrencyMetadata::key()[0].name);
    assert_eq!("msehi", VhUnitQuantityMetadata::key()[0].name);
    assert_eq!("msehi", VhUnitWeightMetadata::key()[0].name);
    assert_eq!("msehi", VhUnitLengthMetadata::key()[0].name);
    assert_eq!("type_code", VhProductTypeCodeMetadata::key()[0].name);
    assert_eq!("bp_role", VhBpRoleMetadata::key()[0].name);
    assert_eq!("spras", VhLanguageMetadata::key()[0].name);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_business_partner_set() {
    static ENTITY_SET_NAME: &str = "BusinessPartnerSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<BusinessPartner>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                assert_eq!(entries.len(), 5);
                assert_eq!(
                    entries[0].id,
                    format!("{}('0100000000')", base_service_name)
                );

                let etag = String::from(entries[0].etag.as_ref().unwrap());
                assert!(etag.starts_with("datetime"));
                let props = entries[0].content.properties.clone().unwrap();

                assert_eq!(props.address.city, Some(String::from("Walldorf")));
                assert_eq!(props.company_name, "SAP");
                assert_eq!(props.currency_code, "EUR");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_contact_set() {
    static ENTITY_SET_NAME: &str = "ContactSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            match Feed::<Contact>::from_str(&xml) {
                Ok(feed) => {
                    assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
                    assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
                    assert_eq!(feed.id, base_service_name);
                    assert_eq!(feed.title, ENTITY_SET_NAME);

                    assert_eq!(feed.links.len(), 1);
                    assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

                    // Check contents of entity set
                    if let Some(entries) = feed.entries {
                        let props = entries[0].content.properties.clone().unwrap();

                        assert_eq!(entries.len(), 5);
                        assert_eq!(
                            props.address.street,
                            Some(String::from("Robert-Koch-Straße"))
                        );
                        assert_eq!(props.first_name, "Karl");
                        assert_eq!(props.last_name, Some(String::from("Müller")));
                        assert_eq!(props.date_of_birth, None);
                    } else {
                        assert!(
                            1 == 2,
                            "{}",
                            format!(
                                "Entity set {} should not be empty!",
                                String::from(ENTITY_SET_NAME)
                            )
                        )
                    }
                }
                Err(err_msg) => assert!(1 == 2, "{:?}", err_msg),
            };
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_product_set() {
    static ENTITY_SET_NAME: &str = "ProductSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let expected_date = "2023-08-31T01:48:52.9972620";
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<Product>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                assert_eq!(entries.len(), 5);

                let etag = String::from(entries[0].etag.as_ref().unwrap());
                assert!(etag.starts_with("datetime"));

                let props = entries[0].content.properties.clone().unwrap();

                assert_eq!(props.product_id, "2GOYBEBFLB");
                assert_eq!(props.category, "Notebooks");
                assert_eq!(props.weight_measure, Some(Decimal::new(4200000, 3)));
                assert_eq!(
                    props.weight_measure,
                    Some(Decimal::from_str("4200.0").unwrap())
                );
                assert_eq!(
                    props.created_at,
                    Some(chrono::NaiveDateTime::from_str(expected_date).unwrap())
                );
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_sales_order_line_item_set() {
    static ENTITY_SET_NAME: &str = "SalesOrderLineItemSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let expected_date = "2018-01-07T23:00:00.0000000";
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<SalesOrderLineItem>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();

                assert_eq!(entries.len(), 9);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.product_id, "HT-1000");
                assert_eq!(props.currency_code, Some(String::from("USD")));
                assert_eq!(
                    props.delivery_date,
                    chrono::NaiveDateTime::from_str(expected_date).unwrap()
                );
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_sales_order_set() {
    static ENTITY_SET_NAME: &str = "SalesOrderSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<SalesOrder>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 5);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.sales_order_id, "0500000000");
                assert_eq!(props.currency_code, Some(String::from("USD")));
                assert_eq!(
                    props.gross_amount,
                    Some(Decimal::from_str("14385.85").unwrap())
                );
                assert_eq!(
                    props.delivery_status_description,
                    Some(String::from("Delivered"))
                );
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_address_type_set() {
    static ENTITY_SET_NAME: &str = "VH_AddressTypeSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhAddressType>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 2);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.address_type, "01");
                assert_eq!(props.shorttext, "Private");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_bp_role_set() {
    static ENTITY_SET_NAME: &str = "VH_BPRoleSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhBpRole>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 2);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.bp_role, "01");
                assert_eq!(props.shorttext, "Customer");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_category_set() {
    static ENTITY_SET_NAME: &str = "VH_CategorySet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhCategory>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[16].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 26);

                assert_eq!(entries[16].etag.as_ref(), None);
                assert_eq!(props.category, "PDAs & Organizers");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_country_set() {
    static ENTITY_SET_NAME: &str = "VH_CountrySet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhCountry>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[119].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 250);

                assert_eq!(entries[119].etag.as_ref(), None);
                assert_eq!(props.land_1, "KN");
                assert_eq!(props.landx, "St Kitts&Nevis");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_currency_set() {
    static ENTITY_SET_NAME: &str = "VH_CurrencySet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhCurrency>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 209);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.waers, "ADP");
                assert_eq!(props.ltext, "Andorran Peseta --&gt; (Old --&gt; EUR)");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_language_set() {
    static ENTITY_SET_NAME: &str = "VH_LanguageSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhLanguage>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 44);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.spras, "SR");
                assert_eq!(props.sptxt, "Serbian");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_product_type_code_set() {
    static ENTITY_SET_NAME: &str = "VH_ProductTypeCodeSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhProductTypeCode>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 2);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.type_code, "AD");
                assert_eq!(props.shorttext, "Advertisement");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_sex_set() {
    static ENTITY_SET_NAME: &str = "VH_SexSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhSex>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 2);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.sex, "F");
                assert_eq!(props.shorttext, "Female");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_unit_length_set() {
    static ENTITY_SET_NAME: &str = "VH_UnitLengthSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhUnitLength>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 11);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.msehi, "\"");
                assert_eq!(props.msehl, "Inch");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_unit_quantity_set() {
    static ENTITY_SET_NAME: &str = "VH_UnitQuantitySet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhUnitQuantity>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 28);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.msehi, "AU");
                assert_eq!(props.msehl, "Activity unit");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_vh_unit_weight_set() {
    static ENTITY_SET_NAME: &str = "VH_UnitWeightSet";
    let base_service_name = format!("{}{}", FEED_XML_BASE, ENTITY_SET_NAME);

    match fetch_xml_as_string(&format!("{}.xml", ENTITY_SET_NAME)) {
        Ok(xml) => {
            let clean_xml = sanitise_xml(xml);
            let feed = Feed::<VhUnitWeight>::from_str(&clean_xml).unwrap();

            assert_eq!(feed.namespace, Some(String::from(ATOM_XML_NAMESPACE)));
            assert_eq!(feed.xml_base, Some(String::from(FEED_XML_BASE)));
            assert_eq!(feed.id, base_service_name);
            assert_eq!(feed.title, ENTITY_SET_NAME);

            assert_eq!(feed.links.len(), 1);
            assert_eq!(feed.links[0].href, ENTITY_SET_NAME);

            // Check contents of entity set
            if let Some(entries) = feed.entries {
                let props = entries[0].content.properties.clone().unwrap();
                assert_eq!(entries.len(), 8);

                assert_eq!(entries[0].etag.as_ref(), None);
                assert_eq!(props.msehi, "G");
                assert_eq!(props.msehl, "Gram");
            } else {
                assert!(
                    1 == 2,
                    "{}",
                    format!(
                        "Entity set {} should not be empty!",
                        String::from(ENTITY_SET_NAME)
                    )
                )
            }
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}
