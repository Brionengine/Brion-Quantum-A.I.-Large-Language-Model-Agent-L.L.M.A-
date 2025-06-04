class InfiniteMind:
    """Stores and expands thoughts in a trivial way."""

    def __init__(self):
        """Initialize an internal list of thoughts."""
        self.thoughts = []

    def expand(self, thoughts):
        """Add thoughts to memory and return all collected thoughts."""
        self.thoughts.extend(thoughts if isinstance(thoughts, list) else [thoughts])
        return self.thoughts
