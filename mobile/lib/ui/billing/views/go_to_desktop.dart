import 'package:bloom/ui/const.dart';
import 'package:flutter/material.dart';

class BillingGoToDesktopView extends StatefulWidget {
  const BillingGoToDesktopView();

  @override
  _BillingGoToDesktopState createState() => _BillingGoToDesktopState();
}

class _BillingGoToDesktopState extends State<BillingGoToDesktopView> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Billing'),
      ),
      body: _buildBody(),
    );
  }

  Container _buildBody() {
    return Container(
      child: const Center(
        child: Text('Billing is not available on mobile yet.'),
      ),
    );
  }
}
