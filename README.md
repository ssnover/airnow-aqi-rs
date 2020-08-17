# airnow_aqi
This is a work-in-progress small crate to provide a convenient client for querying Air Quality Index (AQI) data from the United States Environmental Protection Agency. Data is provided in terms of AQI for Ozone (O3), PM2.5, and PM10. Note that different regions calculate AQI in different ways from the physical measurements. Good starting point is Wikipedia as always: [Air Quality Index](https://en.wikipedia.org/wiki/Air_quality_index).

This API requires an API key. Information about signing up for an API key can be found at the Airnow API website, along with more documentation on their available services: [Airnow API](https://docs.airnowapi.org/).

Once you have an API key, it's simple enough to build a client and request data based on a zip code or (coming soon) a pair of latitude and longitude coordinates along with an optional distance to search for observations.

```rust
let key = key["api_key"].as_str().unwrap();
let client = airnow_aqi::Airnow::new(key.to_string());

let observations = client.get_current_observations(19123, Some(25)).unwrap();

for element in observations {
    println!("{:?}", element);
}

```