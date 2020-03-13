import 'package:bloom/ui/groups/widgets/drawer.dart';
import 'package:flutter/material.dart';

class GroupsPreferencesView extends StatefulWidget {
  const GroupsPreferencesView();

  @override
  _GroupsPreferencesState createState() => _GroupsPreferencesState();
}

class _GroupsPreferencesState extends State<GroupsPreferencesView> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      endDrawer: const GroupsDrawer(),
      appBar: AppBar(
        title: const Text('Billing'),
      ),
      body: _buildBody(),
    );
  }

  Container _buildBody() {
    return Container(
      child: const Center(
        child: Text('Groups billing'),
      ),
    );
  }
}
