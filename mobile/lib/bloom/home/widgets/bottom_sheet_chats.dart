import 'package:bloom/bloom/home/blocs/chat_tab.dart';
import 'package:bloom/bloom/home/views/barcode.dart';
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
          ListTile(
            leading: Icon(Icons.camera_alt),
            title: const Text('Scan QR code'),
            onTap: () => _onScanQRCodeTapped(context),
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

  Future<void> _onScanQRCodeTapped(BuildContext context) async {
    debugPrint('Scan QR code tapped');

    final String barcode = await ChatTabBloc.scan();
    await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => BarcodeView(barcode: barcode),
      ),
    );
    Navigator.of(context).pop();
  }
}

class IncrementBloc {}
