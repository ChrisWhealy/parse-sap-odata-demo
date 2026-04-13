// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Service catalogue
//
// Each entry names a URL into SAP's public API sandbox, a display name, and the sandbox base URL.
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub(crate) struct ServiceDef {
    pub id: &'static str,
    pub display_name: &'static str,
    pub base_url: &'static str,
}

pub(crate) const SERVICES: &[ServiceDef] = &[
    ServiceDef {
        id: "project-service-v2",
        display_name: "Project Service V2",
        base_url: "https://sandbox.api.sap.com/SAPPIN/ProjectService/v1/odata/v2",
    },
    ServiceDef {
        id: "project-partner-service-v2",
        display_name: "Project Partner Service V2",
        base_url: "https://sandbox.api.sap.com/SAPPIN/ProjectPartnerService/v1/odata/v2",
    },
    ServiceDef {
        id: "business-partner-service",
        display_name: "Business Partner Service",
        base_url: "https://sandbox.api.sap.com/SAPPIN/BusinessPartnerService/v1/odata/v2",
    },
    ServiceDef {
        id: "digital-twin-service",
        display_name: "Digital Twin Service",
        base_url: "https://sandbox.api.sap.com/SAPPIN/DigitalTwinService/v1/odata/v2",
    },
];

pub(crate) fn find_service(id: &str) -> Option<&'static ServiceDef> {
    SERVICES.iter().find(|s| s.id == id)
}
