import 'package:flutter/material.dart';

class TabMeView extends StatefulWidget {
  const TabMeView({Key key}) : super(key: key);

  @override
  _TabMeViewState createState() => _TabMeViewState();
}

class _TabMeViewState extends State<TabMeView> {
  static const String _avatar = 'https://www.kerkour.fr/about/sylvain.jpg';
  List<_BlmApp> apps;

  @override
  void initState() {
    apps = getApps();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        const SizedBox(height: 10),
        Center(
            child: CircleAvatar(
          backgroundColor: Colors.grey,
          backgroundImage: NetworkImage(_avatar),
          radius: 42,
        )),
        const SizedBox(height: 21),
        Center(child: const Text('My Name', style: TextStyle(fontSize: 21))),
        const SizedBox(height: 5),
        Center(
            child:
                const Text('@user:domain.com', style: TextStyle(fontSize: 18))),
        const SizedBox(height: 42),
        Expanded(
          child: GridView.count(
            // scrollDirection: Axis.vertical,
            shrinkWrap: true,
            crossAxisCount: 4,
            children: apps
                .map((_BlmApp app) => _buildGridItem(context, app))
                .toList(),
          ),
        ),
      ],
    );
  }

  GestureDetector _buildGridItem(BuildContext context, _BlmApp app) {
    return GestureDetector(
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
    const _BlmApp(
        icon: 'assets/contacts_128.png', name: 'Contacts', route: '/contacts'),
    const _BlmApp(icon: 'assets/notes_128.png', name: 'Notes', route: '/notes'),
    const _BlmApp(
        icon: 'assets/calendar_128.png', name: 'Calendar', route: '/calendar'),
    const _BlmApp(icon: 'assets/drive_128.png', name: 'Drive', route: '/drive'),
    const _BlmApp(
        icon: 'assets/drive_128.png', name: 'Platform', route: '/platform'),
    const _BlmApp(
        icon: 'assets/drive_128.png', name: 'Bitflow', route: '/bitflow'),
  ];
}
