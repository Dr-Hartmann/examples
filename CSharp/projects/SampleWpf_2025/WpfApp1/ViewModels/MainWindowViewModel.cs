using DynamicData.Binding;
using ReactiveUI;
using System.Collections.Specialized;
using System.Reactive.Concurrency;
using System.Reactive.Linq;
using System.Windows.Input;
using WpfApp1.Abstractions;
using WpfApp1.Models;
using WpfApp1.Services;

namespace WpfApp1.ViewModels;

internal class MainWindowViewModel
{

    public MainWindowViewModel()
    {
        // Реактивное программирование
        var inputObs = Model.WhenAnyValue(x => x.Value);
        var boolObs = inputObs.Select(x => x is >= 0 and < 100);

        PushCommand = ReactiveCommand.CreateFromTask(() => ValueService.Push(Model), boolObs);
        RandCommand = ReactiveCommand.CreateFromTask<IValueModel>(ValueService.Random);

        var canPopObs = Model.ValuesList
            .ToObservableChangeSet()
            .Select(_ => Model.ValuesList.Count > 0);

        // Используем ReactiveCommand.Create с этим Observable
        PopCommand = ReactiveCommand.Create(() => ValueService.Pop(Model), canPopObs);
        ClearCommand = ReactiveCommand.Create(() => ValueService.Clear(Model), canPopObs);

        var ascObs = Observable.Interval(TimeSpan.FromMilliseconds(10))
            .ObserveOn(DispatcherScheduler.Current)
            .Skip(1)
            .Take(99);
        var descObs = ascObs.Select(x => 99 - x);
        Observable.Concat(ascObs, descObs)
            .Subscribe(x => Model.Value = (int)x);
    }

    public IValueService ValueService { get; init; } = new ValueService();
    public IValueModel Model { get; init; } = new MainWindowModel();
    public ICommand PushCommand { get; init; }
    public ICommand PopCommand { get; init; }
    public ICommand ClearCommand { get; init; }
    public ICommand RandCommand { get; init; }


    private bool CanPush() => Model.Value > 0;

    private bool CanPop() => Model.ValuesList.Count > 0;
}
