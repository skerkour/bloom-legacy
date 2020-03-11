import 'package:bloom/ui/admin/widgets/drawer.dart';
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
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    final List<User> users = User.getUsers();
    final List<DataRow> rows = users
        .map(
          (User user) => DataRow(cells: <DataCell>[
            DataCell(
              Text(user.username),
            ),
            DataCell(
              Text(user.email),
            ),
            DataCell(
              Text(user.active),
            ),
            DataCell(
              Text(user.active),
            ),
            DataCell(
              Text(user.active),
            ),
          ]),
        )
        .toList();

    final List<DataColumn> columns = <DataColumn>[
      const DataColumn(
        label: Text('Username'),
        numeric: false,
      ),
      const DataColumn(
        label: Text('Email'),
        numeric: false,
      ),
      const DataColumn(
        label: Text('Active'),
        numeric: false,
      ),
      const DataColumn(
        label: Text('Active'),
        numeric: false,
      ),
      const DataColumn(
        label: Text('Active'),
        numeric: false,
      ),
    ];

    return Container(
      child: SingleChildScrollView(
        scrollDirection: Axis.vertical,
        child: SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: DataTable(
            rows: rows,
            columns: columns,
          ),
        ),
      ),
    );
  }
}

class User {
  User({this.username, this.email}) {
    active = 'Active';
  }

  String username;
  String email;
  String active;

  static List<User> getUsers() {
    return <User>[
      User(username: 'Aaryan', email: 'Shah'),
      User(username: 'Ben', email: 'John'),
      User(username: 'Carrie', email: 'Brown'),
      User(username: 'Deep', email: 'Sen'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Emily', email: 'Jane'),
      User(username: 'Deep', email: 'Sen'),
    ];
  }
}
