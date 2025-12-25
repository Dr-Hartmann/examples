from abc import ABC, abstractmethod
from dataclasses import Field, dataclass, field, fields
from pathlib import Path
from typing import Any, Self

from colorama import Fore, init
from torch import cuda
from torch.nn import Module


@dataclass
class TorchConfig(ABC):
    _EXCLUDE_KEY = "exclude_from_output"

    path_input: Path = field(default=Path.home())
    path_output: Path = field(default=Path.home())

    model: Module | None = field(default=None, metadata={_EXCLUDE_KEY: True})
    model_type: str = field(default="yolo")
    model_name: str = field(default="yolo11n.pt")
    model_device: str = field(default="cuda" if cuda.is_available() else "cpu")

    @classmethod
    def create(cls, path: str) -> Self:
        """
        :param cls: параметр абстрактного метода-фабрики
        :param path: путь до файла-конфига
        :type path: str
        :return: экземпляр
        :rtype: Self
        """
        init(autoreset=True)
        cfg = cls()
        data = cfg._parse_cfg_file(Path(path))

        for f in fields(cfg):
            if (key := f.name) in data:
                cfg.__set_value(f, data[key])

        return cfg._set_special_condition(data)

    def to_dict(self) -> dict[str, Any]:
        """
        :return: словарь параметров конфигурации, исключая помеченные поля
        :rtype: dict[str, Any]
        """
        return {
            f.name: getattr(self, f.name)
            for f in fields(self)
            if not f.metadata.get(self._EXCLUDE_KEY)
        }

    def __set_value(self, fil: Field[Any], value: Any) -> None:
        """
        Устанавливает текущие значения параметров или по умолчанию
        :param fil: поле
        :type fil: Field[Any]
        :param value: значение поля
        :type value: Any
        """
        if fil.type is Path and isinstance(value, str):
            setattr(self, fil.name, Path(value))
            return

        if value is None:
            print(
                Fore.YELLOW
                + f"'{fil.name}': передан null. Используется значение по умолчанию: {fil.default}."
            )
            return

        try:
            setattr(self, fil.name, value)
        except Exception as e:
            print(Fore.MAGENTA + f"Ошибка при установке '{fil}': {e}")

    @abstractmethod
    def _parse_cfg_file(self, path: Path) -> dict[str, Any]:
        """
        :param path: абсолютный путь до файла-конфига
        :type path: Path
        :return: словарь параметров из файла-конфига
        :rtype: dict[str, Any]
        """
        pass

    @abstractmethod
    def _set_special_condition(self, data: dict[str, Any]) -> Self:
        """
        Устанавливает специальные условия в конфиге
        :param data: словарь особых условий
        :type data: dict[str, Any]
        :return: экземпляр
        :rtype: Self
        """
        pass
