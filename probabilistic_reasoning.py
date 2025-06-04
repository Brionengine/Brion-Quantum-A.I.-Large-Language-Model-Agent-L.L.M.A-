class ProbabilisticEngine:
    """Calculates naive probabilities for ideas."""

    def __init__(self, base: float = 0.8):
        """Store a base probability used as starting point."""
        self.base = base

    def score(self, idea: str) -> float:
        """Return a probability influenced by the number of words."""
        words = idea.split()
        modifier = min(len(words) / 10.0, 1.0)
        return self.base * (0.5 + 0.5 * modifier)
