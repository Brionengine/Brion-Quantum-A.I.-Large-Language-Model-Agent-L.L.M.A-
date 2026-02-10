import time
import logging
from typing import List, Dict, Any, Optional
from collections import defaultdict

logger = logging.getLogger(__name__)


class QuantumAgent:
    """
    Brion Quantum Agent v2.0

    Autonomous agent with goal evaluation, task prioritization,
    adaptive learning, and quantum-inspired decision making.
    """

    VERSION = "2.0.0"

    def __init__(self, memory=None):
        """Create the agent with a reference to a memory list."""
        self.memory = memory or []
        self.goal_history: List[Dict[str, Any]] = []
        self.task_queue: List[Dict[str, Any]] = []
        self.performance_log: List[Dict[str, float]] = []
        self.learning_rate = 0.01
        self.priority_weights = defaultdict(lambda: 1.0)
        self._start_time = time.time()

    def evaluate_goals(self) -> List[str]:
        """Return goals from memory that contain the word 'goal'."""
        goals = [item for item in self.memory if "goal" in str(item).lower()]
        self.goal_history.append({
            'timestamp': time.time(),
            'goals_found': len(goals),
            'memory_size': len(self.memory),
        })
        return goals

    def prioritize_goals(self) -> List[Dict[str, Any]]:
        """
        Evaluate and prioritize all goals using weighted scoring.
        Returns goals sorted by priority score.
        """
        goals = self.evaluate_goals()
        prioritized = []
        for goal in goals:
            goal_str = str(goal).lower()
            score = self.priority_weights[goal_str]
            # Boost score for quantum-related and critical goals
            if 'quantum' in goal_str:
                score *= 1.5
            if 'critical' in goal_str or 'urgent' in goal_str:
                score *= 2.0
            if 'security' in goal_str:
                score *= 1.3
            prioritized.append({
                'goal': goal,
                'priority_score': score,
                'category': self._categorize_goal(goal_str),
            })
        prioritized.sort(key=lambda x: x['priority_score'], reverse=True)
        return prioritized

    def _categorize_goal(self, goal_text: str) -> str:
        """Categorize a goal based on keywords."""
        categories = {
            'quantum': ['quantum', 'qubit', 'entangle', 'superposition'],
            'security': ['security', 'encrypt', 'protect', 'defense'],
            'learning': ['learn', 'train', 'improve', 'optimize'],
            'integration': ['integrate', 'connect', 'merge', 'unify'],
            'deployment': ['deploy', 'scale', 'production', 'release'],
        }
        for category, keywords in categories.items():
            if any(kw in goal_text for kw in keywords):
                return category
        return 'general'

    def add_task(self, description: str, priority: float = 1.0, metadata: Optional[Dict] = None):
        """Add a task to the agent's task queue."""
        self.task_queue.append({
            'description': description,
            'priority': priority,
            'status': 'pending',
            'created_at': time.time(),
            'metadata': metadata or {},
        })

    def execute_next_task(self) -> Optional[Dict[str, Any]]:
        """Pop and return the highest-priority pending task."""
        pending = [t for t in self.task_queue if t['status'] == 'pending']
        if not pending:
            return None
        pending.sort(key=lambda t: t['priority'], reverse=True)
        task = pending[0]
        task['status'] = 'in_progress'
        task['started_at'] = time.time()
        return task

    def complete_task(self, task: Dict[str, Any], success: bool, reward: float = 0.0):
        """Mark a task as complete and record performance."""
        task['status'] = 'completed' if success else 'failed'
        task['completed_at'] = time.time()
        task['success'] = success
        duration = task['completed_at'] - task.get('started_at', task['created_at'])
        self.performance_log.append({
            'task': task['description'],
            'success': success,
            'reward': reward,
            'duration': duration,
        })
        # Adaptive learning: adjust priority weights
        if success:
            self.priority_weights[task['description']] *= (1.0 + self.learning_rate * reward)

    def remember(self, item: Any):
        """Add an item to memory."""
        self.memory.append(item)

    def recall(self, query: str) -> List[Any]:
        """Search memory for items matching a query string."""
        query_lower = query.lower()
        return [item for item in self.memory if query_lower in str(item).lower()]

    def get_status(self) -> Dict[str, Any]:
        """Return agent status report."""
        uptime = time.time() - self._start_time
        completed = [t for t in self.task_queue if t['status'] == 'completed']
        failed = [t for t in self.task_queue if t['status'] == 'failed']
        success_rate = len(completed) / max(len(completed) + len(failed), 1)
        return {
            'version': self.VERSION,
            'uptime_seconds': round(uptime, 2),
            'memory_size': len(self.memory),
            'goals_tracked': len(self.goal_history),
            'tasks_total': len(self.task_queue),
            'tasks_completed': len(completed),
            'tasks_failed': len(failed),
            'success_rate': round(success_rate, 4),
            'pending_tasks': len([t for t in self.task_queue if t['status'] == 'pending']),
        }
