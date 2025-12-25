from pathlib import Path
from typing import Any
import cv2
from torch.nn import Module
from mlflow import set_tracking_uri, set_experiment, log_params, log_metrics, start_run
from mlflow.tracking import MlflowClient
from sklearn.metrics import accuracy_score, r2_score, recall_score

from scripts.models.core.InferenceModel import InferenceModel

def mlflow_init(experiment_name: str, tracking_uri: str):
    """Инициализация клиента ведения экспериментов (требуется предварительно ```mlflow ui```))"""
    set_tracking_uri(tracking_uri)
    client = MlflowClient(tracking_uri=tracking_uri)
    experiment = client.get_experiment_by_name(experiment_name)
    if experiment is None: 
        experiment_id = client.create_experiment(name=experiment_name)
        set_experiment(experiment_id=experiment_id)
    else: 
        set_experiment(experiment_id=experiment.experiment_id)

def start_mlrun(run_name, experiment_tags: dict[str, Any], params: dict[str, Any], model: InferenceModel):
    
        # y_test = cv2.imread(str(Path("G:/docker-work/test/images/frame_10487.png")))
        # if y_test is None:
        #     print("ОШИБКА: Не удалось загрузить изображение. Проверьте путь.")
        # predictions = model.get_model_pred(y_test)

        # y_test_flat = y_test.flatten()
        # print("Test_flat")
        # print(y_test_flat)
        # predictions_flat = predictions.boxes
        # print("Pred boxes")
        # print(predictions_flat)
        # print("Pred shape")
        # print(predictions_flat.shape)

        # metrics = {
        #     "accuracy_score": accuracy_score(y_test_flat, predictions_flat),
        #     "r2_score": r2_score(y_test_flat, predictions_flat)
        #     # "recall_score": recall_score(y_test, predictions),
        # }
        # log_metrics(metrics)
        print("doooooooooooooone")
