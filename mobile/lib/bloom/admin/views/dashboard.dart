import 'package:bloom/bloom/admin/widgets/drawer.dart';
import 'package:flutter/material.dart';

class AdminDashboardView extends StatefulWidget {
  const AdminDashboardView();

  @override
  _AdminDashboardState createState() => _AdminDashboardState();
}

class _AdminDashboardState extends State<AdminDashboardView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const AdminDrawer(),
      appBar: AppBar(
        title: const Text('Admin'),
      ),
      body: const Center(child: Text('Admin')),
    );
  }
}
