use restson::RestClient;
use settings::{Settings, XDD_API_TOKEN_ENV_VAR};

// TODO : return a Result<> instead and pass Error?
pub fn new(settings: Settings) -> RestClient {
    let res_client = RestClient::new(settings.xdd.endpoint.as_str());
    let mut client = match res_client {
        Ok(client) => client,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };

    // Authenticate to XDD with token from environment
    match settings.xdd.api_token.clone() {
        Some(api_key) => match client.set_header("Authorization", &format!("Bearer {}", api_key)) {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        },
        None => {
            log::error!("Could not find XDD API token. It can be passed through environment under the name {:?}",
                             XDD_API_TOKEN_ENV_VAR);
            std::process::exit(1);
            // TODO : panic! instead?
        }
    };
    client
}
