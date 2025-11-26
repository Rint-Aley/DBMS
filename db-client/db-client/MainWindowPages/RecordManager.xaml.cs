using db_client.QueryWindows;
using System.Windows;
using System.Windows.Controls;

namespace db_client.MainWindowPages
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
                header.Add($"{field.Name}\n{field.Type}");

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
                // TODO: Call rpc
            }
        }

        private void DeleteRecordsButton_Click(object sender, RoutedEventArgs e)
        {
            var filterSettingsWindow = new FilterSettings(Table.Fields);
            if (filterSettingsWindow.ShowDialog() == true)
            {
                var filterOptions = filterSettingsWindow.Result.ToList();
                // TODO: Call rpc
            }
        }

        private void ChangeRecordsButton_Click(object sender, RoutedEventArgs e)
        {
            var changeRecordsWindow = new ChangeRecords(Table.Fields);
            if (changeRecordsWindow.ShowDialog() == true)
            {
                var changes = changeRecordsWindow.Changes.ToList();
                var fiters = changeRecordsWindow.Filters.ToList();
                // TODO: Call rpc
            }
        }

        private void ClearTableButton_Click(object sender, RoutedEventArgs e)
        {
            // TODO: Call rpc
        }
    }
}
