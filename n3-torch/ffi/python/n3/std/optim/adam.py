import torch.optim as optim

from n3 import OptimNode


class Adam(OptimNode):
    learning_rate: float

    def initialize(self, params):
        return optim.Adam(params, lr=self.learning_rate)
