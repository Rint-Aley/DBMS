namespace db_client
{
    public class ChangeOption
    {
        public Field field { get; private set; }
        public object value { get; private set; }
        public ChangeOption(Field field, object value)
        {
            this.field = field;
            this.value = value;
        }
    }
}
