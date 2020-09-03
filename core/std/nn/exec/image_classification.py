from n3.builder import *


class ImageClassification(Trainer):
    def train(self):
        # Step 1. ready to train
        self.model.train()
        self.optimizer.initialize(self.model)

        # Step 2. start training
        for epoch in range(self.epoch):
            # Step 2-1. peek the IO
            for idx, (x, classes) in enumerate(self.data.get_train_dataset()):
                # Step 2-2. clean-up gradients
                self.optimizer.zero_grad()
                # Step 2-3. predict classses
                y_pred = self.model(x=x)['x']
                # Step 2-4. calculate difference (loss)
                loss = self.loss(x=y_pred, y=classes)['x']
                # Step 2-5. calculate gradients
                loss.backward()
                # Step 2-6. step
                self.optimizer.step()
                # Step 2-7. log
                # TODO: not implemented yet
                print(idx, loss, x.device)
                # raise NotImplementedError

            # Step 2-8. eval training
            # TODO: not implemented yet
            raise NotImplementedError

        # Step 3. clean up
        # TODO: not implemented yet
        raise NotImplementedError

    def eval(self):
        raise NotImplementedError
