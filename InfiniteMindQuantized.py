import time
import hashlib
from typing import List, Dict, Any, Optional
from collections import Counter


class InfiniteMind:
    """
    Brion Quantum InfiniteMind v2.0

    Quantum-inspired thought expansion engine with:
    - Semantic clustering and associative recall
    - Thought evolution through recursive refinement
    - Knowledge graph building from accumulated thoughts
    - Novelty detection and creative idea generation
    - Quantized memory with importance weighting
    """

    VERSION = "2.0.0"
    MAX_THOUGHT_DEPTH = 10

    def __init__(self, capacity: int = 10000):
        """Initialize thought storage with optional capacity limit."""
        self.thoughts: List[Dict[str, Any]] = []
        self.raw_thoughts: List[str] = []
        self.associations: Dict[str, List[str]] = {}
        self.importance_scores: Dict[str, float] = {}
        self.thought_frequency: Counter = Counter()
        self.evolution_history: List[Dict[str, Any]] = []
        self.capacity = capacity
        self._creation_time = time.time()

    def expand(self, thoughts, importance: float = 1.0):
        """
        Add thoughts to memory with importance weighting.
        Returns all collected thoughts.
        """
        if not isinstance(thoughts, list):
            thoughts = [thoughts]

        for thought in thoughts:
            thought_str = str(thought)
            self.raw_thoughts.append(thought_str)

            # Create structured thought entry
            entry = {
                'content': thought_str,
                'importance': importance,
                'timestamp': time.time(),
                'id': hashlib.md5(thought_str.encode()).hexdigest()[:12],
                'depth': 0,
                'associations': [],
            }
            self.thoughts.append(entry)
            self.importance_scores[thought_str] = importance
            self.thought_frequency[thought_str] += 1

            # Build associations with existing thoughts
            self._build_associations(entry)

            # Evict least important if over capacity
            if len(self.thoughts) > self.capacity:
                self._evict_least_important()

        return self.raw_thoughts

    def _build_associations(self, entry: Dict[str, Any]):
        """Build word-level associations between thoughts."""
        words = set(entry['content'].lower().split())
        content_key = entry['content']
        self.associations.setdefault(content_key, [])

        for existing in self.thoughts[:-1]:
            existing_words = set(existing['content'].lower().split())
            overlap = words & existing_words
            if overlap:
                strength = len(overlap) / max(len(words | existing_words), 1)
                if strength > 0.1:
                    self.associations[content_key].append(existing['content'])
                    self.associations.setdefault(existing['content'], []).append(content_key)
                    entry['associations'].append(existing['id'])

    def _evict_least_important(self):
        """Remove lowest-importance thought when over capacity."""
        if not self.thoughts:
            return
        min_idx = min(range(len(self.thoughts)), key=lambda i: self.thoughts[i]['importance'])
        evicted = self.thoughts.pop(min_idx)
        if evicted['content'] in self.raw_thoughts:
            self.raw_thoughts.remove(evicted['content'])

    def recall(self, query: str, top_k: int = 5) -> List[Dict[str, Any]]:
        """
        Associative recall: find thoughts most related to query.
        Uses word overlap scoring with importance weighting.
        """
        query_words = set(query.lower().split())
        scored = []
        for thought in self.thoughts:
            thought_words = set(thought['content'].lower().split())
            overlap = len(query_words & thought_words)
            if overlap > 0:
                score = overlap * thought['importance']
                scored.append({'thought': thought, 'relevance': score})
        scored.sort(key=lambda x: x['relevance'], reverse=True)
        return scored[:top_k]

    def evolve_thought(self, thought: str, depth: int = 0) -> str:
        """
        Recursively evolve a thought by combining it with
        its most relevant associations. Brion Quantum Recursive
        Thought Evolution (BQRTE) algorithm.
        """
        if depth >= self.MAX_THOUGHT_DEPTH:
            return thought

        related = self.recall(thought, top_k=3)
        if not related:
            return thought

        # Combine with top association
        top_related = related[0]['thought']['content']
        # Merge unique words (set union preserving order)
        words_a = thought.split()
        words_b = top_related.split()
        seen = set()
        evolved_words = []
        for w in words_a + words_b:
            if w.lower() not in seen:
                seen.add(w.lower())
                evolved_words.append(w)

        evolved = ' '.join(evolved_words)
        self.evolution_history.append({
            'original': thought,
            'evolved': evolved,
            'depth': depth,
            'timestamp': time.time(),
        })
        return evolved

    def detect_novelty(self, thought: str) -> float:
        """
        Score how novel a thought is (0.0 = seen before, 1.0 = completely new).
        """
        if not self.thoughts:
            return 1.0
        related = self.recall(thought, top_k=1)
        if not related:
            return 1.0
        return max(0.0, 1.0 - related[0]['relevance'])

    def get_knowledge_summary(self) -> Dict[str, Any]:
        """Return summary of accumulated knowledge."""
        total = len(self.thoughts)
        avg_importance = sum(t['importance'] for t in self.thoughts) / max(total, 1)
        most_connected = max(
            self.associations.items(),
            key=lambda x: len(x[1]),
            default=('none', [])
        )
        return {
            'version': self.VERSION,
            'total_thoughts': total,
            'unique_thoughts': len(set(self.raw_thoughts)),
            'avg_importance': round(avg_importance, 4),
            'total_associations': sum(len(v) for v in self.associations.values()),
            'most_connected_thought': most_connected[0][:80],
            'most_connected_count': len(most_connected[1]),
            'evolution_steps': len(self.evolution_history),
            'uptime_seconds': round(time.time() - self._creation_time, 2),
        }
