using System;

namespace PlayerBehaviors
{
    public abstract class PlayerBehavior
    {
        private Action _enter;
        private Action _exit;
        private Action<float> _update;

        public PlayerBehavior(Action enter, Action exit, Action<float> update)
        {
            _enter = enter;
            _exit = exit;
            _update = update;
        }
        public virtual void Enter()
        {
            _enter();
        }
        public virtual void Exit()
        {
            _exit();
        }
        public virtual void Update(float tick)
        {
            _update(tick);
        }
    }
}