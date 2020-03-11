import 'package:flutter/material.dart';

class CalculatorCalcParams {
  CalculatorCalcParams({@required this.expression});

  final String expression;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'expression': expression,
    };
    return data;
  }
}

class CalculatorCalcResult {
  CalculatorCalcResult({@required this.result});

  final String result;

  static CalculatorCalcResult fromJson(Map<String, dynamic> json) {
    final String result = json['data']['result'];
    return CalculatorCalcResult(result: result);
  }
}
