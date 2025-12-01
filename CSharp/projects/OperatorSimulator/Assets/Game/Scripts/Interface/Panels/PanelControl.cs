using TMPro;
using UnityEngine;
using UnityEngine.UI;
using Simulation;

[RequireComponent(typeof(Image))]
public class PanelControl : MonoBehaviour
{
    [Header("Buttons")]
    [SerializeField] private Button _playPause;
    [SerializeField] private Button _less;
    [SerializeField] private Button _more;

    [Header("Sprites")]
    [SerializeField] private Sprite _pauseSprite;
    [SerializeField] private Sprite _playSprite;

    [Header("Speed")]
    [SerializeField] private TextMeshProUGUI _speedValue;

    #region ON_CLICK
    private void OnClickPlayPause()
    {
        SimulationSystem.ReversePlayPause();
    }
    private void OnClickLess()
    {
        SimulationSystem.SetSpeed(SimulationSpeedStates.Decrease);
    }
    private void OnClickMore()
    {
        SimulationSystem.SetSpeed(SimulationSpeedStates.Increase);
    }
    #endregion

    #region CORE
    private void Awake()
    {
        _playPauseImage = _playPause.GetComponent<Image>();
        SubscribeAll();
    }
    private void OnDestroy()
    {
        UnsubscribeAll();
    }
    #endregion

    private void ChangeSpeedText(float speedMultiplier, bool isPlayed)
    {
        _speedValue.SetText(speedMultiplier.ToString());
    }
    private void SubscribeAll()
    {
        SimulationSystem.Played += ChangeSprite;
        SimulationSystem.SpeedChanged += ChangeSpeedText;
        _less.onClick.AddListener(OnClickLess);
        _more.onClick.AddListener(OnClickMore);
        _playPause.onClick.AddListener(OnClickPlayPause);
    }
    private void UnsubscribeAll()
    {
        SimulationSystem.Played -= ChangeSprite;
        SimulationSystem.SpeedChanged -= ChangeSpeedText;
        _less.onClick.RemoveListener(OnClickLess);
        _more.onClick.RemoveListener(OnClickMore);
        _playPause.onClick.RemoveListener(OnClickPlayPause);
    }
    private void ChangeSprite(bool _isPlayed)
    {
        if (_isPlayed) _playPauseImage.sprite = _pauseSprite;
        else _playPauseImage.sprite = _playSprite;
    }

    private Image _playPauseImage;
}