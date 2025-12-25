import torch
from torch import version

if __name__ == '__main__':
    print("PyTorch version:", torch.__version__)
    print("cuDNN version:", torch.backends.cudnn.version())
    print("PyTorch built with CUDA version:", version.cuda)
    if torch.cuda.is_available():
        device_count = torch.cuda.device_count()
        print(f"CUDA: Доступна ({device_count} шт.)")

        for i in range(device_count):
            properties = torch.cuda.get_device_properties(i)
            print(f"  [{i}] {properties.name}")
            print(f"      Память: {properties.total_memory / (1024**3):.2f} ГБ")
            print(f"      Архитектура: СС {properties.major}.{properties.minor}")
        
        try:
            import subprocess
            result = subprocess.run(["nvidia-smi"], capture_output=True, text=True)
            print(result.stdout)
        except Exception:
            print("Could not execute command")
    else:
        print("CUDA: Недоступна (работа на CPU)")
        
    if torch.backends.mps.is_available():
        print("MPS (Apple Silicon): Доступна")
    