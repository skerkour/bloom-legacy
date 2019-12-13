import 'package:bloom/bloom/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountProfileView extends StatefulWidget {
  const MyAccountProfileView();

  @override
  _MyAccountProfileState createState() => _MyAccountProfileState();
}

class _MyAccountProfileState extends State<MyAccountProfileView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MyAccountDrawer(),
      appBar: AppBar(
        title: const Text('MyAccount'),
      ),
      body: const Center(child: Text('MyAccountProfile')),
    );
  }
}
