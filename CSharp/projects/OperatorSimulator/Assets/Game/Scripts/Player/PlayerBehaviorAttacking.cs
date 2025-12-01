using System;

namespace PlayerBehaviors
{
    public class PlayerBehaviorAttacking : PlayerBehavior
    {
        public PlayerBehaviorAttacking(Action enter, Action exit, Action<float> update)
            : base(enter, exit, update) { }
    }
}