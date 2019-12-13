import 'package:bloom/bloom/admin/widgets/drawer.dart';
import 'package:flutter/material.dart';

class AdminUsersView extends StatefulWidget {
  const AdminUsersView();

  @override
  _AdminUsersState createState() => _AdminUsersState();
}

class _AdminUsersState extends State<AdminUsersView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const AdminDrawer(),
      appBar: AppBar(
        title: const Text('Admin'),
      ),
      body: const Center(child: Text('Users')),
    );
  }
}
