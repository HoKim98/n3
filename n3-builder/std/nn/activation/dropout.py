import torch.nn as nn

from n3 import ExternNode


class Dropout(ExternNode):
    probability: float

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.Dropout(self.probability)

    def forward(self, x):
        return self._inner(x)
