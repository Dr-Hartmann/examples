using UnityEngine;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.IO;
using System;
using Common.Utilities;
using Common.Variables;

// TODO - убрать првязку к StreamingAssets, Addressables?

namespace UserInterface
{
    /// <summary>
    /// Система локализации интерфейса.
    /// </summary>
    public static class UISystem
    {
        #region PUBLIC
        public static Action LanguageChanged { get; set; }
        public static void UpdateLanguageUI(LocalizationModes mode)
        {
            if (IsEmptyFiles()) return;
            SetCurrentLanguage(mode);
            SetPlayerPrefs();
            ReadFileText();

            if (IsEmptyText()) return;
            LanguageChanged?.Invoke();
        }
        public static string GetText(string key)
        {
            if (IsEmptyText()) return key;
            if (Text.TryGetValue(key, out string value)) return value;
            Utilities.DisplayWarning($"Invalid key - {key}");
            return key;
        }
        #endregion

        #region CORE
        public static void Init(string uiPath, string uiSeparator, string uiEndLine)
        {
            Path = uiPath;
            Separator = uiSeparator;
            EndLine = uiEndLine;
            GetUIFiles();
        }
        public static void Begin()
        {
            UpdateLanguageUI(LocalizationModes.SET_DEFAULT);
        }
        #endregion

        #region handlers
        private static void SetCurrentLanguage(LocalizationModes mode, string languageCode = Variables.DEFAULT_LANGUAGE)
        {
            switch (mode)
            {
                case LocalizationModes.SWITCH_NEXT:
                    List<string> list = Files.Values.ToList();
                    int currentIndex = list.IndexOf(Files.GetValueOrDefault(Variables.CurrentLanguage));
                    int nextIndex = (currentIndex + 1) % Files.Count;
                    Variables.CurrentLanguage = Files.ElementAt(nextIndex).Key;
                    break;

                case LocalizationModes.SET:
                    Variables.CurrentLanguage = languageCode;
                    break;

                case LocalizationModes.SET_DEFAULT:
                    string currentCulture = CultureInfo.InstalledUICulture.TwoLetterISOLanguageName;
                    Variables.CurrentLanguage = PlayerPrefs.GetString("language", currentCulture);
                    if (!Files.TryGetValue(Variables.CurrentLanguage, out var value))
                    {
                        Utilities.DisplayWarning("Language not found");
                        Variables.CurrentLanguage = Variables.DEFAULT_LANGUAGE;
                    }
                    break;

                default:
                    Utilities.DisplayWarning($"Invalid mode - {mode}");
                    break;
            }
        }
        private static void ReadFileText()
        {
            Text = new Dictionary<string, string>();
            string[] lines = File
                .ReadAllText(Files[Variables.CurrentLanguage])
                .Split(EndLine);
            foreach (string item in lines)
            {
                if (!string.IsNullOrWhiteSpace(item))
                {
                    string[] keyValue = item.Split(Separator);
                    if (keyValue.Length == 2)
                    {
                        Text[keyValue[0].Trim()] = keyValue[1].Trim();
                        continue;
                    }
                    Utilities.DisplayWarning($"Invalid line - {item}");
                }
            }
        }
        private static void GetUIFiles()
        {
            Files = new Dictionary<string, string>();
            string localizationPath = Application.streamingAssetsPath + "/" + Path;
            string[] filesPath = Directory.GetFiles(localizationPath, "*.txt");
            foreach (string path in filesPath)
            {
                Files.Add(System.IO.Path.GetFileNameWithoutExtension(path), path);
            }
        }
        private static void SetPlayerPrefs()
        {
            PlayerPrefs.SetString("language", Variables.CurrentLanguage);
            PlayerPrefs.Save();
        }
        private static bool IsEmptyFiles()
        {
            if (Files == null || Files.Count <= 0)
            {
                Utilities.DisplayWarning($"Localization files do not exist");
                return true;
            }
            return false;
        }
        private static bool IsEmptyText()
        {
            if (Text == null || Text.Count <= 0)
            {
                Utilities.DisplayWarning($"There is no text");
                return true;
            }
            return false;
        }
        #endregion

        #region variables
        private static string Path { get; set; }
        private static string Separator { get; set; }
        private static string EndLine { get; set; }
        private static Dictionary<string, string> Files { get; set; }
        private static Dictionary<string, string> Text { get; set; }
        #endregion
    }
}