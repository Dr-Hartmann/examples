using Common.Utilities;
using Dialogues.System;
using GameController.PlayerSpace;
using GameController.UISpace;
using Simulation;
using System.Collections;
using Tips;
using UnityEngine;
using UserInterface;

public class EntryPointGameplay : MonoBehaviour
{
    [Header("Simulation")]
    [SerializeField] private SimulationSettings _simulationSettings;

    [Header("Common")]
    [SerializeField] private CommonSettings _commonSettings;

    [Header("UI")]
    [SerializeField] private UISettings _uiSettings;

    [Header("Tip")]
    [SerializeField] private TipSettings _tipSettings;

    [Header("Dialog")]
    [SerializeField] private DialogSettings _dialogSettings;

    [Header("World")]
    [SerializeField] private WorldSettings _worldSettings;

    [Header("Player")]
    [SerializeField] private PlayerSettings _playerSettings;

    [Header("Valves")]
    [SerializeField] private ValvesCreatorSettings _valvesCreatorSettings;
    [SerializeField] private ValveOneOneSettings _valveOneOneSettings;

    [Header("Interface")]
    [SerializeField] private InterfaceSettings _interfaceSettings;


    private IEnumerator Start()
    {
        if (!CanStart()) yield break;
        Debug.Log("Start");

        SetFrameRate();

        InitUI();

        yield return Wait();

        InitSimulationSystem();
        InitUISystem();
        InitTipSystem();
        InitDialogSystem();

        yield return Wait();

        InitPlayer();
        InitWorld();

        yield return Wait();

        InitValveCreator();

        Debug.Log("End");
        SimulationSystem.Begin();
        UISystem.Begin();
    }
    private IEnumerator Wait()
    {
        float time = .5f;
        while (time >= 0)
        {
            time -= Time.deltaTime;
            yield return null;
        }
    }
    private bool CanStart()
    {
        return _simulationSettings
            && _commonSettings
            && _uiSettings
            && _tipSettings
            && _dialogSettings
            && _worldSettings
            && _playerSettings
            && _valvesCreatorSettings
            && _valveOneOneSettings
            && _interfaceSettings;
    }
    private void SetFrameRate()
    {
        QualitySettings.vSyncCount = _commonSettings.GameVSyncCount;
        Application.targetFrameRate = _commonSettings.GameTargetFrameRate;
    }

    #region INIT
    private void InitSimulationSystem()
    {
        SimulationSystem simSys = Instantiate(_simulationSettings.PrefabSimulationSystem);
        simSys.Init(
            _simulationSettings.SimulationStartState,
            _simulationSettings.SimulationMaxSpeed,
            _simulationSettings.SimulationMinSpeed,
            _simulationSettings.SimulationSpeedStep,
            simSys
        );
    }
    private void InitUISystem()
    {
        UISystem.Init(_uiSettings.UiPath, _uiSettings.UiSeparator, _uiSettings.UiEndLine);
    }
    private void InitTipSystem()
    {
        var tipsPlace = GameObject.FindGameObjectWithTag("TipsPlace");
        if (!tipsPlace)
        {
            Utilities.DisplayWarning("TipsPlace not found");
            return;
        }
        TipSystem.Init(_tipSettings.TipPrefab, _tipSettings.TipsMaxNumber, tipsPlace.GetComponent<RectTransform>());
    }
    private void InitDialogSystem()
    {
        var placeForDialogues = GameObject.FindGameObjectWithTag("UICanvas");
        if (!placeForDialogues)
        {
            Utilities.DisplayWarning("UICanvas not found");
            return;
        }
        DialogSystem.Init(_dialogSettings.PrefabDialogFull, placeForDialogues.GetComponent<RectTransform>(), _dialogSettings.DefaultKey, _dialogSettings.DefaultPath, _dialogSettings.MinHeight, _dialogSettings.MaxHeight, _dialogSettings.AdditionalHeight);
    }
    private void InitWorld()
    {
        Instantiate(_worldSettings.PrefabWorld);
    }
    private void InitPlayer()
    {
        PlayerController playerController = new();
        playerController.Init(_playerSettings.PrefabPlayer, _playerSettings.BaseSpeed);
    }
    private void InitUI()
    {
        if (GameObject.FindGameObjectWithTag("UICanvas"))
        {
            Utilities.DisplayWarning("There is UICanvas already");
            return;
        }
        UIController uiController = new();
        uiController.Init(_interfaceSettings.UIPrefab);
    }
    private void InitValveCreator()
    {
        var valveCenter = Instantiate(_valvesCreatorSettings.ValveCenter);
        var obj = Instantiate(_valvesCreatorSettings.PrefabCreator);
        obj.Init(_valveOneOneSettings, valveCenter, _valvesCreatorSettings.CreateRadius);
    }
    #endregion
}