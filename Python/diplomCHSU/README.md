# Dev-info

Мой ВКР в ЧГУ.

## Get Started

1. создать каталог: `D:/docker-data`;
2. в нём расположить все zip-архивы с датасетами (ВНИМАНИЕ: за любую потерю данных при выполнении действий вне инструкций в рамках данного каталога несёт ответственность ПОЛЬЗОВАТЕЛЬ);
3. собрать образ в каталоге, где расположен Dockerfile:

```shell
docker build -t diplom:prod  -f Dockerfile .
```

4. запустить образ и смонтировать локальный каталог с данными для обучения:

```shell
docker run -it --rm -v D:/docker-data:/root/data diplom
```

5. вызвать ...

### Шаблон файла 'train_config.yaml'

```yaml
# Пути к изображениям (Requirement)
path_input: path/to/images
path_output: path/to/output/model/folder

# Настройки модели обучени (Default)
model_type: yolo
model_name: yolo11n.pt
model_device: cuda #cpu 

# Базовые имена папок (не следует менять)
name_archives: archives
name_images: images
name_labels: labels
name_train: train
name_valid: val
name_test: test

# Может быть изменено, зависит от специфики модели
model_param: { 
    nothing: 0 
    # other1: 1
    # other2: 2
    # other3: 4
    # other4: 8
} # субпараметры классов
model_output_name: bestYolo11n.pt

# Параметры обучения (подгоняются во время экспериментов)
train_epochs: 5
train_image_size: 640
train_batch: 8
train_workers: 12
# Параметр, влияющий на то, какие субпараметры у классов (если они есть)
# не будут объеденины в итоговой модели.
# Например, параметр "Спина" может не быть одновременно с "Очками"
train_skip_condition: 'lambda x: True'

# Какой файл аннотаций нужно искать и как будут названы выходные
annotations_name: annotations
annotations_suffix: .xml

# Имя выходного конфигурационного файла, генерируемого специально для модели
datayaml: data.yaml

# Параметры разбиения обучающей выборки. Следует держать в сумме 1.0
ratio_train: 0.7
ratio_valid: 0.2
ratio_test: 0.1
```

### Шаблон файла 'inference_config.yaml'

```yaml
# Пути к изображениям (Requirement)
path_input: path/to/images
path_output: path/to/output/model/folder

# Настройки модели инференса
model_type: yolo
model_name: path/to/output/model/best.pt
model_device: cuda

# Конфигурация видео
video_save: true
video_fps: 60
video_name: "VID"
video_extention: "avi"
video_codec: "DIVX"
video_height: null

# Воспроизводить окно в реальном времени
window_show: true
window_name: "Inference"

# Классы выходной модели (можно взять из 'data.yaml' или скопировать из консоли во время подготовки к обучению)
model_labels: 
- person
- airplane
- ...

# Порог точности предсказания
model_confidence: 0.4
```

## Dev

### Уставновка UV

```bash
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex" # powershell
winget install --id=astral-sh.uv -e # winget
pipx install uv # python PIP

uv --version # проверить версию
uv self update # обновить
```

Потом переоткрыть терминал для вступления в силу переменных окружения (PATH).

### Создание нового проекта в виртуальном окружении

```bash
uv init [<dir>] # создать venv вручную
uv venv --python 3.13 # если нет Python на ПК, uv сам его установит автономно
```

Команда для инлайн-запуска Python-скрпитов без необходимости вручную активировать и деактивировать виртуальное окружение. При выполнении, `uv` автоматически активирует виртуальное окружение, а после завершения работы скрипта деактивирует его. При первом запуске команды внутри текущей директории будет создана `.venv`, содержащая устанавливаемые зависимости, используемые в окружении. Помимо этого, будет создан кроссплатформенный uv.lock файл.

```bash
uv run <script.py>
uv run --python 3.10 <script.py>
```

```bash
uv add <package> # установка пакетов
uv add --dev pytest <package> # установка пакетов для разработки
uv remove <package> # удалиь пакеты
```

```bash
uv sync # скачивание пакетов в окружении в соответствии с pyproject.toml
uv tree # вывод дерева пакетов

uv tree --outdated # проверить обновления пакетов
uv add <package> --upgrade # обновить пакеты что нуждаются
uv sync --upgrade # обновить все пакеты
```

```bash
uv python install 3.10 3.11 3.12 # установка пайтона
uv python pin 3.10 # выбор приоритетной версии запуска
uvx cowsay -t "Hello Habr" # тестовый запуск библиотеки, установленной во временное venv
uv cache clean # чистка кеша пакетов (не рекомендуется для скорости работы, но иначе занимает много памяти)

# запуск дополнительно с пакетами без вшивания в venv
uv run ‑with jupyter jupyter notebook
uvx ‑with pendulum ‑p 3.13t python
```

### Этапы подготовки окружения для разработки

```bash
uv sync --dev
uv pip install -e .
uv run -m miku_diplom # или `uv run pytest -v`
```

```bash
uv run -m src.miku_diplom  # запуск модуля
```

Размер пакетов: около 5 Гб.
