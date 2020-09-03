import torch.nn as nn

from n3 import ExternNode


class Linear(ExternNode):
    input_channels: int
    output_channels: int

    bias: bool

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._inner = nn.Linear(self.input_channels,
                                self.output_channels,
                                self.bias)

    def forward(self, x):
        return self._inner(x)
