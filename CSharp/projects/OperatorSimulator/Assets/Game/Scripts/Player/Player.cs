using UnityEngine;
using System.Collections.Generic;
using System;
using Simulation;
using PlayerBehaviors;

namespace PlayerSpace
{
    //[RequireComponent(typeof(Rigidbody2D))]
    public class Player : MonoBehaviour
    {
        private const string IS_IDLE = "IsIdle";
        private const string IS_MOVE = "IsMove";
        private const string IS_ATTACK = "IsAttack";

        #region ANIMATOR
        public void SetCanCancel()
        {
            _canCancel = true;
        }
        public void SetCannotCancel()
        {
            _canCancel = false;
        }
        #endregion

        #region PUBLIC
        public Vector2 MoveVector { get; set; }
        public void SetBehaviorIdle()
        {
            SetBehavior(GetBehavior<PlayerBehaviorIdle>());
        }
        public void SetBehaviorMoving()
        {
            SetBehavior(GetBehavior<PlayerBehaviorMoving>());
        }
        public void SetBehaviorAttacking()
        {
            SetBehavior(GetBehavior<PlayerBehaviorAttacking>());
        }
        public void SetBehaviorByDefault()
        {
            SetBehaviorIdle();
        }
        //public bool IsIdle()
        //{
        //    return _animator.GetBool(_isIdle);
        //}
        //public bool IsMoving()
        //{
        //    return _animator.GetBool(_isMove);
        //}
        //public bool IsAttacking()
        //{
        //    return _animator.GetBool(_isAttack);
        //}
        #endregion

        #region CORE
        public void Init(float baseSpeed)
        {
            _baseSpeed = baseSpeed;
            _spriteRenderer = GetComponent<SpriteRenderer>();
            _animator = GetComponent<Animator>();
            _rigidbody = GetComponent<Rigidbody2D>();

            _behaviorMap = new();
            _behaviorMap[typeof(PlayerBehaviorIdle)] = new PlayerBehaviorIdle(EnterIdle, ExitIdle, UpdateIdle);
            _behaviorMap[typeof(PlayerBehaviorMoving)] = new PlayerBehaviorMoving(EnterMoving, ExitMoving, UpdateMoving);
            _behaviorMap[typeof(PlayerBehaviorAttacking)] = new PlayerBehaviorAttacking(EnterAttacking, ExitAttacking, UpdateAttacking);
            SetBehaviorByDefault();

            SimulationSystem.SpeedChanged += UpdateAnimatorSpeed;
            SimulationSystem.TickPassed += BehaviorAction;
            SimulationSystem.Played += StopMoving;
        }
        private void BehaviorAction(float tick)
        {
            if (_behaviorCurrent != null)
            {
                _behaviorCurrent.Update(tick);
            }
        }
        private void OnDestroy()
        {
            SimulationSystem.SpeedChanged -= UpdateAnimatorSpeed;
            SimulationSystem.TickPassed -= BehaviorAction;
            SimulationSystem.Played -= StopMoving;
        }
        #endregion

        #region IDLE
        private void EnterIdle()
        {
            _animator.SetBool(_isIdle, true);
        }
        private void UpdateIdle(float speedMultiplier)
        {

        }
        private void ExitIdle()
        {
            _animator.SetBool(_isIdle, false);
        }
        #endregion

        #region ATTACK
        private void EnterAttacking()
        {
            _animator.SetBool(_isAttack, true);
        }
        private void UpdateAttacking(float speedMultiplier)
        {

        }
        private void ExitAttacking()
        {
            _animator.SetBool(_isAttack, false);
        }
        #endregion

        #region MOVE
        private void EnterMoving()
        {
            _animator.SetBool(_isMove, true);
        }
        private void UpdateMoving(float tick)
        {
            _rigidbody.linearVelocity = MoveVector * _baseSpeed * tick;
            if (_rigidbody.linearVelocity.x > 0) _spriteRenderer.flipX = false;
            else _spriteRenderer.flipX = true;
        }
        private void ExitMoving()
        {
            _rigidbody.linearVelocity = new Vector2(0, 0);
            _animator.SetBool(_isMove, false);
        }
        private void StopMoving(bool isPlayed)
        {
            if (!isPlayed) _rigidbody.linearVelocity = MoveVector * 0f;
        }
        #endregion

        #region handlers
        private /*IEnumerator*/ void SetBehavior(PlayerBehavior newBehavior)
        {
            if (_behaviorCurrent != null)
            {
                _behaviorCurrent.Exit();
            }

            //while (!_canCancel)
            //{
            //    yield return null;
            //}

            _behaviorCurrent = newBehavior;
            _behaviorCurrent.Enter();
        }
        private PlayerBehavior GetBehavior<T>() where T : PlayerBehavior
        {
            return _behaviorMap[typeof(T)];
        }
        private void UpdateAnimatorSpeed(float speedMultiplier, bool isPlayed)
        {
            _animator.speed = Mathf.Abs(speedMultiplier * (isPlayed ? 1f : 0f));
        }
        #endregion

        #region variables
        private SpriteRenderer _spriteRenderer;
        private Animator _animator;
        private Rigidbody2D _rigidbody;
        private PlayerBehavior _behaviorCurrent;
        private Dictionary<Type, PlayerBehavior> _behaviorMap;

        private float _baseSpeed;
        private bool _canCancel = true;
        private int _isIdle = Animator.StringToHash(IS_IDLE);
        private int _isMove = Animator.StringToHash(IS_MOVE);
        private int _isAttack = Animator.StringToHash(IS_ATTACK);
        #endregion
    }
}