using UnityEngine;

[CreateAssetMenu(fileName = "SubstanceData", menuName = "Scriptable Objects/SubstanceData")]
public class SubstanceData : ScriptableObject
{
    [SerializeField] private string _name; // Название вещества
    [SerializeField] private float _density; // Плотность вещества
    [SerializeField] private float _viscosity; // Вязкость вещества

    public string Name => _name;
    public float Density => _density;
    public float Viscosity => _viscosity;
}
