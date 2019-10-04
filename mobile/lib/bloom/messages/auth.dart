import 'package:flutter/material.dart';

class AuthGuiRegistrationStart {
  AuthGuiRegistrationStart({
    @required this.displayName,
    @required this.email,
    @required this.password,
  });

  final String displayName;
  final String email;
  final String password;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'auth.gui.registration_start',
      'data': <String, dynamic>{
        'display_name': displayName,
        'email': email,
        'password': password,
      },
    };
    return data;
  }
}
