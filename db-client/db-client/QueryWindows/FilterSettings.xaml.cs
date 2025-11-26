using System.Collections.ObjectModel;
using System.Windows;

namespace db_client.QueryWindows
{
    /// <summary>
    /// Interaction logic for FilterSettings.xaml
    /// </summary>

    public partial class FilterSettings : Window
    {
        private List<Field> fields;
        public ObservableCollection<FilterOption> Result { get; set; }

        public FilterSettings(List<Field> fields)
        {
            InitializeComponent();
            this.fields = fields;
            Result = new ObservableCollection<FilterOption>();
            DataContext = this;
        }

        private void AddFilterButton_Click(object sender, RoutedEventArgs e)
        {
            var addFilterWindow = new AddFilter(fields);
            if (addFilterWindow.ShowDialog() == true)
            {
                Result.Add(addFilterWindow.Result);
            }
        }

        private void ApplyButton_Click(object sender, RoutedEventArgs e)
        {
            DialogResult = true;
            Close();
        }

        private void CancelButton_Click(object sender, RoutedEventArgs e)
        {
            Close();
        }
    }
}
