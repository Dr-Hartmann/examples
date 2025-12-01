using Microsoft.Extensions.DependencyInjection;
using Package.Hidden;

namespace Package.Opened;

public static class DependencyInjection
{
    public static IServiceCollection MakeItBeautiful(this IServiceCollection services)
    {
        services.AddScoped<IProductRepository, ProductRepository>();
        services.AddScoped<IExampleService, ExampleService>();
        return services;
    }
}
