import 'package:flutter/material.dart';

class DropdownItem {
  const DropdownItem({@required this.label, @required this.value, this.icon});

  final String label;
  final String value;
  final IconData icon;
}
