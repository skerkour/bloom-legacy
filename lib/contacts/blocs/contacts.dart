import 'dart:async';

import 'package:bloom/kernel/blocs/bloc_provider.dart';
import 'package:contacts_service/contacts_service.dart' as contacts_service;
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:bloom/contacts/models/db/contact.dart';

class ContactsBloc extends BlocBase {
  ContactsBloc();

  final StreamController<List<Contact>> _contactsController =
      StreamController<List<Contact>>.broadcast();
  Stream<List<Contact>> get contactsStream => _contactsController.stream;

  @override
  void dispose() {
    _contactsController.close();
  }

  Future<void> getContacts() async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    final bool isFirstLaunch =
        prefs.getBool('contacts_is_first_launch') ?? true;

    if (isFirstLaunch) {
      // check permission
      final PermissionStatus permissionStatus = await _getContactsPermission();
      // import contacts from device

      if (permissionStatus == PermissionStatus.granted) {
        final Iterable<contacts_service.Contact> contacts =
            await contacts_service.ContactsService.getContacts();
        // _contactsController.sink.add(contacts.toList());
        for (contacts_service.Contact contact in contacts) {
          await Contact.create(contact.displayName, contact.identifier);
        }
      } else {
        debugPrint('contacts: contacts permission not granted');
      }

      await prefs.setBool('contacts_is_first_launch', false);
    }

    // find contacts in DB
    _contactsController.sink.add(await Contact.find());
  }

  Future<void> importContacts() async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();

      // check permission
      final PermissionStatus permissionStatus = await _getContactsPermission();
      // import contacts from device

      if (permissionStatus == PermissionStatus.granted) {
        final Iterable<contacts_service.Contact> contacts =
            await contacts_service.ContactsService.getContacts();
        // _contactsController.sink.add(contacts.toList());
        for (contacts_service.Contact contact in contacts) {
          await Contact.create(contact.displayName, contact.identifier);
        }
      } else {
        debugPrint('contacts: contacts permission not granted');
      }

      await prefs.setBool('contacts_is_first_launch', false);

    // find contacts in DB
    _contactsController.sink.add(await Contact.find());
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
