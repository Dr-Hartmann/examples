using UnityEngine;
using UnityEngine.UI;
using Tips;

[RequireComponent(typeof(Button))]
public class CreatorValve : MonoBehaviour
{
    #region CORE
    public void Init(ValveOneOneSettings valveSettings, Transform placeForValves, float createRadius)
    {
        _valveSettings = valveSettings;
        _placeForValves = placeForValves;
        _radius = createRadius;
    }
    private void Awake()
    {
        _thisButton = GetComponent<Button>();
    }
    private void OnEnable()
    {
        _thisButton.onClick.AddListener(OnClick);
    }
    private void OnDisable()
    {
        _thisButton.onClick.RemoveListener(OnClick);
    }
    #endregion

    private void OnClick()
    {
        float parentX;
        float parentY;
        float x;
        float y;
        Vector3 pos;

        parentX = _placeForValves.position.x;
        parentY = _placeForValves.position.y;
        x = Random.Range(parentX - _radius, parentX + _radius);
        y = Random.Range(parentY - _radius, parentY + _radius);
        pos = new Vector3(x, y, 1f);
        GameObject obj = Instantiate(_valveSettings.PrefabValve.gameObject, pos, Quaternion.identity, _placeForValves);
        obj.GetComponent<ValveOneOne>()
            .Init(_valveSettings.StartValveState, _valveSettings.OpeningSpeed, _valveSettings.ClosingSpeed,
            _valveSettings.MultiplierSoonToEndState, _valveSettings.SpriteNeutral, _valveSettings.SpriteClose,
            _valveSettings.SpriteAverage, _valveSettings.SpriteOpen);
        TipSystem.InvokeTipDisplayed($"Создан клапан {Time.time.ToString()}", 30f);
    }
    

    private ValveOneOneSettings _valveSettings;
    private Transform _placeForValves;
    private Button _thisButton;
    private float _radius;
}