class ConfidenceEvaluator:
    """Provides naive confidence scores for generated ideas."""

    def __init__(self, baseline: float = 0.9):
        """Store a baseline confidence used for scoring."""
        self.baseline = baseline

    def evaluate(self, idea: str) -> float:
        """Return the baseline adjusted by idea length."""
        length_factor = min(len(idea) / 100.0, 1.0)
        return self.baseline * (0.5 + 0.5 * length_factor)
