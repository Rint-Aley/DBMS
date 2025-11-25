namespace db_client
{
    public class Field
    {
        public string Name { get; private set; }
        public FieldType type { get; private set; }
        public bool IsPrimaryKey { get; private set; }
        public bool IsIndexed { get; }
        public Field(string name, FieldType type, bool isIndexed = false, bool isPrimaryKey = false)
        {
            Name = name;
            this.type = type;
            IsPrimaryKey = isPrimaryKey;
            IsIndexed = isIndexed;
        }
    }
}
