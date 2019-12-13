import 'package:bloom/bloom/phaser/widgets/drawer.dart';
import 'package:flutter/material.dart';

class PhaserScansView extends StatefulWidget {
  const PhaserScansView();

  @override
  _PhaserScansState createState() => _PhaserScansState();
}

class _PhaserScansState extends State<PhaserScansView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const PhaserDrawer(),
      appBar: AppBar(
        title: const Text('Phaser'),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    final List<Scan> users = Scan.getScans();
    final List<DataRow> rows = users
        .map(
          (Scan user) => DataRow(cells: <DataCell>[
            DataCell(
              Text(user.username),
            ),
            DataCell(
              Text(user.email),
            ),
            DataCell(
              Text(user.state),
            ),
            DataCell(
              Text(user.state),
            ),
            DataCell(
              Text(user.state),
            ),
          ]),
        )
        .toList();

    final List<DataColumn> columns = <DataColumn>[
      const DataColumn(
        label: Text('Scan'),
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

class Scan {
  Scan({this.username, this.email}) {
    state = 'Active';
  }

  String username;
  String email;
  String state;

  static List<Scan> getScans() {
    return <Scan>[
      Scan(username: 'Aaryan', email: 'Shah'),
      Scan(username: 'Ben', email: 'John'),
      Scan(username: 'Carrie', email: 'Brown'),
      Scan(username: 'Deep', email: 'Sen'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Emily', email: 'Jane'),
      Scan(username: 'Deep', email: 'Sen'),
    ];
  }
}
