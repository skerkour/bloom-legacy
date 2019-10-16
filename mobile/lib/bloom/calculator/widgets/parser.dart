import 'dart:collection';

List<String> digits = <String>['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
List<String> operators = <String>['+', '-', '*', '/'];

class Parser {
  const Parser();

  num _eval(num op1, num op2, String op) {
    switch (op) {
      case '+':
        return op1 + op2;
      case '-':
        return op1 - op2;
      case '*':
        return op1 * op2;
      case '/':
        return op1 / op2;
      default:
        return 0;
    }
  }

  int _getPriority(String op) {
    switch (op) {
      case '+':
      case '-':
        return 0;
      case '*':
      case '/':
        return 1;
      default:
        return -1;
    }
  }

  bool _isOperator(String op) {
    return operators.contains(op);
  }

  bool _isDigit(String op) {
    return digits.contains(op);
  }

  num parseExpression(String expr) {
    final Queue<String> operators = ListQueue<String>();
    final Queue<num> operands = ListQueue<num>();

    // True if the last character was a digit
    // to accept numbers with more digits
    bool lastDig = true;

    // INIT
    operands.addLast(0);

    // ignore: avoid_function_literals_in_foreach_calls
    expr.split('').forEach((String c) {
      if (_isDigit(c)) {
        if (lastDig) {
          final num last = operands.removeLast();
          operands.addLast(last * 10 + int.parse(c));
        } else
          operands.addLast(int.parse(c));
      } else if (_isOperator(c)) {
        if (!lastDig) {
          throw ArgumentError('Illegal expression');
        }

        if (operators.isEmpty)
          operators.addLast(c);
        else {
          while (operators.isNotEmpty &&
              operands.isNotEmpty &&
              _getPriority(c) <= _getPriority(operators.last)) {
            final num op1 = operands.removeLast();
            final num op2 = operands.removeLast();
            final String op = operators.removeLast();

            // op1 and op2 in reverse order!
            final num res = _eval(op2, op1, op);
            operands.addLast(res);
          }
          operators.addLast(c);
        }
      }
      lastDig = _isDigit(c);
    });

    while (operators.isNotEmpty) {
      final num op1 = operands.removeLast();
      final num op2 = operands.removeLast();
      final String op = operators.removeLast();

      // op1 and op2 in reverse order!
      final num res = _eval(op2, op1, op);
      operands.addLast(res);
    }

    return operands.removeLast();
  }
}
