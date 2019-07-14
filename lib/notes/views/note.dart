import 'dart:async';

import 'package:bloom/notes/models/db/note.dart';
import 'package:bloom/notes/widgets/bottom_sheet.dart';
import 'package:flutter/material.dart';
import 'package:share/share.dart';

class NoteView extends StatefulWidget {
  const NoteView();

  // final Note note;

  @override
  _NoteState createState() => _NoteState();
}

class _NoteState extends State<NoteView> {
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _bodyController = TextEditingController();
  final FocusNode _titleFocus = FocusNode();
  final FocusNode _bodyFocus = FocusNode();
  // String _initialTitle;
  // String _initialBody;
  Timer _persistenceTimer;
  Color _color;
  Note _note;
  BuildContext _scaffoldContext;

  @override
  void initState() {
    _persistenceTimer =
        Timer.periodic(const Duration(seconds: 5), (Timer timer) {
      // call insert query here
      debugPrint('5 seconds passed');
      debugPrint('editable note id: ${_note.id}');
      _persistData();
    });

    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final RouteSettings routeSettings = ModalRoute.of(context).settings;
    _note = routeSettings.arguments;
    _note ??= Note();

    debugPrint('new note: $_note');

    _titleController.text = _note.title;
    _bodyController.text = _note.body;
    _color = _note.color;
    debugPrint('color: $_color');

    return WillPopScope(
      child: Scaffold(
        appBar: AppBar(
          brightness: Brightness.light,
          leading: BackButton(
            color: Colors.black,
          ),
          actions: _buildAppBarActions(context),
          elevation: 1,
          backgroundColor: _note.color,
        ),
        body: Builder(builder: (BuildContext context) {
          _scaffoldContext = context;
          return _buildBody(context);
        }),
      ),
      onWillPop: _readyToPop,
    );
  }

  Widget _buildBody(BuildContext ctx) {
    return Container(
        color: _note.color,
        padding: const EdgeInsets.only(left: 16, right: 16, top: 12),
        child: SafeArea(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              Container(
                padding: const EdgeInsets.all(5),
                child: EditableText(
                  maxLines: null,
                  controller: _titleController,
                  focusNode: _titleFocus,
                  style: TextStyle(
                    color: Colors.black,
                    fontSize: 22,
                    fontWeight: FontWeight.bold,
                  ),
                  cursorColor: Colors.blue,
                  backgroundCursorColor: Colors.blue,
                ),
              ),
              Divider(color: Colors.grey),
              Expanded(
                child: Container(
                  padding: const EdgeInsets.all(5),
                  child: EditableText(
                    maxLines: null,
                    controller: _bodyController,
                    focusNode: _bodyFocus,
                    style: TextStyle(color: Colors.black, fontSize: 20),
                    backgroundCursorColor: Colors.red,
                    cursorColor: Colors.blue,
                  ),
                ),
              ),
            ],
          ),
          left: true,
          right: true,
          top: false,
          bottom: true,
        ));
  }

  List<Widget> _buildAppBarActions(BuildContext context) {
    final List<Widget> actions = <Widget>[
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        child: InkWell(
          child: GestureDetector(
            child:
                Container(child: Image.asset('assets/pin_36.png'), width: 20),
          ),
        ),
      ),
      Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: _note.archivedAt == null
              ? GestureDetector(
                  onTap: () => _archiveNote(context),
                  child: Icon(
                    Icons.archive,
                    color: Colors.black,
                  ),
                )
              : GestureDetector(
                  onTap: () => _unarchiveNote(context),
                  child: Icon(
                    Icons.unarchive,
                    color: Colors.black,
                  ),
                )),
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        child: InkWell(
          child: GestureDetector(
            onTap: () => _bottomSheet(context),
            child: Icon(
              Icons.more_vert,
              color: Colors.black,
            ),
          ),
        ),
      )
    ];
    return actions;
  }

  void _bottomSheet(BuildContext context) {
    showModalBottomSheet<MoreOptionsSheet>(
      context: context,
      builder: (BuildContext ctx) {
        return MoreOptionsSheet(
          color: _note.color,
          onColorChanged: _changeColor,
          onDeleted: _onDeleted,
          onShared: _onShared,
          // callBackOptionTapped: bottomSheetOptionTappedHandler,
          updatedAt: _note.updatedAt,
        );
      },
    );
  }

  Future<void> _persistData() async {
    _note.body = _bodyController.text;
    _note.title = _titleController.text;
    _note.color = _color;

    if (_note.id == null) {
      if (_note.title.isEmpty && _note.body.isEmpty) {
        debugPrint('note is empty, aborting');
        return;
      }
      _note = await Note.create(_note.title, _note.body, _note.color);
      debugPrint('note created');
    } else {
      _note = await _note.update();
      debugPrint('note updated');
    }
  }

  Future<bool> _readyToPop() async {
    _persistenceTimer.cancel();
    await _persistData();
    return true;
  }

  Future<void> _archiveNote(BuildContext context) async {
    _note = await _note.archive();
    _persistenceTimer.cancel();
    Scaffold.of(_scaffoldContext).showSnackBar(const SnackBar(
      content: Text('Note archived'),
      duration: Duration(seconds: 3),
    ));
    setState(() {});
    Navigator.of(context).pop();
  }

  Future<void> _unarchiveNote(BuildContext context) async {
    _note = await _note.unarchive();
    _persistenceTimer.cancel();
    Scaffold.of(_scaffoldContext).showSnackBar(const SnackBar(
      content: Text('Note unarchived'),
      duration: Duration(seconds: 3),
    ));
    setState(() {});
    Navigator.of(context).pop();
  }

  void _changeColor(Color newColorSelected) {
    debugPrint('color changed');
    setState(() {
      _color = newColorSelected;
      _note.color = newColorSelected;
    });
    _persistData();
  }

  Future<void> _onDeleted() async {
    if (_note.id != null) {
      await _note.delete();
    }
    Navigator.of(context).pop();
  }

  void _onShared() {
    Share.share("${_note.title}\n${_note.body}");
  }
}
