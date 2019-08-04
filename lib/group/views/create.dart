import 'package:flutter/material.dart';

class CreateGroupView extends StatefulWidget {
  const CreateGroupView();

  @override
  _CreateGroupState createState() => _CreateGroupState();
}

class _CreateGroupState extends State<CreateGroupView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(appBar: _buildAppBar(), body: _buildBody());
  }

  Padding _buildBody() {
    return Padding(
      padding: const EdgeInsets.all(21),
      child: Column(
        children: <Widget>[
          Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: <Widget>[
              CircleAvatar(
                child: Icon(Icons.person),
                backgroundColor: Colors.blue,
                radius: 32,
              ),
              const SizedBox(width: 21),
              Expanded(
                child: TextFormField(
                  decoration: InputDecoration(labelText: 'Group name'),
                  autofocus: true,
                ),
              ),
            ],
          ),
          const SizedBox(height: 10),
          TextFormField(
            decoration: InputDecoration(
              labelText: 'Add members',
            ),
            keyboardType: TextInputType.multiline,
            maxLines: 4,
          ),
        ],
      ),
    );
  }

  AppBar _buildAppBar() {
    return AppBar(
      title: const Text('New group'),
      actions: <Widget>[
        IconButton(
          icon: Icon(Icons.check),
          onPressed: _validatePressed,
        ),
      ],
    );
  }

  void _validatePressed() {
    debugPrint('Validate button pressed');
    Navigator.of(context).pop();
  }
}
