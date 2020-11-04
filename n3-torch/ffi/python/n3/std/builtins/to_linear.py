from n3 import ExternNode


class ToLinear(ExternNode):
    def forward(self, x):
        return x.reshape(x.size(0), -1)
