import 'package:bloom/ui/const.dart';
import 'package:bloom/ui/myaccount/views/billing.dart';
import 'package:bloom/ui/myaccount/views/devices.dart';
import 'package:bloom/ui/myaccount/views/security.dart';
import 'package:flutter/material.dart';

class MyAccountDrawer extends StatefulWidget {
  const MyAccountDrawer({Key key}) : super(key: key);

  @override
  _MyAccountDrawerState createState() => _MyAccountDrawerState();
}

class _MyAccountDrawerState extends State<MyAccountDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.arrow_back),
            title: const Text('Back'),
            onTap: () {
              Navigator.of(context).popUntil(
                  (Route<dynamic> route) => route.settings.name == '/');
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.person),
            title: const Text('Profile'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushNamedAndRemoveUntil(
                context,
                PATH_MYACCOUNT,
                (Route<dynamic> route) => route.settings.name == '/',
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
                  builder: (BuildContext context) =>
                      const MyAccountBillingView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_MYACCOUNT,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.security),
            title: const Text('Security'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) =>
                      const MyAccountSecurityView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_MYACCOUNT,
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.devices),
            title: const Text('Devices'),
            onTap: () {
              Navigator.pop(context);
              Navigator.pushAndRemoveUntil<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                  builder: (BuildContext context) =>
                      const MyAccountDevicesView(),
                ),
                (Route<dynamic> route) => route.settings.name == PATH_MYACCOUNT,
              );
            },
          ),
        ],
      ),
    );
  }
}
