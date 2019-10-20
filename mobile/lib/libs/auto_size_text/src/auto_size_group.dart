part of auto_size_text;

/// Controller to synchronize the fontSize of multiple AutoSizeTexts.
class AutoSizeGroup {
  final Map<_AutoSizeTextState, double> _listeners =
      <_AutoSizeTextState, double>{};
  bool _widgetsNotified = false;
  double _fontSize = double.infinity;

  void _register(_AutoSizeTextState text) {
    _listeners[text] = double.infinity;
  }

  void _updateFontSize(_AutoSizeTextState text, double maxFontSize) {
    final double oldFontSize = _fontSize;
    if (maxFontSize <= _fontSize) {
      _fontSize = maxFontSize;
      _listeners[text] = maxFontSize;
    } else if (_listeners[text] == _fontSize) {
      _listeners[text] = maxFontSize;
      _fontSize = double.infinity;
      for (double size in _listeners.values) {
        if (size < _fontSize) {
          _fontSize = size;
        }
      }
    } else {
      _listeners[text] = maxFontSize;
    }

    if (oldFontSize != _fontSize) {
      _widgetsNotified = false;
      Timer.run(_notifyListeners);
    }
  }

  void _notifyListeners() {
    if (_widgetsNotified) {
      return;
    } else {
      _widgetsNotified = true;
    }

    for (_AutoSizeTextState text in _listeners.keys) {
      if (text.mounted) {
        text._notifySync();
      }
    }
  }

  void _remove(_AutoSizeTextState text) {
    _updateFontSize(text, double.infinity);
    _listeners.remove(text);
  }
}
