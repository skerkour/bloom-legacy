import 'dart:math';
import 'point.dart';

List<List<int>> blankGrid() {
  final List<List<int>> rows = <List<int>>[];
  for (int i = 0; i < 4; i++) {
    rows.add(<int>[0, 0, 0, 0]);
  }
  return rows;
}

bool compare(List<List<int>> a, List<List<int>> b) {
  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 4; j++) {
      if (a[i][j] != b[i][j]) {
        return false;
      }
    }
  }
  return true;
}

List<List<int>> copyGrid(List<List<int>> grid) {
  final List<List<int>> extraGrid = blankGrid();
  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 4; j++) {
      extraGrid[i][j] = grid[i][j];
    }
  }
  return extraGrid;
}

List<List<int>> flipGrid(List<List<int>> grid) {
  for (int i = 0; i < 4; i++) {
    final List<int> row = grid[i];
    grid[i] = row.reversed.toList();
  }
  return grid;
}

List<List<int>> transposeGrid(List<List<int>> grid) {
  final List<List<int>> newGrid = blankGrid();
  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 4; j++) {
      newGrid[i][j] = grid[j][i];
    }
  }
  return newGrid;
}

List<List<int>> addNumber(List<List<int>> grid, List<List<int>> gridNew) {
  final List<Point> options = <Point>[];
  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 4; j++) {
      if (grid[i][j] == 0) {
        options.add(Point(i, j));
      }
    }
  }
  if (options.isNotEmpty) {
    final int spotRandomIndex = Random().nextInt(options.length);
    final Point spot = options[spotRandomIndex];
    final int r = Random().nextInt(100);
    grid[spot.x][spot.y] = r > 50 ? 4 : 2;
    gridNew[spot.x][spot.y] = 1;
  }

  return grid;
}
