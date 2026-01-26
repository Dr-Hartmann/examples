import cv2
from .core import InferenceModel
from ultralytics.engine.results import Results
from ultralytics.models import YOLO

from src.configs import InferenceYamlConfig


class YOLOInferenceModel(InferenceModel):
    def __init__(self, inf_cfg_path: str | None = None):
        if inf_cfg_path is not None:
            InferenceModel.__init__(self, inf_cfg_path)

    def get_model_pred(self, frame: cv2.typing.MatLike) -> Results:
        return YOLO(self.inf_cfg.model_name).to(self.inf_cfg.model_device)(
            frame,
            verbose=False,
            conf=self.inf_cfg.model_confidence,
            classes=[
                cls_id
                for cls_id, name in YOLO(self.inf_cfg.model_name).names.items()
                if name in self.inf_cfg.model_labels
            ],
        )[0]

    def _get_inference_cfg(self, inf_cfg_path: str) -> InferenceYamlConfig:
        cfg = InferenceYamlConfig.create(inf_cfg_path)
        if cfg.model_type.lower() != "yolo":
            raise ReferenceError(f"Тип модели '{cfg.model_type}' не поддерживается.")
        cfg.model = YOLO(cfg.model_name)
        return cfg
