class SubconsciousCore:
    """Lightweight module that subtly modifies ideas."""

    def __init__(self, prefix: str = "sub"):
        """Store a prefix used during enhancement."""
        self.prefix = prefix

    def enhance(self, idea: str) -> str:
        """Return the idea prefixed with the configured string."""
        return f"{self.prefix}-{idea}"
