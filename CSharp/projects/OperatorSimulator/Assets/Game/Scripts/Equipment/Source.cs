using UnityEngine;

public class Source : MonoBehaviour
{
    [SerializeField] private float _flowRate; // Постоянный поток источника

    public float GetFlowRate()
    {
        return _flowRate;
    }
}
