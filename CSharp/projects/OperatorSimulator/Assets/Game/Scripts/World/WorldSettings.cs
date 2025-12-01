using UnityEngine;

[CreateAssetMenu(fileName = "WorldSettings", menuName = "Scriptable Objects/new WorldSettings")]
public class WorldSettings : ScriptableObject
{
    [SerializeField] private GameObject _worldPrefab;

    public GameObject PrefabWorld
    {
        get => _worldPrefab;
    }
}