using UnityEngine;
using PlayerSpace;

namespace GameController.PlayerSpace
{
    public class PlayerController : Controller
    {
        #region CORE
        public void Init(Player player, float baseSpeed)
        {
            _player = GameObject.Instantiate(player, new Vector3(Random.value % 100f, Random.value % 100f), new(0, 0, 0, 0));
            _player.Init(baseSpeed);

            _playerAct = InputActions.Player;
            Enable();
        }
        #endregion

        private void StartDialog()
        {

        }

        #region MOVE
        private void MoveStarted()
        {
            _move = true;
            ChangeBehavior();
        }
        private void MovePerformed(Vector2 vector)
        {
            _player.MoveVector = vector;
        }
        private void MoveCanceled()
        {
            _move = false;
            ChangeBehavior();
        }
        #endregion

        #region ATTACK
        private void AttackStarted()
        {
            _attack = true;
            ChangeBehavior();
        }
        private void AttackCanceled()
        {
            _attack = false;
            ChangeBehavior();
        }
        #endregion

        #region handlers
        private void ChangeBehavior()
        {
            if (_attack) _player.SetBehaviorAttacking();
            else if (_move) _player.SetBehaviorMoving();
            else _player.SetBehaviorByDefault();
        }
        public void Enable()
        {
            SubscribePlayer();
            _playerAct.Enable();
        }
        public void Disable()
        {
            UnsubscribePlayer();
            _playerAct.Disable();
        }
        #endregion

        #region subscriptions
        private void SubscribePlayer()
        {
            //_playerAct.Interact.performed += context => StartDialog();

            _playerAct.Move.started += context => MoveStarted();
            _playerAct.Move.performed += context => MovePerformed(context.ReadValue<Vector2>());
            _playerAct.Move.canceled += context => MoveCanceled();

            _playerAct.Attack.started += context => AttackStarted();
            _playerAct.Attack.canceled += context => AttackCanceled();
        }
        private void UnsubscribePlayer()
        {
            //_playerAct.Interact.performed -= context => StartDialog();

            _playerAct.Move.started -= context => MoveStarted();
            _playerAct.Move.performed -= context => MovePerformed(context.ReadValue<Vector2>());
            _playerAct.Move.canceled -= context => MoveCanceled();

            _playerAct.Attack.started -= context => AttackStarted();
            _playerAct.Attack.canceled -= context => AttackCanceled();
        }
        #endregion

        #region variables
        private InputSystem_Actions.PlayerActions _playerAct;
        private Player _player;
        private bool _move;
        private bool _attack;
        #endregion
    }
}