import 'package:flutter/material.dart';

class ChatsBottomSheet extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Wrap(
        children: <Widget>[
          ListTile(
            leading: Icon(Icons.people),
            title: const Text('Group chat'),
            onTap: () => _onGroupChatTapped(context),
          ),
          ListTile(
            leading: Icon(Icons.person_add),
            title: const Text('Add contact'),
            onTap: () => _onAddContactTapped(context),
          ),
          ListTile(
            leading: Icon(Icons.camera_alt),
            title: const Text('Scan QR code'),
            onTap: () => _onScanQRCodeTapped(context),
          ),
          ListTile(
            leading: Icon(Icons.attach_money),
            title: const Text('Request money'),
            onTap: () => _onRequestMoneyTapped(context),
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

  void _onRequestMoneyTapped(BuildContext context) {
    Navigator.of(context).pop();
    debugPrint('Request money tapped');
  }

  void _onScanQRCodeTapped(BuildContext context) {
    Navigator.of(context).pop();
    debugPrint('Scan QR code tapped');
  }
}

class IncrementBloc {}
