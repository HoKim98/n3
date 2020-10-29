from n3_torch_ffi import ExternNode


class AssertShape(ExternNode):
    def forward(self, **kwargs):
        return kwargs
