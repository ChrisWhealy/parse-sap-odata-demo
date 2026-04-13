# Demo App for Reading OData V2 Services From SAP's API Sandbox

This demo app allows you to interact with some of the OData V2 services available on [SAP's API Sandbox](https://api.sap.com/content-type/API/apis/ODATA).

## Prerequisites

1. Clone this repo
2. `cd parse_sap_odata_demo`

### Sign Up To SAP API Sandbox Service

1. To sign up, follow the above link, select login in the top right corner, then complete the registration process.
1. Once signed up, visit the [OData V2 page](https://api.sap.com/content-type/API/apis/ODATA), select any one of the active services, then retrieve your API key by clicking on "Show API Key" in the top right corner.
1. In the repo's top level directory, create a `.env` file containing your API key:

   ```
   ODATA_API_KEY=<your API key value>
   ```

# Local Execution

1. Start the app using `cargo run`.
1. Visit <http://localhost:8080> and you will see a list of available OData services.
1. Select the desired service and you will then see a list of that service's collections.
1. Select the Collection to see the data it contains.

***WARING***<br>
Some Collections listed in the OData service document return `HTTP 404 Not Found`, such as `Project Service V2/Users`