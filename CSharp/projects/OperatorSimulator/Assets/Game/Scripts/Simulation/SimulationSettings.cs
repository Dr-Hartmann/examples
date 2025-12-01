using Simulation;
using UnityEngine;

[CreateAssetMenu(fileName = "SimulationSettings", menuName = "Scriptable Objects/new SimulationSettings")]
public class SimulationSettings : ScriptableObject
{
    [SerializeField] private SimulationSystem _prefabSimulationSystem;
    [SerializeField] private SimulationStates _state = SimulationStates.Pause;
    [SerializeField] private float _maxSpeed = 10f;
    [SerializeField] private float _minSpeed = -4f;
    [SerializeField] private float _speedStep = .25f;

    public SimulationSystem PrefabSimulationSystem
    {
        get => _prefabSimulationSystem;
    }
    public SimulationStates SimulationStartState
    {
        get => _state;
    }
    public float SimulationMaxSpeed
    {
        get => _maxSpeed;
    }
    public float SimulationMinSpeed
    {
        get => _minSpeed;
    }
    public float SimulationSpeedStep
    {
        get => _speedStep;
    }
}