namespace ConsoleApp1;

using HuffmanTree;
using Microsoft.Extensions.DependencyInjection;
using Package.Opened;

internal class Program
{
    static void Main(string[] args)
    {
        HuffmanTree();
        DependencyInjection();
    }

    static void DependencyInjection()
    {
        // Настройка DI.
        IServiceCollection services = new ServiceCollection();

        // Кнопка "Сделать красиво".
        services.MakeItBeautiful();
        var provider = services.BuildServiceProvider();

        // Конечный пользователь не знает как именно реализованы интерфейсы.
        // Но сама программа всё узнает позже.
        var service = provider.GetRequiredService<IExampleService>();

        // Вызвать метод, который "что-то делает".
        // Конечный пользователь и не догадывается,
        // что внутри есть ещё зависимости.
        service.DoServiceWork();
    }

    static void HuffmanTree()
    {
        string input = "this is an example for huffman encoding";
        HuffmanTree huffmanTree = new();
        huffmanTree.Build(input);

        string encoded = huffmanTree.Encode(input);
        Console.WriteLine($"Оригинальный текст: {input}");
        Console.WriteLine($"Закодированный текст: {encoded}");

        string decoded = huffmanTree.Decode(encoded);
        Console.WriteLine($"Декодированный текст: {decoded}");
    }
}
