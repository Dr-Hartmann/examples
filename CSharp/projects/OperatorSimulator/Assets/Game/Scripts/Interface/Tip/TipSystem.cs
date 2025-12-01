using UnityEngine;
using System;

namespace Tips
{
    public static class TipSystem
    {
        //[SerializeField] private Animator _animator;

        #region PUBLIC
        public static void InvokeTipDeleted()
        {
            TipDeleted?.Invoke();
        }
        public static void InvokeTipDisplayed(string message, float lifetime)
        {
            TipDisplayed?.Invoke(message, lifetime);
        }
        #endregion

        #region CORE
        public static void Init(Tip tipPrefab, int tipsMaxNumber, RectTransform tipsPlace)
        {
            _tipsPrefab = tipPrefab;
            _tipsMaxNumber = tipsMaxNumber;
            _tipsPlace = tipsPlace;
            _tipsCounter = 0;

            TipDisplayed += CreateTip;
            TipDeleted += DeleteTip;
        }
        #endregion

        #region handlers
        private static void CreateTip(string message, float lifetime)
        {
            if (_tipsCounter <= _tipsMaxNumber)
            {
                GameObject obj = UnityEngine.Object.Instantiate(_tipsPrefab.gameObject, _tipsPlace);
                Tip tip = obj.GetComponent<Tip>();
                tip.Index = ++_tipsCounter;
                tip.Lifetime = lifetime;
                tip.Message = message;
            }
        }
        private static void DeleteTip()
        {
            --_tipsCounter;
        }
        #endregion

        #region variables
        private static Tip _tipsPrefab { get; set; }
        private static int _tipsMaxNumber { get; set; }
        private static RectTransform _tipsPlace { get; set; }
        private static int _tipsCounter { get; set; }
        private static Action<string, float> TipDisplayed { get; set; }
        private static Action TipDeleted { get; set; }
        #endregion
        
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void InitializeOnLoadMethod()
        {
            TipDisplayed -= CreateTip;
            TipDeleted -= DeleteTip;
        }
    }
}