use async_trait::async_trait;
use handler_lib::{
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
                    time: "2022-09-20T10:00:00Z".to_string(),
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

        print!("{:?}", forecast);
        let forcast_day = forecast.first().unwrap();

        assert_eq!(forcast_day.day, "2022-09-20");
        assert_eq!(forcast_day.spots.contains("Amager Strandpark"), true);
        assert_eq!(forcast_day.spots.contains("Sydvestpynten"), false);
    }
}
