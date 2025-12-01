using ReactiveUI;
using ReactiveUI.Fody.Helpers;
using System.Collections.ObjectModel;
using WpfApp1.Abstractions;

namespace WpfApp1.Models;

internal class MainWindowModel : ReactiveObject, IValueModel
{
    [Reactive] public int Value { get; set; }
    public ObservableCollection<int> ValuesList { get; set; } = [44, 55, 70, 86];
}
