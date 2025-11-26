using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Shapes;

namespace db_client.QueryWindows
{
    /// <summary>
    /// Interaction logic for ChangeRecords.xaml
    /// </summary>
    public partial class ChangeRecords : Window
    {
        public ObservableCollection<FilterOption> Filters { get; set; }
        public ObservableCollection<ChangeOption> Changes { get; set; }
        private readonly List<Field> fields;
        public ChangeRecords(List<Field> fields)
        {
            InitializeComponent();
            this.fields = fields;
            Filters = new ObservableCollection<FilterOption>();
            Changes = new ObservableCollection<ChangeOption>();
            DataContext = this;
        }

        private void AddFilter_Click(object sender, RoutedEventArgs e)
        {
            var addFilterWindow = new AddFilter(fields);
            if (addFilterWindow.ShowDialog() == true)
            {
                Filters.Add(addFilterWindow.Result);
            }
        }

        private void AddChange_Click(object sender, RoutedEventArgs e)
        {
            var addChangeWindow = new AddChange(fields);
            if (addChangeWindow.ShowDialog() == true)
            {
                Changes.Add(addChangeWindow.Result);
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
