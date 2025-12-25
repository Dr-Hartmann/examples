import os
import torch
import random
import numpy as np
    
def fix_seeds(seed: int):
    random.seed(seed)
    np.random.seed(seed)
    torch.manual_seed(seed)
    torch.cuda.manual_seed_all(seed)
    torch.mps.manual_seed(seed)
def enable_determinism():
    os.environ["CUBLAS_WORKSPACE_CONFIG"] = ":4096:8"
    torch.use_deterministic_algorithms(True)


if __name__ == '__main__':
    """Воспроизводимость экспериментов"""    
    seed = 42
    fix_seeds(seed)
    enable_determinism()
