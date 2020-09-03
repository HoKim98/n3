from torch.utils.data import DataLoader
from torchvision import transforms
from torchvision.datasets import MNIST

from n3.builder import DataNode


class Mnist(DataNode):
    _train_dataset = None
    _train_loader = None

    def get_train_dataset(self):
        if self._train_dataset is None:
            self._train_dataset = MNIST(self.dataset_dir,
                                        transform=transforms.ToTensor(),
                                        train=True,
                                        download=True)
            self._train_loader = DataLoader(dataset=self._train_dataset,
                                            batch_size=self.batch_size,
                                            shuffle=True)
        return self._train_loader

    def get_valid_dataset(self):
        raise NotImplementedError

    def get_eval_dataset(self):
        raise NotImplementedError
