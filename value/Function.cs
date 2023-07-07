using System.Linq;

namespace Hatchet 
{
    struct Argument 
    {
        public string Name;
        // TODO Add type
        public bool IsOptional;
        public Expression? DefaultValue;

        public Argument(string name)
        {
            Name = name;
            IsOptional = false;
            DefaultValue = null;
        }
    }

    abstract class Function : Value
    {
        string Name;
        public List<Argument> Arguments;
        
        public Function(string name, List<Argument> arguments)
        {
            Name = name;
            Arguments = arguments;
        }

        ValueType Value.GetType()
        {
            return ValueType.FUNCTION;
        }

        public string GetName() => Name;
        public abstract Value Call(Context context);

        public int MinArgs()
        {
            return Arguments.Count((arg) => !arg.IsOptional);
        }

        public int MaxArgs()
        {
            return Arguments.Count();
        }
    }
}