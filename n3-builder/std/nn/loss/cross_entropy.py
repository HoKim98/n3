import torch.nn as nn

from n3 import ExternNode


class CrossEntropy(ExternNode):
    number_of_classes: int

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.CrossEntropyLoss()

    def forward(self, x, y):
        return self._inner(x, y)
