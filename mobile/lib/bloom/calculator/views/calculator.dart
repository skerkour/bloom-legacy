// import 'package:bloom/bloom/calculator/widgets/drawer.dart';
import 'package:bloom/bloom/calculator/widgets/parser.dart';
import 'package:flutter/material.dart';

class CalculatorView extends StatefulWidget {
  const CalculatorView();

  @override
  _CalculatorState createState() => _CalculatorState();
}

class _CalculatorState extends State<CalculatorView> {
  String _expression = '';
  String _result = '';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // endDrawer: const CalculatorDrawer(),
      appBar: AppBar(
        title: const Text('Calculator'),
        elevation: 0.0,
      ),
      body: Column(
        children: <Widget>[
          _buildDisplay(context),
          _buildKeyboard(context),
        ],
      ),
    );
  }

  Widget _buildDisplay(BuildContext context) {
    final List<Widget> views = <Widget>[
      Expanded(
          flex: 1,
          child: Row(
            children: <Widget>[
              Expanded(
                  child: Text(
                _expression,
                textAlign: TextAlign.right,
                style: const TextStyle(
                  fontSize: 40.0,
                  color: Colors.white,
                ),
              ))
            ],
          )),
    ];

    if (_result.isNotEmpty) {
      views.add(
        Expanded(
            flex: 1,
            child: Row(
              children: <Widget>[
                Expanded(
                    child: Text(
                  _result,
                  textAlign: TextAlign.right,
                  style: const TextStyle(
                    fontSize: 40.0,
                    color: Colors.white,
                  ),
                ))
              ],
            )),
      );
    }

    return Expanded(
        flex: 2,
        child: Container(
          color: Theme.of(context).primaryColor,
          padding: const EdgeInsets.all(16.0),
          child: Column(
            children: views,
          ),
        ));
  }

  void _addKey(String key) {
    String expr = _expression;
    String result = '';
    if (result.isNotEmpty) {
      expr = '';
      result = '';
    }

    if (operators.contains(key)) {
      // Handle as an operator
      if (expr.isNotEmpty && operators.contains(expr[expr.length - 1])) {
        expr = expr.substring(0, expr.length - 1);
      }
      expr += key;
    } else if (digits.contains(key)) {
      // Handle as an operand
      expr += key;
    } else if (key == 'C') {
      // Delete all character
      expr = '';
      // if (expr.isNotEmpty) {
      //   expr = expr.substring(0, expr.length - 1);
      // }
    } else if (key == '=') {
      try {
        const Parser parser = Parser();
        result = parser.parseExpression(expr).toString();
      } on Error {
        result = 'Error';
      }
    }
    setState(() {
      _expression = expr;
      _result = result;
    });
  }

  Widget _buildKeyboard(BuildContext context) {
    return Expanded(
        flex: 4,
        child: Center(
            child: AspectRatio(
          aspectRatio: 1.0, // To center the GridView
          child: GridView.count(
            crossAxisCount: 4,
            childAspectRatio: 1.0,
            padding: const EdgeInsets.all(4.0),
            mainAxisSpacing: 4.0,
            crossAxisSpacing: 4.0,
            children: <String>[
              // @formatter:off
              '7', '8', '9', '/',
              '4', '5', '6', '*',
              '1', '2', '3', '-',
              'C', '0', '=', '+',
              // @formatter:on
            ].map((String key) {
              return GridTile(
                child: _buildKeyboardKey(context, key),
              );
            }).toList(),
          ),
        )));
  }

  Widget _buildKeyboardKey(BuildContext context, dynamic keyValue) {
    return FlatButton(
      child: Text(
        keyValue,
        style: const TextStyle(
          fontWeight: FontWeight.bold,
          fontSize: 26.0,
          color: Colors.black,
        ),
      ),
      color: Theme.of(context).scaffoldBackgroundColor,
      onPressed: () {
        _addKey(keyValue);
      },
    );
  }
}
