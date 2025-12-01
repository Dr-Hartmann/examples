namespace WpfApp1.Abstractions;

public interface IValueService
{
    void Clear(IValueModel model);
    void Pop(IValueModel model);
    Task Push(IValueModel model);
    Task Random(IValueModel model);
}