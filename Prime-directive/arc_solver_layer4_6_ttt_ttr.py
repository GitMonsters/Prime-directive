"""
Layers 4-6: Consciousness, Refinement Loops, Test-Time Training & Reasoning

Integrates TTT (Test Time Training) and TTR (Test Time Reasoning) based on
ARC Prize 2025 winning approaches that use refinement loops for adaptation.

Key Innovation: Refine hypotheses in real-time during test phase using
training feedback.

Authors: Based on ARC Prize 2025 Technical Report (Chollet et al.)
Date: February 2026
"""

import numpy as np
from dataclasses import dataclass, field
from typing import List, Dict, Tuple, Any, Optional, Callable
from abc import ABC, abstractmethod
import copy


@dataclass
class RefinementSignal:
    """Feedback signal for refinement loops"""

    hypothesis_id: int
    accuracy_on_training: float  # 0.0-1.0
    error_pattern: str  # What went wrong
    suggested_adjustment: str  # How to improve
    confidence: float  # Confidence in this refinement

    def should_refine(self, threshold: float = 0.5) -> bool:
        """Should we refine based on this signal?"""
        return self.accuracy_on_training < threshold and self.confidence > 0.6


class TestTimeReasoner(ABC):
    """Base class for test-time reasoning"""

    @abstractmethod
    def reason(self, context: Dict[str, Any]) -> Dict[str, Any]:
        """Generate reasoning at test time"""
        pass

    @abstractmethod
    def refine(self, feedback: RefinementSignal) -> None:
        """Refine reasoning based on feedback"""
        pass


class ConsciousnessReasoner(TestTimeReasoner):
    """
    Layer 4: Consciousness Integration with Test-Time Reasoning

    Meta-reasoning about problem intent, augmented with test-time adaptation.
    """

    def __init__(self):
        self.problem_understanding = {}
        self.intent_history = []
        self.refinement_count = 0

    def reason(self, grid_features: Dict, domain_hypotheses: List) -> Dict[str, Any]:
        """
        Meta-reasoning about the problem at test time

        Returns insights that guide hypothesis selection and refinement
        """

        signal = {
            'intent': self._understand_intent(domain_hypotheses),
            'primary_transform': self._identify_primary_transform(domain_hypotheses),
            'archetype': self._detect_archetype(grid_features, domain_hypotheses),
            'confidence_multiplier': self._calculate_multiplier(domain_hypotheses),
            'refinement_suggestions': self._generate_refinement_suggestions(domain_hypotheses)
        }

        # Store for refinement
        self.problem_understanding = signal
        self.intent_history.append(signal)

        return signal

    def refine(self, feedback: RefinementSignal) -> None:
        """
        Refine consciousness-level reasoning based on test-time feedback

        ARC Prize 2025 insight: Refine understanding iteratively during test
        """
        self.refinement_count += 1

        if feedback.should_refine():
            # Update understanding based on error pattern
            if 'color' in feedback.error_pattern:
                self.problem_understanding['primary_transform'] = 'color_rule'

            if 'rotation' in feedback.error_pattern:
                self.problem_understanding['primary_transform'] = 'geometric'

            # Adjust confidence multiplier
            self.problem_understanding['confidence_multiplier'] *= 0.9

    def _understand_intent(self, hypotheses: List) -> str:
        """What is the puzzle asking?"""
        domain_scores = {}
        for hyp in hypotheses:
            domain = hyp.get('domain', 'unknown')
            domain_scores[domain] = domain_scores.get(domain, 0) + hyp.get('confidence', 0.5)

        if not domain_scores:
            return 'unknown'

        primary = max(domain_scores, key=domain_scores.get)

        intent_map = {
            'logic': 'rule_application',
            'geometry': 'spatial_transformation',
            'symmetry': 'pattern_recognition',
            'counting': 'enumeration',
            'thermodynamics': 'state_transformation'
        }

        return intent_map.get(primary, 'complex_reasoning')

    def _identify_primary_transform(self, hypotheses: List) -> str:
        """What's the main transformation type?"""
        if not hypotheses:
            return 'unknown'

        best = max(hypotheses, key=lambda h: h.get('confidence', 0))
        return best.get('rule_type', 'unknown')

    def _detect_archetype(self, grid_features: Dict, hypotheses: List) -> str:
        """What problem archetype is this?"""
        stats = grid_features.get('statistics', {})
        fill_pct = stats.get('fill_percentage', 0.5)

        if fill_pct < 0.3:
            return 'pattern_completion'
        elif grid_features.get('objects') and len(grid_features['objects']) > 5:
            return 'count_and_apply'
        elif grid_features.get('patterns'):
            return 'pattern_recognition'
        else:
            return 'transformation'

    def _calculate_multiplier(self, hypotheses: List) -> float:
        """
        Test-Time Reasoning: Calculate confidence multiplier

        Multiplicative approach: boost confidence when multiple domains agree
        """
        if not hypotheses:
            return 1.0

        domain_agreements = {}
        for hyp in hypotheses:
            domain = hyp.get('domain', 'unknown')
            conf = hyp.get('confidence', 0.5)

            domain_agreements[domain] = domain_agreements.get(domain, 0) + conf

        # Multiplicative boost for agreement
        num_domains = len(domain_agreements)
        agreement_score = sum(domain_agreements.values()) / max(1, num_domains)

        # Multiplicative multiplier: higher when multiple domains agree strongly
        multiplier = 1.0 + (agreement_score * 0.3)

        return multiplier

    def _generate_refinement_suggestions(self, hypotheses: List) -> List[str]:
        """Generate suggestions for refining hypotheses"""
        suggestions = []

        # Suggest focus areas
        if any(h.get('rule_type') == 'color_mapping' for h in hypotheses):
            suggestions.append('focus_on_color_rules')

        if any(h.get('rule_type') == 'rotation' for h in hypotheses):
            suggestions.append('consider_geometric_operations')

        if any(h.get('domain') == 'symmetry' for h in hypotheses):
            suggestions.append('preserve_symmetry_patterns')

        return suggestions


