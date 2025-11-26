namespace db_client
{
    public class FilterOption
    {
        public Field field { get; private set; }
        public string value { get; private set; }
        public FilterOption(Field field, string value)
        {
            this.field = field;
            this.value = value;
        }
    }
}
