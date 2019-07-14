import 'dart:async';

import 'package:bloom/notes/models/db/note.dart';
import 'package:flutter/material.dart';

class NoteView extends StatefulWidget {
  const NoteView();

  // final Note note;

  @override
  _NotesState createState() => _NotesState();
}

class _NotesState extends State<NoteView> {
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _bodyController = TextEditingController();
  final FocusNode _titleFocus = FocusNode();
  final FocusNode _bodyFocus = FocusNode();
  // String _initialTitle;
  // String _initialBody;
  Timer _persistenceTimer;
  Color _color;
  Note _note;

  @override
  void initState() {
    _persistenceTimer = Timer.periodic(Duration(seconds: 5), (Timer timer) {
      // call insert query here
      debugPrint('5 seconds passed');
      debugPrint('editable note id: ${_note.id}');
      _persistData();
    });

    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final RouteSettings settings = ModalRoute.of(context).settings;
    _note = settings.arguments;
    _note ??= Note();

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
          backgroundColor: Colors.white,
        ),
        body: _buildBody(context),
      ),
      onWillPop: _readyToPop,
    );
  }

  Widget _buildBody(BuildContext ctx) {
    return Container(
        color: Colors.white,
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
        child: GestureDetector(
            onTap: () => _archiveNote(context),
            child: Icon(
              Icons.archive,
              color: Colors.black,
            ),
          ),
      ),
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        child: InkWell(
          child: GestureDetector(
            // onTap: () => bottomSheet(context),
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
    await _note.archive();
    Scaffold.of(context).showSnackBar(SnackBar(
      content: const Text('Note archived'),
      duration: Duration(seconds: 3),
    ));
        Navigator.of(context).pop();

  }
}
