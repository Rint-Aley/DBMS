using System.Windows;

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
            if (ValueTextBox.Text.Length == 0)
            {
                MessageBox.Show("Please add value for filter.");
                return;
            }

            Result = new FilterOption(selectedField, ValueTextBox.Text);

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
