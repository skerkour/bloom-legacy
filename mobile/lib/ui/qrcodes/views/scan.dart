import 'package:barcode_scan/barcode_scan.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
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
    _scan().then((String barcode) {
        if (barcode == null) {
          Navigator.of(context).pop();
        }
        setState(() {
          _barcode = barcode;
        });
        _checkCanLaunch();
      });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final List<Widget> children = <Widget>[];
    if (_barcode != null) {
      children.add(Center(
        child: InkWell(
          child: SelectableText(
            _barcode,
            style: TextStyle(color: _textColor, fontSize: 21),
          ),
          onTap: _launchQrCodeScan,
        ),
      ));
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Scanned Barcode'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(12.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: children,
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

  static Future<String> _scan() async {
    try {
      final String barcode = await BarcodeScanner.scan();
      return barcode;
    } on PlatformException catch (e) {
      if (e.code == BarcodeScanner.CameraAccessDenied) {
        return Future<String>.error('Error accessing camera');
      } else {
        return Future<String>.error('Unknown error');
      }
    } on FormatException {
      return Future<String>.error('Error: incorrect format');
    } catch (e) {
      return Future<String>.error('Unknown error');
    }
  }
}
