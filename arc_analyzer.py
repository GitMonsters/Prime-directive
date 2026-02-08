#!/usr/bin/env python3
"""
ARC Dataset Analyzer
Analyzes the first 50 ARC training tasks to understand patterns, classify task types,
and map to physics domains for the compounding integration solver.

Week 1, Days 1-7 of ARC Prize execution.
"""

import json
import os
from collections import defaultdict
import numpy as np

class ARCAnalyzer:
    """Analyze ARC tasks to understand patterns and task types"""

    def __init__(self, arc_dir='ARC'):
        self.arc_dir = arc_dir
        self.tasks = {}
        self.analyses = {}
        self.categories = defaultdict(list)

    def load_tasks(self, num_tasks=50):
        """Load first N tasks from training set"""
        training_dir = os.path.join(self.arc_dir, 'data', 'training')
        task_files = sorted(os.listdir(training_dir))[:num_tasks]

        print(f"Loading {len(task_files)} tasks...")
        for i, task_file in enumerate(task_files):
            if i % 10 == 0:
                print(f"  Progress: {i}/{len(task_files)}")

            task_path = os.path.join(training_dir, task_file)
            with open(task_path, 'r') as f:
                self.tasks[task_file.replace('.json', '')] = json.load(f)

        print(f"✓ Loaded {len(self.tasks)} tasks")

    def analyze_grid(self, grid):
        """Extract properties of a grid"""
        try:
            grid_array = np.array(grid, dtype=int)
            return {
                'shape': grid_array.shape,
                'colors': sorted(list(set(grid_array.flatten()))),
                'color_counts': {int(k): int(v) for k, v in
                               zip(*np.unique(grid_array.flatten(), return_counts=True))},
                'max_color': int(np.max(grid_array)),
                'min_color': int(np.min(grid_array)),
                'num_unique_colors': len(set(grid_array.flatten())),
            }
        except Exception as e:
            return {'error': str(e)}

    def detect_rotation(self, inp, out):
        """Check if output is rotation of input"""
        inp_arr = np.array(inp, dtype=int)
        out_arr = np.array(out, dtype=int)

        for k in [1, 2, 3]:
            if np.array_equal(np.rot90(inp_arr, k), out_arr):
                return k * 90
        return None

    def detect_reflection(self, inp, out):
        """Check if output is reflection of input"""
        inp_arr = np.array(inp, dtype=int)
        out_arr = np.array(out, dtype=int)

        if np.array_equal(np.fliplr(inp_arr), out_arr):
            return 'horizontal'
        if np.array_equal(np.flipud(inp_arr), out_arr):
            return 'vertical'
        if np.array_equal(np.fliplr(np.flipud(inp_arr)), out_arr):
            return 'both'
        return None

    def analyze_task(self, task_name):
        """Analyze a single task"""
        task = self.tasks[task_name]

        info = {
            'name': task_name,
            'num_train_examples': len(task['train']),
            'num_test_examples': len(task['test']),
            'train_examples': [],
            'transformations': [],
            'task_type_scores': {},
        }

        # Analyze each training example
        for example in task['train']:
            input_grid = example['input']
            output_grid = example['output']

            inp_props = self.analyze_grid(input_grid)
            out_props = self.analyze_grid(output_grid)

            # Detect transformations
            rotation = self.detect_rotation(input_grid, output_grid)
            reflection = self.detect_reflection(input_grid, output_grid)

            example_info = {
                'input_shape': inp_props.get('shape'),
                'output_shape': out_props.get('shape'),
                'input_colors': inp_props.get('colors'),
                'output_colors': out_props.get('colors'),
                'rotation': rotation,
                'reflection': reflection,
                'shape_changed': inp_props.get('shape') != out_props.get('shape'),
                'colors_changed': inp_props.get('colors') != out_props.get('colors'),
            }

            info['train_examples'].append(example_info)
            info['transformations'].append({
                'rotation': rotation,
                'reflection': reflection,
                'shape_changed': example_info['shape_changed'],
                'colors_changed': example_info['colors_changed'],
            })

        return info

    def classify_tasks(self):
        """Classify tasks into 5 main types based on transformations"""

        classifications = {}

        for task_name, analysis in self.analyses.items():
            scores = {
                'grid_transformation': 0,
                'pattern_completion': 0,
                'object_manipulation': 0,
                'logic_rules': 0,
                'spatial_reasoning': 0,
            }

            # Analyze transformations to score task types
            for transform in analysis['transformations']:
                # Grid transformation (rotation, reflection, scaling)
                if transform['rotation'] or transform['reflection']:
                    scores['grid_transformation'] += 2

                # Pattern completion (shape changes but colors consistent)
                if transform['shape_changed'] and not transform['colors_changed']:
                    scores['pattern_completion'] += 2

                # Logic rules (colors change)
                if transform['colors_changed']:
                    scores['logic_rules'] += 1

                # Spatial reasoning (complex changes)
                if transform['shape_changed'] and transform['colors_changed']:
                    scores['spatial_reasoning'] += 1

            # Normalize scores
            total = sum(scores.values())
            if total > 0:
                scores = {k: v / total for k, v in scores.items()}
            else:
                # Default for unclassified
                scores = {k: 0.2 for k in scores}

            dominant_type = max(scores, key=scores.get)
            classifications[task_name] = {
                'type': dominant_type,
                'scores': scores,
                'confidence': scores[dominant_type],
            }

        return classifications

    def map_domains(self):
        """Map which physics domains are useful for each task type"""

        domain_task_mapping = {
            'geometry': ['grid_transformation', 'spatial_reasoning'],
            'symmetry': ['pattern_completion', 'grid_transformation'],
            'logic': ['logic_rules', 'pattern_completion'],
            'classical_mechanics': ['object_manipulation', 'grid_transformation'],
            'topology': ['spatial_reasoning', 'object_manipulation'],
            'counting': ['logic_rules', 'object_manipulation'],
            'wave_phenomena': ['pattern_completion', 'spatial_reasoning'],
            'thermodynamics': ['logic_rules', 'pattern_completion'],
            'electromagnetism': ['grid_transformation', 'spatial_reasoning'],
            'quantum_mechanics': ['pattern_completion', 'logic_rules'],
            'relativity': ['grid_transformation', 'symmetry'],
            'optics': ['pattern_completion', 'spatial_reasoning'],
            'acoustics': ['pattern_completion', 'wave_phenomena'],
            'fluid_dynamics': ['object_manipulation', 'spatial_reasoning'],
            'gravitation': ['object_manipulation', 'classical_mechanics'],
        }

        domain_recommendations = {}

        for task_name, classification in self.classifications.items():
            task_type = classification['type']

            # Score each domain for this task
            domain_scores = {}
            for domain, useful_types in domain_task_mapping.items():
                if task_type in useful_types:
                    domain_scores[domain] = 1.0
                else:
                    domain_scores[domain] = 0.3

            # Rank domains by usefulness
            ranked = sorted(domain_scores.items(), key=lambda x: -x[1])
            domain_recommendations[task_name] = {
                'top_domains': ranked[:5],
                'all_domains': ranked,
            }

        return domain_recommendations

    def generate_summary(self):
        """Generate summary statistics"""

        # Task type distribution
        type_counts = defaultdict(int)
        for classification in self.classifications.values():
            type_counts[classification['type']] += 1

        # Transformation statistics
        transform_counts = defaultdict(int)
        for analysis in self.analyses.values():
            for transform in analysis['transformations']:
                if transform['rotation']:
                    transform_counts['rotation'] += 1
                if transform['reflection']:
                    transform_counts['reflection'] += 1
                if transform['shape_changed']:
                    transform_counts['shape_change'] += 1
                if transform['colors_changed']:
                    transform_counts['color_change'] += 1

        # Grid size statistics
        sizes = defaultdict(int)
        for analysis in self.analyses.values():
            for example in analysis['train_examples']:
                if example['input_shape']:
                    sizes[example['input_shape']] += 1

        summary = {
            'total_tasks_analyzed': len(self.analyses),
            'task_type_distribution': dict(type_counts),
            'transformation_frequency': dict(transform_counts),
            'grid_sizes': {str(k): v for k, v in sorted(sizes.items(), key=lambda x: -x[1])[:10]},
            'top_5_domains': self._get_top_domains(),
        }

        return summary

    def _get_top_domains(self):
        """Calculate which domains are most useful overall"""
        domain_usefulness = defaultdict(float)

        for task_name, recommendations in self.domain_recommendations.items():
            # Score domains based on rank (top 1 = 1.0, top 2 = 0.9, etc.)
            for i, (domain, score) in enumerate(recommendations['top_domains']):
                domain_usefulness[domain] += (1.0 - i * 0.1)

        return sorted(domain_usefulness.items(), key=lambda x: -x[1])

    def run_analysis(self, num_tasks=50):
        """Run complete analysis pipeline"""
        print("\n" + "="*60)
        print("ARC DATASET ANALYSIS - PIPELINE")
        print("="*60)

        # Step 1: Load tasks
        self.load_tasks(num_tasks)

        # Step 2: Analyze each task
        print(f"\nAnalyzing {len(self.tasks)} tasks in detail...")
        for i, task_name in enumerate(self.tasks.keys()):
            if i % 10 == 0:
                print(f"  Progress: {i}/{len(self.tasks)}")
            self.analyses[task_name] = self.analyze_task(task_name)
        print(f"✓ Analysis complete")

        # Step 3: Classify tasks by type
        print("\nClassifying tasks by type...")
        self.classifications = self.classify_tasks()
        print(f"✓ Classification complete")

        # Step 4: Map to domains
        print("\nMapping to physics domains...")
        self.domain_recommendations = self.map_domains()
        print(f"✓ Domain mapping complete")

        # Step 5: Generate summary
        print("\nGenerating summary statistics...")
        summary = self.generate_summary()

        # Display results
        self._display_results(summary)

        # Save results to files
        self._save_results(summary)

        return summary

    def _display_results(self, summary):
        """Display analysis results to console"""
        print("\n" + "="*60)
        print("ANALYSIS RESULTS")
        print("="*60)

        print("\n1. TASK TYPE DISTRIBUTION:")
        for task_type, count in sorted(summary['task_type_distribution'].items(), key=lambda x: -x[1]):
            pct = 100 * count / summary['total_tasks_analyzed']
            print(f"   {task_type:30s}: {count:3d} ({pct:5.1f}%)")

        print("\n2. TRANSFORMATION PATTERNS:")
        for transform, count in sorted(summary['transformation_frequency'].items(), key=lambda x: -x[1]):
            print(f"   {transform:20s}: {count:3d} occurrences")

        print("\n3. GRID SIZE STATISTICS:")
        for size, count in list(summary['grid_sizes'].items())[:5]:
            print(f"   {size:20s}: {count:3d} grids")

        print("\n4. TOP 10 USEFUL PHYSICS DOMAINS:")
        for i, (domain, score) in enumerate(summary['top_5_domains'][:10], 1):
            print(f"   {i:2d}. {domain:30s}: {score:6.2f}")

        print("\n" + "="*60)

    def _save_results(self, summary):
        """Save analysis results to JSON and text files"""

        # Save detailed analysis as JSON
        detailed_results = {
            'tasks': self.analyses,
            'classifications': self.classifications,
            'domain_recommendations': self.domain_recommendations,
            'summary': summary,
        }

        with open('arc_analysis_50.json', 'w') as f:
            json.dump(detailed_results, f, indent=2, default=str)
        print(f"\n✓ Saved detailed analysis to: arc_analysis_50.json")

        # Save summary as text file
        with open('arc_summary_50.txt', 'w') as f:
            f.write("ARC DATASET ANALYSIS SUMMARY (50 TASKS)\n")
            f.write("="*60 + "\n\n")

            f.write("1. TASK TYPE DISTRIBUTION:\n")
            for task_type, count in sorted(summary['task_type_distribution'].items(), key=lambda x: -x[1]):
                pct = 100 * count / summary['total_tasks_analyzed']
                f.write(f"   {task_type:30s}: {count:3d} ({pct:5.1f}%)\n")

            f.write("\n2. TRANSFORMATION PATTERNS:\n")
            for transform, count in sorted(summary['transformation_frequency'].items(), key=lambda x: -x[1]):
                f.write(f"   {transform:20s}: {count:3d}\n")

            f.write("\n3. TOP PHYSICS DOMAINS FOR ARC:\n")
            for i, (domain, score) in enumerate(summary['top_5_domains'][:10], 1):
                f.write(f"   {i:2d}. {domain:30s}: {score:6.2f}\n")

            f.write("\n" + "="*60 + "\n")
            f.write("Analysis complete. Ready for Week 2: Architecture Design\n")

        print(f"✓ Saved summary to: arc_summary_50.txt")


if __name__ == '__main__':
    analyzer = ARCAnalyzer(arc_dir='ARC')
    summary = analyzer.run_analysis(num_tasks=50)
