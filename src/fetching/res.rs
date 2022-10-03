use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

/// Fetch a URL and return the result in a [`Response`].
pub async fn fetch(url: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;
    let res = client.get(url).send().await?;
    Ok(res)
}

/// Fetch a URL and return it's JSON response seralized into a struct provided. Struct should have the [`DeserializeOwned`] trait from `serde`.
pub async fn fetch_res_json<T>(url: String) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let res = fetch(url).await?;
    let json_data = res.json::<T>().await?;

    Ok(json_data)
}
