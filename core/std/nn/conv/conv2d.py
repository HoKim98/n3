import torch.nn as nn

from n3 import ExternNode


class Conv2D(ExternNode):
    kernel_size: int
    padding: int

    stride: int

    input_channels: int
    output_channels: int

    bias: bool

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.Conv2d(self.input_channels,
                                self.output_channels,
                                kernel_size=self.kernel_size,
                                stride=self.stride,
                                padding=self.padding,
                                bias=self.bias)

    def forward(self, x):
        return self._inner(x)
