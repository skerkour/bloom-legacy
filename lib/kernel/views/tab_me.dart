import 'package:flutter/material.dart';

class TabMeView extends StatefulWidget {
  const TabMeView({Key key}) : super(key: key);

  @override
  _TabMeViewState createState() => _TabMeViewState();
}

class _TabMeViewState extends State<TabMeView> {
  List<_BlmApp> apps;

  @override
  void initState() {
    apps = getApps();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: ListView.builder(
        scrollDirection: Axis.vertical,
        shrinkWrap: true,
        itemCount: apps.length,
        itemBuilder: (BuildContext context, int index) {
          return _buildCard(context, apps[index]);
        },
      ),
    );
  }

  Card _buildCard(BuildContext context, _BlmApp app) {
    return Card(
      elevation: 4.0,
      margin: const EdgeInsets.symmetric(horizontal: 10.0, vertical: 6.0),
      child: Container(
        child: _buildListTile(context, app),
      ),
    );
  }

  ListTile _buildListTile(BuildContext context, _BlmApp app) {
    return ListTile(
      contentPadding:
          const EdgeInsets.symmetric(horizontal: 20.0, vertical: 10.0),
      leading: Container(
        padding: const EdgeInsets.only(right: 12.0),
        child: CircleAvatar(
          backgroundImage: AssetImage(app.icon),
          backgroundColor: Colors.transparent,
          radius: 25,
        ),
      ),
      title: Text(app.name),
      onTap: () => Navigator.pushNamedAndRemoveUntil(
        context,
        app.route,
        (Route<dynamic> route) => false,
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
    const _BlmApp(icon: 'assets/notes_128.png', name: 'Notes', route: '/notes'),
    const _BlmApp(
        icon: 'assets/contacts_128.png', name: 'Contacts', route: '/contacts'),
    const _BlmApp(
        icon: 'assets/contacts_128.png', name: 'Calendar', route: '/calendar'),
    const _BlmApp(
        icon: 'assets/contacts_128.png', name: 'Drive', route: '/drive'),
  ];
}
