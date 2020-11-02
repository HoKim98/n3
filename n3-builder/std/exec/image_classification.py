from n3 import *


class ImageClassification(Trainer):
    def _train_epoch_begin(self, epoch, metrics):
        super()._train_epoch_begin(epoch, metrics)
        metrics['accuracy'] = 0.0

    def _train_iter_end(self, metrics, x, y, y_pred, loss):
        super()._train_iter_end(metrics, x, y, y_pred, loss)

        y = y['y']
        y_pred = y_pred['x']
        accuracy = (y == y_pred.max(dim=-1)[1]).sum().item()
        accuracy /= y.size(0)
        metrics['accuracy'] += accuracy

    def eval(self):
        raise NotImplementedError
