import abc
import os
from typing import Any, Dict, List

import inflection
import torch
import torch.nn as nn
from torch import Tensor

from .data import DataNode
from .node import Node, TensorDict
from .optim import OptimNode
from ..util.args import Args
from ..writer import EpochWriter, ExecWriter

Metrics = Dict[str, Any]


class ExecNode(metaclass=abc.ABCMeta):
    _id: int
    _machine: str

    _is_root: bool
    _is_distributed: bool

    _gpu_id: int

    _writer: ExecWriter

    def __init__(self, args: Args, nodes: Dict[str, Node] = {}) -> None:
        for k, v in args.items():
            setattr(self, k.replace(' ', '_'), v)
        for k, v in nodes.items():
            setattr(self, k.replace(' ', '_'), v)
        self._nodes = nodes

        # Attach variables
        env = args['env']
        self._id = env['id']
        self._machine = env['machine']

        self._is_root = env['is root']
        self._is_distributed = env['is distributed']

        self._gpu_id = env['gpu id']

        # Distributed Training
        if self._is_distributed:
            torch.distributed.init_process_group(backend='nccl')

        self._writer = ExecWriter(args,
                                  exec=self.get_name(),
                                  model=self.get_model_name(),
                                  root=self._is_root)

    def get_name(self) -> str:
        return self.__class__.__name__

    def get_model_name(self) -> str:
        raise NotImplementedError

    def nodes(self):
        return self._nodes

    def to(self, node: Node) -> Node:
        # check node
        if not isinstance(node, nn.Module):
            return node

        node = node.to(self._machine)
        if self._is_distributed and any((p.requires_grad for p in node.parameters())):
            device_ids = [self._gpu_id]  # GPU device id
            node = nn.parallel.DistributedDataParallel(node,
                                                       device_ids=device_ids)
        return node

    def tensor_to(self, value) -> Node:
        if isinstance(value, dict):
            return {k: self.tensor_to(v) for k, v in value.items()}
        if isinstance(value, (list, tuple)):
            return [self.tensor_to(v) for v in value]
        return value.to(self._machine)

    def close(self):
        pass


class Trainer(ExecNode, metaclass=abc.ABCMeta):
    data: DataNode
    model: Node

    loss: Node
    optimizer: OptimNode

    epoch: int
    batch_size: int

    def train(self, handler) -> None:
        # Step 1. ready to train
        self._train_begin()

        # Step 2-1. peek the IO
        for writer, dataset in self._writer.do_epoch('train', self.data.get_train_dataset):
            metrics = {
                'loss': 0.0,
            }
            self._train_epoch_begin(writer, metrics)

            for data in dataset:
                data = self._train_iter_begin(data)
                x, y = self.tensor_to(data)
                # Step 2-2. clean-up gradients
                self.optimizer.zero_grad()
                # Step 2-3. predict classses
                y_pred = self.model(**x)
                # Step 2-4. calculate difference (loss)
                loss = self.loss(**y_pred, **y)['x']
                # Step 2-5. calculate gradients
                loss.backward()
                # Step 2-6. step
                self.optimizer.step()
                # Step 2-7. store result
                self._train_iter_end(metrics, x, y, y_pred, loss)
                if not handler.is_running():
                    break

            # Step 2-8. store log
            self._train_epoch_end(writer, metrics)
            if not handler.is_running():
                break

        # Step 3. clean up
        self._train_end()

    def _train_begin(self) -> None:
        self.optimizer._initialize(self.model)
        for name, node in self.nodes().items():
            setattr(self, name, self.to(node))

    def _train_end(self) -> None:
        pass

    def _train_epoch_begin(self, writer: EpochWriter, metrics: Metrics) -> None:
        self.model.train()

    def _train_epoch_end(self, writer: EpochWriter, metrics: Metrics) -> None:
        for name, value in metrics.items():
            writer.write(name, value, use_batch=True)
        writer.flush()

    def _train_iter_begin(self, data: List[Tensor]) -> TensorDict:
        return {'x': self.to(data[0])}, {'y': self.to(data[1])}

    def _train_iter_end(self, metrics: Metrics, x: Tensor, y: Tensor, y_pred: Tensor, loss: Tensor) -> None:
        metrics['loss'] += loss.item()

    @abc.abstractmethod
    def eval(self, handler) -> None:
        raise NotImplementedError

    def publish(self, handler, args: Args) -> None:
        # Step 1. ready to publish
        self.model.eval()

        # Step 2. get dummy input
        x, _ = next(iter(self.data.get_train_dataset()))

        # Step 3. get parameters
        input_names = ['x']  # 모델의 입력값을 가리키는 이름
        output_names = ['out_x']  # 모델의 출력값을 가리키는 이름
        dynamic_axes = {'x': {0: 'batch_size'},  # 가변적인 길이를 가진 차원
                        'out_x': {0: 'batch_size'}}

        export_params = True  # 모델 파일 안에 학습된 모델 가중치를 저장할지의 여부
        opset_version = 10  # 모델을 변환할 때 사용할 ONNX 버전
        do_constant_folding = True  # 최적화시 상수폴딩을 사용할지의 여부

        name = inflection.underscore(self.model.get_name())
        output_path = os.path.join(args.output_path, f'{name}.onnx')

        # Step 4. export to onnx
        torch.onnx.export(self.model, {'x': x}, output_path,
                          input_names=input_names,
                          output_names=output_names,
                          dynamic_axes=dynamic_axes,

                          export_params=export_params,
                          opset_version=opset_version,
                          do_constant_folding=do_constant_folding,
                          )

        # Step 5. do target-specific publishing
        # TODO: to be implemented

    def get_model_name(self) -> str:
        return self.model.get_name()

    def close(self):
        super().close()
        self._writer.close()
