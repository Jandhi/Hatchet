namespace Hatchet {
    class Read : Expression
    {
        public String Name;

        public Read(String name)
        {
            Name = name;
        }

        public Value Evaluate(Context context)
        {
            return context.Read(Name);
        }
    }
}