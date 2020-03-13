import 'package:bloom/ui/const.dart';
import 'package:bloom/ui/groups/views/billing.dart';
import 'package:bloom/ui/groups/views/members.dart';
import 'package:bloom/ui/groups/views/preferences.dart';
import 'package:flutter/material.dart';

class GroupsDrawer extends StatefulWidget {
  const GroupsDrawer({Key key}) : super(key: key);

  @override
  _GroupsDrawerState createState() => _GroupsDrawerState();
}

class _GroupsDrawerState extends State<GroupsDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.group),
            title: const Text('Members'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const GroupsMembersView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_GROUPS,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.credit_card),
            title: const Text('Billing'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) => const GroupsBillingView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_GROUPS,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.settings),
            title: const Text('Preferences'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) =>
                      const GroupsPreferencesView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_GROUPS,
              );
            },
          ),
        ],
      ),
    );
  }
}
