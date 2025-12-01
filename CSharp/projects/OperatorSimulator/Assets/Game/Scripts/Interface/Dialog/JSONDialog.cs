using System;
using System.Collections.Generic;

[Serializable]
public class JSONDialog
{
    public Dictionary<string, Dictionary<string, List<string>>> text { get; set; }
}