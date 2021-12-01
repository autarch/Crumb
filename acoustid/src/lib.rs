use anyhow::Error;
use log::debug;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[cfg(test)]
use mockito;

#[derive(Debug)]
pub struct AcoustIDClient {
    key: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct RequestBody<'a> {
    format: &'static str,
    client: &'a str,
    duration: u64,
    fingerprint: &'a str,
    meta: &'static str,
}

// See https://acoustid.org/webservice for service docs.
#[derive(Debug, Deserialize)]
struct ResponseBody {
    status: String,
    results: Option<Vec<APIResult>>,
}

#[derive(Debug, Deserialize)]
struct APIResult {
    score: f64,
    recordings: Vec<Recording>,
}

#[derive(Debug, Deserialize)]
struct Recording {
    // This is the MusicBrainz recording id
    id: String,
}

#[derive(Debug, Error)]
pub enum AcoustidError {
    #[error("Error making request error")]
    RequestError { error: reqwest::Error },
    #[error("Got {status:} status for request")]
    ResponseStatusCodeError { status: reqwest::StatusCode },
    #[error("Got {status:} in response from AcoustID service")]
    ServiceStatusNotOkError { status: String },
    #[error("AcoustID service returned a response with no results")]
    NoResultsError,
    #[error("AcoustID service returned a response with one or more scores as NaN")]
    ResultScoreIsNan,
}

impl AcoustIDClient {
    pub fn new(key: impl ToString) -> Self {
        Self {
            key: key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn lookup<F: AsRef<str>>(
        &self,
        fingerprint: F,
        duration: impl Into<u64>,
    ) -> Result<Vec<String>, Error> {
        let fingerprint = fingerprint.as_ref();
        let duration = duration.into();

        #[cfg(not(test))]
        let base = "https://api.acoustid.org";
        #[cfg(test)]
        let base = mockito::server_url();

        let mut url = Url::parse(&base)?;
        url.set_path("/v2/lookup");

        let mut query = Url::parse("http://example.com")?;
        query.query_pairs_mut()
            .append_pair("format", "json")
            .append_pair("client", &self.key)
            .append_pair("duration", &format!("{}", duration))
            .append_pair("fingerprint", fingerprint)
            .append_pair("meta", "recordingids");
        let body = query.query().unwrap().to_string();

        let res = self
            .client
            .post(url)
            .body(body)
            .header("Accept-Encoding", "gzip")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        if res.status() != reqwest::StatusCode::OK {
            return Err(AcoustidError::ResponseStatusCodeError {
                status: res.status(),
            }
            .into());
        }

        let rb: ResponseBody = res
            .json()
            .await
            .map_err(|e| AcoustidError::RequestError { error: e })?;
        if rb.status != "ok" {
            return Err(AcoustidError::ServiceStatusNotOkError { status: rb.status }.into());
        }

        let mut results = match rb.results {
            Some(r) => {
                if r.len() == 0 {
                    return Err(AcoustidError::NoResultsError.into());
                }
                r
            }
            None => return Err(AcoustidError::NoResultsError.into()),
        };
        if results.iter().any(|r| r.score.is_nan()) {
            return Err(AcoustidError::ResultScoreIsNan.into());
        }
        // unwrap is safe because we already checked for NaN
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        Ok(results
            .into_iter()
            .map(|res| res.recordings.into_iter().map(|rec| rec.id))
            .flatten()
            .collect::<Vec<String>>())
    }
}

#[cfg(test)]
mod tests {
    use super::{AcoustIDClient, AcoustidError};
    use mockito::mock;

    const PATH: &str = "/v2/lookup";

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_is_not_ok() {
        let _m = mock("POST", PATH).with_status(401).create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        assert!(res.is_err());
        assert!(matches!(
            res.unwrap_err().downcast_ref::<AcoustidError>().unwrap(),
            AcoustidError::ResponseStatusCodeError {
                status: reqwest::StatusCode::UNAUTHORIZED
            },
        ));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_has_json_with_unexpected_structure() {
        let _m = mock("POST", PATH)
            .with_status(200)
            .with_body(r#"{"status": "ok", "results": { "foo": "bar" }}"#)
            .create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        println!("E = {:#?}", res);
        assert!(res.is_err());
        let err = res.unwrap_err().downcast::<AcoustidError>().unwrap();
        assert!(matches!(err, AcoustidError::RequestError { error: _ },));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_has_json_with_status_not_ok() {
        let _m = mock("POST", PATH)
            .with_status(200)
            .with_body(r#"{"status": "not ok"}"#)
            .create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        assert!(res.is_err());
        let err = res.unwrap_err().downcast::<AcoustidError>().unwrap();
        assert!(matches!(
            err,
            AcoustidError::ServiceStatusNotOkError { status: _ },
        ));
        match err {
            AcoustidError::ServiceStatusNotOkError { status: s } => assert_eq!(s, "not ok"),
            _ => (),
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_has_json_with_no_results_key() {
        let _m = mock("POST", PATH)
            .with_status(200)
            .with_body(r#"{"status": "ok"}"#)
            .create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        assert!(res.is_err());
        let err = res.unwrap_err().downcast::<AcoustidError>().unwrap();
        assert!(matches!(err, AcoustidError::NoResultsError));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_has_json_with_empty_results_key() {
        let _m = mock("POST", PATH)
            .with_status(200)
            .with_body(r#"{"status": "ok", "results": []}"#)
            .create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        assert!(res.is_err());
        let err = res.unwrap_err().downcast::<AcoustidError>().unwrap();
        assert!(matches!(err, AcoustidError::NoResultsError));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_has_json_with_one_result() {
        let body = r#"
            {
              "status": "ok",
              "results": [
                {
                  "id": "results-A",
                  "score": 1.0,
                  "recordings": [
                    {
                      "id": "recording-A"
                    }
                  ]
                }
              ]
            }
        "#;
        let _m = mock("POST", PATH).with_status(200).with_body(body).create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        println!("{:#?}", res);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec!["recording-A"]);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_has_json_with_multiple_records() {
        let body = r#"
            {
              "status": "ok",
              "results": [
                {
                  "id": "results-A",
                  "score": 1.0,
                  "recordings": [
                    {
                      "id": "recording-A"
                    },
                    {
                      "id": "recording-B"
                    }
                  ]
                }
              ]
            }
        "#;
        let _m = mock("POST", PATH).with_status(200).with_body(body).create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        println!("{:#?}", res);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec!["recording-A", "recording-B"]);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn response_has_json_with_multiple_result() {
        let body = r#"
            {
              "status": "ok",
              "results": [
                {
                  "id": "results-A",
                  "score": 0.9,
                  "recordings": [
                    {
                      "id": "recording-D"
                    },
                    {
                      "id": "recording-B"
                    }
                  ]
                },
                {
                  "id": "results-A",
                  "score": 1.0,
                  "recordings": [
                    {
                      "id": "recording-C"
                    },
                    {
                      "id": "recording-A"
                    }
                  ]
                }
              ]
            }
        "#;
        let _m = mock("POST", PATH).with_status(200).with_body(body).create();
        let client = AcoustIDClient::new("invalid".to_string());
        let res = client.lookup("fingerprint", 42 as u64).await;
        println!("{:#?}", res);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            vec!["recording-C", "recording-A", "recording-D", "recording-B"],
        );
    }
}
