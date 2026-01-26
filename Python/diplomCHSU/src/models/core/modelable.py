import datetime
import itertools
import random
from abc import ABC, abstractmethod
from hashlib import md5
from pathlib import Path
from pprint import pprint
from typing import Any, Iterable, Iterator

import cv2
from colorama import Fore, init
from natsort import natsorted
from tqdm import tqdm

from src.configs import InferenceYamlConfig, TrainYamlConfig
from src.myutils import IMG_TYPES, clear_directorys, get_file_paths, move_files, need_unpack


class TrainableModel(ABC):
    def __init__(self, train_cfg_path: str) -> None:
        """
        Создать и установить конфиг для тренировки.
        :param self: Description
        :param train_cfg_path: Description
        :type train_cfg_path: str
        """
        init(autoreset=True)
        self.train_cfg = self._get_train_cfg(train_cfg_path)

    def train(self, target_name_cfg: str) -> None:
        """
        Точка входа тренировки.
        """
        print(Fore.GREEN + "Train started...")

        ann = self.train_cfg.annotations_name + self.train_cfg.annotations_suffix

        if need_unpack([self.train_cfg.path_output]):
            clear_directorys([self.train_cfg.path_output])
            self.__unpack_archs_and_join_anns(ann)

        dirs = self.__prepare_dirs()
        self.__pts = {
            self.train_cfg.name_train: dirs[self.train_cfg.name_train],
            self.train_cfg.name_valid: dirs[self.train_cfg.name_valid],
            self.train_cfg.name_test: dirs[self.train_cfg.name_test],
        }

        self.__split_imgs_to_train_val_test()
        self.__split_ann_to_train_val_test(ann)

        combos = self.__get_combos(ann)
        label_map = self.__get_normalized_cost(combos)
        self.__convert_ann_to_model(ann, list(label_map))

        self.__generate_config(list(label_map), target_name_cfg)
        self._start_train()

    def __unpack_archs_and_join_anns(self, ann: str) -> None:
        # TODO - отсеять ненужные сегменты аннотаций
        """
        Распаковывает архивы и объединяет аннотации в один файл.
        :param ann: имя файла аннотаций
        :type ann: str
        """
        print(Fore.BLUE + "Распаковка архивов и объединение аннотаций...")

        tp = self.train_cfg.path_output.parent / ".temp"
        tp.mkdir(parents=True, exist_ok=True)

        if not (archives := self._get_arch_paths()):
            print(
                Fore.YELLOW
                + f"В '{self.train_cfg.path_input}' не найдено zip-архивов, отмена распаковки..."
            )
            return

        root = self._create_new_root()
        root = self._unpack_move_join_imgs_and_ann(root, archives, tp, ann)
        self._write_new_tree(root, self.train_cfg.path_output / ann)

        print(Fore.MAGENTA + "Распаковка и объединение завершены.")

    def __prepare_dirs(self) -> dict[str, Path]:
        """
        Создаёт необходимые для работы директории.
        :return: словарь директорий и путей к ним
        :rtype: dict[str, Path]
        """
        print(Fore.BLUE + "Создание необходимых директорий...")

        dirs: dict[str, Path] = {}
        names = [
            self.train_cfg.name_train,
            self.train_cfg.name_valid,
            self.train_cfg.name_test,
        ]
        for dset in names:
            dirs[dset] = self.train_cfg.path_output / dset
            (dirs[dset] / self.train_cfg.name_images).mkdir(parents=True, exist_ok=True)
            clear_directorys([dirs[dset] / self.train_cfg.name_labels])
            (dirs[dset] / self.train_cfg.name_labels).mkdir(parents=True, exist_ok=True)

        print(Fore.MAGENTA + "Рабочие директории подготовлены.")
        return dirs

    def __split_imgs_to_train_val_test(self) -> None:
        """
        Разбивает изображения по каталогам train, val, test.
        """
        print(
            Fore.BLUE
            + f"Разбиение на каталоги {self.train_cfg.name_train}, {self.train_cfg.name_valid}, {self.train_cfg.name_test}..."
        )

        # TODO - внести случайного выбора промежутка test
        imgs = get_file_paths([self.train_cfg.path_output], IMG_TYPES)
        if not imgs:
            print(
                Fore.YELLOW
                + f"Изображений в {self.train_cfg.path_output.absolute().name} не найдено. Разбиение отменено..."
            )
            return

        imgs = natsorted(imgs, key=lambda p: p.stem)
        size_total = len(imgs)
        print(
            Fore.BLUE + f"Всего изображений (естественно отсортировано): {size_total}"
        )

        multiple = 1.0 / (
            self.train_cfg.ratio_train
            + self.train_cfg.ratio_valid
            + self.train_cfg.ratio_test
        )
        size_train = int(size_total * self.train_cfg.ratio_train * multiple)
        size_val = int(size_total * self.train_cfg.ratio_valid * multiple)

        test = imgs[size_train + size_val :]
        imgs = imgs[: size_train + size_val]
        random.shuffle(imgs)
        train = imgs[:size_train]
        val = imgs[size_train : size_train + size_val]

        len_train = len(train)
        len_val = len(val)
        len_test = len(test)

        assert len_train + len_val + len_test == size_total
        print(Fore.BLUE + f"Train: {len_train}, Val: {len_val}, Test: {len_test}")
        assert {p.name for p in train}.isdisjoint([p.name for p in val])
        assert {p.name for p in train}.isdisjoint([p.name for p in test])
        assert {p.name for p in val}.isdisjoint([p.name for p in test])

        dset_paths = [
            self.__pts[self.train_cfg.name_train] / self.train_cfg.name_images,
            self.__pts[self.train_cfg.name_valid] / self.train_cfg.name_images,
            self.__pts[self.train_cfg.name_test] / self.train_cfg.name_images,
        ]
        move_files([train, val, test], dset_paths)

        print(Fore.MAGENTA + "Разделение датасета завершено.")

    def __split_ann_to_train_val_test(self, ann: str) -> None:
        """
        Разбивает аннотации на train, val, test.
        :param ann: имя файла аннотаций
        :type ann: str
        """
        print(Fore.BLUE + "Создание аннотаций...")

        dset_imgs = {
            self.train_cfg.name_train: {
                f.name
                for f in get_file_paths(
                    [self.__pts[self.train_cfg.name_train]], IMG_TYPES
                )
            },
            self.train_cfg.name_valid: {
                f.name
                for f in get_file_paths(
                    [self.__pts[self.train_cfg.name_valid]], IMG_TYPES
                )
            },
            self.train_cfg.name_test: {
                f.name
                for f in get_file_paths(
                    [self.__pts[self.train_cfg.name_test]], IMG_TYPES
                )
            },
        }
        img_to_dset = {
            name: dset for dset, names in dset_imgs.items() for name in names
        }

        new_roots = {
            self.train_cfg.name_train: self._create_new_root(),
            self.train_cfg.name_valid: self._create_new_root(),
            self.train_cfg.name_test: self._create_new_root(),
        }

        root = self._get_ann_root(self.train_cfg.path_output / ann)
        if (lbls := self._find(root, "labels")) is not None:
            for rt in new_roots:
                new_roots[rt] = self._append(new_roots[rt], lbls)

        for img in self._findall(root, "image"):
            img_name = self._get_attr_value(img, "name")
            if (
                img_name is not None
                and (target_dset := img_to_dset.get(img_name)) is not None
            ):
                new_roots[target_dset] = self._append(new_roots[target_dset], img)

        for set_n, new_r in new_roots.items():
            self._write_new_tree(
                new_r, self.__pts[set_n] / self.train_cfg.name_labels / ann
            )

        print(Fore.MAGENTA + "Создание аннотаций завершено.")

    def __get_combos(self, ann: str) -> Iterable[Iterable[str]]:
        """
        Получить всех бинарные комбинации предметной области.
        """
        root = self._get_ann_root(self.train_cfg.path_output / ann)
        attr = self._get_unique_attributes(root)
        if not attr:
            print(
                Fore.YELLOW
                + f"Комбинации не найдены, используется значение по умолчанию: '{[['nothing']]}'"
            )
            return [["nothing"]]

        all_com: Iterable[Iterable[str]] = []
        for combo_tuple in itertools.product(("False", "True"), repeat=len(list(attr))):
            values = dict(zip(attr, combo_tuple))
            if (
                self.train_cfg.train_skip_condition is None
                or not self.train_cfg.train_skip_condition(values)
            ):
                true_attrs = [k for k, v in values.items() if v == "True"]
                all_com.append(true_attrs if true_attrs else ["nothing"])

        return all_com

    def __get_normalized_cost(
        self, combos: Iterable[Iterable[str]]
    ) -> dict[str, float]:
        """
        Получить словарь комбинаций с нормализованными выходами.
        """
        print(Fore.BLUE + "Получение словаря комбинаций с нормализованными выходами...")

        hash_map: dict[str, float] = {}
        for c in combos:
            combo_key = "_".join(sorted(c))
            cost = sum(self.train_cfg.model_param.get(p, 0.0) for p in c)
            hash_map[combo_key] = cost

        if not hash_map:
            return {}

        min_v = min(hash_map.values())
        max_v = max(hash_map.values())

        out = (
            {k: (v - min_v) / (max_v - min_v) for k, v in hash_map.items()}
            if max_v > min_v
            else dict.fromkeys(hash_map, 0.0)
        )
        print(Fore.MAGENTA + "Комбинации:")
        pprint(out)
        return out

    def __convert_ann_to_model(self, ann: str, names: list[str]) -> None:
        """
        Преобразовать аннотации в формат текущей модели.
        """
        print(Fore.BLUE + "Преобразование аннотаций в формат текущей модели...")

        for dset in self.__pts.values():
            root = self._get_ann_root(dset / self.train_cfg.name_labels / ann)
            for img_obj in self._findall(root, "image"):
                f_name = self._get_attr_value(img_obj, "name")
                attribs = self._get_attribs(img_obj)
                if not f_name or "width" not in attribs or "height" not in attribs:
                    print(Fore.YELLOW + f"Пропуск {f_name} с неполными атрибутами.")
                    continue
                model_lines = self.__get_model_ann(img_obj, names)
                self._write_new_ann(
                    model_lines, dset / self.train_cfg.name_labels, f_name
                )

        print(Fore.MAGENTA + "Аннотации трансформированы.")

    def __get_model_ann(self, img: Any, names: list[str]) -> Iterable[str]:
        model_lines: Iterable[str] = []
        for box in self._findall(img, "box"):
            if (lbl := self.__get_concat_label(box)) not in names:
                continue

            try:
                model_bb = self._get_model_bb(
                    size=(
                        int(self._get_attr_value(img, "width")),
                        int(self._get_attr_value(img, "height")),
                    ),
                    box=self.__get_bb(box),
                )
                line = f"{names.index(lbl)} {' '.join([str(round(a, 6)) for a in model_bb])}\n"
                model_lines.append(line)
            except ValueError:
                continue

        return model_lines

    def __get_concat_label(self, root: Any) -> str:
        """Объединение параметров класса."""
        list_name = [
            self._get_attr_value(att, "name")
            for att in self._findall(root, "attribute")
            if (text := self._get_text(att)) is not None and text.lower() == "true"
        ]
        list_name = list(filter(None, list_name))
        return "_".join(sorted(list_name)) if list_name else "nothing"

    def __get_bb(self, root: Any) -> tuple[float, float, float, float]:
        """
        Возвращает bounding box из корня аттрибута изображения.
        :param root: корень аттрибута
        :type root: Any
        :return: координаты bb
        :rtype: tuple[float, float, float, float]
        """
        xtl = float(self._get_attr_value(root, "xtl"))
        ytl = float(self._get_attr_value(root, "ytl"))
        xbr = float(self._get_attr_value(root, "xbr"))
        ybr = float(self._get_attr_value(root, "ybr"))
        return (xtl, ytl, xbr, ybr)

    def __generate_config(self, names: list[str], target_name_cfg: str) -> None:
        config_data = {
            "path": str(self.train_cfg.path_output),
            "train": str(self.__pts[self.train_cfg.name_train]),
            "val": str(self.__pts[self.train_cfg.name_valid]),
            "test": str(self.__pts[self.train_cfg.name_test]),
            "nc": len(names),
            "names": names,
        }

        try:
            self._write_cfg_file(config_data, target_name_cfg)
            print(
                Fore.GREEN
                + f"Файл конфигурации успешно создан: {self.train_cfg.path_output / 'data.yaml'}"
            )
        except IOError as e:
            print(
                Fore.RED
                + f"Ошибка записи файла {self.train_cfg.path_output / 'data.yaml'}: {e}"
            )

    @abstractmethod
    def _get_train_cfg(self, train_cfg_path: str) -> TrainYamlConfig:
        """
        Возвращает конфиг для тренировки.
        :param train_cfg_path: путь до файла конфига тренировки
        :type train_cfg_path: str
        :return: экземпляр конфига тренировки
        :rtype: TrainConfig
        """
        pass

    @abstractmethod
    def _get_model_bb(
        self, size: tuple[int, int], box: tuple[float, float, float, float]
    ) -> tuple[float, float, float, float]:
        """
        Преобразует bounding box из аннотаций в формат модели
        :param size: ... #TODO
        :type size: tuple[int, int]
        :param box: координаты точек для вычисления bb
        :type box: tuple[float, float, float, float]
        :return: координаты bb модели
        :rtype: tuple[float, float, float, float]
        """
        pass

    @abstractmethod
    def _get_ann_root(self, file: Path) -> Any | None:
        """
        Возвращает корень дерева файла аннотаций.
        :param file: путь до файла аннотаций
        :type file: Path
        :return: корень дерева аннотаций
        :rtype: Any | None
        """
        pass

    @abstractmethod
    def _create_new_root(self) -> Any:
        """
        Создаёт и возвращает новый корень аннотаций.
        :return: новый корень аннотаций
        :rtype: Any
        """
        pass

    @abstractmethod
    def _append(self, root: Any, obj: Any) -> Any:
        """
        Добавляет новый аттрибут в корень аннотаций.
        :param root: корень дерева аннотаций
        :type root: Any
        :param obj: объект для добавления
        :type obj: Any
        :return: модифицированный корень аннотаций
        :rtype: Any
        """
        pass

    @abstractmethod
    def _write_new_tree(self, root: Any, path: Path) -> None:
        """
        Записывает дерево аннотаций в файл.
        :param root: корень дерева
        :type root: Any
        :param path: целевой путь с именем файла
        :type path: Path
        """
        pass

    @abstractmethod
    def _findall(self, root: Any, name: str) -> Iterable[Any]:
        """
        Возвращает все корни с указанным именем.
        :param root: корень дерева аннотаций
        :type root: Any
        :param name: имя параметра
        :type name: str
        :return: перечисление корней
        :rtype: Iterable[Any]
        """
        pass

    @abstractmethod
    def _find(self, root: Any, obj: str) -> Any | None:
        """
        Возвращает классы в аннотациях.
        :param root: корень дерева аннотаций
        :type root: Any
        :param obj: Description
        :type obj: str
        :return: Description
        :rtype: Any | None
        """
        pass

    @abstractmethod
    def _start_train(self) -> None:
        """
        Запускает тренировку модели.
        """
        pass

    @abstractmethod
    def _get_unique_attributes(self, root: Any) -> Iterable[str]:
        """
        Возвращает список уникальных аттрибутов.
        :param root: корень дерева
        :type root: Any
        :return: перечисление уникальных аттрибутов
        :rtype: Iterable[str]
        """
        pass

    @abstractmethod
    def _get_attr_value(self, root: Any, name: str) -> str:
        """
        Возвращает значение аттрибута.
        :param root: корень дерева
        :type root: Any
        :param name: имя аттрибута
        :type name: str
        :return: строковое значение аттрибута
        :rtype: str
        """
        pass

    @abstractmethod
    def _get_attribs(self, root: Any) -> dict[str, str]:
        """
        Возвращает словарь всех аттрибутов.
        :param root: корень дерева
        :type root: Any
        :return: сллварь имён аттрибутов
        :rtype: dict[str, str]
        """
        pass

    @abstractmethod
    def _get_text(self, root: Any) -> str | None:
        """
        Возвращает содержимое текста аттрибута.
        :param root: корень дерева
        :type root: Any
        :return: содержимое текста аттрибута
        :rtype: str | None
        """
        pass

    @abstractmethod
    def _write_new_ann(
        self, model_lines: Iterable[str], path: Path, filename: str
    ) -> None:
        """
        Записывает аннотации в пригодном для модели виде и пути.
        :param model_lines: любое перечисление ... #TODO
        :type model_lines: Iterable[str]
        :param path: целевой путь записи файла аннотаций
        :type path: Path
        :param filename: имя нового файла
        :type filename: str
        """
        pass

    @abstractmethod
    def _write_cfg_file(
        self, config_data: dict[str, Any], target_name_cfg: str
    ) -> None:
        """
        Записывает файл конфига для старта тренировки модели.
        :param self: Description
        :param target_name_cfg: Description
        :type target_name_cfg: str
        :param config_data: словарь необходимых для тренировки модели параметров
        :type config_data: dict[str, Any]
        """
        pass

    @abstractmethod
    def _get_arch_paths(self) -> Iterator[Path]:
        """
        Возвращает пути к архивам для тренировки.
        :return: генератор путей к архивам
        :rtype: Iterator[Path]
        """
        pass

    @abstractmethod
    def _unpack_move_join_imgs_and_ann(
        self, root: Any, archives: Iterator[Path], tp: Path, ann: str
    ) -> Any:
        """
        Работа с архивами желаемого типа
        :param root: корень дерева
        :type root: Any
        :param archives: генератор доступных архивов
        :type archives: Iterator[Path]
        :param tp: путь до временного каталога
        :type tp: Path
        :param ann: имя файла аннотаций
        :type ann: str
        :return: новый модифицированный корень
        :rtype: Any
        """
        pass


