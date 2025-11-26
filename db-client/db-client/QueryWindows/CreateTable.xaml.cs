using System.Collections.ObjectModel;
using System.Windows;

namespace db_client.QueryWindows
{
    /// <summary>
    /// Interaction logic for CreateTable.xaml
    /// </summary>
    public partial class CreateTable : Window
    {
        public Table? Result { get; set; }
        private ObservableCollection<Field> Fields { get; set; }

        public CreateTable()
        {
            InitializeComponent();
            Fields = [];
            FieldsListView.ItemsSource = Fields;
        }

        private void AddField_Click(object sender, RoutedEventArgs e)
        {
            var addFieldWindow = new AddField();
            if (addFieldWindow.ShowDialog() == true)
            {
                Field newField = addFieldWindow.Result!;
                Fields.Add(newField);
            }
        }

        private void DeleteField_Click(object sender, RoutedEventArgs e)
        {
            if (FieldsListView.SelectedItem is Field field)
                Fields.Remove(field);
        }

        private void Ok_Click(object sender, RoutedEventArgs e)
        {
            if (TableNameTextBox.Text.Length == 0)
            {
                MessageBox.Show("Please specify table name.");
                return;
            }
            if (Fields.Count == 0)
            {
                MessageBox.Show("Table shold consist of at least one field.");
                return;
            }

            Result = new Table(TableNameTextBox.Text, Fields.ToList());

            DialogResult = true;
            Close();
        }

        private void Cancel_Click(object sender, RoutedEventArgs e)
        {
            Close();
        }
    }
}
