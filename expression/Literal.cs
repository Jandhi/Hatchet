namespace Hatchet {
    class Literal : Expression
    {
        public Value Value;

        public Literal(Value value)
        {
            Value = value;
        }

        public Value Evaluate(Context context)
        {
            return Value;
        }
    }
}