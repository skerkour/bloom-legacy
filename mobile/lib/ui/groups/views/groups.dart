// import 'package:bloom/ui/const.dart';
// import 'package:flutter/material.dart';

// class GroupHomeView extends StatefulWidget {
//   const GroupHomeView({@required this.name});
//   final String name;

//   @override
//   _GroupHomeState createState() => _GroupHomeState();
// }

// class _GroupHomeState extends State<GroupHomeView> {
//   String _name;

//   @override
//   void initState() {
//     _name = widget.name;
//     super.initState();
//   }

//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       appBar: AppBar(
//         title: Text(_name),
//       ),
//       body: _buildBody(),
//     );
//   }

//   Container _buildBody() {
//     final List<GridTile> apps =
//         getApps().map((_BlmApp app) => _buildGridItem(context, app)).toList();
//     return Container(
//       padding: const EdgeInsets.only(top: 21, left: 6, right: 6),
//       child: GridView.count(
//         padding: const EdgeInsets.only(left: 12, right: 12),
//         mainAxisSpacing: 8,
//         crossAxisSpacing: 4,
//         shrinkWrap: true,
//         crossAxisCount: 4,
//         children: apps,
//       ),
//     );
//   }

//   GridTile _buildGridItem(BuildContext context, _BlmApp app) {
//     return GridTile(
//       child: GestureDetector(
//         child: Column(
//           children: <Widget>[
//             CircleAvatar(
//               backgroundImage: AssetImage(app.icon),
//               backgroundColor: Colors.transparent,
//               radius: 32,
//             ),
//             const SizedBox(height: 5),
//             Text(app.name, style: const TextStyle(fontSize: 16)),
//           ],
//         ),
//         onTap: () => Navigator.pushNamed(
//           context,
//           app.route,
//           // (Route<dynamic> route) => false,
//         ),
//       ),
//     );
//   }
// }

// class _BlmApp {
//   const _BlmApp(
//       {@required this.icon, @required this.name, @required this.route});
//   final String icon;
//   final String name;
//   final String route;
// }

// List<_BlmApp> getApps() {
//   return <_BlmApp>[
//     const _BlmApp(icon: ICON_NOTES_256, name: 'Notes', route: '/notes'),
//     const _BlmApp(
//         icon: ICON_CALENDAR_256, name: 'Calendar', route: '/calendar'),
//     const _BlmApp(icon: ICON_NOTES_256, name: 'Notes', route: '/notes'),
//     const _BlmApp(icon: ICON_ARCADE_256, name: 'Arcade', route: '/arcade'),
//   ];
// }
import 'package:bloom/ui/notes/widgets/drawer.dart';
import 'package:flutter/material.dart';

class GroupsView extends StatefulWidget {
  const GroupsView({Key key}) : super(key: key);

  @override
  _GroupsViewState createState() => _GroupsViewState();
}

class _GroupsViewState extends State<GroupsView> {
  static const List<_Group> _groups = <_Group>[
    _Group(name: 'Bloom', description: 'Empowering people'),
    _Group(name: '42', description: 'The future of learning'),
    _Group(name: 'Tesla', description: 'Not so green transport'),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
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
