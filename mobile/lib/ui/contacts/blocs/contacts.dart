import 'dart:async';

import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
import 'package:bloom/core/contacts/models.dart';
import 'package:contacts_service/contacts_service.dart' as contacts_service;
import 'package:flutter/foundation.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:shared_preferences/shared_preferences.dart';

class ContactsBloc extends BlocBase {
  ContactsBloc();

  final StreamController<List<Contact>> _contactsController =
      StreamController<List<Contact>>.broadcast();
  Stream<List<Contact>> get contactsStream => _contactsController.stream;

  @override
  void dispose() {
    _contactsController.close();
  }

  Future<void> listContacts() async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    final bool isFirstLaunch =
        prefs.getBool('contacts_is_first_launch') ?? true;

    if (isFirstLaunch) {
      await importContacts();
    }

    _contactsController.sink.add(await Contact.list());
  }

  Future<int> importContacts() async {
    int importedCount = 0;
    final SharedPreferences prefs = await SharedPreferences.getInstance();

    // check permission
    final PermissionStatus permissionStatus = await _getContactsPermission();

    // import contacts from device
    if (permissionStatus == PermissionStatus.granted) {
      final Set<String> existingsContactsDeviceIds =
          await Contact.findDeviceIds();

      final Iterable<contacts_service.Contact> contacts =
          await contacts_service.ContactsService.getContacts();

      for (contacts_service.Contact deviceContact in contacts) {
        if (!existingsContactsDeviceIds.contains(deviceContact.identifier)) {
          importedCount += 1;
          final Contact importedConatct =
              Contact.fromDeviceContact(deviceContact);
          await importedConatct.create();
        }
      }
    } else {
      debugPrint('contacts: contacts permission not granted');
    }

    await prefs.setBool('contacts_is_first_launch', false);

    // find contacts in DB
    _contactsController.sink.add(await Contact.list());
    return importedCount;
  }

  Future<PermissionStatus> _getContactsPermission() async {
    final PermissionStatus permission = await PermissionHandler()
        .checkPermissionStatus(PermissionGroup.contacts);
    if (permission != PermissionStatus.granted &&
        permission != PermissionStatus.neverAskAgain) {
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
