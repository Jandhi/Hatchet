using Hatchet;

// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");

var func = new ExternalFunction("add", new List<Argument>{
    new Argument("a"),
    new Argument("b"),
}, (ctx) => {
    var a = (Integer) ctx.Read("a");
    var b = (Integer) ctx.Read("b");

    return new Integer(a.Value + b.Value);
});

var call = new FunctionCall(func, new List<Expression>{
    new Literal(new Integer(1)),
    new Literal(new Integer(20)),
});


var value = (Integer) call.Evaluate(Context.Empty());

Console.WriteLine($"The result is {value.Value}");