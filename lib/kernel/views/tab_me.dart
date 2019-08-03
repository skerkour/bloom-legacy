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
        const SizedBox(height: 21),
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
        const SizedBox(height: 21),
        Container(
          child: ListView.builder(
            scrollDirection: Axis.vertical,
            shrinkWrap: true,
            itemCount: apps.length,
            itemBuilder: (BuildContext context, int index) {
              return _buildListTile(context, apps[index]);
            },
          ),
        ),
      ],
    );
  }

  ListTile _buildListTile(BuildContext context, _BlmApp app) {
    return ListTile(
      leading: CircleAvatar(
        backgroundImage: AssetImage(app.icon),
        backgroundColor: Colors.transparent,
        radius: 25,
      ),
      title: Text(app.name),
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
  ];
}
