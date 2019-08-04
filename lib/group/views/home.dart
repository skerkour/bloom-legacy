import 'package:flutter/material.dart';

class GroupHomeView extends StatefulWidget {
  const GroupHomeView({@required this.name});
  final String name;

  @override
  _GroupHomeState createState() => _GroupHomeState();
}

class _GroupHomeState extends State<GroupHomeView> {
  String _name;

  @override
  void initState() {
    _name = widget.name;
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(_name),
      ),
      body: Center(child: Text(_name)),
    );
  }
}
