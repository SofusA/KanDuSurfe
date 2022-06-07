using System.Collections.Generic;

public class Spot
{
    public string Name { get; set; }
    public double Latitude { get; set; }
    public double Longitude { get; set; }
    public List<Direction> Directions { get; set; }

    public Spot(string name, double latitude, double longitude, List<Direction> directions)
    {
        Name = name;
        Latitude = latitude;
        Longitude = longitude;
        Directions = directions;
    }
}