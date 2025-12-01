using UnityEngine;
//TODO: переписать на оптимизированный ECS
public class Solver : MonoBehaviour
{
    [SerializeField] private PipeSegment[] _pipeSegments; // Все сегменты труб в системе

    public void SolveFlow()
    {
        foreach (var pipe in _pipeSegments)
        {
            pipe.Update();
        }
    }
}
