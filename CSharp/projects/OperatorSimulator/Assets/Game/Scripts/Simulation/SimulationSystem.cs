using UnityEngine;
using UnityEngine.SceneManagement;
using System;

// TODO - сделать переменнуя частоту кадров, файл 'settings.ini'?
// TODO - сделать раздел "настройки"
// TODO - time scale нужен?

namespace Simulation
{
    /// <summary>Контролирует состояние симуляции и сообщает её скорость.</summary>
    public class SimulationSystem : MonoBehaviour
    {
        #region PUBLIC
        public static Action<float> TickPassed { get; set; }
        public static Action<bool> Played { get; set; }
        public static Action<float, bool> SpeedChanged { get; set; }
        public static void SetState(SimulationStates state)
        {
            switch (state)
            {
                case SimulationStates.Play:
                    //Time.timeScale = 1;
                    Played?.Invoke(true);
                    break;

                case SimulationStates.Pause:
                    //Time.timeScale = 0;
                    Played?.Invoke(false);
                    break;

                case SimulationStates.Stop:

                    break;

                default:
                    Common.Utilities.Utilities.DisplayWarning("Unknown state");
                    SimulationState = SimulationStates.Pause;
                    Played?.Invoke(false);
                    return;
            }

            SimulationState = state;

            SpeedChanged?.Invoke(CurrentSpeed, IsPlayed);
        }
        /// <summary>speed - абсолютный плоказатель скорости или множитель при инкременте и декременте.</summary>
        public static void SetSpeed(SimulationSpeedStates state, float speed = 1)
        {
            switch (state)
            {
                case SimulationSpeedStates.Set:
                    CurrentSpeed = speed;
                    break;

                case SimulationSpeedStates.Increase:
                    CurrentSpeed += SpeedStep * speed;
                    break;

                case SimulationSpeedStates.Decrease:
                    CurrentSpeed -= SpeedStep * speed;
                    break;

                default:
                    Common.Utilities.Utilities.DisplayWarning("Unknown state");
                    break;
            }

            if (CurrentSpeed > MaxSpeed) CurrentSpeed = MaxSpeed;
            else if (CurrentSpeed < MinSpeed) CurrentSpeed = MinSpeed;

            SpeedChanged?.Invoke(CurrentSpeed, IsPlayed);
        }
        public static void ReversePlayPause()
        {
            if (IsPlayed) SetState(SimulationStates.Pause);
            else SetState(SimulationStates.Play);
        }
        #endregion

        #region CORE
        public void Init(SimulationStates startState, float maxSpeed, float minSpeed, float speedStep, SimulationSystem instance)
        {
            if (Instance != null) { Destroy(this.gameObject); return; }
            DontDestroyOnLoad(this.gameObject);
            SimulationState = startState;
            MaxSpeed = maxSpeed;
            MinSpeed = minSpeed;
            SpeedStep = speedStep;
            Instance = instance;
        }
        public static void Begin()
        {
            SetState(SimulationState);
        }
        private void FixedUpdate()
        {
            if (IsPlayed)
            {
                TickPassed?.Invoke(Time.deltaTime * CurrentSpeed);
            }
            else if (IsPaused)
            {

            }
            else if (IsStopped)
            {
                SceneManager.LoadScene(SceneManager.GetActiveScene().buildIndex);
            }
        }
        #endregion

        #region handlers
        private static float CurrentSpeed { get; set; } = 1f;
        private static bool IsPlayed => SimulationState == SimulationStates.Play;
        private static bool IsPaused => SimulationState == SimulationStates.Pause;
        private static bool IsStopped => SimulationState == SimulationStates.Stop;
        #endregion

        #region variables
        private static SimulationSystem Instance { get; set; }
        private static SimulationStates SimulationState { get; set; }
        private static float MaxSpeed { get; set; }
        private static float MinSpeed { get; set; }
        private static float SpeedStep { get; set; }
        #endregion
    }
}