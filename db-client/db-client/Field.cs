namespace db_client
{
    public class Field
    {
        public string Name { get; private set; }
        public FieldType Type { get; private set; }
        public bool IsPrimaryKey { get; set; }
        public bool IsIndexed { get; set; }
        public Field(string name, FieldType type, bool isIndexed = false, bool isPrimaryKey = false)
        {
            Name = name;
            Type = type;
            IsPrimaryKey = isPrimaryKey;
            IsIndexed = isIndexed;
        }
    }
}
