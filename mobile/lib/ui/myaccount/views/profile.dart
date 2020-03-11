import 'package:bloom/ui/myaccount/widgets/drawer.dart';
import 'package:flutter/material.dart';

class MyAccountProfileView extends StatefulWidget {
  const MyAccountProfileView();

  @override
  _MyAccountProfileState createState() => _MyAccountProfileState();
}

class _MyAccountProfileState extends State<MyAccountProfileView> {
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
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: const <Widget>[
          Text('Public Profile', style: TextStyle(fontSize: 32)),
          Text('Avatar', style: TextStyle(fontSize: 21)),
          Text('Username', style: TextStyle(fontSize: 21)),
          Text('Display name', style: TextStyle(fontSize: 21)),
          Text('Bio', style: TextStyle(fontSize: 21)),
          Divider(),
          Text('Personal Information', style: TextStyle(fontSize: 32)),
          Text('Email', style: TextStyle(fontSize: 21)),
        ],
      ),
    );
  }
}
