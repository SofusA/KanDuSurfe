public class Direction
{
    public int Minimum { get; set; }
    public int Maximum { get; set; }

    public Direction(int minimum, int maximum)
    {
        Minimum = minimum;
        Maximum = maximum;
    }
}