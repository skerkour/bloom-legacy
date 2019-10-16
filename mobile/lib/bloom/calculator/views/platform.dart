// import 'package:bloom/bloom/calculator/widgets/drawer.dart';
import 'package:flutter/material.dart';

class CalculatorView extends StatefulWidget {
  const CalculatorView();

  @override
  _CalculatorState createState() => _CalculatorState();
}

class _CalculatorState extends State<CalculatorView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // endDrawer: const CalculatorDrawer(),
      appBar: AppBar(
        title: const Text('Calculator'),
      ),
      body: const Center(child: Text('Calculator')),
    );
  }
}
