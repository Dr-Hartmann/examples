from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable, Literal

import yaml
from albumentations import Compose
from .core import TorchConfig


@dataclass
class YamlConfig(TorchConfig):
    def _parse_cfg_file(self, path: Path) -> dict[str, Any]:
        with open(path, "r", encoding="utf-8") as f:
            return yaml.safe_load(f) or {}


@dataclass
class InferenceYamlConfig(YamlConfig):
    video_save: bool = field(default=True)
    video_fps: int = field(default=20)
    video_name: str = field(default="VID")
    video_extention: str = field(default="mkv")
    video_codec: str = field(default="FFV1")
    video_height: int | None = field(default=None)

    window_show: bool = field(default=True)
    window_name: str = field(default="Inference")

    model_confidence: float = field(default=0.6)
    model_labels: list[str] = field(default_factory=lambda: ["person"])

    display_color: tuple[int, int, int] = field(default=(0, 255, 0))

    def _set_special_condition(self, data: dict[str, Any]) -> TorchConfig:
        return self


@dataclass
class TrainYamlConfig(YamlConfig):
    name_archives: str = field(default="archives")
    name_images: str = field(default="images")
    name_labels: str = field(default="labels")
    name_train: str = field(default="train")
    name_valid: str = field(default="val")
    name_test: str = field(default="test")

    annotations_name: str = field(default="annotations")
    annotations_suffix: str = field(default=".xml")

    model_param: dict[str, int] = field(default_factory=lambda: {"nothing": 0})
    model_output_name: str = field(default="betterModel.pt")

    train_epochs: int = field(default=10)
    train_image_size: int = field(default=640)
    train_batch: int = field(default=2)
    train_workers: int = field(default=1)
    train_skip_condition: (
        Callable[[dict[str, Literal["False", "True"]]], bool] | None
    ) = field(default=None)

    transforms: Compose | None = field(
        default=None, metadata={TorchConfig._EXCLUDE_KEY: True}
    )
    datayaml: str = field(default="train_data.yaml")

    ratio_train: float = field(default=0.7)
    ratio_valid: float = field(default=0.2)
    ratio_test: float = field(default=0.1)

    def _set_special_condition(self, data: dict[str, Any]) -> TorchConfig:
        skip_condition_str = data.get("train_skip_condition")
        if skip_condition_str:
            self.train_skip_condition = eval(skip_condition_str)
        return self
