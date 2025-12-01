using TMPro;
using UnityEngine;

namespace UserInterface.Text
{
    /// <summary>
    /// Организует подписку и отписку на событие <see cref="UISystem.LanguageChanged"/> и изменяет компонент <see cref="TextMeshProUGUI"/> в зависимости от ключа.
    /// Язык явным образом не указывается. Адаптирует размер родительского контейнера <see cref="RectTransform"/>
    /// </summary>
    [RequireComponent(typeof(TextMeshProUGUI))]
    public class LocalizedText : MonoBehaviour
    {
        [SerializeField] private string _key;
        [SerializeField] private RectTransform _parent;

        /// <summary>Обновляет текст и регулирует ширину и высоту родителя.</summary>
        public void UpdateText()
        {
            if (IsNull()) return;
            string newText = UISystem.GetText(_key);
            _text.SetText(newText);

            if (_parent)
            {
                _parent.sizeDelta = new Vector2(_text.preferredWidth + 50f, _text.preferredHeight + 30f);
            }
        }

        #region CORE
        private void Awake()
        {
            _text = this.GetComponent<TextMeshProUGUI>();
            UISystem.LanguageChanged += UpdateText;
        }
        private void Start()
        {
            UpdateText();
        }
        private void OnDestroy()
        {
            UISystem.LanguageChanged -= UpdateText;
        }
        #endregion

        private bool IsNull()
        {
            if (_text == null || _key == "" || _parent == null)
            {
                Common.Utilities.Utilities.DisplayWarning($"Invalid ui-text object - {_key}");
                return true;
            }
            return false;
        }

        private TextMeshProUGUI _text;
    }
}