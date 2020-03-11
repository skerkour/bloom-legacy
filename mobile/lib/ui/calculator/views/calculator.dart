import 'package:bloom/core/calculator/messages.dart';
import 'package:bloom/core/calculator/methods.dart';
import 'package:bloom/core/core.dart';
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

  Widget _buildKeyboard(BuildContext context) {
    // return Expanded(
    //   flex: 4,
    //   child: Center(
    //     child: AspectRatio(
    //       aspectRatio: 1, // To center the GridView
    //       child: Column(
    // 	  children: <Widget>[
    //         Row(
    // 			  children: <Widget>[
    //              _buildKeyboardKey(context, '7'),
    //              _buildKeyboardKey(context, '8'),
    //              _buildKeyboardKey(context, '9'),
    //              _buildKeyboardKey(context, '/'),
    //           ],
    //         ),
    //         Row(
    // 			  children: <Widget>[
    //              _buildKeyboardKey(context, '4'),
    //              _buildKeyboardKey(context, '5'),
    //              _buildKeyboardKey(context, '6'),
    //              _buildKeyboardKey(context, '*'),
    //           ],
    //         ),
    //         Row(
    // 			  children: <Widget>[
    //              _buildKeyboardKey(context, '1'),
    //              _buildKeyboardKey(context, '2'),
    //              _buildKeyboardKey(context, '3'),
    //              _buildKeyboardKey(context, '-'),
    //           ],
    //         ),
    //         Row(
    // 			  children: <Widget>[
    //              _buildKeyboardKey(context, '.'),
    //              _buildKeyboardKey(context, '0'),
    //              _buildKeyboardKey(context, 'C'),
    //              _buildKeyboardKey(context, '+'),
    //           ],
    //         ),

    //          Row(
    // 			  children: <Widget>[
    //              _buildKeyboardKey(context, '='),
    //           ],
    //         ),
    //       ],
    //       ),
    //   ),
    // ),
    // );
    return Expanded(
      flex: 4,
      child: Center(
        child: AspectRatio(
          aspectRatio: 1, // To center the GridView
          child: GridView.count(
            crossAxisCount: 4,
            // childAspectRatio: 1.0,
            padding: const EdgeInsets.all(4.0),
            mainAxisSpacing: 4.0,
            crossAxisSpacing: 4.0,
            children: <String>[
              // @formatter:off
              '7', '8', '9', '/',
              '4', '5', '6', '*',
              '1', '2', '3', '-',
              '.', '0', '=', '+',
              // 'C', '', '', '=',
              // 'C', '', '', '=',
              // @formatter:on
            ].map((String key) {
              return GridTile(
                child: _buildKeyboardKey(context, key),
              );
            }).toList(),
          ),
        ),
      ),
    );
  }

  Widget _buildKeyboardKey(BuildContext context, String keyValue) {
    return Padding(
      padding: const EdgeInsets.all(12.0),
      child: GestureDetector(
        child: FlatButton(
          child: Text(
            keyValue == '=' ? '=/c' : keyValue,
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
        ),
        onTap: () {
          _addKey(keyValue);
        },
        onLongPress: keyValue == '=' ? _clear : null,
      ),
    );
  }

  void _clear() {
    setState(() {
      _expression = '';
      _result = '';
    });
  }

  Future<void> _addKey(String key) async {
    String expr = _expression;
    String result = '';

    if (operators.contains(key)) {
      // Handle as an operator
      if (expr.isNotEmpty && operators.contains(expr[expr.length - 1])) {
        expr = expr.substring(0, expr.length - 1);
      }
      expr += ' $key ';
    } else if (digits.contains(key)) {
      // Handle as an operand
      if (_result.isNotEmpty) {
        expr = key;
      } else {
        expr += key;
      }
    } else if (key == 'C') {
      // Delete all characters and reset result
      expr = '';
      result = '';
    } else if (key == '=') {
      try {
        final CalculatorCalcResult ret = CalculatorCalcResult.fromJson(
            await coreCall(
                CalculatorMethod.calc, CalculatorCalcParams(expression: expr)));
        result = ret.result;
      } catch (err) {
        result = 'Error';
      }
    }
    setState(() {
      _expression = expr;
      _result = result;
    });
  }
}

List<String> digits = <String>[
  '0',
  '1',
  '2',
  '3',
  '4',
  '5',
  '6',
  '7',
  '8',
  '9',
  '.',
];
List<String> operators = <String>['+', '-', '*', '/'];
