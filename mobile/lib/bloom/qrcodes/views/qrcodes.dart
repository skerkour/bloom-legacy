import 'package:bloom/bloom/const.dart';
import 'package:bloom/bloom/qrcodes/widgets/bottom_sheet.dart';
import 'package:flutter/material.dart';

class QrCodesView extends StatefulWidget {
  const QrCodesView();

  @override
  _QrCodesState createState() => _QrCodesState();
}

class _QrCodesState extends State<QrCodesView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('QR Codes'),
      ),
      body: _buildBody(),
      floatingActionButton: _buildFloatingActionButton(),
    );
  }

  Widget _buildBody() {
    final List<Code> codes = Code.getCodes();
    codes.sort((Code a, Code b) {
      if (a.starred) {
        return -1;
      }
      if (b.starred) {
        return 1;
      }
      return 0;
    });

    return Container(
      child: ListView.separated(
          separatorBuilder: (BuildContext context, int index) =>
              const Divider(),
          itemCount: codes.length,
          itemBuilder: (BuildContext context, int index) {
            final Code code = codes[index];

            return ListTile(
              leading: CircleAvatar(
                backgroundImage: const AssetImage(ICON_QRCODES_256),
                backgroundColor: Colors.transparent,
                radius: 28,
              ),
              title: Text(code.name),
              subtitle: Text(code.data),
              trailing: code.starred ? Icon(Icons.star) : null,
            );
          }),
    );
  }

  FloatingActionButton _buildFloatingActionButton() {
    return FloatingActionButton(
      onPressed: () => showModalBottomSheet<dynamic>(
        context: context,
        builder: (BuildContext bc) {
          return QrCodesBottomSheet();
        },
      ),
      child: Icon(Icons.add),
      backgroundColor: Colors.red,
    );
  }
}

class Code {
  Code({this.name, this.data, this.starred = false});

  String name;
  String data;
  bool starred;

  static List<Code> getCodes() {
    return <Code>[
      Code(
        name: 'My super qrcode',
        data: 'https://bloom.sh',
      ),
      Code(
        name: 'My Wifi password',
        data: 'my super password',
        starred: true,
      ),
    ];
  }
}
