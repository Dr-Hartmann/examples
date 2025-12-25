FROM python:3.12-slim
RUN apt-get update && apt-get install -y gcc && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN pip install torch torchvision --index-url https://download.pytorch.org/whl/cu128

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . .

CMD ["bash", "-c", "echo '\e[36Goooooooooooooooooood morning Vietnam!\e[0m::PROD' && exec bash"]
