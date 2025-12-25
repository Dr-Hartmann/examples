import argparse

import mlflow
from colorama import Fore, init

# from scripts.mlruns import mlflow_init
from models.YOLOModel import YOLOModel

if __name__ == "__main__":
    init(autoreset=True)
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--tconfig", help="Path to 'train.yaml' file.")
    parser.add_argument("-i", "--iconfig", help="Path to 'inference.yaml' file.")
    args = parser.parse_args()
    if not args.tconfig:
        raise FileNotFoundError(
            Fore.RED
            + "Не указан путь до конфигурационного файла. Используюйте оператор '-c path/to/train_config.yaml'."
        )

    experiment_tags = {
        "project_name": "diplomCHSU",
        "team": "Apatit",
        "mlflow.note.content": "Тест тренировки модели",
    }

    # mlflow_init("GrigApatit1", "http://localhost:5000")
    model = YOLOModel(train_cfg_path=args.tconfig, inf_yaml_cfg_path=args.iconfig)
    with mlflow.start_run(tags=experiment_tags) as run:
        mlflow.log_params(model.train_cfg.to_dict())
        model.train()
