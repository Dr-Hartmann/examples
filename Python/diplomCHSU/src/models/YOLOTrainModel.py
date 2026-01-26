from pathlib import Path
from typing import Iterable

from .core import YamlTrainableModel
from torch.backends import cudnn
from ultralytics import settings
from ultralytics.models import YOLO

from src.configs import TrainYamlConfig

# from torch.utils.data import DataLoader
# from .TrainDataset import TrainDataset


class YOLOTrainModel(YamlTrainableModel):
    def __init__(self, train_cfg_path: str | None = None):
        YamlTrainableModel.__init__(self, train_cfg_path)

    def _get_train_cfg(self, train_cfg_path: str) -> TrainYamlConfig:
        cfg = TrainYamlConfig.create(train_cfg_path)
        if cfg.model_type.lower() != "yolo":
            raise ReferenceError(f"Тип модели '{cfg.model_type}' не поддерживается.")
        if cfg.model_device.lower() != "cpu":
            cudnn.benchmark = True
        cfg.model = YOLO(cfg.model_name)
        return cfg

    def _get_model_bb(
        self, size: tuple[int, int], box: tuple[float, float, float, float]
    ) -> tuple[float, float, float, float]:
        dw = 1.0 / size[0]
        dh = 1.0 / size[1]
        x_center = ((box[0] + box[2]) / 2.0) * dw
        y_center = ((box[1] + box[3]) / 2.0) * dh
        w = (box[2] - box[0]) * dw
        h = (box[3] - box[1]) * dh
        return (x_center, y_center, w, h)

    def _write_new_ann(
        self, model_lines: Iterable[str], path: Path, filename: str
    ) -> None:
        if not model_lines:
            print("Не переданы параметры для записи аннотаций YOLO.")
            return

        path.mkdir(parents=True, exist_ok=True)
        with open(path / (Path(filename).stem + ".txt"), "w") as out_file:
            out_file.writelines(model_lines)

    def _start_train(self) -> None:
        # TODO
        # import os
        # os.environ["MLFLOW_TRACKING_URI"] = "file:./mlruns"
        # settings.update({"mlflow": False})

        YOLO(self.train_cfg.model_name).train(
            data=self.train_cfg.datayaml,
            epochs=self.train_cfg.train_epochs,
            imgsz=self.train_cfg.train_image_size,
            batch=self.train_cfg.train_batch,
            workers=self.train_cfg.train_workers,
            device=0
            if self.train_cfg.model_device.lower() == "cuda"
            else self.train_cfg.model_device.lower(),
            project=self.train_cfg.path_input,
            name=self.train_cfg.model_output_name,
        )

    # def __get_samples(self, root: Any) -> list[dict[str, object]]:
    #     """Получить изображения и их аннотации"""
    #     print(Fore.BLUE + "Получение изображений и их аннотаций...")

    #     samples: list[dict[str, object]] = []
    #     for img in self._findall(root, "image"):
    #         boxes, labels = [], []
    #         for box in self._findall(img, "box"):
    #             bb = self.__get_bb(box)
    #             boxes.append(bb)
    #             lbl = self._get_concat_label(box)
    #             labels.append(self.label_map[lbl])

    #         samples.append(
    #             {
    #                 "name": self._get_attr_value(img, "name"),
    #                 "width": int(self._get_attr_value(img, "width")),
    #                 "height": int(self._get_attr_value(img, "height")),
    #                 "boxes": boxes,
    #                 "labels": labels,
    #             }
    #         )

    #     print(Fore.MAGENTA + "Изображения и аннотации получены.")
    #     return samples

    # def get_train_dataloader(self, cfg: TrainConfig) -> DataLoader:
    #     root = self._get_ann_root(self.__p_train / cfg.name_labels / cfg.model_type / cfg.annotations_name)
    #     return DataLoader(
    #         dataset=TrainDataset(self.p_imgs / cfg.name_train, self.__get_samples(root), cfg.transforms),
    #         batch_size=cfg.train_batch,
    #         shuffle=True,
    #         num_workers = cfg.train_workers,
    #         collate_fn=lambda batch: tuple(zip(*batch)),
    #     )

    # def get_val_dataloader(self, cfg: TrainConfig) -> DataLoader:
    #     root = self._get_ann_root(self.__p_val / cfg.name_valid / cfg.model_type / cfg.annotations_name)
    #     return DataLoader(
    #         dataset=TrainDataset(self.p_imgs / cfg.name_valid, self.__get_samples(root), cfg.transforms),
    #         batch_size=cfg.train_batch,
    #         shuffle=True,
    #         num_workers = cfg.train_workers,
    #         collate_fn=lambda batch: tuple(zip(*batch)),
    #     )

    # def get_test_dataloader(self, cfg: TrainConfig) -> DataLoader:
    #     root = self._get_ann_root(self.__p_test / cfg.name_train / cfg.model_type / cfg.annotations_name)
    #     return DataLoader(
    #         dataset=TrainDataset(self.p_imgs / cfg.name_test, self.__get_samples(root)),
    #         batch_size=cfg.train_batch,
    #         shuffle=False,
    #         num_workers = cfg.train_workers,
    #         collate_fn=lambda batch: tuple(zip(*batch)),
    #     )
