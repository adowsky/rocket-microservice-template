use super::{MetricError, Metrics};

mod tests {
    use super::{MetricError, Metrics};
    use common_macros::hash_map;

    #[test]
    fn should_collect_counter_metrics_with_labels() -> Result<(), MetricError> {
        // given
        let metrics = Metrics::new();
        let tags = hash_map! { "tag" => "tag".to_string() };

        // when
        metrics.increment_labelled("my_name", 1.2, &tags)?;
        metrics.increment_labelled("my_name", 1.5, &tags)?;

        // then
        assert!(metrics.collect()?.contains("my_name{tag=\"tag\"} 2.7\n"));
        Ok(())
    }

    #[test]
    fn should_collect_counter_metrics() -> Result<(), MetricError> {
        // given
        let metrics = Metrics::new();

        // when
        metrics.increment("my_name", 1.2)?;
        metrics.increment("my_name", 1.5)?;

        // then
        assert!(metrics.collect()?.contains("my_name 2.7\n"));
        Ok(())
    }

    #[test]
    #[should_panic(expected = " PrometheusError(AlreadyReg)")]
    fn should_error_when_calling_labelled_and_nonlabelled_counter_with_same_name() {
        // given
        let metrics = Metrics::new();

        // when
        metrics.increment("my_name", 1.2).unwrap();
        metrics
            .increment_labelled("my_name", 1.5, &hash_map! { "tag" => "tag".to_string() })
            .unwrap();
    }

    #[test]
    fn should_collect_histogram_metrics_with_labels() -> Result<(), MetricError> {
        // given
        let metrics = Metrics::new();
        let tags = hash_map! { "tag" => "tag".to_string() };

        // when
        metrics.record_labelled("my_name", 1.2, &tags)?;
        metrics.record_labelled("my_name", 1.5, &tags)?;

        let collected_metrics = metrics.collect()?;
        // then
        assert!(collected_metrics.contains("my_name_count{tag=\"tag\"} 2"));
        assert!(collected_metrics.contains("my_name_sum{tag=\"tag\"} 2.7"));
        Ok(())
    }

    #[test]
    fn should_collect_histogram_metrics() -> Result<(), MetricError> {
        // given
        let metrics = Metrics::new();

        // when
        metrics.record("my_name", 1.2)?;
        metrics.record("my_name", 1.5)?;

        let collected_metrics = metrics.collect()?;
        // then
        assert!(collected_metrics.contains("my_name_count 2"));
        assert!(collected_metrics.contains("my_name_sum 2.7"));
        Ok(())
    }

    #[test]
    #[should_panic(expected = " PrometheusError(AlreadyReg)")]
    fn should_error_when_calling_labelled_and_nonlabelled_histogram_with_same_name() {
        // given
        let metrics = Metrics::new();

        // when
        metrics.record("my_name", 1.2).unwrap();
        metrics
            .record_labelled("my_name", 1.5, &hash_map! { "tag" => "tag".to_string() })
            .unwrap();
    }

    #[test]
    #[should_panic(expected = " PrometheusError(AlreadyReg)")]
    fn should_error_when_calling_histogram_and_counter_with_same_name() {
        // given
        let metrics = Metrics::new();

        // when
        metrics.record("my_name", 1.2).unwrap();
        metrics.increment("my_name", 1.5).unwrap();
    }
}
