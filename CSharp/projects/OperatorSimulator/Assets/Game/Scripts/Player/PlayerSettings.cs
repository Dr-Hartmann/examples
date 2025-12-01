using PlayerSpace;
using UnityEngine;

[CreateAssetMenu(fileName = "PlayerSettings", menuName = "Scriptable Objects/new PlayerSettings")]
public class PlayerSettings : ScriptableObject
{
    [SerializeField] private Player _playerPrefab;
    [SerializeField] private float _baseSpeed = 8f;

    public float BaseSpeed
    {
        get => _baseSpeed;
    }
    public Player PrefabPlayer
    {
        get => _playerPrefab;
    }
}