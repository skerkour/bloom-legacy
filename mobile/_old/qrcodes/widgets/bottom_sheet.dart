import 'package:bloom/ui/home/blocs/chat_tab.dart';
import 'package:bloom/ui/qrcodes/views/scanned.dart';
import 'package:flutter/material.dart';

class QrCodesBottomSheet extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Wrap(
        children: <Widget>[
          ListTile(
            leading: Icon(Icons.add_to_photos),
            title: const Text('Create QR code'),
            onTap: () => _onScanQRCodeTapped(context),
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

  Future<void> _onScanQRCodeTapped(BuildContext context) async {
    debugPrint('Scan QR code tapped');

    final String barcode = await ChatTabBloc.scan();
    await Navigator.push<dynamic>(
      context,
      MaterialPageRoute<dynamic>(
        builder: (BuildContext context) => QrCodeScannedView(barcode: barcode),
      ),
    );
    Navigator.of(context).pop();
  }
}

class IncrementBloc {}
