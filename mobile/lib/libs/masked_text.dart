library masked_text;

import 'package:flutter/material.dart';

class MaskedTextField extends StatefulWidget {
  const MaskedTextField({
    this.mask,
    this.escapeCharacter = 'x',
    // this.maskedTextFieldController,
    this.maxLength = 100,
    this.keyboardType = TextInputType.text,
    this.decoration = const InputDecoration(),
    this.focusNode,
    this.onChange,
    this.controller,
  });

  // final TextEditingController maskedTextFieldController;

  final String mask;
  final String escapeCharacter;

  final int maxLength;
  final TextInputType keyboardType;
  final InputDecoration decoration;
  final FocusNode focusNode;
  final TextEditingController controller;

  final ValueChanged<String> onChange;

  @override
  State<StatefulWidget> createState() => _MaskedTextFieldState();
}

class _MaskedTextFieldState extends State<MaskedTextField> {
  @override
  Widget build(BuildContext context) {
    int lastTextSize = 0;

    return TextField(
      // controller: widget.maskedTextFieldController,
      maxLength: widget.maxLength,
      keyboardType: widget.keyboardType,
      decoration: widget.decoration,
      focusNode: widget.focusNode,
      controller: widget.controller,
      onChanged: (String text) {
        // its deleting text
        if (text.length < lastTextSize) {
          if (widget.mask[text.length] != widget.escapeCharacter) {
            widget.controller.selection = TextSelection.fromPosition(
                TextPosition(offset: widget.controller.text.length));
          }
        } else {
          // its typing
          if (text.length >= lastTextSize) {
            final int position = text.length;

            if ((widget.mask[position - 1] != widget.escapeCharacter) &&
                (text[position - 1] != widget.mask[position - 1])) {
              widget.controller.text = _buildText(text);
            }

            if (widget.mask[position] != widget.escapeCharacter)
              widget.controller.text =
                  '${widget.controller.text}${widget.mask[position]}';
          }

          // Android on change resets cursor position(cursor goes to 0)
          // so you have to check if it reset, then put in the end(just on android)
          // as IOS bugs if you simply put it in the end
          if (widget.controller.selection.start <
              widget.controller.text.length) {
            widget.controller.selection = TextSelection.fromPosition(
                TextPosition(offset: widget.controller.text.length));
          }
        }

        // update cursor position
        lastTextSize = widget.controller.text.length;

        if (widget.onChange != null) {
          widget.onChange(widget.controller.text);
        }
      },
    );
  }

  String _buildText(String text) {
    String result = '';

    for (int i = 0; i < text.length - 1; i++) {
      result += text[i];
    }

    result += widget.mask[text.length - 1];
    result += text[text.length - 1];

    return result;
  }
}
