public interface IValve
{
    float OpenPercent { get; }
    static float MultiplierTime(params float[] array)
    {
        float result = 1f;
        foreach (float item in array) result *= item;
        return result;
    }
    bool IsOpen { get; }
    bool IsClose { get; }
    bool IsOpening { get; }
    bool IsClosing { get; }
}
