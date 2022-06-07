using System;
using System.Collections.Generic;

public class SpotResponse
{
    public string Name { get; set; }
    public List<DateTime> SurfDays { get; set; }

    public SpotResponse(string name, List<DateTime> surfDays)
    {
        Name = name;
        SurfDays = surfDays;
    }
}