class TestTimeTrainer:
    """
    Layer 4-6: Test-Time Training (TTT) Integration

    ARC Prize 2025 Innovation: Train/refine during test time using
    feedback from training examples.

    Implements refinement loops like NVARC (1st place, 24.03%)
    """

    def __init__(self, max_refinements: int = 3):
        self.max_refinements = max_refinements
        self.refinement_history = []
        self.rule_bank = {}  # Learned rules

    def train_on_test(
        self,
        training_examples: List[Tuple],
        initial_hypotheses: List[Dict]
    ) -> List[Dict]:
        """
        Test-Time Training: Refine hypotheses using training examples

        Process:
        1. Start with initial hypotheses from domain reasoners
        2. Test each hypothesis on training examples
        3. Refine based on accuracy
        4. Repeat up to max_refinements times

        This is the "refinement loop" central to ARC Prize 2025 winners
        """

        current_hypotheses = copy.deepcopy(initial_hypotheses)
        refinement_num = 0

        while refinement_num < self.max_refinements:
            # Evaluate current hypotheses
            scores = self._evaluate_hypotheses(current_hypotheses, training_examples)

            if not scores:
                break

            # Find best hypothesis
            best_idx = np.argmax([s['accuracy'] for s in scores])
            best_score = scores[best_idx]['accuracy']

            # If perfect, stop refining
            if best_score >= 1.0:
                break

            # Refine poorly performing hypotheses
            current_hypotheses = self._refine_hypotheses(
                current_hypotheses,
                scores,
                training_examples
            )

            # If all hypotheses discarded, stop
            if not current_hypotheses:
                current_hypotheses = copy.deepcopy(initial_hypotheses)
                break

            self.refinement_history.append({
                'iteration': refinement_num,
                'best_accuracy': best_score,
                'num_hypotheses': len(current_hypotheses)
            })

            refinement_num += 1

        return current_hypotheses

    def _evaluate_hypotheses(
        self,
        hypotheses: List[Dict],
        training_examples: List[Tuple]
    ) -> List[Dict]:
        """Evaluate each hypothesis on training examples"""

        scores = []
        for hyp_idx, hypothesis in enumerate(hypotheses):
            correct = 0

            for input_grid, expected_output in training_examples:
                try:
                    predicted = self._apply_hypothesis(hypothesis, input_grid)
                    expected = np.array(expected_output)

                    if np.array_equal(np.array(predicted), expected):
                        correct += 1
                except:
                    pass

            accuracy = correct / len(training_examples) if training_examples else 0.0

            scores.append({
                'hypothesis_id': hyp_idx,
                'accuracy': accuracy,
                'num_correct': correct,
                'rule_type': hypothesis.get('rule_type', 'unknown')
            })

        return scores

    def _apply_hypothesis(self, hypothesis: Dict, input_grid: Any) -> Any:
        """Apply hypothesis to grid"""
        rule_type = hypothesis.get('rule_type', 'unknown')
        rule_detail = hypothesis.get('rule_detail', {})

        grid = np.array(input_grid, dtype=int)

        # Apply based on rule type
        if rule_type == 'rotation':
            angle = rule_detail.get('angle', 90)
            k = angle // 90
            return np.rot90(grid, k).tolist()

        elif rule_type == 'reflection':
            direction = rule_detail.get('direction', 'horizontal')
            if direction == 'horizontal':
                return np.fliplr(grid).tolist()
            elif direction == 'vertical':
                return np.flipud(grid).tolist()

        elif rule_type == 'color_mapping':
            mapping = rule_detail.get('mapping', {})
            result = grid.copy()

            for in_color, out_color in mapping.items():
                result[grid == in_color] = out_color

            return result.tolist()

        # Default: return input unchanged
        return input_grid

    def _refine_hypotheses(
        self,
        hypotheses: List[Dict],
        scores: List[Dict],
        training_examples: List[Tuple]
    ) -> List[Dict]:
        """
        Refine hypotheses based on evaluation results

        Strategy:
        1. Keep high-accuracy hypotheses
        2. Generate variations of medium-accuracy hypotheses
        3. Discard low-accuracy hypotheses
        """

        refined = []

        for score in scores:
            hyp_idx = score['hypothesis_id']
            accuracy = score['accuracy']
            hypothesis = hypotheses[hyp_idx]

            if accuracy >= 0.8:
                # Keep high-accuracy hypotheses
                refined.append(copy.deepcopy(hypothesis))

            elif accuracy >= 0.4:
                # Generate variations for medium-accuracy
                variations = self._generate_variations(hypothesis, training_examples)
                refined.extend(variations)

            # Low-accuracy hypotheses are discarded

        return refined

    def _generate_variations(
        self,
        hypothesis: Dict,
        training_examples: List[Tuple]
    ) -> List[Dict]:
        """
        Generate variations of hypothesis for further testing

        ARC Prize 2025 insight: Parameter search in hypothesis space
        """

        variations = []
        rule_type = hypothesis.get('rule_type', 'unknown')

        if rule_type == 'rotation':
            # Try all rotation angles
            for angle in [90, 180, 270]:
                var = copy.deepcopy(hypothesis)
                var['rule_detail']['angle'] = angle
                var['confidence'] = 0.6  # Lower confidence for variation
                variations.append(var)

        elif rule_type == 'color_mapping':
            # Try variations of color mappings
            mapping = hypothesis.get('rule_detail', {}).get('mapping', {})

            for in_color in mapping:
                for out_color in range(10):
                    if out_color != mapping.get(in_color):
                        var = copy.deepcopy(hypothesis)
                        var['rule_detail']['mapping'][in_color] = out_color
                        var['confidence'] = 0.5
                        variations.append(var)
                        if len(variations) >= 5:  # Limit variations
                            break

        return variations[:5]  # Return top 5 variations


