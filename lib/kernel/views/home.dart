import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({Key key, this.title}) : super(key: key);

  final String title;

  @override
  _HomeViewState createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  List<_BlmApp> apps;

  @override
  void initState() {
    apps = getApps();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BlmDrawer(),
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: _buildBody(context),
    );
  }

  Container _buildBody(BuildContext context) {
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
        icon: 'assets/contacts_128.png', name: 'Notes', route: '/notes'),
    const _BlmApp(
        icon: 'assets/contacts_128.png', name: 'Contacts', route: '/contacts'),
  ];
}
