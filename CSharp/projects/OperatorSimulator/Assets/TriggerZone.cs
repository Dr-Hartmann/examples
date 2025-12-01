using UnityEngine;
using Dialogues.System;

public class TriggerZone : MonoBehaviour
{
    [SerializeField] private TriggerDialog _dialog;

    private void OnTriggerEnter2D(Collider2D collision)
    {
        if (collision.tag == "Player")
        {
            DialogSystem.StartDialog(_dialog.DialogKeyEnter, _dialog.DialogPathEnter);
        }
    }
    private void OnTriggerExit2D(Collider2D collision)
    {
        if (collision.tag == "Player")
        {
            DialogSystem.StartDialog(_dialog.DialogKeyExit, _dialog.DialogPathExit);
        }
    }
}