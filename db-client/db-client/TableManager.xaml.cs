using db_client.QueryWindows;
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
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace db_client
{
    /// <summary>
    /// Interaction logic for TableManager.xaml
    /// </summary>
    public partial class TableManager : Page
    {
        public TableManager()
        {
            InitializeComponent();
            TablesListView.Items.Add(new Table("Some table", [new Field("id", FieldType.U64, true, true), new Field("some_value",FieldType.I32, true)]));
            TablesListView.Items.Add(new { Name = "PoSosy" });
        }

        private void CreateTableButton_Click(object sender, RoutedEventArgs e)
        {
            var createTableWindow = new CreateTable();
            createTableWindow.ShowDialog();
            // TODO: get table info from `createTableWindow`, pass it to grpc, and update the list
        }

        private void OpenTableButton_Click(object sender, RoutedEventArgs e)
        {
            if (TablesListView.SelectedItem is Table table)
            {
                var recordManagerPage = new RecordManager(table);
                NavigationService?.Navigate(recordManagerPage);
            }
        }

        private void DeleteTableButton_Click(object sender, RoutedEventArgs e)
        {
            var selectedTable = TablesListView.SelectedItem;
            // TODO: Delete table logic
            TablesListView.Items.Remove(selectedTable);
        }

        private void BackupButton_Click(object sender, RoutedEventArgs e)
        {

        }

        private void ExportButton_Click(object sender, RoutedEventArgs e)
        {

        }
    }
}