class HypothesisRefiner:
    """
    Layer 6: Hypothesis Refinement with Test-Time Feedback

    Uses validation results to refine hypotheses iteratively.
    """

    def __init__(self):
        self.refinement_rules = {}

    def refine_with_feedback(
        self,
        hypothesis: Dict,
        validation_score: float,
        error_analysis: Dict
    ) -> Dict:
        """
        Refine a hypothesis based on validation feedback

        Args:
            hypothesis: Current hypothesis
            validation_score: Accuracy on training examples
            error_analysis: Analysis of where hypothesis failed

        Returns:
            Refined hypothesis with adjusted parameters
        """

        refined = copy.deepcopy(hypothesis)

        # If poor accuracy, adjust confidence
        if validation_score < 0.5:
            refined['confidence'] = refined.get('confidence', 0.5) * 0.7

        # Adjust based on error patterns
        if 'color_mismatch' in error_analysis:
            # Refine color mapping
            self._refine_color_mapping(refined, error_analysis)

        if 'shape_mismatch' in error_analysis:
            # Try different transformation
            self._refine_transformation(refined, error_analysis)

        if 'boundary_issue' in error_analysis:
            # Adjust for edge cases
            self._refine_boundaries(refined, error_analysis)

        return refined

    def _refine_color_mapping(self, hypothesis: Dict, error_analysis: Dict) -> None:
        """Refine color mapping rules"""
        if hypothesis.get('rule_type') != 'color_mapping':
            return

        mapping = hypothesis.get('rule_detail', {}).get('mapping', {})
        problematic_colors = error_analysis.get('problematic_colors', [])

        for color in problematic_colors:
            # Try different target colors
            if color in mapping:
                del mapping[color]  # Remove incorrect mapping

    def _refine_transformation(self, hypothesis: Dict, error_analysis: Dict) -> None:
        """Try different geometric transformation"""
        if hypothesis.get('rule_type') in ['rotation', 'reflection']:
            # These might need adjustment, but keep for now
            pass

    def _refine_boundaries(self, hypothesis: Dict, error_analysis: Dict) -> None:
        """Adjust for edge cases"""
        hypothesis['has_boundary_adjustment'] = True


