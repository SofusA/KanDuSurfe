using System;
using System.Collections.Generic;

public class YrResponse
{
    public PropertiesData Properties { get; set; }
}

public class PropertiesData
{
    public List<TimeSerie> Timeseries { get; set; }
}

public class TimeSerie
{
    public DateTime Time { get; set; }
    public Data Data { get; set; }
}

public class Data
{
    public InstantData Instant { get; set; }
}
 
public class InstantData
{
    public DetailsData Details { get; set; }
}

public class DetailsData
{
    public double Air_temperature { get; set; }
    public double Relative_humidity { get; set; }
    public double Wind_from_direction { get; set; }
    public double Wind_speed { get; set; }
}