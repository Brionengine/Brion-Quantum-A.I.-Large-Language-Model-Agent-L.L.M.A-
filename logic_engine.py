class LogicCore:
    """Very small validation engine for generated ideas."""

    def __init__(self, banned_words=None):
        """Store a list of banned words for validation."""
        self.banned_words = set(banned_words or [])

    def validate(self, idea: str) -> bool:
        """Return True if no banned words are present."""
        return not any(word in idea for word in self.banned_words)