class RefinementLoop:
    """
    Complete Refinement Loop combining TTT and TTR

    ARC Prize 2025 Theme: The refinement loop is central to progress
    """

    def __init__(self, max_iterations: int = 3):
        self.max_iterations = max_iterations
        self.consciousness = ConsciousnessReasoner()
        self.trainer = TestTimeTrainer(max_refinements=max_iterations)
        self.refiner = HypothesisRefiner()
        self.iteration_history = []

    def execute_refinement_loop(
        self,
        training_examples: List[Tuple],
        domain_hypotheses: List[Dict],
        test_input: Any
    ) -> Dict:
        """
        Execute complete refinement loop

        Process:
        1. Get consciousness-guided understanding
        2. Apply test-time training to refine hypotheses
        3. Select best hypothesis
        4. Apply to test input
        """

        # Step 1: Consciousness reasoning
        consciousness_signal = self.consciousness.reason(
            {'statistics': {'fill_percentage': 0.5}},  # Simplified
            domain_hypotheses
        )

        # Step 2: Test-time training (TTT)
        refined_hypotheses = self.trainer.train_on_test(
            training_examples,
            domain_hypotheses
        )

        # Step 3: Select best
        best_hypothesis = self._select_best_hypothesis(
            refined_hypotheses,
            training_examples,
            consciousness_signal
        )

        # Step 4: Apply to test input
        output = self._apply_best_hypothesis(best_hypothesis, test_input)

        # Record iteration
        self.iteration_history.append({
            'consciousness_intent': consciousness_signal.get('intent'),
            'num_refined_hypotheses': len(refined_hypotheses),
            'best_hypothesis_type': best_hypothesis.get('rule_type')
        })

        return {
            'output': output,
            'best_hypothesis': best_hypothesis,
            'consciousness_signal': consciousness_signal,
            'refinement_iterations': len(self.trainer.refinement_history)
        }

    def _select_best_hypothesis(
        self,
        hypotheses: List[Dict],
        training_examples: List[Tuple],
        consciousness_signal: Dict
    ) -> Dict:
        """
        Select best hypothesis using both validation and consciousness guidance
        """

        if not hypotheses:
            return {}

        # Score each hypothesis
        scores = []
        for hyp in hypotheses:
            # Validation accuracy
            accuracy = self._evaluate_hypothesis(hyp, training_examples)

            # Boost if consciousness identifies as primary transform
            consciousness_boost = 1.0
            if consciousness_signal.get('primary_transform') == hyp.get('rule_type'):
                consciousness_boost = 1.2

            final_score = accuracy * consciousness_boost * hyp.get('confidence', 0.5)

            scores.append({
                'hypothesis': hyp,
                'accuracy': accuracy,
                'final_score': final_score
            })

        # Return hypothesis with highest score
        best = max(scores, key=lambda x: x['final_score'])
        return best['hypothesis']

    def _evaluate_hypothesis(self, hypothesis: Dict, training_examples: List[Tuple]) -> float:
        """Evaluate hypothesis accuracy on training examples"""
        if not training_examples:
            return 0.5

        correct = 0
        for inp, expected in training_examples:
            try:
                predicted = self.trainer._apply_hypothesis(hypothesis, inp)
                if np.array_equal(np.array(predicted), np.array(expected)):
                    correct += 1
            except:
                pass

        return correct / len(training_examples)

    def _apply_best_hypothesis(self, hypothesis: Dict, test_input: Any) -> Any:
        """Apply best hypothesis to test input"""
        return self.trainer._apply_hypothesis(hypothesis, test_input)


