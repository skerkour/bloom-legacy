import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';

class BarcodeView extends StatefulWidget {
  const BarcodeView({@required this.barcode});

  final String barcode;

  @override
  _BarcodeState createState() => _BarcodeState();
}

class _BarcodeState extends State<BarcodeView> {
  String _barcode;
  Color _textColor = Colors.black;

  @override
  void initState() {
    _barcode = widget.barcode;
    _checkCanLaunch();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Scanned Barcode'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(12.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Center(
              child: InkWell(
                child: Text(
                  _barcode,
                  style: TextStyle(color: _textColor, fontSize: 21),
                ),
                onTap: _launchBarcode,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _launchBarcode() async {
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
