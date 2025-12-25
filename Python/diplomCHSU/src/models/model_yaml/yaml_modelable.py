import copy
import yaml
import shutil
import xml.etree.ElementTree as ET

from tqdm import tqdm
from pathlib import Path
from colorama import Fore
from zipfile import ZipFile
from typing import Any, Iterable, Iterator

from core import TrainableModel, InferenceModel
from myutils import get_file_paths, move_files, IMG_TYPES


class YamlTrainableModel(TrainableModel, InferenceModel):
    def __init__(self, train_cfg_path: str | None, inf_cfg_path: str | None):
        if train_cfg_path is not None:
            TrainableModel.__init__(self, train_cfg_path)
        if inf_cfg_path is not None:
            InferenceModel.__init__(self, inf_cfg_path)

    def _write_new_tree(self, root: ET.Element, path: Path) -> None:
        ET.ElementTree(root).write(path, encoding="utf-8", xml_declaration=True)

    def _get_ann_root(self, file: Path) -> ET.Element | None:
        root = None
        try:
            tree = ET.parse(file)
            root = tree.getroot()
        except ET.ParseError as e:
            print(Fore.RED + f"Ошибка парсинга XML-файла {file}: {e}")
        return root

    def _findall(self, root: ET.Element, obj: str) -> Iterable[ET.Element]:
        return root.findall(f".//{obj}")

    def _find(self, root: ET.Element, obj: str) -> ET.Element | None:
        return root.find(f".//{obj}")

    def _get_attr_value(self, root: ET.Element, name: str) -> str:
        return root.attrib[name]

    def _get_attribs(self, root: ET.Element) -> dict[str, str]:
        return root.attrib

    def _get_text(self, root: ET.Element) -> str | None:
        return root.text

    # TODO
    def _get_unique_attributes(self, root: ET.Element) -> Iterable[str]:
        return {
            n.text
            for n in root.findall(".//label//attribute/name")
            if n is not None and n.text
        }

    def _create_new_root(self) -> ET.Element:
        return ET.Element("annotations")

    def _append(self, root: ET.Element, obj: ET.Element) -> ET.Element:
        root.append(copy.deepcopy(obj))
        return root

    def _write_cfg_file(
        self, config_data: dict[str, Any], target_name_cfg: str = "data.yaml"
    ) -> None:
        with open(target_name_cfg, "w") as f:
            yaml.dump(config_data, f, sort_keys=False)

    def _get_arch_paths(self) -> Iterator[Path]:
        return get_file_paths([self.train_cfg.path_input], ["*.zip"])

    def _unpack_move_join_imgs_and_ann(self, root, archives, tp, ann) -> ET.Element:
        for arch in tqdm(archives, desc="Распаковка zip-архивов..."):
            with ZipFile(arch, "r") as zip:
                zip.extractall(tp)
                root = self._append(root, self._get_ann_root(tp / ann))
                imgs = get_file_paths([tp], IMG_TYPES)
                move_files([imgs], [self.train_cfg.path_output])
                shutil.rmtree(tp)
        return root
