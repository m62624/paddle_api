use crate::entities::Meta;
use reqwest::Response;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaddleError {
    pub error: ErrorDetail,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "type")]
    pub e_type: String,
    pub code: String,
    pub detail: String,
    pub documentation_url: String,
    pub errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for PaddleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)?;
        Ok(())
    }
}

impl std::error::Error for PaddleError {}

impl PaddleError {
    pub async fn handle_response(response: Response) -> Result<Response, anyhow::Error> {
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let error = response.json::<PaddleError>().await?;
            return Err(anyhow::anyhow!("{}", error));
        }
        Ok(response)
    }
}
