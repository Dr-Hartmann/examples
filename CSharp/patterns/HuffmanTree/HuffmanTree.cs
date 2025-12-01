namespace HuffmanTree;

public class HuffmanTree
{
    private HuffmanNode Root { get; set; } = new();

    public void Build(string source)
    {
        Dictionary<char, int> frequencies = [];
        List<HuffmanNode> nodes = [];

        foreach (var c in source)
        {
            frequencies[c] = frequencies.GetValueOrDefault(c, 0) + 1;
        }

        foreach (var i in frequencies)
        {
            nodes.Add(new() { Char = i.Key, Freq = i.Value });
        }

        while (nodes.Count > 1)
        {
            var orderedNodes = nodes.OrderBy(n => n.Freq).ToList();

            if (orderedNodes.Count >= 2)
            {
                var left = orderedNodes[0];
                var right = orderedNodes[1];

                var parent = new HuffmanNode()
                {
                    Freq = left.Freq + right.Freq,
                    Left = left,
                    Right = right,
                };

                //left.Parent = parent;
                //right.Parent = parent;

                nodes.Add(parent);
                nodes.Remove(left);
                nodes.Remove(right);
            }
        }

        Root = nodes.FirstOrDefault() ?? Root;
    }

    private static void GenerateCodes(HuffmanNode node, string code, Dictionary<char, string> codes)
    {
        if (node is null) return;

        if (node.Left is null && node.Right is null)
        {
            codes.Add(node.Char, code);
            return;
        }

        if (node.Left is not null)
            GenerateCodes(node.Left, code + '0', codes);

        if (node.Right is not null)
            GenerateCodes(node.Right, code + '1', codes);
    }

    public string Encode(string source)
    {
        Dictionary<char, string> codes = [];
        GenerateCodes(Root, string.Empty, codes);
        return string.Join(string.Empty, source.Select(c => codes[c]));
    }

    public string Decode(string encoded)
    {
        string decodedString = string.Empty;
        var current = Root;

        foreach (char bit in encoded)
        {
            if (bit is '0')
                current = current.Left!;
            else if (bit is '1')
                current = current.Right!;

            if (current.Left is null && current.Right is null)
            {
                decodedString += current.Char;
                current = Root;
            }
        }

        return decodedString;
    }
}