class InferenceModel(ABC):
    def __init__(self, inf_cfg_path: str) -> None:
        self.inf_cfg = self._get_inference_cfg(inf_cfg_path)

    @abstractmethod
    def get_model_pred(self, frame: cv2.typing.MatLike) -> Any:
        """
        Возвращает предсказание модели.
        :param frame: исходный кадр
        :type frame: cv2.typing.MatLike
        :return: кадр с предсказанием
        :rtype: Results
        """
        pass

    def inference(self):
        """
        Точка входа инференса
        """
        init(autoreset=True)
        print(Fore.BLUE + "Inference started...")

        self.__imgs = list(
            get_file_paths(
                [self.inf_cfg.path_input],
                (".jpg", ".jpeg", ".png", ".bmp", ".tif", ".tiff", ".webp"),
            )
        )
        if not self.__imgs:
            raise FileExistsError(
                Fore.RED
                + f"{self.inf_cfg.path_input / 'dataset'}: не найдено изображений!"
            )
        self.__imgs = natsorted(self.__imgs)

        self.__wrtr, self.__out_s = self.__get_video_writer()
        self.__create_window()
        self.__start_inference()

    def __get_video_writer(self) -> tuple[cv2.VideoWriter | None, tuple[int, int]]:
        """
        Активирует запись видео в файл, используя размеры первого кадра.
        :return: экземпляр записи видео и размер видео
        :rtype: tuple[VideoWriter | None, tuple[int, int]]
        """
        if not self.inf_cfg.video_save or self.inf_cfg.path_output is None:
            return None, (-1, -1)

        frm = cv2.imread(str(self.__imgs[0]))
        if frm is None:
            return None, (-1, -1)

        h, w = frm.shape[:2]
        if self.inf_cfg.video_height:
            w, h = int(w * self.inf_cfg.video_height / h), self.inf_cfg.video_height

        now = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
        raw = str(datetime.datetime.now().timestamp()).encode()
        hsh = md5(raw).hexdigest()[:6]

        writer = cv2.VideoWriter(
            str(
                self.inf_cfg.path_output
                / f"{self.inf_cfg.video_name}_{now}_{hsh}.{self.inf_cfg.video_extention}"
            ),
            cv2.VideoWriter.fourcc(*self.inf_cfg.video_codec),
            self.inf_cfg.video_fps,
            (w, h),
        )

        if not writer.isOpened():
            print(
                Fore.RED
                + f"Не удалось открыть VideoWriter. Проверьте кодек '{self.inf_cfg.video_codec}' и путь."
            )
            return None, (w, h)

        return writer, (w, h)

    def __get_bb_frm(self, frm: cv2.typing.MatLike) -> cv2.typing.MatLike:
        """
        Возвращает обработанный кадр.
        :param frm: обработанный кадр
        :type frm: cv2.typing.MatLike
        :return: Description
        :rtype: MatLike
        """
        pred = self.get_model_pred(frm)
        if pred.boxes:
            for box in pred.boxes:
                cls_name = pred.names[int(box.cls[0])]
                conf = float(box.conf[0])

                x1, y1, x2, y2 = map(int, box.xyxy[0].tolist())
                cv2.rectangle(frm, (x1, y1), (x2, y2), self.inf_cfg.display_color, 2)
                cv2.putText(
                    frm,
                    f"{cls_name} {conf:.2f}",
                    (x1, y1 - 10),
                    cv2.FONT_HERSHEY_SIMPLEX,
                    0.8,
                    self.inf_cfg.display_color,
                    2,
                )

        return frm

    def __create_window(self) -> None:
        """
        Создаёт окно вывода инференса в реальном времени.
        """
        if self.inf_cfg.window_show:
            cv2.namedWindow(self.inf_cfg.window_name, cv2.WINDOW_NORMAL)

    def __start_inference(self):
        """
        Запускает инференс. Esc - закончить инференс.
        """
        for path in tqdm(self.__imgs, desc="Прогресс инференса"):
            frm = cv2.imread(str(path))
            if frm is None:
                continue

            bb_frm = self.__get_bb_frm(frm)
            self.__write_video(bb_frm)
            self.__show_frame(path, frm)
            if cv2.waitKey(30) & 0xFF == 27:
                break

        self.__stop_write()
        self.__close_all_windows()

    def __write_video(self, frame: cv2.typing.MatLike) -> None:
        """
        Записывает видео в файл.
        :param frame: кадр
        :type frame: cv2.typing.MatLike
        """
        if self.__wrtr:
            img = cv2.resize(frame, self.__out_s, interpolation=cv2.INTER_AREA)
            self.__wrtr.write(img)

    def __show_frame(self, img_path: Path, frm: cv2.typing.MatLike) -> None:
        """
        Выводит на экран обработанный кадр.
        :param img_path: путь до изображения
        :type img_path: Path
        :param frm: кадр изображения
        :type frm: cv2.typing.MatLike
        """
        if self.inf_cfg.window_show:
            cv2.putText(
                frm,
                img_path.name,
                (20, 40),
                cv2.FONT_HERSHEY_SIMPLEX,
                1,
                self.inf_cfg.display_color,
                2,
                cv2.LINE_AA,
            )
            cv2.imshow(self.inf_cfg.window_name, frm)

    def __stop_write(self):
        """
        Останавливает запись и собирает видео.
        """
        if self.__wrtr:
            self.__wrtr.release()

    def __close_all_windows(self):
        """
        Закрывает экран вывода.
        """
        if self.inf_cfg.window_show:
            cv2.destroyAllWindows()

    @abstractmethod
    def _get_inference_cfg(self, inf_cfg_path: str) -> InferenceYamlConfig:
        """
        Возвращает конфиг для инференса.
        :param inf_yaml_cfg_path: Description
        :type inf_yaml_cfg_path: str
        :return: Description
        :rtype: InferenceConfig
        """
        pass
