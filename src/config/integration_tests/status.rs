use super::super::status::HealthStatusResponse;
mod tests {
    use super::HealthStatusResponse;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_health_status() {
        let client = Client::tracked(crate::rocket()).expect("Rocket client");
        // when
        let response = client.get("/status/health").dispatch();

        // then
        assert_eq!(response.status(), Status::Ok);

        let body: HealthStatusResponse =
            serde_json::from_str(response.into_string().unwrap().as_str()).unwrap();
        assert_eq!(
            body,
            HealthStatusResponse {
                status: "STARTING_UP".to_string()
            }
        )
    }
}
