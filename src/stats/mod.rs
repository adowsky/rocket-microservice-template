use std::string::FromUtf8Error;

use flurry::HashMap as ConcurrentHashMap;
use prometheus::{CounterVec, Encoder, Histogram, HistogramOpts, Registry, TextEncoder};

#[cfg(test)]
mod tests;

pub(crate) enum MetricError {}

#[derive(Clone)]
pub(crate) struct Metrics {
    registry: Registry,
    counters: ConcurrentHashMap<String, CounterVec>,
    histograms: ConcurrentHashMap<String, Histogram>,
}

impl Metrics {
    pub(crate) fn new() -> Metrics {
        Metrics {
            registry: Registry::new(),
            counters: ConcurrentHashMap::new(),
            histograms: ConcurrentHashMap::new(),
        }
    }

    pub(crate) fn collect(&self) -> Result<String, FromUtf8Error> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut encoded_metrics = Vec::new();
        encoder.encode(&metric_families, &mut encoded_metrics);
        String::from_utf8(encoded_metrics)
    }

    pub(crate) fn increment(&self, name: &str, increment_by: f64) -> Result<(), ()> {
        match self.counters.get(name, &self.counters.guard()) {
            Some(counter) => counter.inc_by(increment_by),
            None => self.register_counter(name, increment_by),
        }
        Ok(())
    }

    fn register_counter(&self, name: &str, initial_value: f64) {
        let counter = CounterVec::new(name, name).expect("Counter to be created sucessfully");
        counter.inc_by(initial_value);

        self.registry
            .register(Box::new(counter.clone()))
            .expect(format!("Counter {} to be registered sucessfully", name).as_str());

        self.counters
            .insert(String::from(name), counter, &self.counters.guard());
    }

    pub(crate) fn record(&self, name: &str, value: f64) -> Result<(), ()> {
        match self.histograms.get(name, &self.histograms.guard()) {
            Some(histogram) => histogram.observe(value),
            None => self.register_histogram_and_observe(name, value),
        }
        Ok(())
    }

    fn register_histogram_and_observe(&self, name: &str, value: f64) {
        let histogram = Histogram::with_opts(HistogramOpts::new(name, name))
            .expect("Histogram to be created sucessfully");
        histogram.observe(value);

        self.registry
            .register(Box::new(histogram.clone()))
            .expect(format!("Histogram {} to be registered sucessfully", name).as_str());

        self.histograms
            .insert(String::from(name), histogram, &self.histograms.guard());
    }
}
