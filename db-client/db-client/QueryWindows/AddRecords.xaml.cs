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
    /// Interaction logic for AddRecords.xaml
    /// </summary>
    public partial class AddRecords : Window
    {

        private readonly List<Field> _fields;
        private readonly List<TextBox> _textBoxes = new();

        public List<string>? Result { get; private set; }

        public AddRecords(List<Field> fields)
        {
            InitializeComponent();
            _fields = fields;
            BuildForm();
        }

        private void BuildForm()
        {
            foreach (var field in _fields)
            {
                var panel = new StackPanel
                {
                    Orientation = Orientation.Vertical,
                    Margin = new Thickness(0, 5, 0, 5)
                };

                var label = new TextBlock
                {
                    Text = field.Name + (field.IsPrimaryKey ? " (PK)" : ""),
                    FontWeight = FontWeights.Bold
                };

                var tb = new TextBox
                {
                    Margin = new Thickness(0, 3, 0, 0),
                    Tag = field
                };

                panel.Children.Add(label);
                panel.Children.Add(tb);

                FieldsPanel.Children.Add(panel);
                _textBoxes.Add(tb);
            }
        }

        private void AddButton_Click(object sender, RoutedEventArgs e)
        {
            Result = new List<string>(_fields.Count);

            foreach (var tb in _textBoxes)
            {
                Result.Add(tb.Text);
            }

            DialogResult = true;
            Close();
        }

        private void CancelButton_Click(object sender, RoutedEventArgs e)
        {
            Close();
        }
    }
}
