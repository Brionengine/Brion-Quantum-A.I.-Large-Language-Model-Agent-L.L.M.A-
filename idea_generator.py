import random


class IdeaEngine:
    """Produces simple idea variations from input data."""

    def __init__(self):
        """Set up the random seed to ensure deterministic ideas."""
        random.seed(0)

    def generate(self, input_data: str):
        """Return a list containing the input and a shuffled variant."""
        chars = list(input_data)
        random.shuffle(chars)
        return [input_data, "".join(chars)]
