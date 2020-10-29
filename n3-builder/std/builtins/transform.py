from n3_torch_ffi import ExternNode


class Transform(ExternNode):
    def forward(self, x):
        return x.reshape(-1, *self.output_shapes['x'].dims)
