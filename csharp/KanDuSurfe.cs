using Microsoft.AspNetCore.Mvc;
using Microsoft.Azure.WebJobs;
using Microsoft.Azure.WebJobs.Extensions.Http;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;
using System.Collections.Generic;

public class KanDuSurfe
{
    [FunctionName("KanDuSurfe")]
    public IActionResult Run(
        [HttpTrigger(AuthorizationLevel.Function, "get", "post", Route = null)] HttpRequest req,
        ILogger log)
    {
        log.LogInformation("C# HTTP trigger function processed a request.");

        var spots = SpotService.GetSpots();

        var response = new List<SpotResponse>();
        var forecastService = new ForecastService();

        foreach (var spot in spots)
        {
            response.Add(forecastService.GetForecast(spot));
        }

        return new OkObjectResult(response);
    }
}
