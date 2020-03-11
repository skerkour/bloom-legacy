import 'package:bloom/ui/const.dart';
import 'package:flutter/material.dart';

class TabMeView extends StatefulWidget {
  const TabMeView({Key key}) : super(key: key);

  @override
  _TabMeViewState createState() => _TabMeViewState();
}

class _TabMeViewState extends State<TabMeView> {
  static const String _avatar = 'assets/images/sylvain.jpg';

  @override
  Widget build(BuildContext context) {
    final List<GridTile> apps =
        getApps().map((_BlmApp app) => _buildGridItem(context, app)).toList();

    return Column(
      children: <Widget>[
        const SizedBox(height: 6),
        Center(
          child: GestureDetector(
            child: const CircleAvatar(
              backgroundColor: Colors.grey,
              backgroundImage: AssetImage(_avatar),
              radius: 40,
            ),
            onTap: () => Navigator.pushNamed(context, '/auth'),
          ),
        ),
        const SizedBox(height: 21),
        const Center(
            child: Text('Sylvain Kerkour', style: TextStyle(fontSize: 21))),
        const SizedBox(height: 5),
        const Center(
          child: Text('@sylvain', style: TextStyle(fontSize: 18)),
        ),
        ButtonBar(
          alignment: MainAxisAlignment.center,
          children: <Widget>[
            FlatButton.icon(
              onPressed: () => Navigator.pushNamed(context, PATH_PREFERENCES),
              icon: const Icon(Icons.settings),
              label: const Text('Preferences'),
            ),
            FlatButton.icon(
              onPressed: () => Navigator.pushNamed(context, PATH_MYACCOUNT),
              icon: const Icon(Icons.person),
              label: const Text('My Account'),
            ),
          ],
        ),
        const Divider(),
        const SizedBox(height: 10),
        Expanded(
          child: GridView.count(
            padding: const EdgeInsets.only(left: 12, right: 12),
            mainAxisSpacing: 8,
            crossAxisSpacing: 4,
            shrinkWrap: true,
            crossAxisCount: 4,
            children: apps,
          ),
        ),
      ],
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
              radius: 28,
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
    const _BlmApp(icon: ICON_NOTES_256, name: 'Notes', route: PATH_NOTES),
    const _BlmApp(
        icon: ICON_CALENDAR_256, name: 'Calendar', route: PATH_CALENDAR),
    const _BlmApp(icon: ICON_DRIVE_256, name: 'Drive', route: PATH_DRIVE),
    const _BlmApp(
        icon: ICON_CONTACTS_256, name: 'Contacts', route: PATH_CONTACTS),
    const _BlmApp(icon: ICON_ARCADE_256, name: 'Arcade', route: PATH_ARCADE),
    const _BlmApp(
        icon: ICON_CALCULATOR_256, name: 'Calculator', route: PATH_CALCULATOR),
    const _BlmApp(icon: ICON_BITFLOW_256, name: 'Bitflow', route: PATH_BITFLOW),
    const _BlmApp(
        icon: ICON_QRCODES_256, name: 'QR Codes', route: PATH_QRCODES),
    const _BlmApp(icon: ICON_ADMIN_256, name: 'Admin', route: PATH_ADMIN),
  ];
}
