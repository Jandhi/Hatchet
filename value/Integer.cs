namespace Hatchet
{
    class Integer : Value
    {
        public int Value;

        ValueType Value.GetType()
        {
            return ValueType.INT;
        }

        public Integer(int value)
        {
            Value = value;
        }
    }
}