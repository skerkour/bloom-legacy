import 'package:bloom/calendar/widgets/drawer.dart';
import 'package:bloom/contacts/widgets/drawer.dart';
import 'package:bloom/drive/widgets/drawer.dart';
import 'package:bloom/kernel/blocs/app.dart';
import 'package:bloom/kernel/views/auth.dart';
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
    return Drawer(
      child: Column(
        // Important: Remove any padding from the ListView.
        children: <Widget>[
          Stack(
            children: <Widget>[
              GestureDetector(
                onTap: _detailsPressed,
                child: UserAccountsDrawerHeader(
                  accountName: const Text('Click to sign in or register'),
                  accountEmail: const Text(''),
                  currentAccountPicture:
                      CircleAvatar(child: Icon(Icons.person)),
                ),
              ),
              Container(
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: <Widget>[
                    Container(
                      padding: const EdgeInsets.only(right: 10.0, top: 30.0),
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
                              color: Colors.white,
                            ),
                          )),
                    )
                  ],
                ),
              ),
            ],
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
    );
  }

  void _detailsPressed() {
    Navigator.of(context).pop();
    Navigator.of(context).push<dynamic>(MaterialPageRoute<dynamic>(
      builder: (BuildContext context) => const AuthView(),
    ));
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
        initialData: appBloc.currentApp,
        stream: appBloc.outCurrentApp,
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
            case Apps.CALENDAR:
              return const CalendarDrawer();
            case Apps.DRIVE:
              return const DriveDrawer();
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
    const _BlmApp(
        icon: 'assets/contacts_128.png', title: 'Calendar', route: '/calendar'),
    const _BlmApp(
        icon: 'assets/contacts_128.png', title: 'Drive', route: '/drive'),
  ];

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: apps.map((_BlmApp app) => _buildApp(app)).toList(),
    );
  }

  Widget _buildApp(_BlmApp app) {
    return ListTile(
      leading: CircleAvatar(
        backgroundImage: AssetImage(app.icon),
        backgroundColor: Colors.transparent,
        radius: 25,
      ),
      title: Text(app.title),
      onTap: () => Navigator.pushNamedAndRemoveUntil(
        context,
        app.route,
        (Route<dynamic> route) => false,
      ),
    );
  }
}
