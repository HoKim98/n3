import abc

import torch.nn as nn

from .node import Node, TensorDict, TensorDictOrX
from ..util.out import Outs

_MAGIC_VARIABLES = ['exec']


class ExternNodeBase(Node, metaclass=abc.ABCMeta):
    def __init__(self, input: Outs, output: Outs, **kwargs) -> None:
        super().__init__(input, output)
        for k, v in kwargs.items():
            if k in _MAGIC_VARIABLES:
                continue
            setattr(self, k, v)

    @abc.abstractmethod
    def forward(self, **kwargs: TensorDict) -> TensorDictOrX:
        raise NotImplementedError

    def __repr__(self, depth: int = 0) -> str:
        indent = ' ' * (depth * 4) + ' ' * 2
        indent_node = ' ' * ((depth+1) * 4)

        name = f'[node extern object {self.__class__.__name__}]'
        input = f'\n{indent}[input]\n' + \
            '\n'.join(f'{indent_node}{k}: {repr(v)}'
                      for k, v in self._node_input.items())
        output = f'\n{indent}[output]\n' + \
            '\n'.join(f'{indent_node}{k}: {repr(v)}'
                      for k, v in self._node_output.items())
        return name + input + output


class ExternNode(ExternNodeBase, nn.Module, metaclass=abc.ABCMeta):
    def __init__(self, input: Outs, output: Outs, **kwargs) -> None:
        ExternNodeBase.__init__(self, input, output, **kwargs)
        nn.Module.__init__(self)

    @abc.abstractmethod
    def forward(self, **kwargs: TensorDict) -> TensorDictOrX:
        raise NotImplementedError
