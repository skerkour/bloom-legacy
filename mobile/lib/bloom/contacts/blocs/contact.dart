import 'dart:async';

import 'package:bloom/bloom/contacts/models/contact.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:flutter/material.dart';

class ContactBloc extends BlocBase {
  ContactBloc({@required Contact contact}) {
    _contact = contact;
  }

  Contact _contact;
  Contact get contact => _contact;

  final StreamController<Contact> _contactController =
      StreamController<Contact>.broadcast();
  // StreamSink<Contact> get _contactStream => _contactController.sink;
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
    _deletedStream.add(_contact);
  }
}
