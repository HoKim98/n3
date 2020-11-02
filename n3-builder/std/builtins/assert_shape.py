from n3 import ExternNode


class AssertShape(ExternNode):
    def forward(self, **kwargs):
        return kwargs
