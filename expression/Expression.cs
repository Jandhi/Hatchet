namespace Hatchet
{
    interface Expression
    {
        Value Evaluate(Context context);
    }
}