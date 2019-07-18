import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';
import 'package:contacts_service/contacts_service.dart';
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';

class ContactsBloc extends BlocBase {
  ContactsBloc();

  final StreamController<List<Contact>> _contactsController =
      StreamController<List<Contact>>.broadcast();
  Stream<List<Contact>> get contactsOut => _contactsController.stream;

  @override
  void dispose() {
    _contactsController.close();
  }

  Future<void> getContacts() async {
    final PermissionStatus permissionStatus = await _getContactsPermission();
    if (permissionStatus == PermissionStatus.granted) {
      final Iterable<Contact> contacts = await ContactsService.getContacts();
      _contactsController.sink.add(contacts.toList());
    } else {
      debugPrint('contacts permission not granted');
    }
  }

  Future<PermissionStatus> _getContactsPermission() async {
    final PermissionStatus permission = await PermissionHandler()
        .checkPermissionStatus(PermissionGroup.contacts);
    if (permission != PermissionStatus.granted &&
        permission != PermissionStatus.disabled) {
      final Map<PermissionGroup, PermissionStatus> permissionStatus =
          await PermissionHandler()
              .requestPermissions(<PermissionGroup>[PermissionGroup.contacts]);
      return permissionStatus[PermissionGroup.contacts] ??
          PermissionStatus.unknown;
    } else {
      return permission;
    }
  }
}
