import 'dart:async';

import 'package:bloom/core/contacts/models.dart';
import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
import 'package:flutter/material.dart';
// import 'package:contacts_service/contacts_service.dart' as contacts_service;

class ContactBloc extends BlocBase {
  ContactBloc({@required Contact contact}) {
    _contact = contact;
  }

  Contact _contact;
  Contact get contact => _contact;

  final StreamController<Contact> _contactController =
      StreamController<Contact>.broadcast();
  Stream<Contact> get contactStream => _contactController.stream;

  final StreamController<Contact> _contactDeletedController =
      StreamController<Contact>.broadcast();
  StreamSink<Contact> get _deletedStream => _contactDeletedController.sink;
  Stream<Contact> get deletedStream => _contactDeletedController.stream;

  @override
  void dispose() {
    _contactController.close();
    _contactDeletedController.close();
  }

  Future<void> delete() async {
    await contact.delete();
    // try {
    //   await contacts_service.ContactsService.deleteContact(
    //       _contact.toDeviceContact());
    // } catch (err) {
    //   debugPrint('Error deleting device contact: $err');
    // }
    _deletedStream.add(_contact);
  }
}
