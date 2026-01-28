import argparse

# import mlflow
# from scripts.mlruns import mlflow_init
from src.models import YOLOTrainModel


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-c", "--config", help="Path to 'train_config.yaml' file.")
    args = parser.parse_args()
    if not args.config:
        raise FileNotFoundError(
            "Не указан путь до конфигурационного файла. Используюйте оператор '-c path/to/train_config.yaml'."
        )

    # experiment_tags = {
    #     "project_name": "diplomCHSU",
    #     "team": "Apatit",
    #     "mlflow.note.content": "Тест тренировки модели",
    # }

    # mlflow_init("GrigApatit1", "http://localhost:5000")
    # model = YOLOTrainModel(train_cfg_path=args.tconfig, inf_yaml_cfg_path=args.iconfig)
    model = YOLOTrainModel(train_cfg_path=args.config)
    # with mlflow.start_run(tags=experiment_tags) as run:
    #     mlflow.log_params(model.train_cfg.to_dict())
    #     model.train("traincfg")

    model.train()


if __name__ == "__main__":
    main()
