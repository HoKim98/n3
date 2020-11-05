from tqdm import tqdm


class EpochIter:
    def __init__(self, dataset, bar):
        self._dataset = iter(dataset)
        self._bar = bar

    def __iter__(self):
        return self

    def __next__(self):
        value = next(self._dataset)
        if self._bar is not None:
            self._bar.update()
        return value


class Epoch:
    def __init__(self, writer, head, num_batch, value):
        super().__init__()
        self._writer = writer
        self._head = head
        self._num_batch = num_batch
        self._value = value

    def write(self, name, value, use_batch=False):
        if self._writer is None:
            return
        if isinstance(value, (int, float)):
            if use_batch:
                value = value / self._num_batch
            self._write_scalar(name, value)
        else:
            raise Exception(f'not supported type: {type(value)}')

    def _write_scalar(self, name, value):
        self._writer.add_scalar(self._tag(name), value, self._value)

    def flush(self):
        if self._writer is not None:
            self._writer.flush()

    def _tag(self, name):
        return f'{self._head}/{name}'

    def __int__(self):
        return self._value

    def __repr__(self):
        return str(int(self))


class EpochWriter:
    def __init__(self, writer, head, fn_dataset, start, end):
        super().__init__()
        self._writer = writer
        self._head = head
        self._fn_dataset = fn_dataset
        self._start = start
        self._end = end

        self._bar = None

    def __iter__(self):
        return self

    def __next__(self):
        if self._start == self._end:
            if self._bar is not None:
                self._bar.close()
            raise StopIteration

        dataset = self._fn_dataset()

        if self._bar is None:
            self._num_batch = len(dataset)
            if self._writer is not None:
                total = (self._end - self._start) * self._num_batch
                self._bar = tqdm(total=total)

        epoch = Epoch(self._writer, self._head, self._num_batch, self._start)
        self._start += 1

        return epoch, EpochIter(iter(dataset), self._bar)
