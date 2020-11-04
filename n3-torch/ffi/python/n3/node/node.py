import abc
from typing import Dict, List, Union

import torch
import torch.nn as nn

from ..util.out import Out, Outs

Indices = Union[str, List['Indices']]
TensorDict = Dict[str, torch.Tensor]
TensorDictOrX = Union[TensorDict, torch.Tensor]


def _index(data: TensorDict, key: Indices) -> torch.Tensor:
    # indices
    if isinstance(key, list):
        return [_index(data, k) for k in key]
    return data[key]


class Node(metaclass=abc.ABCMeta):
    def __init__(self, input: Outs, output: Outs) -> None:
        self._node_input = input
        self._node_output = output

    def get_name(self) -> str:
        return self.__class__.__name__

    def forward(self, **kwargs: TensorDict) -> TensorDictOrX:
        raise NotImplementedError

    def __repr__(self, depth: int = 0) -> str:
        return super().__repr__()


class NodeExecutable(Node, nn.Module):
    def __init__(self, input: Outs, output: Outs, name: str, tensor_graph: List[Node]) -> None:
        Node.__init__(self, input, output)
        nn.Module.__init__(self)
        self._name = name
        self._tensor_graph = nn.ModuleList(tensor_graph)

    def get_name(self) -> str:
        return self._name

    def forward(self, *args: List[torch.Tensor], **kwargs: TensorDict) -> TensorDict:
        if len(args) == 1 and isinstance(args[0], dict) and not kwargs:
            return self(**args[0])

        output = {Out(1, k): x for k, x in kwargs.items()}
        x = {}

        for node in self._tensor_graph:
            x: TensorDict = {k: _index(output, n)
                             for k, n in node._node_input.items()}
            x: TensorDictOrX = node.forward(**x)
            if not isinstance(x, dict):
                x = {'x': x}
            x = {n: x[k] for k, n in node._node_output.items()}
            output = {**output, **x}

        return {k.name: v for k, v in x.items()}

    def __repr__(self, depth: int = 0) -> str:
        indent = ' ' * (depth * 4) + ' ' * 2
        indent_node = ' ' * ((depth+1) * 4)

        prefix = '' if depth else '* '

        name = f'{prefix}[node object {self._name}]'
        input = f'\n{indent}[input]\n' + \
            '\n'.join(f'{indent_node}{k}: {repr(v)}'
                      for k, v in self._node_input.items())
        output = f'\n{indent}[output]\n' + \
            '\n'.join(f'{indent_node}{k}: {repr(v)}'
                      for k, v in self._node_output.items())
        tensor_graph = '\n'.join(f'{indent}({id}) {n.__repr__(depth+1)}'
                                 for id, n in enumerate(self._tensor_graph))
        return name + input + output + '\n' + tensor_graph
