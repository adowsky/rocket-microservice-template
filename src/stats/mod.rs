#[cfg(test)]
mod tests;

use std::sync::Arc;
use std::{collections::HashMap, ops::Deref, string::FromUtf8Error};

use flurry::HashMap as ConcurrentHashMap;
use prometheus::{
    Counter, CounterVec, Encoder, Histogram, HistogramOpts, HistogramVec, Opts, Registry,
    TextEncoder,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MetricError {
    #[error("Prometheus metrics returned error")]
    PrometheusError(#[from] prometheus::Error),
    #[error("Cannot create string")]
    TextError(#[from] FromUtf8Error)
}

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

    pub(crate) fn collect(&self) -> Result<String, MetricError> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut encoded_metrics = Vec::new();
        encoder.encode(&metric_families, &mut encoded_metrics)?;
        Ok(String::from_utf8(encoded_metrics)?)
    }

    pub(crate) fn increment_labelled(
        &self,
        name: &'static str,
        increment_by: f64,
        labels: &HashMap<&str, String>,
    ) -> Result<(), MetricError> {
        let labels_ref: HashMap<&str, &str> = labels
            .iter()
            .map(|(k, v)| (k.deref(), v.as_str()))
            .collect();
        self.ensure_labelled_counter_exists(name, labels)?;
        self.labelled_counters
            .get(name, &self.labelled_counters.guard())
            .unwrap()
            .get_metric_with(&labels_ref)?
            .inc_by(increment_by);
        Ok(())
    }

    fn ensure_labelled_counter_exists(
        &self,
        name: &'static str,
        labels: &HashMap<&str, String>,
    ) -> Result<(), MetricError> {
        let label_keys: Vec<&str> = labels.keys().map(|k| k.deref()).collect();
        if !self
            .labelled_counters
            .contains_key(name, &self.labelled_counters.guard())
        {
            let counter = CounterVec::new(Opts::new(name, name), &label_keys)?;

            self.registry.register(Box::new(counter.clone()))?;
            self.labelled_counters.insert(
                String::from(name),
                counter,
                &self.labelled_counters.guard(),
            );
        }
        Ok(())
    }

    pub(crate) fn increment(
        &self,
        name: &'static str,
        increment_by: f64,
    ) -> Result<(), MetricError> {
        self.ensure_counter_exists(name)?;
        self.counters
            .get(name, &self.counters.guard())
            .unwrap()
            .inc_by(increment_by);
        Ok(())
    }

    fn ensure_counter_exists(&self, name: &'static str) -> Result<(), MetricError> {
        if !self.counters.contains_key(name, &self.counters.guard()) {
            let counter = Counter::new(name, name)?;

            self.registry.register(Box::new(counter.clone()))?;
            self.counters
                .insert(String::from(name), counter, &self.counters.guard());
        }
        Ok(())
    }

    pub(crate) fn record(&self, name: &'static str, value: f64) -> Result<(), MetricError> {
        self.ensure_histogram_exists(name)?;

        self.histograms
            .get(name, &self.counters.guard())
            .unwrap()
            .observe(value);
        Ok(())
    }

    fn ensure_histogram_exists(&self, name: &'static str) -> Result<(), MetricError> {
        if !self.histograms.contains_key(name, &self.histograms.guard()) {
            let histogram = Histogram::with_opts(HistogramOpts::new(name, name))?;

            self.registry.register(Box::new(histogram.clone()))?;
            self.histograms
                .insert(String::from(name), histogram, &self.histograms.guard());
        }
        Ok(())
    }

    pub(crate) fn record_labelled(
        &self,
        name: &'static str,
        value: f64,
        labels: &HashMap<&str, String>,
    ) -> Result<(), MetricError> {
        self.ensure_labelled_histogram_exists(name, labels)?;
        let labels_ref: HashMap<&str, &str> = labels
            .iter()
            .map(|(k, v)| (k.deref(), v.as_str()))
            .collect();

        self.labelled_histograms
            .get(name, &self.counters.guard())
            .unwrap()
            .get_metric_with(&labels_ref)?
            .observe(value);
        Ok(())
    }

    fn ensure_labelled_histogram_exists(
        &self,
        name: &'static str,
        labels: &HashMap<&str, String>,
    ) -> Result<(), MetricError> {
        let label_keys: Vec<&str> = labels.keys().map(|k| k.deref()).collect();
        if !self
            .labelled_histograms
            .contains_key(name, &self.labelled_histograms.guard())
        {
            let histogram = HistogramVec::new(HistogramOpts::new(name, name), &label_keys)?;

            self.registry.register(Box::new(histogram.clone()))?;
            self.labelled_histograms.insert(
                String::from(name),
                histogram,
                &self.labelled_histograms.guard(),
            );
        }
        Ok(())
    }
}
