namespace Hatchet
{
    class Namespace 
    {
        public string Name;
        public Dictionary<string, Value> Values;

        public Namespace(string name)
        {
            Name = name;
            Values = new Dictionary<string, Value>();
        }
    }

    class Context
    {
        public List<Namespace> Namespaces;

        Context(List<Namespace> namespaces)
        {
            Namespaces = namespaces;
        }

        public static Context Empty()
        {
            return new Context(new List<Namespace>{new Namespace("root")});
        }

        public Context Derive(string namespaceName)
        {
            var newList = new List<Namespace>();
            newList.AddRange(Namespaces);
            Namespaces.Add(new Namespace(namespaceName));
            return new Context(newList);
        }

        public Value Read(string name)
        {
            for(var i = Namespaces.Count() - 1; i >= 0; i--)
            {
                if(Namespaces[i].Values.ContainsKey(name))
                {
                    return Namespaces[i].Values[name];
                }
            }

            throw new HatchetError($"Reading {name} failed: variable not found.");
        }
    }
}