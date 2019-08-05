import 'package:flutter/material.dart';

class TabDiscoverView extends StatefulWidget {
  const TabDiscoverView({Key key}) : super(key: key);

  @override
  _TabDiscoverViewState createState() => _TabDiscoverViewState();
}

class _TabDiscoverViewState extends State<TabDiscoverView> {
  @override
  Widget build(BuildContext context) {
    return ListView(
      children: <Widget>[
        ListTile(
          leading: Icon(Icons.search),
          title: const Text('Search web'),
        ),
      ],
    );
  }
}
