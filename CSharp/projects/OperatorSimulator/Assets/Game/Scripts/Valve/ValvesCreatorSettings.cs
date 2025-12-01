using UnityEngine;

[CreateAssetMenu(fileName = "ValvesCreatorSettings", menuName = "Scriptable Objects/new ValvesCreatorSettings")]
public class ValvesCreatorSettings : ScriptableObject
{
    [SerializeField] private CreatorValve _prefabCreator;
    [SerializeField] private float _createRadius = 5f;
    [SerializeField] private Transform _valveCenter;

    public CreatorValve PrefabCreator
    {
        get => _prefabCreator;
    }
    public float CreateRadius
    {
        get => _createRadius;
    }
    public Transform ValveCenter
    {
        get => _valveCenter;
    }
}