import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';

class QrCodeScanView extends StatefulWidget {
  const QrCodeScanView();

  @override
  _QrCodeScanState createState() => _QrCodeScanState();
}

class _QrCodeScanState extends State<QrCodeScanView> {
  String _barcode;
  Color _textColor = Colors.black;

  @override
  void initState() {
    _checkCanLaunch();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Scan Barcode'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(12.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Center(
              child: InkWell(
                child: SelectableText(
                  _barcode,
                  style: TextStyle(color: _textColor, fontSize: 21),
                ),
                onTap: _launchQrCodeScan,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _launchQrCodeScan() async {
    if (await canLaunch(_barcode)) {
      await launch(_barcode);
    }
  }

  Future<void> _checkCanLaunch() async {
    if (await canLaunch(_barcode)) {
      setState(() {
        _textColor = Colors.blue;
      });
    }
  }
}
