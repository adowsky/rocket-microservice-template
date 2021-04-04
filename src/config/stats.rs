use crate::stats::Metrics;
use async_trait::async_trait;
use common_macros::hash_map;
use rocket::fairing::Fairing;
use rocket::{fairing::Kind, get, Data, Request, Response, Rocket, State};
use serde::Deserialize;
use std::{time::Instant};

pub(crate) const DEFAULT_PROMETHEUS_ENDPOINT: &str = "/prometheus";

#[derive(Deserialize)]
struct PrometheusConfig {
    prometheus_endpoint: Option<String>,
}

#[get("/")]
fn collect_metrics(metrics: State<Metrics>) -> String {
    match metrics.collect() {
        Ok(metrics) => metrics,
        //TODO handle errors
        Err(_) => "".to_string(),
    }
}

pub(crate) fn configure_prometheus_metrics(rocket: Rocket) -> Rocket {
    let prometeus_url = rocket
        .figment()
        .extract::<PrometheusConfig>()
        .expect("Prometheus configuration")
        .prometheus_endpoint
        .unwrap_or(DEFAULT_PROMETHEUS_ENDPOINT.to_string());

    let metrics = Metrics::new();
    rocket
        .attach(metrics.clone())
        .manage(metrics)
        .mount(prometeus_url, routes![collect_metrics])
}

#[async_trait]
impl Fairing for Metrics {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Metrics",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data) {
        req.local_cache(|| Instant::now());
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let request_start_time = req.local_cache(|| Instant::now());
        let time_elapsed = Instant::now().duration_since(*request_start_time);
        if req.route().is_none() {
            return;
        }

        let record_result = self.record_labelled(
            "request_millis",
            time_elapsed.as_millis() as f64,
            &hash_map! {
                "path" => req.route().unwrap().uri.to_string(),
                "http_method" => req.route().unwrap().method.to_string(),
                "response_code" => res.status().code.to_string()
            },
        );

        if let Err(error) = record_result {
            error!("Cannot record request metrics {}", error)
        }
    }
}
