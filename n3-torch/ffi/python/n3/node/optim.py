import abc
from typing import List

import torch
import torch.optim as optim

from .extern import ExternNodeBase
from .node import Node


class OptimNode(ExternNodeBase, metaclass=abc.ABCMeta):
    _inner: optim.Optimizer = None

    @abc.abstractmethod
    def initialize(self, params) -> None:
        pass

    def _initialize(self, *models: List[Node]) -> None:
        if self._inner is None:
            params = [p for m in models for p in m.parameters()]
            self._inner = self.initialize(params)

    def forward(self) -> None:
        raise Exception('optim node cannot be directly called')

    def zero_grad(self) -> None:
        self._inner.zero_grad()

    def step(self) -> None:
        self._inner.step()
