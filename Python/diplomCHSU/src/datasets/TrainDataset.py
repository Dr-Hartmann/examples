from pathlib import Path

import torch
from albumentations import Compose
from PIL.Image import open
from torch.utils.data import Dataset


class TrainDataset(Dataset):
    def __init__(
        self,
        imgs_path: Path,
        samples: list[dict[str, object]],
        transforms: Compose | None = None,
    ):
        self.img_path = imgs_path
        self.samples = samples
        self.transforms = transforms

    def __len__(self):
        return len(self.samples)

    def __getitem__(self, idx):
        sample = self.samples[idx]
        img = open(self.img_path / sample["name"]).convert("RGB")
        if self.transforms:
            img = self.transforms(image=img)

        boxes = torch.tensor(sample["boxes"], dtype=torch.float32)
        target = {
            "boxes": boxes,
            "labels": torch.tensor(sample["labels"], dtype=torch.float32),
            "image_id": torch.tensor([idx]),
            "area": (boxes[:, 2] - boxes[:, 0]) * (boxes[:, 3] - boxes[:, 1]),
            "iscrowd": torch.zeros((len(boxes),), dtype=torch.int64),
        }

        return img, target
