import 'package:flutter/material.dart';

class BlmDrawer extends StatefulWidget {
  const BlmDrawer({Key key}) : super(key: key);

  @override
  _BlmDrawerState createState() => _BlmDrawerState();
}

class _BlmDrawerState extends State<BlmDrawer>
    with SingleTickerProviderStateMixin {
  final List<Tab> tabs = <Tab>[
    Tab(text: ''), // current app tab
    Tab(text: ''), // Apps tab
  ];
  TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(vsync: this, length: tabs.length);
    _tabController.addListener(_onTabChanged);
  }

  _onTabChanged() {
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
                    children: <Widget>[SettingsCurrentApp(), BloomApps()],
                  ),
                  SafeArea(
                    child: Padding(
                      padding: const EdgeInsets.only(bottom: 30),
                      child: Align(
                        alignment: Alignment.bottomCenter,
                        child: ClipRRect(
                          borderRadius: BorderRadius.all(Radius.circular(16)),
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
  SettingsCurrentApp({Key key}) : super(key: key);

  _SettingsCurrentAppState createState() => _SettingsCurrentAppState();
}

class _SettingsCurrentAppState extends State<SettingsCurrentApp> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text("SETTINGS"),
    );
  }
}

class BloomApps extends StatefulWidget {
  BloomApps({Key key}) : super(key: key);

  _BloomAppsState createState() => _BloomAppsState();
}

class _BloomAppsState extends State<BloomApps> {
  List<String> apps = [
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes",
    "Calendar",
    "Bitflow",
    "Notes"
  ];

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: apps.map((app) => _buildApp(app)).toList(),
    );
  }

  Widget _buildApp(String app) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Row(
        children: <Widget>[
          CircleAvatar(
            backgroundColor: Colors.blue,
            child: Container(
                child: Center(child: Text(app.substring(0, 1).toUpperCase()))),
          ),
          SizedBox(
            width: 16,
          ),
          Text(app)
        ],
      ),
    );
  }
}
