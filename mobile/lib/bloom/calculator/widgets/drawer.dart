import 'package:flutter/material.dart';

class CalculatorDrawer extends StatefulWidget {
  const CalculatorDrawer({Key key}) : super(key: key);

  @override
  _CalculatorDrawerState createState() => _CalculatorDrawerState();
}

class _CalculatorDrawerState extends State<CalculatorDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.list),
            title: const Text('Calculator'),
            onTap: () => debugPrint('Calculator tapped'),
          ),
        ],
      ),
    );
  }
}
