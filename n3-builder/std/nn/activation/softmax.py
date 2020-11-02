import torch.nn as nn

from n3 import ExternNode


class Softmax(ExternNode):
    dimension: int

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.Softmax(self.dimension)

    def forward(self, x):
        return self._inner(x)
