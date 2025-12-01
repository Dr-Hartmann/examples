using UnityEngine;

[CreateAssetMenu(fileName = "TriggerDialog", menuName = "Dialogues/new TriggerDialog")]
public class TriggerDialog : DialogSettings
{
    [SerializeField] private string _dialogPathEnter;
    [SerializeField] private string _dialogKeyEnter;

    [SerializeField] private string _dialogPathExit;
    [SerializeField] private string _dialogKeyExit;

    public string DialogPathEnter
    {
        get => _dialogPathEnter;

    }
    public string DialogKeyEnter
    {
        get => _dialogKeyEnter;
    }
    public string DialogPathExit
    {
        get => _dialogPathExit;

    }
    public string DialogKeyExit
    {
        get => _dialogKeyExit;
    }
}