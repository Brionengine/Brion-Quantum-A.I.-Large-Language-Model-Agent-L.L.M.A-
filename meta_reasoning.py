class MetaReasoner:
    """Toy meta reasoning module that slightly rewrites ideas."""

    def __init__(self):
        """Initialize with an internal history of refinements."""
        self.history = []

    def refine(self, idea: str) -> str:
        """Return a refined idea by appending a marker and storing it."""
        refined = f"{idea}-refined"
        self.history.append(refined)
        return refined
