import 'package:bloom/ui/kernel/widgets/dropdown_item.dart';
import 'package:flutter/material.dart';

class CreateGroupView extends StatefulWidget {
  const CreateGroupView();

  @override
  _CreateGroupState createState() => _CreateGroupState();
}

class _CreateGroupState extends State<CreateGroupView> {
  String _dropdownValue;
  List<DropdownMenuItem<String>> _dropdownItems;
  static const String _PUBLIC = 'PUBLIC';
  static const String _PRIVATE = 'PRIVATE';

  @override
  void initState() {
    _dropdownValue = _PRIVATE;
    _dropdownItems = <DropdownItem>[
      const DropdownItem(label: 'Private', value: _PRIVATE, icon: Icons.lock),
      const DropdownItem(label: 'Public', value: _PUBLIC, icon: Icons.public),
    ].map<DropdownMenuItem<String>>((DropdownItem item) {
      return DropdownMenuItem<String>(
        value: item.value,
        child: Row(
          children: <Widget>[
            item.icon != null ? Icon(item.icon) : null,
            const SizedBox(width: 5),
            Text(item.label),
          ],
        ),
      );
    }).toList();

    super.initState();
  }

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
              const CircleAvatar(
                child: Icon(Icons.person),
                backgroundColor: Colors.blue,
                radius: 32,
              ),
              const SizedBox(width: 21),
              Expanded(
                child: TextFormField(
                  decoration: const InputDecoration(labelText: 'Group name'),
                  autofocus: true,
                ),
              ),
            ],
          ),
          const SizedBox(height: 10),
          TextFormField(
            decoration: const InputDecoration(
              labelText: 'Add members',
            ),
            keyboardType: TextInputType.multiline,
            maxLines: 4,
          ),
          const SizedBox(height: 10),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: <Widget>[
              DropdownButton<String>(
                value: _dropdownValue,
                onChanged: (String newValue) {
                  setState(() {
                    _dropdownValue = newValue;
                  });
                },
                items: _dropdownItems,
              ),
            ],
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
