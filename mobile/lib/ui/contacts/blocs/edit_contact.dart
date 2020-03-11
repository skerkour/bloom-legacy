import 'dart:async';

import 'package:bloom/core/contacts/models.dart';
import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
// import 'package:contacts_service/contacts_service.dart' as contacts_service;
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
    @required List<Email> emails,
    @required List<Phone> phones,
  }) async {
    _contact.firstName = firstName;
    _contact.lastName = lastName;
    _contact.notes = notes;
    _contact.deviceId = deviceId;
    _contact.birthday = birthday;
    _contact.emails = emails;
    _contact.phones = phones;

    if (_contact.id == null) {
      if (_contact.firstName.isEmpty &&
          _contact.lastName.isEmpty &&
          _contact.notes.isEmpty) {
        debugPrint('contact is empty, aborting');
        return;
      }

      _contact = await _contact.create();
      // try {
      //   await contacts_service.ContactsService.addContact(
      //       _contact.toDeviceContact());
      // } catch (err) {
      //   debugPrint('Error creating device contact: $err');
      // }

      debugPrint('contact created');
    } else {
      _contact = await _contact.update();
      // try {
      //   await contacts_service.ContactsService.updateContact(
      //       _contact.toDeviceContact());
      // } catch (err) {
      //   debugPrint('Error updating device contact: $err');
      // }
      debugPrint('contact updated');
    }
    _contactStream.add(_contact);
  }
}
