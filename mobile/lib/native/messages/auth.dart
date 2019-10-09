import 'package:flutter/material.dart';

class AuthRegistrationStart {
  AuthRegistrationStart({
    @required this.displayName,
    @required this.email,
  });

  final String displayName;
  final String email;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'auth.registration_start',
      'data': <String, dynamic>{
        'display_name': displayName,
        'email': email,
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
    @required this.password,
  });

  final String id;
  final String username;
  final String password;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'auth.gui.registration_complete',
      'data': <String, dynamic>{
        'id': id,
        'username': username,
        'password': password,
      },
    };
    return data;
  }
}

class AuthGuiSignIn {
  AuthGuiSignIn({
    @required this.username,
    @required this.password,
  });

  final String username;
  final String password;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'auth.gui.sign_in',
      'data': <String, dynamic>{
        'username': username,
        'password': password,
      },
    };
    return data;
  }
}
