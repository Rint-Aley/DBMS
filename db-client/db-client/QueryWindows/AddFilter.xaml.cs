using System;
using System.Collections.Generic;
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
    /// Interaction logic for AddFilter.xaml
    /// </summary>
    public partial class AddFilter : Window
    {

        public FilterOption Result { get; private set; }

        private readonly List<Field> _fields;

        public AddFilter(List<Field> fields)
        {
            InitializeComponent();
            _fields = fields;

            FieldComboBox.ItemsSource = _fields;
            FieldComboBox.SelectedIndex = 0;
        }

        private void Ok_Click(object sender, RoutedEventArgs e)
        {
            var selectedField = FieldComboBox.SelectedItem as Field;
            if (selectedField == null)
            {
                MessageBox.Show("Please select a field.");
                return;
            }
            
            // TODO: add typing
            object typedValue = ValueTextBox.Text;

            Result = new FilterOption(selectedField, typedValue);

            DialogResult = true;
            Close();
        }

        private void Cancel_Click(object sender, RoutedEventArgs e)
        {
            DialogResult = false;
            Close();
        }
    }
}
