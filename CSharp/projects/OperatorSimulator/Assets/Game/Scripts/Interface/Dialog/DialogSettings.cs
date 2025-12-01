using Dialogues.Dialog;
using UnityEngine;

[CreateAssetMenu(fileName = "DialogSettings", menuName = "Scriptable Objects/new DialogSettings")]
public class DialogSettings : ScriptableObject
{
    [SerializeField] private DialogFullWidth _dialogFull;
    [SerializeField] private string _defaultPath;
    [SerializeField] private string _defaultKey;

    //[SerializeField] private float _maxWidth = 1700f;
    [SerializeField] private float _minHeight = 200f;
    [SerializeField] private float _maxHeight = 500f;
    [SerializeField] private float _additionalHeight = 100f;

    public DialogFullWidth PrefabDialogFull
    {
        get => _dialogFull;
    }
    public string DefaultPath
    {
        get => _defaultPath;

    }
    public string DefaultKey
    {
        get => _defaultKey;
    }
    public float MinHeight
    {
        get => _minHeight;
    }
    public float MaxHeight
    {
        get => _maxHeight;
    }
    public float AdditionalHeight
    {
        get => _additionalHeight;
    }
}