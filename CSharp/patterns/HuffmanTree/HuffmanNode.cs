namespace HuffmanTree;

//Node* trees[256], * symbols[256];
//public int Size { get; }

internal class HuffmanNode : IComparable<HuffmanNode>
{
    public char Char { get; set; }
    public int Freq { get; set; }
    public HuffmanNode? Left { get; set; }
    public HuffmanNode? Right { get; set; }
    //public HuffmanNode? Parent { get; set; }
    public int CompareTo(HuffmanNode? other)
    {
        if (other is null) return 1;
        return Freq.CompareTo(other.Freq);
    }
};
