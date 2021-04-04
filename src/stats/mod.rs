#[cfg(test)]
mod tests;


use std::{collections::HashMap, ops::Deref, string::FromUtf8Error};
use std::sync::Arc;

use flurry::HashMap as ConcurrentHashMap;
use prometheus::{Counter, CounterVec, Encoder, Histogram, HistogramOpts, HistogramVec, Opts, Registry, TextEncoder, core::Collector};

pub(crate) enum MetricError {}

const NO_TAGS: Vec<&'static str> = Vec::new();

#[derive(Clone)]
pub(crate) struct Metrics {
    registry: Registry,
    counters: Arc<ConcurrentHashMap<String, Counter>>,
    labelled_counters: Arc<ConcurrentHashMap<String, CounterVec>>,

    histograms: Arc<ConcurrentHashMap<String, Histogram>>,
    labelled_histograms: Arc<ConcurrentHashMap<String, HistogramVec>>,
}

impl Metrics {
    pub(crate) fn new() -> Metrics {
        Metrics {
            registry: Registry::new(),
            counters: Arc::new(ConcurrentHashMap::new()),
            labelled_counters: Arc::new(ConcurrentHashMap::new()),
            histograms: Arc::new(ConcurrentHashMap::new()),
            labelled_histograms: Arc::new(ConcurrentHashMap::new()),
        }
    }

    pub(crate) fn collect(&self) -> Result<String, FromUtf8Error> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut encoded_metrics = Vec::new();
        encoder.encode(&metric_families, &mut encoded_metrics);
        String::from_utf8(encoded_metrics)
    }

    pub(crate) fn increment(
        &self,
        name: &'static str,
        increment_by: f64,
        labels: &HashMap<&str, &str>,
    ) {
        self.ensure_counter_exists(name, labels);
        self
            .labelled_counters
            .get(name, &self.counters.guard())
            .unwrap()
            .get_metric_with(labels)
            .expect("Cannot create labelled metric")
            .inc_by(increment_by);

    }

    fn ensure_counter_exists(&self, name: &'static str, labels: &HashMap<&str, &str>) {
        let label_keys: Vec<&str> = labels.keys().map(|k| k.deref()).collect();
        if !self.labelled_counters.contains_key(name, &self.counters.guard()) {
            let counter = CounterVec::new(Opts::new(name, name), &label_keys)
            .expect("Labelled counter cannot be created");

            self.registry.register(Box::new(counter.clone()));
            self.labelled_counters
                .insert(String::from(name), counter, &self.labelled_counters.guard());
        }
    }

    pub(crate) fn record(&self, name: &'static str, value: f64, labels: &HashMap<&str, String>) {
        self.ensure_histogram_exists(name, labels);
        let labels_ref: HashMap<&str, &str> = labels.iter().map(|(k,v)| (k.deref(), v.as_str())).collect();

        self
        .labelled_histograms
        .get(name, &self.counters.guard())
        .unwrap()
        .get_metric_with(&labels_ref)
        .expect("Cannot create labelled metric")
        .observe(value);
    }

    fn ensure_histogram_exists(&self, name: &'static str, labels: &HashMap<&str, String>) {
        let label_keys: Vec<&str> = labels.keys().map(|k| k.deref()).collect();
        if !self.labelled_histograms.contains_key(name, &self.counters.guard()) {
            let histogram = HistogramVec::new(HistogramOpts::new(name, name), &label_keys)
            .expect("Labelled histogram cannot be created");

            self.registry.register(Box::new(histogram.clone()));
            self.labelled_histograms
                .insert(String::from(name), histogram, &self.labelled_histograms.guard());
        }
    }
}
