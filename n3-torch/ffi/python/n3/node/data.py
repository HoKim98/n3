import abc
import os
from typing import Iterable

from inflection import underscore
import torch

from .extern import ExternNodeBase
from ..util.args import Args
from ..util.dirs import DATA_DIR

Dataset = Iterable


class DataNode(ExternNodeBase, metaclass=abc.ABCMeta):
    def __init__(self, args: Args, **kwargs) -> None:
        super().__init__(args, **kwargs)
        name = underscore(self.__class__.__name__).replace('_', '-')
        self._dataset_dir = os.path.join(args['env']['root'], DATA_DIR, name)
        self._batch_size = args['batch size']

    @property
    def dataset_dir(self) -> str:
        return self._dataset_dir

    @property
    def batch_size(self) -> int:
        return self._batch_size

    def forward(self) -> None:
        raise Exception('data node cannot be directly called')

    @abc.abstractmethod
    def get_train_dataset(self) -> Iterable[Dataset]:
        raise NotImplementedError

    @abc.abstractmethod
    def get_valid_dataset(self) -> Iterable[Dataset]:
        raise NotImplementedError

    @abc.abstractmethod
    def get_eval_dataset(self) -> Iterable[Dataset]:
        raise NotImplementedError
