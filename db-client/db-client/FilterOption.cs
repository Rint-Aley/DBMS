namespace db_client
{
    public class FilterOption
    {
        public Field field { get; private set; }
        public object value { get; private set; }
        public FilterOption(Field field, object value)
        {
            this.field = field;
            this.value = value;
        }
    }
}
