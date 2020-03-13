import 'package:bloom/ui/groups/widgets/drawer.dart';
import 'package:flutter/material.dart';

class GroupsBillingView extends StatefulWidget {
  const GroupsBillingView();

  @override
  _GroupsBillingState createState() => _GroupsBillingState();
}

class _GroupsBillingState extends State<GroupsBillingView> {
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
