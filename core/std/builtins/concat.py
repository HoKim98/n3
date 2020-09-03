import torch

from n3 import ExternNode


class Concat(ExternNode):
    axis: int
    
    def __init__(self, input, output, **kwargs):
        super().__init__(input, output, **kwargs)
        if self.axis >= 0:
            self.axis += 1

    def forward(self, x):
        return torch.cat(x, dim=self.axis)
