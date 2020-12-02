import time

from n3 import *


class DummyImageClassification(Trainer):
    def train(self, kwargs):
        self._writer.attach_rust_kwargs(kwargs)

        self.model.train()
        self.optimizer._initialize(self.model)

        data = iter(self.data.get_train_dataset())
        _x, _classes = next(data)

        time.sleep(1)
        self._train_end()

    def eval(self, kwargs):
        raise NotImplementedError
