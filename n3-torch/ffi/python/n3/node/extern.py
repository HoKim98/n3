import abc

import torch.nn as nn

from .node import Node, TensorDict, TensorDictOrX
from ..util.args import Args, Values
from ..util.out import Outs


class ExternNodeBase(Node, metaclass=abc.ABCMeta):
    def __init__(self, args: Args, input: Outs, output: Outs, values: Values) -> None:
        super().__init__(input, output)
        for k, v in values.items():
            setattr(self, k.replace(' ', '_'), v)

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
    def __init__(self, args: Args, input: Outs, output: Outs, values: Values) -> None:
        ExternNodeBase.__init__(self, args, input, output, values)
        nn.Module.__init__(self)

    @abc.abstractmethod
    def forward(self, **kwargs: TensorDict) -> TensorDictOrX:
        raise NotImplementedError
