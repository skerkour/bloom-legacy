List<dynamic> operate(List<int> row, int score) {
  row = slide(row);
  final List<dynamic> result = combine(row, score);
  final int sc = result[0];
  row = result[1];
  row = slide(row);

  print('from func $sc');
  return <dynamic>[sc, row];
}

List<int> filter(List<int> row) {
  final List<int> temp = <int>[];
  for (int i = 0; i < row.length; i++) {
    if (row[i] != 0) {
      temp.add(row[i]);
    }
  }
  return temp;
}

List<int> slide(List<int> row) {
  List<int> arr = filter(row);
  final int missing = 4 - arr.length;
  final List<int> zeroes = zeroArray(missing);
  arr = zeroes + arr;
  return arr;
}

List<int> zeroArray(int length) {
  final List<int> zeroes = <int>[];
  for (int i = 0; i < length; i++) {
    zeroes.add(0);
  }
  return zeroes;
}

List<dynamic> combine(List<int> row, int score) {
  for (int i = 3; i >= 1; i--) {
    final int a = row[i];
    final int b = row[i - 1];
    if (a == b) {
      row[i] = a + b;
      score += row[i];
      row[i - 1] = 0;
    }
  }
  return <dynamic>[score, row];
}

bool isGameWon(List<List<int>> grid) {
  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 4; j++) {
      if (grid[i][j] == 2048) {
        return true;
      }
    }
  }
  return false;
}

bool isGameOver(List<List<int>> grid) {
  for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 4; j++) {
      if (grid[i][j] == 0) {
        return false;
      }
      if (i != 3 && grid[i][j] == grid[i + 1][j]) {
        return false;
      }
      if (j != 3 && grid[i][j] == grid[i][j + 1]) {
        return false;
      }
    }
  }
  return true;
}
