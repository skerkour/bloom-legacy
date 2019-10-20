import 'package:flutter/material.dart';

class CalculatorGuiExpression {
  CalculatorGuiExpression({@required this.expression});

  final String expression;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'calculator.gui.expression',
      'data': <String, dynamic>{
        'expression': expression,
      },
    };
    return data;
  }
}

class CalculatorGuiResult {
  CalculatorGuiResult({@required this.result});

  final String result;

  static CalculatorGuiResult fromJson(Map<String, dynamic> json) {
    final String result = json['data']['result'];
    return CalculatorGuiResult(result: result);
  }
}
