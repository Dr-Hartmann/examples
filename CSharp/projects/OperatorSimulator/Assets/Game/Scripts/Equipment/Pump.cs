using UnityEngine;

public class Pump : MonoBehaviour
{
    [SerializeField] private AnimationCurve _pressureCurve; // Кривая зависимости давления от времени
    [SerializeField] private float _maxFlowRate; // Максимальный поток

    private float _currentPressure;
    private float _elapsedTime;

    private void Start()
    {
        _elapsedTime = 0f;
    }

    private void Update()
    {
        _elapsedTime += Time.deltaTime;

        // Обновление давления на основе AnimationCurve
        _currentPressure = _pressureCurve.Evaluate(_elapsedTime);
    }

    public float GetFlowRate()
    {
        // Расчет потока на основе давления
        return Mathf.Clamp(_currentPressure / _maxFlowRate, 0f, _maxFlowRate);
    }
}
