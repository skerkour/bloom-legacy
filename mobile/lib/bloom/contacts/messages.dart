import 'models/contact.dart';

class ContactsGuiListContacts {
  ContactsGuiListContacts();

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'contacts.gui.list_contacts',
      'data': <String, dynamic>{},
    };
    return data;
  }
}


class ContactsGuiContacts {
  ContactsGuiContacts({this.contacts});

  final List<Contact> contacts;

  static ContactsGuiContacts fromJson(Map<String, dynamic> json) {
    final List<dynamic> list = json['data']['contacts'];
    final List<Contact> contacts = list.map((dynamic i) => Contact.fromJson(i)).toList();
    return ContactsGuiContacts(contacts: contacts);
  }
}
