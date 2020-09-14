import torch.nn as nn

from n3 import ExternNode


class BatchNorm2D(ExternNode):
    channels: int

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.BatchNorm2d(
            num_features=self.channels,
        )

    def forward(self, x):
        return self._inner(x)