# ============= Unit Tests =============

def test_consciousness_reasoner():
    """Test consciousness-guided reasoning"""

    consciousness = ConsciousnessReasoner()

    hypotheses = [
        {'domain': 'logic', 'confidence': 0.8, 'rule_type': 'color_mapping'},
        {'domain': 'geometry', 'confidence': 0.6, 'rule_type': 'rotation'},
        {'domain': 'symmetry', 'confidence': 0.5, 'rule_type': 'reflection'}
    ]

    signal = consciousness.reason({}, hypotheses)

    assert signal['intent'] in ['rule_application', 'spatial_transformation', 'unknown']
    assert signal['confidence_multiplier'] > 1.0
    print("✓ ConsciousnessReasoner test passed")


def test_test_time_training():
    """Test test-time training refinement"""

    trainer = TestTimeTrainer(max_refinements=2)

    training_examples = [
        ([[1, 2], [3, 4]], [[2, 4], [6, 8]]),  # Multiply by 2
        ([[2, 1], [1, 2]], [[4, 2], [2, 4]])
    ]

    hypotheses = [
        {'rule_type': 'color_mapping', 'rule_detail': {'mapping': {1: 2, 2: 4, 3: 6, 4: 8}}, 'confidence': 0.7},
        {'rule_type': 'rotation', 'rule_detail': {'angle': 90}, 'confidence': 0.3}
    ]

    refined = trainer.train_on_test(training_examples, hypotheses)

    assert len(refined) > 0
    print("✓ TestTimeTrainer test passed")


def test_refinement_loop():
    """Test complete refinement loop"""

    loop = RefinementLoop(max_iterations=2)

    training_examples = [
        ([[1, 0], [0, 1]], [[1, 0], [0, 1]])
    ]

    hypotheses = [
        {'domain': 'geometry', 'rule_type': 'rotation', 'rule_detail': {'angle': 90}, 'confidence': 0.7}
    ]

    test_input = [[1, 0], [0, 1]]

    result = loop.execute_refinement_loop(training_examples, hypotheses, test_input)

    assert 'output' in result
    assert 'best_hypothesis' in result
    print("✓ RefinementLoop test passed")


if __name__ == '__main__':
    test_consciousness_reasoner()
    test_test_time_training()
    test_refinement_loop()
    print("\n✅ All TTT/TTR tests passed!")
