using UnityEngine;

[CreateAssetMenu(fileName = "ValveOneOneSettings", menuName = "Scriptable Objects/new ValveOneOneSettings")]
public class ValveOneOneSettings : ScriptableObject
{
    [Header("Settings")]
    [SerializeField] private ValveOneOne _prefabValve;
    [SerializeField] private ValveStates _startValveState = ValveStates.Close;
    [SerializeField] private float _openingSpeed = 1f;
    [SerializeField] private float _closingSpeed = .8f;
    [SerializeField] private float _multiplierSoonToEndState = 5f;

    [Header("Sprites")]
    [SerializeField] private Sprite _spriteNeutral;
    [SerializeField] private Sprite _spriteClose;
    [SerializeField] private Sprite _spriteAverage;
    [SerializeField] private Sprite _spriteOpen;

    public ValveOneOne PrefabValve
    {
        get => _prefabValve;
    }
    public ValveStates StartValveState
    {
        get => _startValveState;
    }
    public float OpeningSpeed
    {
        get => _openingSpeed;
    }
    public float ClosingSpeed
    {
        get => _closingSpeed;
    }
    public float MultiplierSoonToEndState
    {
        get => _multiplierSoonToEndState;
    }
    public Sprite SpriteNeutral
    {
        get => _spriteNeutral;
    }
    public Sprite SpriteClose
    {
        get => _spriteClose;
    }
    public Sprite SpriteAverage
    {
        get => _spriteAverage;
    }
    public Sprite SpriteOpen
    {
        get => _spriteOpen;
    }
}