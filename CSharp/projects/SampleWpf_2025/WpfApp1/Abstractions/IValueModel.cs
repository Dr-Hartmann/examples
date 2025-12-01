using System.Collections.ObjectModel;
using System.ComponentModel;

namespace WpfApp1.Abstractions;

public interface IValueModel : INotifyPropertyChanged
{
    int Value { get; set; }
    ObservableCollection<int> ValuesList { get; set; }
}