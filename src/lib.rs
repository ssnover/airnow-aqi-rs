use reqwest::blocking::Client;
use serde::Deserialize;

static AIRNOW_API_URL_PREFIX: &str = "http://www.airnowapi.org/aq/observation/";
static AIRNOW_API_URL_POSTFIX: &str = "/current/?format=application/json";

pub struct Airnow {
    key: String,
    client: Client,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Category {
    pub number: u8,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObservationResponse {
    pub date_observed: String,
    pub hour_observed: u8,
    pub local_time_zone: String,
    pub reporting_area: String,
    pub state_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub parameter_name: String,
    #[serde(rename = "AQI")]
    pub aqi: u64,
    pub category: Category,
}

impl Airnow {
    pub fn new(api_key: String) -> Airnow {
        Airnow {
            key: api_key,
            client: Client::new(),
        }
    }

    pub fn get_current_observations_by_zipcode(
        &self,
        zip: u32,
        distance: Option<u64>,
    ) -> Result<Vec<ObservationResponse>, ()> {
        let mut complete_url = format!(
            "{}zipCode{}&zipCode={}&API_KEY={}",
            AIRNOW_API_URL_PREFIX, AIRNOW_API_URL_POSTFIX, zip, self.key
        );
        if let Some(distance) = distance {
            complete_url.push_str(format!("&distance={}", distance).as_str());
        }
        let response_body = self
            .client
            .get(&complete_url)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let obs: Vec<ObservationResponse> = serde_json::from_str(&response_body).unwrap();
        Ok(obs)
    }

    pub fn get_current_observations_by_coordinate(
        &self,
        latitude: f64,
        longitude: f64,
        distance: Option<u64>,
    ) -> Result<Vec<ObservationResponse>, ()> {
        let mut complete_url = format!(
            "{}latLong{}&latitude={}&longitude={}&API_KEY={}",
            AIRNOW_API_URL_PREFIX, AIRNOW_API_URL_POSTFIX, latitude, longitude, self.key
        );
        if let Some(distance) = distance {
            complete_url.push_str(format!("&distance={}", distance).as_str());
        }
        let response_body = self
            .client
            .get(&complete_url)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let obs: Vec<ObservationResponse> = serde_json::from_str(&response_body).unwrap();
        Ok(obs)
    }
}
