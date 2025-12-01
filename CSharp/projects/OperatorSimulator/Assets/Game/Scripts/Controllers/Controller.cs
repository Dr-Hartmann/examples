namespace GameController
{
    public abstract class Controller
    {
        protected static InputSystem_Actions InputActions
        {
            get
            {
                if (_inputActions == null)
                {
                    _inputActions = new();
                }
                return _inputActions;
            }
        }
        private static InputSystem_Actions _inputActions { get; set; }
    }
}
