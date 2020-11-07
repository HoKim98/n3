import glob
import os
import sys

import inflection
import tensorboardX

from .epoch import EpochWriter
from ..util.args import Args
from ..util.dirs import LOGS_DIR


class ExecWriter:
    def __init__(self, args: Args, exec: str, model: str, root: bool) -> None:
        super().__init__()
        self._exec_name = inflection.underscore(exec)
        self._model_name = inflection.underscore(model)

        if root:
            logdir = os.path.join(args['env']['root'], LOGS_DIR,
                                  self._exec_name, self._model_name)
            logdir = _increment_dir(logdir)
            self._writer = tensorboardX.SummaryWriter(logdir)
        else:
            self._writer = None

        self._epoch = args['epoch']

    def do_epoch(self, tag, fn_dataset):
        tag = f'{self._exec_name}/{tag}'
        return EpochWriter(self._writer, tag, fn_dataset, 0, self._epoch)

    def close(self):
        if self._writer is not None:
            self._writer.close()
            self._writer = None


def spawn_daemon(env):
    from tensorboard.main import run_main

    logdir = os.path.join(env.root, LOGS_DIR)
    sys.argv = [sys.argv[0]] + ['--logdir', logdir] + sys.argv[3:]
    run_main()


def _increment_dir(logdir):
    num_dirs = len(glob.glob(rf'{logdir}/exp*/'))
    return os.path.join(logdir, f'exp{num_dirs}')
