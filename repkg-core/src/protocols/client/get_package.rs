use miette::Result;
use reqwest::{Client, Method, Url};

use crate::protocols::{
    get_package::{GetPackageRequest, GetPackageResponse},
    Error::*,
};

pub async fn get_package(
    url: Url,
    GetPackageRequest {
        name,
        version,
        branch,
        namespace: _,
    }: &GetPackageRequest,
) -> Result<Vec<GetPackageResponse>> {
    let url = url
        .join(&format!("api/packages/{name}/{branch}/{version}"))
        .map_err(ParseError)?;

    let response = Client::new()
        .request(Method::GET, url)
        .send()
        .await
        .map_err(RequestError)?;
    let response = response
        .json::<Vec<GetPackageResponse>>()
        .await
        .map_err(InvalidResponse)?;

    Ok(response)
}
