namespace Hatchet
{
    class HatchetError : Exception
    {
        string Description;

        public HatchetError(string description) : base()
        {
            Description = description;
        }
    }
}