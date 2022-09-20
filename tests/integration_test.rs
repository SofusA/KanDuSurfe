use async_trait::async_trait;
use handler::{
    functions::get_response::get_forecast_from_provider,
    models::{
        forecast::{Data, Details, ForeCastRoot, ForecastProvider, Instant, Properties, Series},
        spot::Spot,
    },
};

struct TestForecast {}

#[async_trait]
impl ForecastProvider for TestForecast {
    async fn get_forecast(&self, _spot: &Spot) -> ForeCastRoot {
        let forecast_root = ForeCastRoot {
            properties: Properties {
                timeseries: vec![Series {
                    time: "2022-09-20T08:00:00Z".to_string(),
                    data: Data {
                        instant: Instant {
                            details: Details {
                                air_temperature: 20.0,
                                wind_from_direction: 90.0,
                                relative_humidity: 80.0,
                                wind_speed: 8.0,
                            },
                        },
                    },
                }],
            },
        };

        return forecast_root;
    }

    fn new() -> TestForecast {
        return TestForecast {};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::TestForecast;

    #[tokio::test]
    async fn get_forecast_from_provider_test() {
        let test_forecast_provider: TestForecast = ForecastProvider::new();
        let forecast = get_forecast_from_provider(test_forecast_provider).await;

        assert_eq!(forecast.msg.contains("2022-09-20"), true);
        assert_eq!(forecast.msg.contains("Amager Strandpark"), true);
        assert_eq!(forecast.msg.contains("Sydvestpynten"), false);
        assert_eq!(forecast.surfable, true);
    }
}
