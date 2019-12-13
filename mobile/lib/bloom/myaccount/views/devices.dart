import 'package:bloom/bloom/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountDevicesView extends StatefulWidget {
  const MyAccountDevicesView();

  @override
  _MyAccountDevicesState createState() => _MyAccountDevicesState();
}

class _MyAccountDevicesState extends State<MyAccountDevicesView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MyAccountDrawer(),
      appBar: AppBar(
        title: const Text('MyAccount'),
      ),
      body: const Center(child: Text('MyAccountDevices')),
    );
  }
}
