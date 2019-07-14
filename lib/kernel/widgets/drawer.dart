import 'package:bloom/contacts/widgets/drawer.dart';
import 'package:bloom/kernel/blocs/drawer_bloc.dart';
import 'package:bloom/kernel/widgets/home_drawer.dart';
import 'package:bloom/notes/widgets/drawer.dart';
import 'package:flutter/material.dart';

class BlmDrawer extends StatefulWidget {
  const BlmDrawer({Key key}) : super(key: key);

  @override
  _BlmDrawerState createState() => _BlmDrawerState();
}

class _BlmDrawerState extends State<BlmDrawer>
    with SingleTickerProviderStateMixin {
  final List<Tab> tabs = <Tab>[
    const Tab(text: ''), // current app tab
    const Tab(text: ''), // Apps tab
  ];
  TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(vsync: this, length: tabs.length);
    _tabController.addListener(_onTabChanged);
  }

  void _onTabChanged() {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.white,
      width: MediaQuery.of(context).size.width * 0.7,
      child: SafeArea(
        bottom: false,
        child: Column(
          children: <Widget>[
            Container(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: <Widget>[
                  Container(
                    padding: const EdgeInsets.only(right: 10.0, top: 10.0),
                    child: InkWell(
                        onTap: () {
                          setState(() {
                            if (_tabController.index == 1) {
                              _tabController.index = 0;
                            } else {
                              _tabController.index = 1;
                            }
                          });
                        },
                        child: Padding(
                          padding: const EdgeInsets.all(8.0),
                          child: Icon(
                            Icons.apps,
                            color: Colors.black,
                          ),
                        )),
                  )
                ],
              ),
            ),
            Expanded(
              child: Stack(
                children: <Widget>[
                  TabBarView(
                    controller: _tabController,
                    children: const <Widget>[SettingsCurrentApp(), BloomApps()],
                  ),
                  SafeArea(
                    child: Padding(
                      padding: const EdgeInsets.only(bottom: 30),
                      child: Align(
                        alignment: Alignment.bottomCenter,
                        child: ClipRRect(
                          borderRadius:
                              const BorderRadius.all(Radius.circular(16)),
                          child: Container(
                            color: Colors.grey.shade300,
                            width: 45,
                            height: 18,
                            child: Row(
                              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                              children: <Widget>[
                                CircleAvatar(
                                  radius: 4,
                                  backgroundColor: _tabController.index == 0
                                      ? Colors.black
                                      : Colors.grey,
                                ),
                                CircleAvatar(
                                  radius: 4,
                                  backgroundColor: _tabController.index == 1
                                      ? Colors.black
                                      : Colors.grey,
                                )
                              ],
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            )
          ],
        ),
      ),
    );
  }
}

class SettingsCurrentApp extends StatefulWidget {
  const SettingsCurrentApp({Key key}) : super(key: key);

  @override
  _SettingsCurrentAppState createState() => _SettingsCurrentAppState();
}

class _SettingsCurrentAppState extends State<SettingsCurrentApp> {
  @override
  Widget build(BuildContext context) {
    return StreamBuilder<Apps>(
        initialData: drawerBloc.apps,
        stream: drawerBloc.outApps,
        builder: (BuildContext context, AsyncSnapshot<Apps> snapshot) {
          switch (snapshot.data) {
            case Apps.HOME:
              return const HomeDrawer();
              break;
            case Apps.CONTACTS:
              return const ContactsDrawer();
              break;
            case Apps.NOTES:
              return const NotesDrawer();
            default:
              return Container();
          }
        });
  }
}

class BloomApps extends StatefulWidget {
  const BloomApps({Key key}) : super(key: key);

  @override
  _BloomAppsState createState() => _BloomAppsState();
}

class _BlmApp {
  const _BlmApp(
      {@required this.icon, @required this.title, @required this.route});
  final String icon;
  final String title;
  final String route;
}

class _BloomAppsState extends State<BloomApps> {
  final List<_BlmApp> apps = <_BlmApp>[
    const _BlmApp(icon: 'assets/contacts_128.png', title: 'Home', route: '/'),
    const _BlmApp(
        icon: 'assets/contacts_128.png', title: 'Notes', route: '/notes'),
    const _BlmApp(
        icon: 'assets/contacts_128.png', title: 'Contacts', route: '/contacts'),
  ];

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: apps.map((_BlmApp app) => _buildApp(app)).toList(),
    );
  }

  Widget _buildApp(_BlmApp app) {
    return InkWell(
      onTap: () => Navigator.pushNamedAndRemoveUntil(
        context,
        app.route,
        (Route<dynamic> route) => false,
      ),
      child: Padding(
        padding: const EdgeInsets.all(4.0),
        child: Row(
          children: <Widget>[
            CircleAvatar(
              backgroundImage: AssetImage(app.icon),
              backgroundColor: Colors.transparent,
              radius: 25,
            ),
            const SizedBox(
              width: 12,
            ),
            Text(app.title)
          ],
        ),
      ),
    );
  }
}
