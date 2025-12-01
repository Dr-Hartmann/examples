using UnityEngine;

[CreateAssetMenu(fileName = "InterfaceSettings", menuName = "Scriptable Objects/new InterfaceSettings")]
public class InterfaceSettings : ScriptableObject
{
    [SerializeField] private GameObject _interfacePrefab;

    public GameObject UIPrefab
    {
        get => _interfacePrefab;
    }
}