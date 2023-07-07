namespace Hatchet
{
    enum ValueType
    {
        INT,
        STRING,
        BOOL,
        FUNCTION,
        OBJECT,
        POINTER,
        NONE
    }

    interface Value 
    {
        ValueType GetType();
    }

    class None : Value
    {
        ValueType Value.GetType()
        {
            return ValueType.NONE;
        }
    }
}