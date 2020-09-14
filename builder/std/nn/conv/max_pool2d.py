import torch.nn as nn

from n3 import ExternNode


class MaxPool2D(ExternNode):
    kernel_size: int
    padding: int

    stride: int

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.MaxPool2d(
            kernel_size=self.kernel_size,
            padding=self.padding,
            stride=self.stride,
        )

    def forward(self, x):
        return self._inner(x)
