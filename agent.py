class QuantumAgent:
    """Simple agent that evaluates textual goals stored in memory."""

    def __init__(self, memory):
        """Create the agent with a reference to a memory list."""
        self.memory = memory

    def evaluate_goals(self):
        """Return goals from memory that contain the word 'goal'."""
        return [item for item in self.memory if "goal" in str(item).lower()]
