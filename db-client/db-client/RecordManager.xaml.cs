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
    /// Interaction logic for RecordManager.xaml
    /// </summary>
    public partial class RecordManager : Page
    {
        private List<List<object>> records;
        private readonly List<string> header;
        private List<List<object>> Records { 
            get { return records; }
            set { 
                records.Clear();
                records = [header.OfType<Object>().ToList()];
                records.AddRange(value);
            } 
        }
        private Table Table { get; set; }
        public RecordManager(Table table)
        {
            InitializeComponent();
            Table = table;
            
            header = new List<string>(Table.Fields.Count);
            foreach (var field in Table.Fields)
                header.Add($"{field.Name}\n{field.type}");

            records = [];
            //Records = [[1, 3], [2, 102]];
            // TODO: Call get records here
            RecordsDataGrid.ItemsSource = ConvertToDataTable(Records).DefaultView;
        }

        private System.Data.DataTable ConvertToDataTable(List<List<object>> tableData)
        {
            var dataTable = new System.Data.DataTable();

            if (tableData.Count > 0)
            {
                // Create columns based on first row (headers)
                var headers = tableData[0];
                foreach (var header in headers)
                {
                    dataTable.Columns.Add(header.ToString());
                }

                // Add data rows
                for (int i = 1; i < tableData.Count; i++)
                {
                    var row = dataTable.NewRow();
                    for (int j = 0; j < tableData[i].Count; j++)
                    {
                        row[j] = tableData[i][j];
                    }
                    dataTable.Rows.Add(row);
                }
            }

            return dataTable;
        }

        private void ExitButton_Click(object sender, RoutedEventArgs e)
        {
            NavigationService?.GoBack();
        }

        private void AddRecordsButton_Click(object sender, RoutedEventArgs e)
        {
            var addRecordsWindow = new AddRecords(Table.Fields);
            if (addRecordsWindow.ShowDialog() == true)
            {
                List<string> newRecordValues = addRecordsWindow.Result!;
                // TODO: Call fucntion
            }
        }

        private void DeleteRecordsButton_Click(object sender, RoutedEventArgs e)
        {
            var filterSettingsWindow = new FilterSettings();
            filterSettingsWindow.ShowDialog();
            // TODO: Collect information and call fucntion
        }

        private void SelectRecordsButton_Click(object sender, RoutedEventArgs e)
        {
            var filterSettingsWindow = new FilterSettings();
            filterSettingsWindow.ShowDialog();
            // TODO: Change Preferences
        }

        private void ChangeRecordsButton_Click(object sender, RoutedEventArgs e)
        {
            var changeRecordsWindow = new ChangeRecords();
            changeRecordsWindow.ShowDialog();
            // TODO: Collect information and call fucntion
        }

        private void ClearTableButton_Click(object sender, RoutedEventArgs e)
        {
            // TODO: Call clear table
        }
    }
}
