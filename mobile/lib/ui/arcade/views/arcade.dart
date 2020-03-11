import 'package:bloom/ui/arcade/views/2048.dart';
import 'package:flutter/material.dart';

class ArcadeView extends StatefulWidget {
  const ArcadeView();

  @override
  _ArcadeState createState() => _ArcadeState();
}

class _ArcadeState extends State<ArcadeView> {
  static const String _2048Icon = 'assets/icons/2048.png';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Arcade'),
      ),
      body: _builBody(),
    );
  }

  Widget _builBody() {
    return Column(
      children: <Widget>[
        const SizedBox(height: 10),
        Center(
          child: GestureDetector(
            child: Image.asset(_2048Icon, height: 128, width: 128),
            onTap: () {
              Navigator.push<dynamic>(
                context,
                MaterialPageRoute<dynamic>(
                    builder: (BuildContext context) => A2048View()),
              );
            },
          ),
        ),
      ],
    );
  }
}
