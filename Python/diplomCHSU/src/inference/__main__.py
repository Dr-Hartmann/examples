import argparse

from src.models import YOLOInferenceModel

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-c", "--config", help="Path to 'inference.yaml' file.")
    args = parser.parse_args()
    if not args.config:
        raise FileNotFoundError(
            "Не указан путь до конфигурационного файла. Используюйте оператор '-c path/to/inference_config.yaml'."
        )

    model = YOLOInferenceModel(inf_cfg_path=args.config)
    model.inference()
