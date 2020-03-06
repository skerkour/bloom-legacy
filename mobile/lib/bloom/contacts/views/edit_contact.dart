import 'package:bloom/bloom/contacts/blocs/edit_contact.dart';
import 'package:bloom/bloom/contacts/models/contact.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class EditContactView extends StatefulWidget {
  const EditContactView({this.contact});

  final Contact contact;

  @override
  _EditContactState createState() => _EditContactState();
}

class _EditContactState extends State<EditContactView> {
  EditContactBloc _bloc;
  final TextEditingController _firstNameController = TextEditingController();
  final TextEditingController _lastNameController = TextEditingController();
  DateTime _birthday;
  final TextEditingController _notesController = TextEditingController();
  final TextEditingController _phoneController = TextEditingController();
  final TextEditingController _emailController = TextEditingController();

  @override
  void initState() {
    final Contact contact = widget.contact ?? Contact();
    _bloc = EditContactBloc(contact: contact);

    // TODO(z0mbie42): missing fields
    _firstNameController.text = contact.firstName;
    _lastNameController.text = contact.lastName;
    _notesController.text = contact.notes;
    _birthday = contact.birthday;

    if (contact.phones.isNotEmpty) {
      _phoneController.text = contact.phones[0].phone;
    }
    if (contact.emails.isNotEmpty) {
      _emailController.text = contact.emails[0].email;
    }

    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    _firstNameController.dispose();
    _lastNameController.dispose();
    _notesController.dispose();
    _phoneController.dispose();
    _emailController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(_bloc.contact.firstName),
        actions: _buildAppBarActions(context),
        leading: IconButton(
          icon: Icon(Icons.close),
          onPressed: () => Navigator.of(context).pop(),
        ),
      ),
      body: StreamBuilder<Contact>(
        initialData: _bloc.contact,
        stream: _bloc.contactStream,
        builder: (BuildContext context, AsyncSnapshot<Contact> snapshot) {
          if (snapshot.hasData) {
            return _buildBody(snapshot.data);
          } else {
            return const Center(child: CircularProgressIndicator());
          }
        },
      ),
    );
  }

  Widget _buildBody(Contact contact) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(12.0),
        child: Column(
          children: <Widget>[
            TextField(
              decoration: const InputDecoration(labelText: 'First name'),
              controller: _firstNameController,
            ),
            TextField(
              decoration: const InputDecoration(labelText: 'Last name'),
              controller: _lastNameController,
            ),
            _DatePicker(
              labelText: 'Birthday',
              selectedDate: _birthday,
              selectDate: (DateTime date) {
                setState(() {
                  _birthday = date;
                });
              },
            ),
            TextField(
              decoration: const InputDecoration(labelText: 'Phone'),
              controller: _phoneController,
            ),
            TextField(
              decoration: const InputDecoration(labelText: 'Email'),
              controller: _emailController,
            ),
            TextFormField(
              maxLines: 8,
              decoration: const InputDecoration(hintText: 'Notes'),
              controller: _notesController,
            ),
          ],
        ),
      ),
    );
  }

  List<Widget> _buildAppBarActions(BuildContext context) {
    return <Widget>[
      FlatButton(
        onPressed: _onSavePressed,
        child: const Text('Save', style: TextStyle(color: Colors.white)),
      )
    ];
  }

  // TODO(z0mbie42): missing fields
  Future<void> _onSavePressed() async {
    final List<Email> emails = <Email>[];
    final List<Phone> phones = <Phone>[];

    if (_emailController.text.trim().isNotEmpty) {
      emails.add(Email(email: _emailController.text.trim(), label: 'Other'));
    }

    if (_phoneController.text.trim().isNotEmpty) {
      phones.add(Phone(phone: _phoneController.text.trim(), label: 'Other'));
    }
    await _bloc.save(
      firstName: _firstNameController.text,
      lastName: _lastNameController.text,
      notes: _notesController.text,
      deviceId: _bloc.contact.deviceId,
      birthday: _birthday,
      emails: emails,
      phones: phones,
    );
    Navigator.of(context).pop();
  }
}

class _InputDropdown extends StatelessWidget {
  const _InputDropdown({
    Key key,
    this.child,
    this.labelText,
    this.valueText,
    this.valueStyle,
    this.onPressed,
  }) : super(key: key);

  final String labelText;
  final String valueText;
  final TextStyle valueStyle;
  final VoidCallback onPressed;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onPressed,
      child: InputDecorator(
        decoration: InputDecoration(
          labelText: labelText,
        ),
        baseStyle: valueStyle,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          mainAxisSize: MainAxisSize.min,
          children: <Widget>[
            Text(valueText, style: valueStyle),
            Icon(
              Icons.arrow_drop_down,
              color: Theme.of(context).brightness == Brightness.light
                  ? Colors.grey.shade700
                  : Colors.white70,
            ),
          ],
        ),
      ),
    );
  }
}

class _DatePicker extends StatelessWidget {
  const _DatePicker({
    Key key,
    this.labelText,
    this.selectedDate,
    this.selectDate,
  }) : super(key: key);

  final String labelText;
  final DateTime selectedDate;
  final ValueChanged<DateTime> selectDate;

  Future<void> _selectDate(BuildContext context) async {
    final DateTime picked = await showDatePicker(
      context: context,
      initialDate: DateTime.now(),
      firstDate: DateTime(1900, 1),
      lastDate: DateTime(3000),
    );
    if (picked != null && picked != selectedDate) {
      selectDate(picked);
    }
  }

  @override
  Widget build(BuildContext context) {
    final TextStyle valueStyle = Theme.of(context).textTheme.subtitle2;
    return _InputDropdown(
      labelText: labelText,
      valueText:
          selectedDate == null ? '' : DateFormat.yMMMd().format(selectedDate),
      valueStyle: valueStyle,
      onPressed: () {
        _selectDate(context);
      },
    );
  }
}
