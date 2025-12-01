using UnityEngine;

[CreateAssetMenu(fileName = "GUISettings", menuName = "Scriptable Objects/new GUISettings")]
public class UISettings : ScriptableObject
{
    [SerializeField] private string _uiLocalizationPath = "Localization";
    [SerializeField] private string _uiSeparator = "===";
    [SerializeField] private string _uiEndLine = "</END>";

    public string UiPath
    {
        get => _uiLocalizationPath;
    }
    public string UiSeparator
    {
        get => _uiSeparator;
    }
    public string UiEndLine
    {
        get => _uiEndLine;
    }
}