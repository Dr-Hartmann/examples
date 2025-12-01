using Package.Opened;

namespace Package.Hidden;

internal class ProductRepository : IProductRepository
{
    public void DoRepoWork()
    {
        Console.WriteLine("ProductRepository work is completed.");
    }
}
