import 'package:bloom/bloom/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountSecurityView extends StatefulWidget {
  const MyAccountSecurityView();

  @override
  _MyAccountSecurityState createState() => _MyAccountSecurityState();
}

class _MyAccountSecurityState extends State<MyAccountSecurityView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MyAccountDrawer(),
      appBar: AppBar(
        title: const Text('MyAccount'),
      ),
      body: const Center(child: Text('MyAccountSecurity')),
    );
  }
}
