import 'package:bloom/bloom/const.dart';
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
    // final List<GridTile> mainApps = getMainApps()
    // .map((_BlmMainApp app) => _buildMainGridItem(context, app))
    // .toList();
    return Column(
      children: <Widget>[
        const SizedBox(height: 10),
        Center(
          child: GestureDetector(
            child: const CircleAvatar(
              backgroundColor: Colors.grey,
              backgroundImage: AssetImage(_avatar),
              radius: 42,
            ),
            onTap: () {
              Navigator.pushNamed(context, '/auth');
            },
          ),
        ),
        const SizedBox(height: 21),
        const Center(child: Text('My Name', style: TextStyle(fontSize: 21))),
        const SizedBox(height: 5),
        const Center(
          child: Text('@user:domain.com', style: TextStyle(fontSize: 18)),
        ),
        const SizedBox(height: 21),
        // GridView.count(
        //   padding: const EdgeInsets.only(left: 12, right: 12),
        //   mainAxisSpacing: 8,
        //   crossAxisSpacing: 4,
        //   shrinkWrap: true,
        //   crossAxisCount: 4,
        //   children: mainApps,
        // ),
        const SizedBox(height: 10),
        const Divider(),
        const SizedBox(height: 21),
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
  // GridTile _buildMainGridItem(BuildContext context, _BlmMainApp app) {
  //   return GridTile(
  //     child: GestureDetector(
  //       child: Column(
  //         children: <Widget>[
  //           CircleAvatar(
  //             child: Icon(app.icon),
  //             backgroundColor: app.backgroundColor,
  //             foregroundColor: app.foregroundColor,
  //             radius: 25,
  //           ),
  //           const SizedBox(height: 5),
  //           Text(app.name, style: const TextStyle(fontSize: 16)),
  //         ],
  //       ),
  //       onTap: () => Navigator.pushNamed(
  //         context,
  //         app.route,
  //         // (Route<dynamic> route) => false,
  //       ),
  //     ),
  //   );
  // }
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
    const _BlmApp(
        icon: ICON_CALCULATOR_256, name: 'Calculator', route: '/calculator'),
    const _BlmApp(
        icon: ICON_CONTACTS_256, name: 'Contacts', route: '/contacts'),
    const _BlmApp(icon: ICON_ARCADE_256, name: 'Arcade', route: '/arcade'),
  ];
}

class _BlmMainApp {
  const _BlmMainApp(
      {@required this.icon,
      @required this.name,
      @required this.route,
      @required this.backgroundColor,
      this.foregroundColor = Colors.white});
  final IconData icon;
  final String name;
  final String route;
  final Color backgroundColor;
  final Color foregroundColor;
}

List<_BlmMainApp> getMainApps() {
  return <_BlmMainApp>[
    const _BlmMainApp(
        icon: Icons.person,
        name: 'Account',
        route: '/account',
        backgroundColor: Colors.blue),
    const _BlmMainApp(
        icon: Icons.calendar_today,
        name: 'Calendar',
        route: '/calendar',
        backgroundColor: Colors.blue),
    const _BlmMainApp(
        icon: Icons.account_balance_wallet,
        name: 'Wallet',
        route: '/wallet',
        backgroundColor: Colors.green),
    const _BlmMainApp(
        icon: Icons.settings,
        name: 'Settings',
        route: '/settings',
        backgroundColor: Colors.white10,
        foregroundColor: Colors.grey),
  ];
}
