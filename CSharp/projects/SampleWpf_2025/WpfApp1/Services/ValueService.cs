using System.Reactive.Concurrency;
using System.Reactive.Linq;
using WpfApp1.Abstractions;

namespace WpfApp1.Services;

internal class ValueService : IValueService
{
    public async Task Push(IValueModel model)
    {
        if (model.Value < 0 || model.Value > 99) return;
        await Task.Delay(100);
        model.ValuesList.Add(model.Value);
        model.Value = 0;
    }

    public void Pop(IValueModel model)
    {
        if (!model.ValuesList.Any()) return;
        model.Value = model.ValuesList.First();
        model.ValuesList.RemoveAt(0);
    }

    public async Task Random(IValueModel model)
    {
        var randObs = Observable.Interval(TimeSpan.FromMilliseconds(10))
                 .ObserveOn(DispatcherScheduler.Current)
                 .Take(20)
                 .Select(x => System.Random.Shared.Next(0, 90))
                 .Subscribe(x => model.Value = x);
        for (var i = 0; i < 3; i++)
        {
            await Task.Delay(500);
            model.Value += System.Random.Shared.Next(0, 3);
        }
    }

    public void Clear(IValueModel model)
    {
        model.ValuesList.Clear();
    }
}
