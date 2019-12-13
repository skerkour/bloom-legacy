import 'package:bloom/bloom/admin/views/users.dart';
import 'package:bloom/bloom/admin/widgets/drawer.dart';
import 'package:bloom/bloom/const.dart';
import 'package:flutter/material.dart';

class AdminDashboardView extends StatefulWidget {
  const AdminDashboardView();

  @override
  _AdminDashboardState createState() => _AdminDashboardState();
}

class _AdminDashboardState extends State<AdminDashboardView> {
  String _version = '1.0.0';
  int _users = 9000;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const AdminDrawer(),
      appBar: AppBar(
        title: const Text('Admin'),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    return Container(
      padding: const EdgeInsets.all(10.0),
      child: ListView(children: <Widget>[
        ListTile(
          title: const Text('Bloom'),
          subtitle: Text('v$_version'),
        ),
        const Divider(),
        ListTile(
          title: const Text('Users'),
          subtitle: Text('$_users'),
          onTap: () {
            Navigator.pushAndRemoveUntil<dynamic>(
              context,
              MaterialPageRoute<dynamic>(
                builder: (BuildContext context) => const AdminUsersView(),
              ),
              (Route<dynamic> route) => route.settings.name == PATH_ADMIN,
            );
          },
        ),
        const Divider(),
      ]),
    );
  }
}
