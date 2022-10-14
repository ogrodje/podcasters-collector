use ureq::Response;

pub use anchor_collector::anchor_client::AnchorClient;
pub use anchor_collector::error::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_json_failure() -> () {
        Response::new(200, "Ok", "broken-payload")
            .map_err(|e| AnchorError::HttpError(e.to_string()))
            .and_then(|r| AnchorClient::parse_json(r))
            .expect("Boom");
    }

    #[test]
    #[should_panic]
    fn parse_json_failure_500() -> () {
        Response::new(500, "Ok", "broken-payload")
            .map_err(|e| AnchorError::HttpError(e.to_string()))
            .and_then(|r| AnchorClient::parse_json(r))
            .expect("Boom");
    }

    #[test]
    fn parse_json_ok() -> () {
        let json_result = Response::new(200, "Ok", "{\"result\": \"ok\"}")
            .map_err(|e| AnchorError::HttpError(e.to_string()))
            .and_then(|r| AnchorClient::parse_json(r))
            .unwrap();

        assert!(json_result["result"] == "ok", "Parsing JSON has failed.")
    }

    #[test]
    fn parse_string_ok() -> () {
        let response = Response::new(200, "ok", "ok").unwrap();
        assert_eq!(AnchorClient::parse_string(response).unwrap(), "ok")
    }

    #[test]
    #[should_panic]
    fn parse_string_failure() -> () {
        let response = Response::new(500, "ok", "x").unwrap();
        assert_eq!(AnchorClient::parse_string(response).unwrap(), "ok")
    }
}
