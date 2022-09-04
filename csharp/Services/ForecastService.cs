using System;
using System.Linq;
using System.Collections.Generic;
using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json;

public class ForecastService
{
    private readonly HttpClient _client;
    private readonly (int startHour, int stopHour, double windSpeed) _surfConstants;

    public ForecastService()
    {
        _client = new HttpClient();
        _client.DefaultRequestHeaders.UserAgent.Add(new System.Net.Http.Headers.ProductInfoHeaderValue("KanDuSurfe", "0.1"));

        _surfConstants = (8, 20, 7.5);
    }

    public SpotResponse GetForecast(Spot spot)
    {
        var forecastUrl = $"https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={spot.Latitude}&lon={spot.Longitude}";

        var response = _client.GetAsync(forecastUrl).Result;
        var result = response.Content.ReadAsStringAsync().Result;
        var yrResponse = JsonConvert.DeserializeObject<YrResponse>(result);

        var surfDays = new List<DateTime>();

        foreach(var forecast in yrResponse.Properties.Timeseries) 
        {
            var forecastHour = forecast.Time.Hour;
            var forecastData = forecast.Data.Instant.Details;

            if (forecastHour < _surfConstants.startHour || forecastHour > _surfConstants.stopHour ) continue;
            if (forecastData.Wind_speed < _surfConstants.windSpeed) continue;
            if (!SurfableDirection(spot.Directions, forecastData.Wind_from_direction)) continue;

            surfDays.Add(forecast.Time);
        }

        return new SpotResponse(spot.Name, surfDays);
    }

    private bool SurfableDirection(List<Direction> allowedDirections, double forecastDirection)
    {
        foreach(var direction in allowedDirections)
        {
            if (direction.Minimum < forecastDirection && direction.Maximum > forecastDirection) {
                return true;
            }   
        }
        return false;
    }
}