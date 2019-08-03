import 'package:flutter/material.dart';

class TabChatsView extends StatefulWidget {
  const TabChatsView({Key key}) : super(key: key);

  @override
  _TabChatsViewState createState() => _TabChatsViewState();
}

class _TabChatsViewState extends State<TabChatsView> {
  @override
  Widget build(BuildContext context) {
    return Center(child: const Text('Chats'));
  }
}
