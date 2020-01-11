import 'package:flutter/material.dart';

class CalculatorExpression {
  CalculatorExpression({@required this.expression});

  final String expression;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'expression': expression,
    };
    return data;
  }
}

class CalculatorResult {
  CalculatorResult({@required this.result});

  final String result;

  static CalculatorResult fromJson(Map<String, dynamic> json) {
    final String result = json['data']['result'];
    return CalculatorResult(result: result);
  }
}
