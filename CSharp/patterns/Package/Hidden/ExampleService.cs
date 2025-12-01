using Package.Opened;

namespace Package.Hidden;

internal class ExampleService(IProductRepository service) : IExampleService
{
    public void DoServiceWork()
    {
        Console.WriteLine("ExampleService is working");
        service.DoRepoWork();
    }
}
