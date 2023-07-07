namespace Hatchet 
{
    class UserFunction : Function
    {
        List<Expression> Content;   

        UserFunction(List<Argument> arguments, List<Expression> content) : base("anonymous", arguments)
        {
            Content = content;
        }

        public override Value Call(Context context)
        {
            Value retVal = new None();

            foreach(var expr in Content)
            {
                retVal = expr.Evaluate(context);
            }

            return retVal;
        }
    }
}