namespace Hatchet {
    class FunctionCall : Expression
    {
        public Function Function;
        public List<Expression> Arguments;

        public FunctionCall(Function function, List<Expression> arguments)
        {
            Function = function;
            Arguments = arguments;

            if(Arguments.Count() < Function.MinArgs())
            {
                throw new HatchetError($"Function {Function.GetName()} takes at least {Function.MinArgs()} arguments. Given: {Arguments.Count()}");
            }
        }

        public Value Evaluate(Context context)
        {
            var newContext = context.Derive(Function.GetName());

            for(var i = 0; i < Arguments.Count(); i++)
            {
                var value = Arguments[i].Evaluate(context);
                var argName = Function.Arguments[i].Name;
                newContext.Namespaces.Last().Values.Add(argName, value);
            }

            return Function.Call(newContext);
        }
    }
}