using UnityEngine;

public class PipeSegment : MonoBehaviour
{
    [SerializeField] private Pump _sourcePump;
    [SerializeField] private Tank _destinationTank;

    public void Update()
    {
        float flowRate = _sourcePump.GetFlowRate();
        _destinationTank.Fill(flowRate * Time.deltaTime);
    }
}
