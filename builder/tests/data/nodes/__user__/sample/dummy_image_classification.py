from n3 import *


class DummyImageClassification(Trainer):
    def train(self):
        # Step 1. ready to train
        self.model.train()
        self.optimizer.initialize(self.model)

        data = iter(self.data.get_train_dataset())
        _x, _classes = next(data)

    def eval(self):
        raise NotImplementedError
