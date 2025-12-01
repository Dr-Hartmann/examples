using UnityEngine;
using UnityEngine.UI;
using UserInterface;

[RequireComponent(typeof(Button))]
public class LocalizationButton : MonoBehaviour
{
    private Button _thisButton;

    private void Awake()
    {
        _thisButton = this.gameObject.GetComponent<Button>();
    }
    private void OnEnable()
    {
        _thisButton.onClick.AddListener(OnClick);
    }
    private void OnDisable()
    {
        _thisButton.onClick.RemoveAllListeners();
    }
    private void OnDestroy()
    {
        OnDisable();
    }
    private void OnClick()
    {
        UISystem.UpdateLanguageUI(LocalizationModes.SWITCH_NEXT);
    }
}