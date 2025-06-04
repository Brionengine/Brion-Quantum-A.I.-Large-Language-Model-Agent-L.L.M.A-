class QuantumMemory:
    """Simple list-based memory structure used by the agent."""

    def __init__(self):
        self.memories = []

    def store(self, memory):
        """Persist a memory item."""
        self.memories.append(memory)
