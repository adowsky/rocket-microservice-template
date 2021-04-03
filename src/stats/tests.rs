use super::Metrics;

#[test]
fn test_counter() {
    // given
    let metrics = Metrics::new();

    // when
    metrics.increment("my_name", 1.2).unwrap();
    metrics.increment("my_name", 1.5).unwrap();


    // then
    assert!(metrics.collect().unwrap().contains("my_name 2.7\n"));
}

#[test]
fn test_histogram() {
    // given
    let metrics = Metrics::new();

    // when
    metrics.record("my_name", 1.2).unwrap();
    metrics.record("my_name", 1.5).unwrap();
    let collected_metrics = metrics.collect().unwrap();

    // then
    assert!(collected_metrics.contains("my_name_count 2"));
    assert!(collected_metrics.contains("my_name_sum 2.7"));
}