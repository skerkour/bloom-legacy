import 'package:bloom/ui/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountSecurityView extends StatefulWidget {
  const MyAccountSecurityView();

  @override
  _MyAccountSecurityState createState() => _MyAccountSecurityState();
}

class _MyAccountSecurityState extends State<MyAccountSecurityView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const MyAccountDrawer(),
      appBar: AppBar(
        title: const Text('MyAccount'),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    return Container(
      padding: const EdgeInsets.all(21),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          const Text('Change password', style: TextStyle(fontSize: 21)),
          const SizedBox(height: 22),
          const TextField(
            obscureText: true,
            decoration: InputDecoration(
              border: OutlineInputBorder(),
              labelText: 'Current password',
            ),
          ),
          const SizedBox(height: 11),
          const TextField(
            obscureText: true,
            decoration: InputDecoration(
              border: OutlineInputBorder(),
              labelText: 'New password',
            ),
          ),
          const SizedBox(height: 11),
          const TextField(
            obscureText: true,
            decoration: InputDecoration(
              border: OutlineInputBorder(),
              labelText: 'New password verification',
            ),
          ),
          const SizedBox(height: 11),
          ButtonBar(
            alignment: MainAxisAlignment.end,
            children: <Widget>[
              RaisedButton(
                child: const Text('Update password'),
                padding: const EdgeInsets.all(8.0),
                textColor: Colors.white,
                color: Colors.blue,
                onPressed: () {},
              )
            ],
          ),
        ],
      ),
    );
  }
}
