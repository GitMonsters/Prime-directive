"""
Layer 1: GridAnalyzer

Extracts features from ARC input grids.
Input: Raw NxM grid (list of lists, colors 0-9)
Output: GridFeatures data structure with all extracted features

Author: ARC Prize Solver
Date: February 2026
"""

import numpy as np
from scipy import ndimage
from dataclasses import dataclass, field
from typing import List, Dict, Tuple, Any, Optional
from collections import defaultdict


@dataclass
class GridFeatures:
    """Extracted features from ARC grid"""

    shape: Tuple[int, int]  # (height, width)
    colors: Dict[str, Any] = field(default_factory=dict)
    objects: List[Dict[str, Any]] = field(default_factory=list)
    regions: List[Dict[str, Any]] = field(default_factory=list)
    patterns: List[Dict[str, Any]] = field(default_factory=list)
    statistics: Dict[str, float] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for passing to reasoners"""
        return {
            'shape': self.shape,
            'colors': self.colors,
            'objects': self.objects,
            'regions': self.regions,
            'patterns': self.patterns,
            'statistics': self.statistics
        }


class GridAnalyzer:
    """Extract features from ARC grids"""

    def analyze(self, grid: List[List[int]]) -> GridFeatures:
        """
        Analyze input grid and extract features

        Args:
            grid: List[List[int]] - NxM grid with colors 0-9

        Returns:
            GridFeatures object with all extracted features
        """
        try:
            grid_array = np.array(grid, dtype=int)
        except (ValueError, TypeError) as e:
            raise ValueError(f"Invalid grid format: {grid}") from e

        if grid_array.ndim != 2:
            raise ValueError(f"Grid must be 2D, got {grid_array.ndim}D")

        # Extract all features
        return GridFeatures(
            shape=tuple(grid_array.shape),
            colors=self._extract_colors(grid_array),
            objects=self._detect_objects(grid_array),
            regions=self._detect_regions(grid_array),
            patterns=self._detect_patterns(grid_array),
            statistics=self._calculate_statistics(grid_array)
        )

    def _extract_colors(self, grid: np.ndarray) -> Dict[str, Any]:
        """Extract color information from grid"""
        unique_colors = np.unique(grid)

        color_counts = {}
        for color in unique_colors:
            color_counts[int(color)] = int(np.sum(grid == color))

        return {
            'present': sorted([int(c) for c in unique_colors]),
            'counts': color_counts,
            'num_colors': len(unique_colors),
            'background_color': int(unique_colors[0]) if len(unique_colors) > 0 else 0,
            'foreground_colors': sorted([int(c) for c in unique_colors[1:]])
        }

    def _detect_objects(self, grid: np.ndarray) -> List[Dict[str, Any]]:
        """
        Detect connected components (objects) in grid

        Uses connected component labeling from scipy
        """
        # Create binary grid (anything non-zero is an object)
        binary = grid > 0

        if not np.any(binary):
            return []  # No objects

        # Label connected components
        labeled, num_objects = ndimage.label(binary)

        objects = []
        for i in range(1, num_objects + 1):
            mask = labeled == i
            coords = np.argwhere(mask)

            if len(coords) == 0:
                continue

            # Get object properties
            color = int(grid[coords[0, 0], coords[0, 1]])
            size = int(np.sum(mask))
            bbox = self._get_bbox(coords)
            centroid = self._get_centroid(coords)

            bbox_area = (bbox[0][1] - bbox[0][0] + 1) * (bbox[1][1] - bbox[1][0] + 1)

            objects.append({
                'id': i,
                'color': color,
                'size': size,
                'bbox': bbox,
                'centroid': centroid,
                'bounding_box_area': bbox_area,
                'fill_ratio': size / bbox_area if bbox_area > 0 else 0.0
            })

        return objects

    def _detect_regions(self, grid: np.ndarray) -> List[Dict[str, Any]]:
        """
        Detect rectangular regions of same color
        """
        regions = []

        # Get unique colors (excluding background)
        colors = np.unique(grid)
        for color in colors:
            if color == 0:
                continue  # Skip background

            # Find all cells of this color
            mask = grid == color
            coords = np.argwhere(mask)

            if len(coords) == 0:
                continue

            bbox = self._get_bbox(coords)
            region = {
                'color': int(color),
                'bbox': bbox,
                'size': int(np.sum(mask)),
                'is_rectangular': self._is_rectangular_region(grid, color, bbox)
            }
            regions.append(region)

        return regions

    def _detect_patterns(self, grid: np.ndarray) -> List[Dict[str, Any]]:
        """
        Detect repeating patterns in grid
        """
        patterns = []

        # Check for horizontal repetitions
        for row_idx in range(grid.shape[0]):
            row = grid[row_idx]
            h_pattern = self._find_repeating_sequence(row)
            if h_pattern:
                patterns.append({
                    'type': 'horizontal',
                    'row': row_idx,
                    'period': h_pattern['period'],
                    'pattern': h_pattern['pattern'].tolist()
                })

        # Check for vertical repetitions
        for col_idx in range(grid.shape[1]):
            col = grid[:, col_idx]
            v_pattern = self._find_repeating_sequence(col)
            if v_pattern:
                patterns.append({
                    'type': 'vertical',
                    'col': col_idx,
                    'period': v_pattern['period'],
                    'pattern': v_pattern['pattern'].tolist()
                })

        return patterns

    def _calculate_statistics(self, grid: np.ndarray) -> Dict[str, float]:
        """Calculate grid-level statistics"""
        total_cells = int(grid.size)
        filled_cells = int(np.sum(grid > 0))
        empty_cells = total_cells - filled_cells

        # Calculate color entropy
        unique, counts = np.unique(grid, return_counts=True)
        probabilities = counts / total_cells
        entropy = -np.sum(probabilities * np.log2(probabilities + 1e-10))

        return {
            'total_cells': total_cells,
            'filled_cells': filled_cells,
            'empty_cells': empty_cells,
            'fill_percentage': float(filled_cells / total_cells) if total_cells > 0 else 0.0,
            'color_entropy': float(entropy),
            'num_colors': len(unique),
            'grid_height': int(grid.shape[0]),
            'grid_width': int(grid.shape[1]),
            'grid_area': int(grid.shape[0] * grid.shape[1])
        }

    # ============= Helper Methods =============

    def _get_bbox(self, coords: np.ndarray) -> Tuple[Tuple[int, int], Tuple[int, int]]:
        """Get bounding box from coordinates"""
        y_coords = coords[:, 0]
        x_coords = coords[:, 1]

        return (
            (int(np.min(y_coords)), int(np.max(y_coords))),
            (int(np.min(x_coords)), int(np.max(x_coords)))
        )

    def _get_centroid(self, coords: np.ndarray) -> Tuple[float, float]:
        """Get centroid from coordinates"""
        y_center = float(np.mean(coords[:, 0]))
        x_center = float(np.mean(coords[:, 1]))
        return (y_center, x_center)

    def _is_rectangular_region(self, grid: np.ndarray, color: int, bbox: Tuple) -> bool:
        """Check if region is perfectly rectangular"""
        y_min, y_max = bbox[0]
        x_min, x_max = bbox[1]

        # Extract region
        region = grid[y_min:y_max+1, x_min:x_max+1]

        # Check if all cells are the target color
        return bool(np.all(region == color))

    def _find_repeating_sequence(self, sequence: np.ndarray) -> Optional[Dict[str, Any]]:
        """Find repeating pattern in 1D sequence"""
        if len(sequence) < 2:
            return None

        # Try different period lengths
        for period in range(1, len(sequence) // 2 + 1):
            pattern = sequence[:period]

            # Check if pattern repeats
            num_repeats = len(sequence) // period
            remainder = len(sequence) % period

            is_repeating = True
            for i in range(num_repeats):
                if not np.array_equal(sequence[i*period:(i+1)*period], pattern):
                    is_repeating = False
                    break

            # Check remainder
            if is_repeating and remainder > 0:
                if not np.array_equal(sequence[num_repeats*period:], pattern[:remainder]):
                    is_repeating = False

            if is_repeating:
                return {
                    'period': period,
                    'pattern': pattern,
                    'confidence': 1.0 if remainder == 0 else 0.8
                }

        return None

    def compare_grids(self, grid1: List[List[int]], grid2: List[List[int]]) -> Dict[str, Any]:
        """Compare two grids and extract transformation information"""
        g1 = np.array(grid1, dtype=int)
        g2 = np.array(grid2, dtype=int)

        return {
            'same_shape': g1.shape == g2.shape,
            'shape_change': (g1.shape, g2.shape),
            'color_diff': self._color_difference(g1, g2),
            'is_rotation': self._check_rotation(g1, g2),
            'is_reflection': self._check_reflection(g1, g2),
            'pixels_changed': int(np.sum(g1 != g2))
        }

    def _color_difference(self, g1: np.ndarray, g2: np.ndarray) -> Dict[int, int]:
        """Extract color mapping from g1 to g2"""
        if g1.shape != g2.shape:
            return {}

        mapping = defaultdict(lambda: defaultdict(int))

        for i in range(g1.shape[0]):
            for j in range(g1.shape[1]):
                in_color = int(g1[i, j])
                out_color = int(g2[i, j])
                mapping[in_color][out_color] += 1

        # Extract most common mapping
        result = {}
        for in_color, out_colors in mapping.items():
            most_common = max(out_colors, key=out_colors.get)
            result[in_color] = most_common

        return result

    def _check_rotation(self, g1: np.ndarray, g2: np.ndarray) -> Optional[int]:
        """Check if g2 is a rotation of g1"""
        for k in [1, 2, 3]:
            if np.array_equal(np.rot90(g1, k), g2):
                return k * 90
        return None

    def _check_reflection(self, g1: np.ndarray, g2: np.ndarray) -> Optional[str]:
        """Check if g2 is a reflection of g1"""
        if np.array_equal(np.fliplr(g1), g2):
            return 'horizontal'
        if np.array_equal(np.flipud(g1), g2):
            return 'vertical'
        if np.array_equal(np.fliplr(np.flipud(g1)), g2):
            return 'both'
        return None


# ============= Unit Tests =============

def test_grid_analyzer():
    """Basic tests for GridAnalyzer"""

    analyzer = GridAnalyzer()

    # Test 1: Simple 3x3 grid
    grid1 = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8]
    ]

    features1 = analyzer.analyze(grid1)
    assert features1.shape == (3, 3), f"Shape mismatch: {features1.shape}"
    assert len(features1.colors['present']) == 9, f"Color count: {len(features1.colors['present'])}"
    print("✓ Test 1 passed: Basic grid analysis")

    # Test 2: Grid with objects
    grid2 = [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0]
    ]

    features2 = analyzer.analyze(grid2)
    assert len(features2.objects) == 1, f"Expected 1 object, got {len(features2.objects)}"
    assert features2.objects[0]['size'] == 4, f"Object size: {features2.objects[0]['size']}"
    print("✓ Test 2 passed: Object detection")

    # Test 3: Repeating pattern
    grid3 = [
        [1, 2, 1, 2, 1]
    ]

    features3 = analyzer.analyze(grid3)
    patterns = [p for p in features3.patterns if p['type'] == 'horizontal']
    assert len(patterns) > 0, "No patterns detected"
    print("✓ Test 3 passed: Pattern detection")

    # Test 4: Grid comparison
    grid_a = [
        [1, 2],
        [3, 4]
    ]
    grid_b = [
        [4, 3],
        [2, 1]
    ]

    comparison = analyzer.compare_grids(grid_a, grid_b)
    assert comparison['pixels_changed'] == 4, f"Pixels changed: {comparison['pixels_changed']}"
    print("✓ Test 4 passed: Grid comparison")

    print("\n✅ All GridAnalyzer tests passed!")


if __name__ == '__main__':
    test_grid_analyzer()
