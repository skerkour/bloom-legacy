import 'dart:async';

import 'package:bloom/bloom/contacts/models/contact.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:flutter/material.dart';

class EditContactBloc extends BlocBase {
  EditContactBloc({@required Contact contact}) {
    _contact = contact;
  }

  Contact _contact;
  Contact get contact => _contact;

  final StreamController<Contact> _contactController =
      StreamController<Contact>.broadcast();
  StreamSink<Contact> get _contactStream => _contactController.sink;
  Stream<Contact> get contactStream => _contactController.stream;

  @override
  void dispose() {
    _contactController.close();
  }

  // TODO(z0mbie42): missing fields
  Future<void> save({
    @required String firstName,
    @required String lastName,
    @required String notes,
    @required String deviceId,
    @required DateTime birthday,
  }) async {
    _contact.firstName = firstName;
    _contact.lastName = lastName;
    _contact.notes = notes;
    _contact.deviceId = deviceId;
    _contact.birthday = birthday;

    if (_contact.id == null) {
      if (_contact.firstName.isEmpty &&
          _contact.lastName.isEmpty &&
          _contact.notes.isEmpty) {
        debugPrint('contact is empty, aborting');
        return;
      }
      _contact = await _contact.create();
      debugPrint('note created');
    } else {
      _contact = await _contact.update();
      debugPrint('note updated');
    }
    _contactStream.add(_contact);
  }
}
