use super::*;

impl<'a> Client<'a> {
    /// You can test your authentication credentials
    /// by making a request to the Paddle API.
    ///
    /// `403 Forbidden` will be returned if the authentication fails.
    // https://developer.paddle.com/api-reference/about/authentication#test-authentication
    pub async fn test_authentication(&self) -> Result<(), PaddleError> {
        Self::include_data_meta(
            self.client
                .get(self.url.join("event-types")?)
                .headers(self.default_headers()?)
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?,
        )
    }

    // Response
    //
    // If successful, you should get
    // a response that includes a `data` array and a `meta` object.
    //
    // https://developer.paddle.com/api-reference/about/authentication#test-authentication-response
    fn include_data_meta(body: String) -> Result<(), PaddleError> {
        let v: Value = serde_json::from_str(&body)?;

        if let Some(data) = v.get("data") {
            if let Some(array) = data.as_array() {
                if array.is_empty() {
                    return Err(PaddleError::AuthenticationFailed(
                        "Data is empty".to_string(),
                    ));
                }
            } else {
                return Err(PaddleError::AuthenticationFailed(
                    "Data is not a collection".to_string(),
                ));
            }
        } else {
            return Err(PaddleError::AuthenticationFailed(
                "Data is missing".to_string(),
            ));
        }

        if v.get("meta").is_none() {
            return Err(PaddleError::AuthenticationFailed(
                "Meta is missing".to_string(),
            ));
        }

        Ok(())
    }
}
