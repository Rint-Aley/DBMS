using System.Windows;

namespace db_client.QueryWindows
{
    /// <summary>
    /// Interaction logic for AddChange.xaml
    /// </summary>
    public partial class AddChange : Window
    {
        public ChangeOption Result { get; private set; }

        private readonly List<Field> _fields;

        public AddChange(List<Field> fields)
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
                MessageBox.Show("Please add value to change.");
                return;
            }

            Result = new ChangeOption(selectedField, ValueTextBox.Text);

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
