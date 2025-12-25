import sys
import shutil
import textwrap
from pathlib import Path
from colorama import Fore, init


def get_terminal_width() -> int:
    try:
        return shutil.get_terminal_size().columns
    except OSError:
        return 120


def print_separator_title(
    title: str, term_width: int, color: str = Fore.MAGENTA
) -> None:
    print(Fore.CYAN + "=" * term_width)
    print(color + f"=== {title} ===")


def print_paths(term_width: int) -> None:
    def print_table_row(key: str, value: str, key_width: int) -> None:
        value_width = term_width - key_width - 5
        indent = " " * (key_width + 3)
        wrapper = textwrap.TextWrapper(width=value_width, subsequent_indent=indent)
        prefix = Fore.YELLOW + key.ljust(key_width) + Fore.CYAN + " | " + Fore.GREEN
        wrapped_text = wrapper.fill(value).replace("\n", "\n" + indent)
        print(prefix + wrapped_text)

    paths = {
        "Файл исполнения": str(Path(sys.argv[0])),
        "Абсолютный путь": str(Path(sys.argv[0]).resolve()),
        "Рабочая директория": str(Path.cwd()),
        "Домашняя директория": str(Path.home()),
        "Интерпретатор Python": sys.executable,
        "__file__": globals().get("__file__", "нет (Jupyter)"),
    }

    key_width = max(len(k) for k in paths) + 2

    print(Fore.CYAN + "=" * term_width)
    print(Fore.YELLOW + f"{'Название'.ljust(key_width)} | {Fore.GREEN}Значение")
    print(Fore.CYAN + "=" * term_width)

    for k, v in paths.items():
        print_table_row(k, v, key_width)


def print_sys_paths(term_width: int):
    def print_sys_path_item(index: int, path_str: str, term_width: int) -> None:
        path_width = term_width - 6
        path_indent = " " * 6
        wrapper = textwrap.TextWrapper(width=path_width, subsequent_indent=path_indent)
        prefix = Fore.CYAN + f"{index:2d}. " + Fore.GREEN
        wrapped_text = wrapper.fill(path_str).replace("\n", "\n" + path_indent)
        print(prefix + wrapped_text)

    for i, p in enumerate(sys.path, 1):
        print_sys_path_item(i, p, term_width)


if __name__ == "__main__":
    init(autoreset=True)
    term_width = get_terminal_width()
    print_separator_title("Path(sys.argv[0])", term_width)
    print_paths(term_width)
    print_separator_title("sys.path", term_width)
    print_sys_paths(term_width)
