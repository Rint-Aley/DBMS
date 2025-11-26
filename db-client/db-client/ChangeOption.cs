namespace db_client
{
    public class ChangeOption
    {
        public Field field { get; private set; }
        public string value { get; private set; }
        public ChangeOption(Field field, string value)
        {
            this.field = field;
            this.value = value;
        }
    }
}
