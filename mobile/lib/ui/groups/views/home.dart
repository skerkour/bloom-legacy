import 'package:bloom/ui/const.dart';
import 'package:flutter/material.dart';

class GroupHomeView extends StatefulWidget {
  const GroupHomeView({@required this.name});
  final String name;

  @override
  _GroupHomeState createState() => _GroupHomeState();
}

class _GroupHomeState extends State<GroupHomeView> {
  String _name;

  @override
  void initState() {
    _name = widget.name;
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(_name),
      ),
      body: _buildBody(),
    );
  }

  Container _buildBody() {
    final List<GridTile> apps =
        getApps().map((_BlmApp app) => _buildGridItem(context, app)).toList();
    return Container(
      padding: const EdgeInsets.only(top: 21, left: 6, right: 6),
      child: GridView.count(
        padding: const EdgeInsets.only(left: 12, right: 12),
        mainAxisSpacing: 8,
        crossAxisSpacing: 4,
        shrinkWrap: true,
        crossAxisCount: 4,
        children: apps,
      ),
    );
  }

  GridTile _buildGridItem(BuildContext context, _BlmApp app) {
    return GridTile(
      child: GestureDetector(
        child: Column(
          children: <Widget>[
            CircleAvatar(
              backgroundImage: AssetImage(app.icon),
              backgroundColor: Colors.transparent,
              radius: 32,
            ),
            const SizedBox(height: 5),
            Text(app.name, style: const TextStyle(fontSize: 16)),
          ],
        ),
        onTap: () => Navigator.pushNamed(
          context,
          app.route,
          // (Route<dynamic> route) => false,
        ),
      ),
    );
  }
}

class _BlmApp {
  const _BlmApp(
      {@required this.icon, @required this.name, @required this.route});
  final String icon;
  final String name;
  final String route;
}

List<_BlmApp> getApps() {
  return <_BlmApp>[
    const _BlmApp(icon: ICON_NOTES_256, name: 'Notes', route: '/notes'),
    const _BlmApp(
        icon: ICON_CALENDAR_256, name: 'Calendar', route: '/calendar'),
    const _BlmApp(icon: ICON_NOTES_256, name: 'Notes', route: '/notes'),
    const _BlmApp(icon: ICON_ARCADE_256, name: 'Arcade', route: '/arcade'),
  ];
}
