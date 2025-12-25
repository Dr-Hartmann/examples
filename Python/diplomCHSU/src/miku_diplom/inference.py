import argparse

from colorama import Fore, init
from models.YOLOModel import YOLOModel

if __name__ == "__main__":
    init(autoreset=True)
    parser = argparse.ArgumentParser()
    parser.add_argument("-c", "--config", help="Path to 'config.yaml' file.")
    args = parser.parse_args()
    if not args.config:
        raise FileNotFoundError(Fore.RED + "Не указан путь до конфигурационного файла. Используюйте оператор '-c path/to/inference_config.yaml'.")
    
    model = YOLOModel(inf_yaml_cfg_path=args.config)
    model.inference()
