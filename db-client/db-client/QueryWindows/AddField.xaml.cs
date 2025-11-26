using System.Windows;

namespace db_client.QueryWindows
{
    /// <summary>
    /// Interaction logic for AddField.xaml
    /// </summary>
    public partial class AddField : Window
    {
        public Field? Result { get; private set; }
        public AddField()
        {
            InitializeComponent();
            TypeComboBox.ItemsSource = Enum.GetValues(typeof(FieldType));
        }

        private void Ok_Click(object sender, RoutedEventArgs e)
        {
            if (NameTextBox.Text.Length == 0)
                return;
            if (TypeComboBox.SelectedItem == null)
                return;
            Result = new Field(NameTextBox.Text, (FieldType)TypeComboBox.SelectedItem);
            DialogResult = true;
            Close();
        }

        private void Cancel_Click(object sender, RoutedEventArgs e)
        {
            Close();
        }
    }
}
