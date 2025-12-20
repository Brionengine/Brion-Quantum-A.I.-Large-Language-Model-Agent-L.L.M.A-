// Change Evaluation System
// Assesses aesthetics and functionality of changes

use serde::{Deserialize, Serialize};
use crate::agents::version_control::Change;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub change_id: String,
    pub aesthetic_score: f64,
    pub functionality_score: f64,
    pub overall_score: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub should_keep: bool,
}

pub struct ChangeEvaluator {
    aesthetic_weights: HashMap<String, f64>,
    functionality_weights: HashMap<String, f64>,
    min_score_threshold: f64,
}

impl ChangeEvaluator {
    pub fn new() -> Self {
        let mut aesthetic_weights = HashMap::new();
        aesthetic_weights.insert("visual_harmony".to_string(), 0.25);
        aesthetic_weights.insert("color_scheme".to_string(), 0.20);
        aesthetic_weights.insert("typography".to_string(), 0.15);
        aesthetic_weights.insert("spacing".to_string(), 0.15);
        aesthetic_weights.insert("modern_design".to_string(), 0.25);

        let mut functionality_weights = HashMap::new();
        functionality_weights.insert("performance".to_string(), 0.30);
        functionality_weights.insert("accessibility".to_string(), 0.20);
        functionality_weights.insert("code_quality".to_string(), 0.25);
        functionality_weights.insert("user_experience".to_string(), 0.25);

        Self {
            aesthetic_weights,
            functionality_weights,
            min_score_threshold: 0.6, // Minimum score to keep changes
        }
    }

    pub fn evaluate_change(&self, change: &Change) -> EvaluationResult {
        let aesthetic_score = self.evaluate_aesthetics(change);
        let functionality_score = self.evaluate_functionality(change);
        let overall_score = (aesthetic_score * 0.4 + functionality_score * 0.6);
        
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Analyze issues
        if aesthetic_score < 0.5 {
            issues.push("Aesthetic quality below acceptable threshold".to_string());
            recommendations.push("Review color scheme and visual design".to_string());
        }

        if functionality_score < 0.5 {
            issues.push("Functionality concerns detected".to_string());
            recommendations.push("Review code quality and performance impact".to_string());
        }

        if overall_score < self.min_score_threshold {
            issues.push("Overall score below minimum threshold".to_string());
            recommendations.push("Consider rolling back this change".to_string());
        }

        // Check for specific patterns
        if change.after.len() > change.before.len() * 2 {
            issues.push("Significant size increase detected".to_string());
            recommendations.push("Consider optimization".to_string());
        }

        if change.after.contains("TODO") || change.after.contains("FIXME") {
            issues.push("Incomplete code detected".to_string());
            recommendations.push("Complete implementation before deployment".to_string());
        }

        EvaluationResult {
            change_id: change.id.clone(),
            aesthetic_score,
            functionality_score,
            overall_score,
            issues,
            recommendations,
            should_keep: overall_score >= self.min_score_threshold,
        }
    }

    fn evaluate_aesthetics(&self, change: &Change) -> f64 {
        let mut score = 0.5; // Base score

        // Analyze content for aesthetic indicators
        let content = &change.after.to_lowercase();

        // Check for modern CSS features
        if content.contains("var(--") || content.contains("rgba(") {
            score += 0.1;
        }

        // Check for responsive design
        if content.contains("@media") || content.contains("viewport") {
            score += 0.1;
        }

        // Check for animations/transitions
        if content.contains("transition") || content.contains("animation") {
            score += 0.1;
        }

        // Check for quantum theme consistency
        if content.contains("quantum") || content.contains("#00d4ff") {
            score += 0.1;
        }

        // Check for semantic HTML
        if content.contains("<section") || content.contains("<article") {
            score += 0.1;
        }

        score.min(1.0)
    }

    fn evaluate_functionality(&self, change: &Change) -> f64 {
        let mut score = 0.5; // Base score

        let content = &change.after;

        // Check for error handling
        if content.contains("try") || content.contains("catch") || content.contains("error") {
            score += 0.1;
        }

        // Check for async/await (modern JavaScript)
        if content.contains("async") || content.contains("await") {
            score += 0.1;
        }

        // Check for accessibility
        if content.contains("aria-") || content.contains("alt=") || content.contains("role=") {
            score += 0.15;
        }

        // Check for performance optimizations
        if content.contains("requestAnimationFrame") || content.contains("debounce") || content.contains("throttle") {
            score += 0.1;
        }

        // Check for security
        if content.contains("escapeHtml") || content.contains("sanitize") || !content.contains("innerHTML") {
            score += 0.1;
        }

        // Check for code quality (proper structure)
        if content.matches('{').count() == content.matches('}').count() {
            score += 0.05;
        }

        // Penalize for obvious issues
        if content.contains("console.log") && !content.contains("// debug") {
            score -= 0.05;
        }

        score.min(1.0).max(0.0)
    }

    pub fn compare_changes(&self, old_change: &Change, new_change: &Change) -> EvaluationResult {
        // Evaluate the new change in context of the old one
        let base_evaluation = self.evaluate_change(new_change);
        
        // Additional comparison logic
        let mut result = base_evaluation;
        
        // Check if new change is an improvement
        let old_eval = self.evaluate_change(old_change);
        if result.overall_score > old_eval.overall_score {
            result.recommendations.push("This change improves upon the previous version".to_string());
        } else if result.overall_score < old_eval.overall_score {
            result.issues.push("This change may be a regression".to_string());
            result.should_keep = false;
        }

        result
    }
}

impl Default for ChangeEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

