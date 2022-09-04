using System.Collections.Generic;

public class SpotService
{
    public static List<Spot> GetSpots()
    {
        return CreateSpots();
    }

    private static List<Spot> CreateSpots()
    {
        return new List<Spot>{
            new Spot(
                name: "Sydvestpynten",
                latitude: 55.565231,
                longitude: 12.558465,
                new List<Direction>
                {
                    new Direction( minimum: 315, maximum: 180)
                }
            ),
            new Spot(
                name: "Amager Strandpark",
                latitude: 55.653506,
                longitude: 12.645545,
                new List<Direction>
                {
                    new Direction( minimum: 157, maximum: 45)
                }
            ),
            new Spot(
                name: "Lynæs",
                latitude: 55.942500,
                longitude: 11.862056,
                new List<Direction>
                {
                    new Direction( minimum: 315, maximum: 135)
                }
            ),
            new Spot(
                name: "Poppelvej",
                latitude: 55.559259,
                longitude: 12.612591,
                new List<Direction>
                {
                    new Direction( minimum: 180, maximum: 90)
                }
            ),
            new Spot(
                name: "Farø",
                latitude: 54.942552,
                longitude: 12.001952,
                new List<Direction>
                {
                    new Direction( minimum: 225, maximum: 337),
                    new Direction( minimum: 45, maximum: 135),
                }
            )
        };
    }
}