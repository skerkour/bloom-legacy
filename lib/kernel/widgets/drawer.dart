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
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: <Widget>[
                  Expanded(
                    child: Container(
                      child: ListTile(
                        leading: Icon(
                          Icons.close,
                          color: Colors.black,
                        ),
                        onTap: () {
                          // change app state...
                          Navigator.pop(context); // close the drawer
                        },
                      ),
                    ),
                  ),
                  Container(
                    child: InkWell(
                        onTap: () {
                          setState(() {
                            _tabController.index = 1;
                          });
                        },
                        child: Padding(
                          padding: const EdgeInsets.all(8.0),
                          child: Icon(
                            Icons.dashboard,
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
                          borderRadius: const BorderRadius.all(Radius.circular(16)),
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
    return Container(
      child: const Text('App settings'),
    );
  }
}

class BloomApps extends StatefulWidget {
  const BloomApps({Key key}) : super(key: key);

    @override
  _BloomAppsState createState() => _BloomAppsState();
}

 class _BlmApp {
    const _BlmApp({ @required this.title, @required this.route });
   final String title;
   final String route;
 }

class _BloomAppsState extends State<BloomApps> {
  final List<_BlmApp> apps = <_BlmApp>[
    const _BlmApp(title: 'Home', route: '/'),
    const _BlmApp(title: 'Notes', route: '/notes'),
    const _BlmApp(title: 'Contacts', route: '/contacts'),
  ];

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: apps.map((_BlmApp app) => _buildApp(app)).toList(),
    );
  }

  Widget _buildApp(_BlmApp app) {
    return InkWell(
      onTap: () => Navigator.pushNamed(context, app.route),
      child: Padding(
        padding: const EdgeInsets.all(10.0),
        child: Row(
          children: <Widget>[
            CircleAvatar(
              backgroundColor: Colors.blue,
              child: Container(
                  child:
                      Center(child: Text(app.title.substring(0, 1).toUpperCase()))),
            ),
            const SizedBox(
              width: 16,
            ),
            Text(app.title)
          ],
        ),
      ),
    );
  }
}
