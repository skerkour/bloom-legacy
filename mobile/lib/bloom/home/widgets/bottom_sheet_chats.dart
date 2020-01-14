import 'package:flutter/material.dart';

class ChatsBottomSheet extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Wrap(
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.people),
            title: const Text('Group chat'),
            onTap: () => _onGroupChatTapped(context),
          ),
          ListTile(
            leading: Icon(Icons.person_add),
            title: const Text('Add contact'),
            onTap: () => _onAddContactTapped(context),
          ),
        ],
      ),
    );
  }

  void _onGroupChatTapped(BuildContext context) {
    Navigator.of(context).pop();
    debugPrint('Group chat tapped');
  }

  void _onAddContactTapped(BuildContext context) {
    Navigator.of(context).pop();
    debugPrint('Add contact tapped');
  }
}

class IncrementBloc {}
