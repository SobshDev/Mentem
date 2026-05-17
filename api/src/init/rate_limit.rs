use std::time::Duration;

use axum::Router;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;

pub fn general(router: Router) -> Router
{
    let config = GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .per_second(10)
        .burst_size(20)
        .finish()
        .expect("invalid rate limit config");

    let limiter = config.limiter().clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;
            limiter.retain_recent();
        }
    });

    router.layer(GovernorLayer::new(config))
}

pub fn auth_login<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let config = GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .per_second(1)
        .burst_size(5)
        .finish()
        .expect("invalid auth login rate limit config");

    let limiter = config.limiter().clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;
            limiter.retain_recent();
        }
    });

    router.layer(GovernorLayer::new(config))
}

pub fn auth_register<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let config = GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .per_second(1)
        .burst_size(5)
        .finish()
        .expect("invalid auth register rate limit config");

    let limiter = config.limiter().clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;
            limiter.retain_recent();
        }
    });

    router.layer(GovernorLayer::new(config))
}
