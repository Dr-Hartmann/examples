import shutil
from pathlib import Path
from typing import Iterable, Iterator

IMG_TYPES = [".jpg", ".jpeg", ".png", ".bmp", ".tif", ".tiff", ".webp"]


def need_unpack(paths: Iterable[Path]) -> bool:
    """
    Проверяет нужно ли распаковывать архивы.

    :param paths: любое перечисление абсолютных путей метоположения изображений
    :type paths: Iterable[Path]
    :return: нужно ли распаковывать архивы
    :rtype: bool
    """
    if get_file_paths(paths, IMG_TYPES):
        print(f"В {[p.name + ', ' for p in paths]} уже есть изображения. Продолжить?")
        if input("[Y | N (default)]: ").strip().lower() == "y":
            return True
    return False


def get_file_paths(paths: Iterable[Path], extensions: Iterable[str]) -> Iterator[Path]:
    """
    Возвращает файлы целевых расширений.

    :param paths: любое перечисление абсолютных путей метоположения файлов
    :type paths: Iterable[Path]
    :param extensions: целевые расщирения
    :type extensions: Iterable[str]
    :return: итератор абсолютных путей к файлам целевых расширений
    :rtype: Iterator[Path]
    """
    ext_patterns = [f"*{ext}" for ext in extensions]

    def path_generator():
        for root in paths:
            for pattern in ext_patterns:
                yield from root.rglob(pattern)

    return path_generator()


def clear_directorys(paths: Iterable[Path]):
    """
    Удаляет всё содержимое каталога, но сохраняет сам каталог.

    :param paths: любое перечисление абсолютных путей метоположения
    :type paths: Iterable[Path]
    """
    for p in paths:
        if p.exists():
            shutil.rmtree(p)
        p.mkdir(parents=True, exist_ok=True)


def move_files(files: Iterable[Iterable[Path]], paths: Iterable[Path]) -> None:
    """
    Перемещает файлы (не работает при наличии файлов с теми же именами).

    :param files: любое перечисление абсолютных путей исходного метоположения
    :type files: Iterable[Iterable[Path]]
    :param paths: любое перечисление абсолютных путей целевого метоположения
    :type paths: Iterable[Path]
    """
    for file, path in zip(files, paths):
        print(f"Перемещение в: {path}...")
        path.mkdir(parents=True, exist_ok=True)
        for f in file:
            shutil.move(f, path / f.name)


def move_and_change_files(
    files: Iterable[Iterable[Path]], paths: Iterable[Path]
) -> None:
    """
    Перемещает файлы с заменой.

    :param files: любое перечисление абсолютных путей исходного метоположения
    :type files: Iterable[Iterable[Path]]
    :param paths: любое перечисление абсолютных путей целевого метоположения
    :type paths: Iterable[Path]
    """
    for file, path in zip(files, paths):
        print(f"Перемещение с заменой в: {path}...")
        path.mkdir(parents=True, exist_ok=True)
        for f in file:
            f.replace(path / f.name)
