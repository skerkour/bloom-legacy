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

class AuthGuiRegistrationStarted {
  AuthGuiRegistrationStarted({
    @required this.id,
  });

  final String id;

  static AuthGuiRegistrationStarted fromJson(Map<String, dynamic> json) {
    return AuthGuiRegistrationStarted(id: json['data']['id']);
  }
}

class AuthGuiRegistrationVerify {
  AuthGuiRegistrationVerify({
    @required this.id,
    @required this.code,
  });

  final String id;
  final String code;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'auth.registration_verify',
      'data': <String, dynamic>{
        'id': id,
        'code': code,
      },
    };
    return data;
  }
}

class AuthGuiRegistrationComplete {
  AuthGuiRegistrationComplete({
    @required this.id,
    @required this.username,
  });

  final String id;
  final String username;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'auth.registration_complete',
      'data': <String, dynamic>{
        'id': id,
        'username': username,
      },
    };
    return data;
  }
}
