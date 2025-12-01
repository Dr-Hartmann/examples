using UnityEngine;
using Simulation;

public class PanelStop : MonoBehaviour
{
    private void Awake()
    {
        SimulationSystem.Played += SetActive;
    }
    private void OnDestroy()
    {
        SimulationSystem.Played -= SetActive;
    }
    private void SetActive(bool _isPlayed)
    {
        this.gameObject.SetActive(!_isPlayed); 
    }
}