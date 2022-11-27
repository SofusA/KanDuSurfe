use async_trait::async_trait;
use handler_lib::{
    functions::get_response::get_forecast_from_provider,
    models::{
        forecast::{Data, Details, ForeCastRoot, ForecastProvider, Instant, Properties, Series},
        spot::Spot,
    },
};

struct TestForecast {
    timeseries: Vec<Series>
}

impl TestForecast {
    fn add_timeseries(&mut self, timeseries: Vec<Series>) {
        self.timeseries = timeseries;
    }
}

#[async_trait]
impl ForecastProvider for TestForecast {
    async fn get_forecast(&self, _spot: &Spot) -> ForeCastRoot {
        let forecast_root = ForeCastRoot {
            properties: Properties {
                timeseries: self.timeseries.clone()
            },
        };

        return forecast_root;
    }

    fn new() -> TestForecast {
        return TestForecast {
            timeseries: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::TestForecast;

    #[tokio::test]
    async fn get_forecast_from_provider_test() {
        let mut test_forecast_provider: TestForecast = ForecastProvider::new();

        test_forecast_provider.add_timeseries(vec![Series {
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
        },
        Series {
            time: "2022-09-20T11:00:00Z".to_string(),
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
        },
        Series {
            time: "2022-09-20T12:00:00Z".to_string(),
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
        }]);

        let forecast = get_forecast_from_provider(test_forecast_provider).await;
        let forcast_day = forecast.first().unwrap();

        assert_eq!(forcast_day.day, "2022-09-20");
        assert_eq!(forcast_day.spots.contains(&"Amager Strandpark".to_string()), true);
        assert_eq!(forcast_day.spots.contains(&"Sydvestpynten".to_string()), false);
    }

    #[tokio::test]
    async fn invalid_dates_test_single() {
        let mut test_forecast_provider: TestForecast = ForecastProvider::new();

        test_forecast_provider.add_timeseries(vec![Series {
            time: "2023-02-20T10:00:00Z".to_string(),
            data: Data {
                instant: Instant {
                    details: Details {
                        air_temperature: 20.0,
                        wind_from_direction: 100.0,
                        relative_humidity: 80.0,
                        wind_speed: 8.0,
                    },
                },
            },
        },
        Series {
            time: "2023-02-20T12:00:00Z".to_string(),
            data: Data {
                instant: Instant {
                    details: Details {
                        air_temperature: 20.0,
                        wind_from_direction: 100.0,
                        relative_humidity: 80.0,
                        wind_speed: 8.0,
                    },
                },
            },
        },
        Series {
            time: "2023-02-20T13:00:00Z".to_string(),
            data: Data {
                instant: Instant {
                    details: Details {
                        air_temperature: 20.0,
                        wind_from_direction: 100.0,
                        relative_humidity: 80.0,
                        wind_speed: 8.0,
                    },
                },
            },
        }
        ]);

        let forecast = get_forecast_from_provider(test_forecast_provider).await;

        println!("{:?}", forecast);

        assert_eq!(forecast.first().unwrap().spots.contains(&"Poppelvej".to_string()), false);

    }
}
