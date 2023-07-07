namespace Hatchet 
{
    class ExternalFunction : Function
    {
        Func<Context, Value> Func;

        public ExternalFunction(string name, List<Argument> arguments, Func<Context, Value> func) : base(name, arguments) 
        {   
            Func = func;
        }


        public override Value Call(Context context)
        {
            return Func(context);
        }
    }
}