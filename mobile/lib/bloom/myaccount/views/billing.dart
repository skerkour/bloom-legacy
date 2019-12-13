import 'package:bloom/bloom/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountBillingView extends StatefulWidget {
  const MyAccountBillingView();

  @override
  _MyAccountBillingState createState() => _MyAccountBillingState();
}

class _MyAccountBillingState extends State<MyAccountBillingView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MyAccountDrawer(),
      appBar: AppBar(
        title: const Text('MyAccount'),
      ),
      body: const Center(child: Text('MyAccountBilling')),
    );
  }
}
