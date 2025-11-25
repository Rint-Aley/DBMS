namespace db_client
{
    public class Table
    {
        public String Name { get; private set; }
        public List<Field> Fields { get; private set; }

        public Table(string name, List<Field> fields)
        {
            Name = name;
            Fields = fields;
        }
    }
}
