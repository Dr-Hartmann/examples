using TMPro;
using UnityEngine;
using Tips;

[RequireComponent(typeof(RectTransform))]
public class Tip : MonoBehaviour
{
    private float _creationTime;
    private TextMeshProUGUI _text;
    private string _message = "__message__";

    public float Lifetime { get; set; } = 20f;
    public int Index { get; set; } = 0;
    public string Message
    {
        get => _message;
        set
        {
            _message = value;
            _text.text = _message;
        }
    }

    private void Awake()
    {
        _text = GetComponentInChildren<TextMeshProUGUI>();
    }
    private void OnEnable()
    {
        _creationTime = Time.time;
        Message = _message;
    }
    private void OnDisable()
    {
        TipSystem.InvokeTipDeleted();
        Destroy(this.gameObject);
    }
    private void Update()
    {
        if (Time.time - _creationTime > Lifetime)
        {
            gameObject.SetActive(false);
        }
    }
}