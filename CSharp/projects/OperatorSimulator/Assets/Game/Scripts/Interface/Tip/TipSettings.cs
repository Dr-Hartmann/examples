using UnityEngine;

[CreateAssetMenu(fileName = "TipSettings", menuName = "Scriptable Objects/new TipSettings")]
public class TipSettings : ScriptableObject
{
    [SerializeField] private Tip _tipPrefab;
    [SerializeField] private int _tipsMaxNumber = 20;

    public Tip TipPrefab
    {
        get => _tipPrefab;
    }
    public int TipsMaxNumber
    {
        get => _tipsMaxNumber;
    }
}