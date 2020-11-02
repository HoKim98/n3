import torch.nn as nn

from n3 import ExternNode


class Relu(ExternNode):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.ReLU()

    def forward(self, x):
        return self._inner(x)
