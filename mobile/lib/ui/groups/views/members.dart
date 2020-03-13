import 'package:bloom/ui/groups/widgets/drawer.dart';
import 'package:flutter/material.dart';

class GroupsMembersView extends StatefulWidget {
  const GroupsMembersView({Key key}) : super(key: key);

  @override
  _GroupsMembersViewState createState() => _GroupsMembersViewState();
}

class _GroupsMembersViewState extends State<GroupsMembersView> {
  static const List<_Group> _groups = <_Group>[
    _Group(name: 'Bloom', description: 'Empowering people'),
    _Group(name: '42', description: 'The future of learning'),
    _Group(name: 'Tesla', description: 'Not so green transport'),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      endDrawer: const GroupsDrawer(),
      appBar: AppBar(
        title: const Text('Groups'),
      ),
      body: ListView.builder(
        itemCount: _groups.length,
        itemBuilder: (BuildContext context, int index) {
          final _Group group = _groups[index];
          return ListTile(
            leading: CircleAvatar(child: Text(group.name[0].toUpperCase())),
            title: Text(group.name),
            subtitle: Text(group.description),
            isThreeLine: false,
            // onTap: () {
            //   Navigator.push<dynamic>(
            //     context,
            //     MaterialPageRoute<dynamic>(
            //       builder: (BuildContext context) =>
            //           GroupHomeView(name: group.name),
            //     ),
            //   );
            // },
          );
        },
      ),
    );
  }
}

class _Group {
  const _Group({@required this.name, @required this.description, this.icon});
  final String icon;
  final String name;
  final String description;
}
