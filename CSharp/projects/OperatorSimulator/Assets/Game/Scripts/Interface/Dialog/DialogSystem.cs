using UnityEngine;
using System.Collections.Generic;
using Newtonsoft.Json;
using Dialogues.Dialog;
using Common.Utilities;

namespace Dialogues.System
{
    /// <summary>Организует чтение диалогов из .json-файла в класс <see cref="global::JSONDialog"/>
    /// и получение нужного набора по ключу и запуск диалога.</summary>
    public static class DialogSystem
    {
        #region PUBLIC
        public static void StartDialog(string key, string path)
        {
            if (!ReadDialogJSON(path)) return;
            FullDialog.StartDialog(key, MinHeight, MaxHeight, AdditionalHeight);
        }
        public static Dictionary<string, List<string>> GetDialog(string key)
        {
            if (JsonDialog.text.TryGetValue(key, out var dialog)) return dialog;
            Utilities.DisplayWarning("Key is empty");
            JsonDialog.text.TryGetValue(DefaultKey, out var dialogNull);
            return dialogNull;
        }
        #endregion

        #region CORE
        public static void Init(DialogFullWidth fullDialogPrefab, RectTransform placeForDialogues, string defaultKey, string defaultPath, float minHeight, float maxHeight, float additionalHeight)
        {
            DefaultKey = defaultKey;
            PlaceForDialogues = placeForDialogues;
            DefaultPath = defaultPath;
            MinHeight = minHeight;
            MaxHeight = maxHeight;
            AdditionalHeight = additionalHeight;

            JsonDialog = new();
            GameObject obj = GameObject.Instantiate(fullDialogPrefab.gameObject, PlaceForDialogues);    
            FullDialog = obj.GetComponent<DialogFullWidth>();
            FullDialog.gameObject.SetActive(false);
        }
        #endregion

        #region handlers
        private static bool ReadDialogJSON(string path)
        {
            TextAsset json = Resources.Load(path) as TextAsset;
            JsonDialog = JsonConvert.DeserializeObject<JSONDialog>(json.text);

            if (JsonDialog.text.Count > 0) return true;

            Utilities.DisplayWarning("Path is empty");
            json = Resources.Load(DefaultPath) as TextAsset;
            JsonDialog = JsonConvert.DeserializeObject<JSONDialog>(json.text);
            return false;
        }
        #endregion

        #region variables
        private static JSONDialog JsonDialog { get; set; }
        private static DialogFullWidth FullDialog { get; set; }
        private static RectTransform PlaceForDialogues { get; set; }
        private static string DefaultKey { get; set; }
        private static string DefaultPath { get; set; }
        private static float MinHeight { get; set; }
        private static float MaxHeight { get; set; }
        private static float AdditionalHeight { get; set; }
        #endregion
    }
}