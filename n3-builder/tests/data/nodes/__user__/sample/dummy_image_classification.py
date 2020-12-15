import time

from n3 import *


class DummyImageClassification(Trainer):
    def train(self, kwargs):
        self._writer.attach_rust_kwargs(kwargs)

        self.model.train()
        self.optimizer._initialize(self.model)

        data = iter(self.data.get_train_dataset())
        x, y = next(data)

        self.optimizer.zero_grad()
        y_pred = self.model(x=x)
        loss = self.loss(**y_pred, y=y)['x']
        loss.backward()

        self._train_end()

    def eval(self, kwargs):
        raise NotImplementedError
