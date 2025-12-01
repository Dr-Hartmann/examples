using UnityEngine;
using Simulation;

namespace GameController.UISpace
{
    public class UIController : Controller
    {
        #region CORE
        public void Init(GameObject uiInterface)
        {
            GameObject.Instantiate(uiInterface);

            _uiAct = InputActions.UI;
            Enable();
        }
        #endregion

        #region SIMULATION
        private void PlayPause()
        {
            SimulationSystem.ReversePlayPause();
        }
        private void SpeedMore()
        {
            SimulationSystem.SetSpeed(SimulationSpeedStates.Increase);
        }
        private void SpeedLess()
        {
            SimulationSystem.SetSpeed(SimulationSpeedStates.Decrease);
        }
        private void Restart()
        {
            SimulationSystem.SetState(SimulationStates.Stop);
        }
        #endregion

        #region handlers
        public void Enable()
        {
            SubscribeUI();
            _uiAct.Enable();
        }
        public void Disable()
        {
            UnsubscribeUI();
            _uiAct.Disable();
        }
        #endregion

        #region subscriptions
        private void SubscribeUI()
        {
            _uiAct.PlayPause.performed += context => PlayPause();
            _uiAct.SpeedMore.performed += context => SpeedMore();
            _uiAct.SpeedLess.performed += context => SpeedLess();
            _uiAct.Restart.performed += context => Restart();
        }
        private void UnsubscribeUI()
        {
            _uiAct.PlayPause.performed -= context => PlayPause();
            _uiAct.SpeedMore.performed -= context => SpeedMore();
            _uiAct.SpeedLess.performed -= context => SpeedLess();
            _uiAct.Restart.performed -= context => Restart();
        }
        #endregion

        #region variables
        private InputSystem_Actions.UIActions _uiAct;
        #endregion
    }
}