fn main() {
    parse_sap_odata::parser::gen_src("service_ProjectServiceV2", "service.ProjectServiceV2");
    parse_sap_odata::parser::gen_src("service_ProjectPartnerServiceV2", "service.ProjectPartnerServiceV2");
    parse_sap_odata::parser::gen_src("publicApi_DigitalTwinService", "publicApi.DigitalTwinService");
    parse_sap_odata::parser::gen_src("businessPartnerNetwork_PublicApiBusinessPartnerService", "businessPartnerNetwork.PublicApiBusinessPartnerService");
}
