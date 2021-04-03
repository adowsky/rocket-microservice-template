# Rocket microsevice template
Template with preconfigured Rocket instance to start fast with Rust + Rocket stack.

## What included

- Preconfigured Rocket 1.5 (with tokio runtime),
- registered Prometheus mertic endpoint with Metric facade bindable by Rocket's state feature,
- simple domain service injectable through Rocket state modelled in DDD approach,
- example of integration tests for endpoint