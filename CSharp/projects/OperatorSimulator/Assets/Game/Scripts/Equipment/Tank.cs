using UnityEngine;

public class Tank : MonoBehaviour
{
    [SerializeField] private float _capacity; // Вместимость бака

    private float _currentVolume;

    public void Fill(float volume)
    {
        _currentVolume = Mathf.Clamp(_currentVolume + volume, 0, _capacity);
    }

    public float GetCurrentVolume()
    {
        return _currentVolume;
    }
}
