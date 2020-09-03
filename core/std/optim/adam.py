import torch.optim as optim

from n3.builder import OptimNode


class Adam(OptimNode):
    learning_rate: float

    def _initialize(self, params):
        return optim.Adam(params, lr=self.learning_rate)
