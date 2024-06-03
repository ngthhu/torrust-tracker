use anyhow::Context;
use serde::Serialize;

use super::dto::ResponseDto;

pub trait ToJson {
    ///
    /// Returns a string with the JSON serialized version of the response
    ///
    /// # Errors
    ///
    /// Will return an error if serialization fails.
    ///
    fn to_json_string(&self) -> anyhow::Result<String>
    where
        Self: Serialize,
    {
        let pretty_json = serde_json::to_string_pretty(self).context("response JSON serialization")?;

        Ok(pretty_json)
    }
}

impl ToJson for ResponseDto {}
